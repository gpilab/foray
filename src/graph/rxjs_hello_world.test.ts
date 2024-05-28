import { Node, Graph } from './rxjs_hello_world.ts';

describe('Node and Graph functionality', () => {
  it("shouldn't populate output stream if no output is given", () => {
    const constantNode = new Node((x: number) => x);
    const testSub = jest.fn()
    constantNode.outputStream$.subscribe(testSub);
    //wait
    expect(constantNode.currentValue).toBeUndefined
  });
  it('should process a single node correctly', done => {
    const constantNode = new Node((x: number) => x);
    constantNode.inputStreams[0].next(10);
    constantNode.outputStream$.subscribe(testSub);
    expect(testSub).not.toHaveBeenCalled()
  });

  it('should process multiple connected nodes correctly', done => {
    const graph = new Graph();

    const constantNode = new Node((x) => x);
    const doubleNode = new Node((x: number) => x * 2);
    const addNode = new Node((x, y) => x + y);

    graph.addNode(constantNode);
    graph.addNode(doubleNode);
    graph.addNode(addNode);

    graph.connectNodes(constantNode, doubleNode);
    graph.connectNodes(doubleNode, addNode, 0);
    graph.connectNodes(constantNode, addNode, 1);

    const start_value = 5
    constantNode.inputStreams[0].next(start_value);
    addNode.outputStream$.subscribe((_v) => {
      expect(addNode.currentValue).toBe(15); // 5 * 2 + 5 = 15
      done()
    })
  }, 50);


  it('should handle node connection order correctly', done => {
    const graph = new Graph();

    const incrementNode = new Node((x: number) => x + 1);
    const squareNode = new Node((x: number) => x * x);

    graph.addNode(incrementNode);
    graph.addNode(squareNode);

    // Connect incrementNode -> squareNode
    graph.connectNodes(incrementNode, squareNode);

    incrementNode.inputStreams[0].next(2); // Increment 2 to 3, then square 3 to 9

    squareNode.outputStream$.subscribe(result => {
      expect(result).toBe(9);
      done();
    });
  }, 50);
});
