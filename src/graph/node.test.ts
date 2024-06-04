import { ReplaySubject } from 'rxjs';
import { Node, port, port2 } from './node.ts';

const waitForPopulation = async (delay: number) => {
  await new Promise(r => setTimeout(r, delay))
}

const createConstantNode = () => new Node(port("x", "number"), "number", (x: number) => x);
const createSumNode = () => new Node(port2("x", "number", "y", "number"), "number", (x: number, y: number) => x + y);
const createRepeatNode = () => new Node(port2("c", "string", "n", "number"), "string", (c: string, n: number) => c.repeat(n));

describe('Node functionality', () => {

  it("should not populate output stream if no input is given", async () => {
    //const constantNode = new Node((x: number) => x, port("x", "number"), "number");
    const constantNode = createConstantNode()
    const outSub = jest.fn()
    constantNode.outputStream$.subscribe(outSub) //listen to output to see if it's called

    await waitForPopulation(50) // wait to make sure everything has run
    expect(constantNode.currentValue).toBeUndefined()
    expect(outSub).toHaveBeenCalledTimes(0)
  });

  it("should populate output stream if input is given", () => {
    //const constantNode = new Node((x: number) => x, port("x", "number"), "number");
    const constantNode = createConstantNode()
    const start_value = 7
    const outSub = jest.fn((v) => {
      expect(v).toEqual(start_value)
    })

    constantNode.outputStream$.subscribe(outSub);
    constantNode.getInputStream("x").next(start_value);

    expect(constantNode.currentValue).toEqual(start_value)
    expect(outSub).toHaveBeenCalledTimes(1)
  });

  it("should populate output stream equal to the number times input is supplied", () => {
    const constantNode = createConstantNode()
    const values = [7, 9, 11]
    const input$ = constantNode.getInputStream("x")
    const output$ = constantNode.outputStream$
    const outSub = jest.fn((v) => v)

    output$.subscribe(outSub);
    input$.next(values[0]);
    input$.next(values[1]);
    input$.next(values[2]);

    //await waitForPopulation(50)
    expect(constantNode.currentValue).toEqual(values[2])
    expect(outSub).toHaveBeenCalledTimes(3)
    expect(outSub).toHaveNthReturnedWith(1, values[0])
    expect(outSub).toHaveNthReturnedWith(2, values[1])
    expect(outSub).toHaveNthReturnedWith(3, values[2])
  })
  it("node w/ multiple inputs should not fire if all inputs are not supplied", () => {
    const sumNode = createSumNode()
    const input1$ = sumNode.getInputStream("x")
    const output$ = sumNode.outputStream$
    const outSub = jest.fn((v) => v)

    output$.subscribe(outSub);
    input1$.next(7);

    //await waitForPopulation(50)
    expect(sumNode.currentValue).toEqual(undefined)
    expect(outSub).toHaveBeenCalledTimes(0)
  })

  it("multiple inputs should fire when all inputs are supplied, or changed", () => {
    const sumNode = createSumNode()
    const input1$ = sumNode.getInputStream("x")!
    const input2$ = sumNode.getInputStream("y")!
    const output$ = sumNode.outputStream$
    const outSub = jest.fn((v) => v)

    output$.subscribe(outSub);

    input1$.next(999);
    expect(sumNode.currentValue).toBeUndefined()
    expect(outSub).toHaveBeenCalledTimes(0)

    input1$.next(7);
    expect(sumNode.currentValue).toBeUndefined()
    expect(outSub).toHaveBeenCalledTimes(0)

    input2$.next(9);
    expect(sumNode.currentValue).toEqual(16)
    expect(outSub).toHaveNthReturnedWith(1, 16)
    expect(outSub).toHaveBeenCalledTimes(1)

    input2$.next(11);
    expect(sumNode.currentValue).toEqual(18)
    expect(outSub).toHaveNthReturnedWith(2, 18)
    expect(outSub).toHaveBeenCalledTimes(2)

    input2$.next(13);
    expect(sumNode.currentValue).toEqual(20)
    expect(outSub).toHaveNthReturnedWith(3, 20)
    expect(outSub).toHaveBeenCalledTimes(3)

  })
  it("nodes can have different data types", () => {

    //@ts-expect-error - Swapped input types
    new Node(port2("c", "number", "n", "string"), "string", (c: string, n: number) => c.repeat(n));

    //@ts-expect-error - Incorrect output
    new Node(port2("c", "string", "n", "number"), "number", (c: string, n: number) => c.repeat(n));

    const repeatNode = createRepeatNode()
    const input1$ = repeatNode.getInputStream("c")
    const input2$ = repeatNode.getInputStream("n")
    const output$ = repeatNode.outputStream$
    const outSub = jest.fn((v) => v)
    output$.subscribe(outSub);

    input1$.next("a");
    input2$.next(5);
    expect(repeatNode.currentValue).toEqual("aaaaa")
    expect(outSub).toHaveNthReturnedWith(1, "aaaaa")
    expect(outSub).toHaveBeenCalledTimes(1)
  })

  it("should restrict what accessors can be used to get data types", () => {
    const sumNode = createSumNode()

    try {
      //@ts-expect-error
      sumNode.getInputType("a") // not one of the defined inputs!
    } catch { }

    sumNode.getInputType("x") // valid
    sumNode.getInputType("y") // valid
  })


  it("should restrict what accessors can be used when multiple types are input types are defined", () => {
    const repeatNode = createRepeatNode()
    try {
      //@ts-expect-error
      repeatNode.getInputType("a") // not one of the defined inputs!
    } catch { }

    repeatNode.getInputType("c") // valid
    repeatNode.getInputType("n") // valid
  })
  it("should restrict what getInputType returns", () => {
    const sumNode = createSumNode()

    try {
      //return types are correctly inferred
      //@ts-expect-error
      sumNode.getInputType("y") == "string"
      //@ts-expect-error
      sumNode.getInputType("x") == "string"
    } catch { }

    //@ts-expect-no-error
    sumNode.getInputType("x") == "number"
    //@ts-expect-no-error
    sumNode.getInputType("y") == "number"

    expect(sumNode.getInputType("x")).toEqual("number")
    expect(sumNode.getInputType("y")).toEqual("number")
  })

  it("multiple inputs types should still restrict what getInputType returns", () => {
    const repeatNode = createRepeatNode()

    try {
      //@ts-expect-error
      repeatNode.getInputType("c") == "boolean" // none of the inputs
      //@ts-expect-error
      repeatNode.getInputType("c") == "number" // wrong type for input
      //@ts-expect-error
      repeatNode.getInputType("n") == "string"
    } catch { }

    //@ts-expect-no-error
    repeatNode.getInputType("c") == "string"
    //@ts-expect-no-error
    repeatNode.getInputType("n") == "number"
  })
  it("should restrict what accessors can be used to inputStreams", () => {
    const sumNode = createSumNode()
    const repeatNode = createRepeatNode()

    try {
      //@ts-expect-error
      sumNode.getInputStream("a")
      //@ts-expect-error
      repeatNode.getInputStream("a")
    } catch { }

    //@ts-expect-no-error
    sumNode.getInputStream("x")
    //@ts-expect-no-error
    sumNode.getInputStream("y")
    //@ts-expect-no-error
    repeatNode.getInputStream("c")
    //@ts-expect-no-error
    repeatNode.getInputStream("n")
  })
  it("should restrict what getInputStream returns", () => {
    const sumNode = createSumNode()
    //return types are correctly inferred

    try {
      //@ts-expect-error
      sumNode.getInputStream("y") == new ReplaySubject<string>(1)
      //@ts-expect-error
      sumNode.getInputStream("x") == new ReplaySubject<boolean>(1)
    } catch { }

    //@ts-expect-no-error
    sumNode.getInputStream("x") == new ReplaySubject<number>
    //@ts-expect-no-error
    sumNode.getInputStream("y") == new ReplaySubject<number>

    expect(sumNode.getInputStream("x")).toEqual(new ReplaySubject<number>(1))//not exhastive, this will always match other types
    expect(sumNode.getInputStream("y")).toEqual(new ReplaySubject<number>(1))//not exhastive, this will always match other types
  })
  it("multiple inputs types should still restrict what getInputType returns", () => {
    const repeatNode = createRepeatNode()
    try {
      //@ts-expect-error
      repeatNode.getInputStream("c") == new ReplaySubject<number>(1)
      //@ts-expect-error
      repeatNode.getInputStream("n") == new ReplaySubject<string>(1)
    } catch { }

    //@ts-expect-no-error
    repeatNode.getInputStream("c") == new ReplaySubject<string>
    //@ts-expect-no-error
    repeatNode.getInputStream("n") == new ReplaySubject<number>

    expect(repeatNode.getInputStream("c")).toEqual(new ReplaySubject<string>(1)) //not exhastive, this will always match other types
    expect(repeatNode.getInputStream("n")).toEqual(new ReplaySubject<number>(1)) //not exhastive, this will always match other types
  })
})

