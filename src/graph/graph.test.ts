import { Node, outPort, port, port2 } from './node.ts';
import { Graph } from './graph.ts';

const createConstantNode = () => new Node(port("x", "number"), outPort("number"), (x: number) => x, "c", "Constant");
const createIncrementNode = () => new Node(port("x", "number"), outPort("number"), (x: number) => x + 1);
const createDoubleNode = () => new Node(port("x", "number"), outPort("number"), (x: number) => x * 2);
const createDoubleStringNode = () => new Node(port("x", "string"), outPort("string"), (x: string) => x + x);
const createSquareNode = () => new Node(port("x", "number"), outPort("number"), (x: number) => x * x);
const createSumNode = () => new Node(port2("x", "number", "y", "number"), outPort("number"), (x: number, y: number) => x + y, "abc", "Add");

describe('Graph functionality', () => {
  // it("should not allow incompatible connections", () => {
  //   const graph = new Graph();
  //   const incrementNode = new Node((x: number) => x + 1, [["x", "number"] as const], "number");
  //   const doubleStringNode = new Node((x: string) => x + x, [["x", "string"] as const], "string");
  //   const outSub = jest.fn((v) => v)
  //   const output$ = doubleStringNode.outputStream$
  //   output$.subscribe(outSub);
  //
  //   graph.addNode(incrementNode);
  //   graph.addNode(doubleStringNode);
  //
  //   try {
  //     expect(graph.connectNodes(incrementNode, doubleStringNode, "x")).toThrow()
  //   } catch {
  //     incrementNode.getInputStream("x")!.next(2);
  //
  //     expect(doubleStringNode.currentValue).toEqual(undefined) // data should never have been passed down
  //     expect(outSub).toHaveBeenCalledTimes(0)
  //     expect(graph.getConnections(incrementNode)!.length).toEqual(0)
  //   }
  // });

  it("should throw error if connection types don't match", () => {
    const graph = new Graph();
    const incrementNode = createIncrementNode()
    const doubleStringNode = createDoubleStringNode()
    const outSub = jest.fn((v) => v)
    const output$ = doubleStringNode.outputPort$
    output$.subscribe(outSub);

    graph.addNode(incrementNode);
    graph.addNode(doubleStringNode);

    try {
      //@ts-expect-error
      expect(graph.connectNodes(incrementNode, doubleStringNode, "x")).toThrow()
    } catch {
      incrementNode.getInputStream("x")!.next(2);

      expect(doubleStringNode.currentValue).toEqual(undefined) // data should never have been passed down
      expect(outSub).toHaveBeenCalledTimes(0)
      expect(graph.getConnectedNodes(incrementNode)!.length).toEqual(0)
    }
  });

  it('should process multiple connected nodes correctly', () => {
    const graph = new Graph();
    const sumNode = createSumNode()
    const constantNode1 = createConstantNode()
    const constantNode2 = createConstantNode()
    const doubleNode = createDoubleNode()
    const outSub = jest.fn((v) => v)
    const output$ = sumNode.outputPort$
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

    constantNode1.getInputStream("x").next(5)
    expect(sumNode.currentValue).toBeUndefined()
    expect(outSub).toHaveBeenCalledTimes(0)

    constantNode2.getInputStream("x").next(7)
    expect(sumNode.currentValue).toEqual(17) //5*2 + 7
    expect(outSub).toHaveBeenCalledTimes(1)

    constantNode2.getInputStream("x").next(9)
    expect(sumNode.currentValue).toEqual(19)
    expect(outSub).toHaveBeenCalledTimes(2)

    constantNode2.getInputStream("x").next(11)
    expect(sumNode.currentValue).toEqual(21)
    expect(outSub).toHaveBeenCalledTimes(3)
  });


  it('should handle node connection order correctly', () => {
    const graph = new Graph();
    const incrementNode = createIncrementNode()
    const squareNode = createSquareNode()
      ;
    const outSub = jest.fn((v) => v)
    const output$ = squareNode.outputPort$
    output$.subscribe(outSub);

    graph.addNode(incrementNode);
    graph.addNode(squareNode);

    // Connect incrementNode -> squareNode
    graph.connectNodes(incrementNode, squareNode, "x");

    incrementNode.getInputStream("x").next(2);

    expect(squareNode.currentValue).toEqual(9)// Increment 2 to 3, then square 3 to 9
    expect(outSub).toHaveBeenCalledTimes(1)
    expect(graph.getConnectedNodes(incrementNode)?.length).toEqual(1)
  });

  it("Graphs can't have the same node added twice", () => {

    const graph = new Graph();
    const constantNode = createConstantNode()

    graph.addNode(constantNode)

    try {
      expect(graph.addNode(constantNode)).toThrow()
    } catch (e) {
      expect(e).toEqual(Error("Attempted to add node c to graph, but it already has been added!"))
    }

  })

  it("should give the correct number of connections", () => {

    const graph = new Graph();
    const constantNodeIn = createConstantNode()
    const constantNode1 = createConstantNode()
    const constantNode2 = createConstantNode()
    const constantNode3 = createConstantNode()
    const constantNode4 = createConstantNode()

    graph.addNodes([constantNodeIn, constantNode1, constantNode2, constantNode3, constantNode4])
    graph.connectNodes(constantNodeIn, constantNode1, "x")
    graph.connectNodes(constantNodeIn, constantNode2, "x")
    graph.connectNodes(constantNodeIn, constantNode3, "x")
    graph.connectNodes(constantNodeIn, constantNode4, "x")

    expect(graph.getConnectedNodes(constantNodeIn)!.length).toEqual(4)
  })

  it("nodes fire when all inputs are satisfied, and any input changes", () => {
    const graph = new Graph();
    const constantNode = createConstantNode()
    const constOutSub = jest.fn((v) => v)
    constantNode.outputPort$.subscribe(constOutSub)

    const squareNode = createSquareNode()
    const incrementNode = createIncrementNode()
    const sumNode1 = createSumNode()
    const sum1OutSub = jest.fn((v) => v)
    sumNode1.outputPort$.subscribe(sum1OutSub)

    const sumNode2 = createSumNode()
    const sum2OutSub = jest.fn((v) => v)
    sumNode2.outputPort$.subscribe(sum2OutSub)

    graph.addNodes([constantNode, incrementNode, squareNode, sumNode1, sumNode2])
    graph.connectNodes(constantNode, incrementNode, "x") // connection 1
    graph.connectNodes(incrementNode, squareNode, "x")
    graph.connectNodes(constantNode, sumNode1, "x")// connection 2
    graph.connectNodes(constantNode, sumNode1, "y")// connection 3
    graph.connectNodes(constantNode, sumNode2, "x")// connection 4
    graph.connectNodes(sumNode1, sumNode2, "y")

    expect(graph.getConnectedNodes(constantNode)!.length).toEqual(4)

    //kick off first input
    constantNode.getInputStream("x").next(2);

    //start and end nodes ony run once
    expect(constOutSub).toHaveBeenCalledTimes(1)
    expect(sum1OutSub).toHaveBeenCalledTimes(1)
    expect(sum2OutSub).toHaveBeenCalledTimes(1)

    constantNode.getInputStream("x").next(3);

    expect(constOutSub).toHaveBeenCalledTimes(2) // 2 inputs, one for each "next"

    // this behavior could be suprising to users. I'm not certain if this is how it should behave.
    // maybe calculations should be canceled and restarted if new inputs come in before computation is complete?
    // that seems like it could have some complicated effects...
    expect(sum1OutSub).toHaveBeenCalledTimes(3) // fires two additional times, one when "x" is changed, and one for "y"
    expect(sum2OutSub).toHaveBeenCalledTimes(4) // fires three additional times, 2 from sum1 (y), and 1 from constant being updated

  });
})

