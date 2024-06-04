import { Node, port, port2 } from './node.ts';
import { Graph } from './graph.ts';

const createConstantNode = () => new Node((x: number) => x, port("x", "number"), "number");
const createIncrementNode = () => new Node((x: number) => x + 1, port("x", "number"), "number");
const createDoubleNode = () => new Node((x: number) => x * 2, port("x", "number"), "number");
const createDoubleStringNode = () => new Node((x: string) => x + x, port("x", "string"), "string");
const createSquareNode = () => new Node((x: number) => x * x, port("x", "number"), "number");
const createSumNode = () => new Node((x: number, y: number) => x + y, port2("x", "number", "y", "number"), "number");

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
    const output$ = doubleStringNode.outputStream$
    output$.subscribe(outSub);

    graph.addNode(incrementNode);
    graph.addNode(doubleStringNode);

    try {
      expect(graph.connectNodes(incrementNode, doubleStringNode, "x")).toThrow()
    } catch {
      incrementNode.getInputStream("x")!.next(2);

      expect(doubleStringNode.currentValue).toEqual(undefined) // data should never have been passed down
      expect(outSub).toHaveBeenCalledTimes(0)
      expect(graph.getConnections(incrementNode)!.length).toEqual(0)
    }
  });

  it('should process multiple connected nodes correctly', () => {
    const graph = new Graph();
    const sumNode = createSumNode()
    const constantNode1 = createConstantNode()
    const constantNode2 = createConstantNode()
    const doubleNode = createDoubleNode()
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
    const squareNode = new Node((x: number) => x * x, [{ name: "x", portType: "number" }] as const, "number");
    ;
    const outSub = jest.fn((v) => v)
    const output$ = squareNode.outputStream$
    output$.subscribe(outSub);

    graph.addNode(incrementNode);
    graph.addNode(squareNode);

    // Connect incrementNode -> squareNode
    graph.connectNodes(incrementNode, squareNode, "x");

    incrementNode.getInputStream("x").next(2);

    expect(squareNode.currentValue).toEqual(9)// Increment 2 to 3, then square 3 to 9
    expect(outSub).toHaveBeenCalledTimes(1)
    expect(graph.getConnections(incrementNode)?.length).toEqual(1)
  });

  it("should give the correct number of connections", () => {
    const graph = new Graph();
    const constantNode = createConstantNode()
    const squareNode = createSquareNode()
    const incrementNode = createIncrementNode()
    const sumNode1 = createSumNode()
    const sumNode2 = createSumNode()
    const outSub = jest.fn((v) => v)
    const output$ = sumNode2.outputStream$
    output$.subscribe(outSub);

    graph.addNodes([constantNode, incrementNode, squareNode, sumNode1, sumNode2])
    graph.connectNodes(constantNode, incrementNode, "x") // connection 1
    graph.connectNodes(incrementNode, squareNode, "x")
    graph.connectNodes(constantNode, sumNode1, "x")// connection 2
    graph.connectNodes(constantNode, sumNode1, "y")// connection 3
    graph.connectNodes(constantNode, sumNode2, "x")// connection 4
    graph.connectNodes(sumNode1, sumNode2, "y")
    // TODO finish test!!!
    expect(graph.getConnections(constantNode)!.length).toEqual(4)

    constantNode.getInputStream("x").next(2);

    // expect(outSub).toHaveBeenCalledTimes(0)
    // expect(graph.getConnections(incrementNode)!.length).toEqual(0)

  });
})

