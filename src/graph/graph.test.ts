import { Node, Graph } from './graph.ts';

const waitForPopulation = async (delay: number) => {
  await new Promise(r => setTimeout(r, delay))
}

describe('Node functionality', () => {

  it("should not populate output stream if no input is given", async () => {
    const constantNode = new Node((x: number) => x, { x: "number" }, "number");
    const outSub = jest.fn()
    constantNode.outputStream$.subscribe(outSub) //listen to output to see if it's called

    await waitForPopulation(50) // wait to make sure everything has run
    expect(constantNode.currentValue).toBeUndefined()
    expect(outSub).toHaveBeenCalledTimes(0)
  });

  it("should populate output stream if input is given", () => {
    const constantNode = new Node((x: number) => x, { x: "number" }, "number");
    const start_value = 7
    const outSub = jest.fn((v) => {
      expect(v).toEqual(start_value)
    })

    constantNode.outputStream$.subscribe(outSub);
    constantNode.inputStreams.get("x")!.next(start_value);

    expect(constantNode.currentValue).toEqual(start_value)
    expect(outSub).toHaveBeenCalledTimes(1)
  });

  it("should populate output stream equal to the number times input is supplied", () => {
    const constantNode = new Node((x: number) => x, { x: "number" }, "number");
    const values = [7, 9, 11]
    const input$ = constantNode.inputStreams.get("x")!
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
    const sumNode = new Node((x: number, y: number) => x + y, { x: "number", y: "number" }, "number");
    const input1$ = sumNode.inputStreams.get("x")!
    const output$ = sumNode.outputStream$
    const outSub = jest.fn((v) => v)

    output$.subscribe(outSub);
    input1$.next(7);

    //await waitForPopulation(50)
    expect(sumNode.currentValue).toEqual(undefined)
    expect(outSub).toHaveBeenCalledTimes(0)
  })

  it("multiple inputs should fire when all inputs are supplied, or changed", () => {
    const sumNode = new Node((x: number, y: number) => x + y, { x: "number", y: "number" }, "number");
    const input1$ = sumNode.inputStreams.get("x")!
    const input2$ = sumNode.inputStreams.get("y")!
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
    const repeatNode = new Node((c: string, n: number) => c.repeat(n), { c: "string", n: "number" }, "string");
    const input1$ = repeatNode.inputStreams.get("c")!
    const input2$ = repeatNode.inputStreams.get("n")!
    const output$ = repeatNode.outputStream$
    const outSub = jest.fn((v) => v)
    output$.subscribe(outSub);

    input1$.next("a");
    input2$.next(5);
    expect(repeatNode.currentValue).toEqual("aaaaa")
    expect(outSub).toHaveNthReturnedWith(1, "aaaaa")
    expect(outSub).toHaveBeenCalledTimes(1)
  })
})

describe('Graph functionality', () => {
  it('should process multiple connected nodes correctly', () => {
    const graph = new Graph();
    const constantNode1 = new Node((x: number) => x, { x: "number" }, "number");
    const constantNode2 = new Node((x: number) => x, { x: "number" }, "number");
    const doubleNode = new Node((x: number) => x * 2, { x: "number" }, "number");
    const sumNode = new Node((x: number, y: number) => x + y, { x: "number", y: "number" }, "number");
    const outSub = jest.fn((v) => v)
    const output$ = sumNode.outputStream$
    output$.subscribe(outSub);


    graph.addNode(sumNode);
    graph.addNode(doubleNode);
    graph.addNode(constantNode1);
    graph.addNode(constantNode2);

    graph.connectNodes(constantNode1, doubleNode, "x");
    graph.connectNodes(doubleNode, sumNode, "x");
    graph.connectNodes(constantNode2, sumNode, "y");

    expect(sumNode.currentValue).toBeUndefined()
    expect(outSub).toHaveBeenCalledTimes(0)

    constantNode1.inputStreams.get("x")!.next(5)
    expect(sumNode.currentValue).toBeUndefined()
    expect(outSub).toHaveBeenCalledTimes(0)

    constantNode2.inputStreams.get("x")!.next(7)
    expect(sumNode.currentValue).toEqual(17) //5*2 + 7
    expect(outSub).toHaveBeenCalledTimes(1)

    constantNode2.inputStreams.get("x")!.next(9)
    expect(sumNode.currentValue).toEqual(19)
    expect(outSub).toHaveBeenCalledTimes(2)

    constantNode2.inputStreams.get("x")!.next(11)
    expect(sumNode.currentValue).toEqual(21)
    expect(outSub).toHaveBeenCalledTimes(3)
  });


  it('should handle node connection order correctly', () => {
    const graph = new Graph();
    const incrementNode = new Node((x: number) => x + 1, { x: "number" } as const, "number");
    const squareNode = new Node((x: number) => x * x, { x: "number" } as const, "number");
    ;
    const outSub = jest.fn((v) => v)
    const output$ = squareNode.outputStream$
    output$.subscribe(outSub);

    graph.addNode(incrementNode);
    graph.addNode(squareNode);

    // Connect incrementNode -> squareNode
    graph.connectNodes(incrementNode, squareNode, "x");

    incrementNode.inputStreams.get("x")!.next(2);

    expect(squareNode.currentValue).toEqual(9)// Increment 2 to 3, then square 3 to 9
    expect(outSub).toHaveBeenCalledTimes(1)
    expect(graph.getConnections(incrementNode)?.length).toEqual(1)
  });

  it("should throw error if connection types don't match", () => {
    const graph = new Graph();
    const incrementNode = new Node((x: number) => x + 1, { x: "number" } as const, "number");
    const doubleStringNode = new Node((x: string) => x + x, { x: "string" } as const, "string");
    const outSub = jest.fn((v) => v)
    const output$ = doubleStringNode.outputStream$
    output$.subscribe(outSub);

    graph.addNode(incrementNode);
    graph.addNode(doubleStringNode);

    try {
      expect(graph.connectNodes(incrementNode, doubleStringNode, "x")).toThrow()
    } catch {
      incrementNode.inputStreams.get("x")!.next(2);

      expect(doubleStringNode.currentValue).toEqual(undefined) // data should never have been passed down
      expect(outSub).toHaveBeenCalledTimes(0)
      expect(graph.getConnections(incrementNode)!.length).toEqual(0)
    }

  });
  it("should give the correct number of connections", () => {
    const graph = new Graph();
    const constantNode = new Node((x: number) => x, { x: "number" }, "number");
    const incrementNode = new Node((x: number) => x + 1, { x: "number" } as const, "number");
    const squareNode = new Node((x: number) => x * x, { x: "number" } as const, "number");
    const sumNode1 = new Node((x: number, y: number) => x + y, { x: "number", y: "number" }, "number");
    const sumNode2 = new Node((x: number, y: number) => x + y, { x: "number", y: "number" }, "number");
    const outSub = jest.fn((v) => v)
    const output$ = sumNode2.outputStream$
    output$.subscribe(outSub);

    graph.addNodes([constantNode, incrementNode, squareNode, sumNode1, sumNode2]);
    graph.connectNodes(constantNode, incrementNode, "x")
    graph.connectNodes(incrementNode, squareNode, "x")
    graph.connectNodes(constantNode, sumNode1, "x")
    graph.connectNodes(constantNode, sumNode1, "y")
    graph.connectNodes(constantNode, sumNode2, "x")
    graph.connectNodes(sumNode1, sumNode2, "y")
    // TODO finish test!!!
    expect(graph.getConnections(constantNode)!.length).toEqual(4)

    constantNode.getInputStream("x").next(2);

    // expect(outSub).toHaveBeenCalledTimes(0)
    // expect(graph.getConnections(incrementNode)!.length).toEqual(0)

  });
});


