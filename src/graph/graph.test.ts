import { Node, } from './node.ts';
import { Graph } from './graph.ts';
import { createAddNode, createConstantNode, outPortFunction, port } from './nodeDefinitions.ts';

const createIncrementNode = () => new Node(port("x", "number"), outPortFunction("number"), (x: number) => x + 1);
const createDoubleNode = () => new Node(port("x", "number"), outPortFunction("number"), (x: number) => x * 2);
const createDoubleStringNode = () => new Node(port("x", "string"), outPortFunction("string"), (x: string) => x + x);
const createSquareNode = () => new Node(port("x", "number"), outPortFunction("number"), (x: number) => x * x);

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
    const sumNode = createAddNode("s1")
    const constantNode1 = createConstantNode("c1")
    const constantNode2 = createConstantNode("c2")
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

    constantNode1.pushValue(5, "none")
    expect(sumNode.currentValue).toBeUndefined()
    expect(outSub).toHaveBeenCalledTimes(0)

    constantNode2.pushValue(7, "none")
    expect(sumNode.currentValue).toEqual(17) //5*2 + 7
    expect(outSub).toHaveBeenCalledTimes(1)

    constantNode2.pushValue(9, "none")
    expect(sumNode.currentValue).toEqual(19)
    expect(outSub).toHaveBeenCalledTimes(2)

    constantNode2.pushValue(11, "none")
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
    const constantNode = createConstantNode("c1")

    graph.addNode(constantNode)

    try {
      expect(graph.addNode(constantNode)).toThrow()
    } catch (e) {
      expect(e).toEqual(Error("Attempted to add node c1 to graph, but it already has been added!"))
    }

  })

  it("should give the correct number of connections", () => {

    const graph = new Graph();
    const cIn = createConstantNode("cIn")
    const a1 = createAddNode("a1")
    const a2 = createAddNode("a2")
    const a3 = createAddNode("a3")
    const a4 = createAddNode("a4")

    graph.addNodes([cIn, a1, a2, a3, a4])
    graph.connectNodes(cIn, a1, "x")
    graph.connectNodes(cIn, a2, "x")
    graph.connectNodes(cIn, a3, "x")
    graph.connectNodes(cIn, a4, "x")

    expect(graph.getConnectedNodes(cIn)!.length).toEqual(4)
  })

  it("nodes fire when all inputs are satisfied, and any input changes", () => {
    const graph = new Graph();
    const c1 = createConstantNode("c1")
    const constOutSub = jest.fn((v) => v)
    c1.outputPort$.subscribe(constOutSub)

    const squareNode = createSquareNode()
    const incrementNode = createIncrementNode()
    const a1 = createAddNode("a1")
    const a1OutSub = jest.fn((v) => v)
    a1.outputPort$.subscribe(a1OutSub)

    const a2 = createAddNode("a2")
    const a2OutSub = jest.fn((v) => v)
    a2.outputPort$.subscribe(a2OutSub)

    graph.addNodes([c1, incrementNode, squareNode, a1, a2])
    graph.connectNodes(c1, incrementNode, "x") // connection 1
    graph.connectNodes(incrementNode, squareNode, "x")
    graph.connectNodes(c1, a1, "x")// connection 2
    graph.connectNodes(c1, a1, "y")// connection 3
    graph.connectNodes(c1, a2, "x")// connection 4
    graph.connectNodes(a1, a2, "y")

    expect(graph.getConnectedNodes(c1)!.length).toEqual(4)

    //kick off first input
    c1.pushValue(2, "none");

    //start and end nodes ony run once
    expect(constOutSub).toHaveBeenCalledTimes(1)
    expect(a1OutSub).toHaveBeenCalledTimes(1)
    expect(a2OutSub).toHaveBeenCalledTimes(1)

    c1.pushValue(3, "none");

    expect(constOutSub).toHaveBeenCalledTimes(2) // 2 inputs, one for each "next"

    // this behavior could be suprising to users. I'm not certain if this is how it should behave.
    // maybe calculations should be canceled and restarted if new inputs come in before computation is complete?
    // that seems like it could have some complicated effects...
    expect(a1OutSub).toHaveBeenCalledTimes(3) // fires two additional times, one when "x" is changed, and one for "y"
    expect(a2OutSub).toHaveBeenCalledTimes(4) // fires three additional times, 2 from sum1 (y), and 1 from constant being updated

  });
})

