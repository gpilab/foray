import { InputTypeLabelByKey, NodeInputs, Node } from "./node";

/**
 * The Graph handles connections between nodes
 *
 * Once nodes are connected, Graph doesn't need to do any 
 * additional work for data to be passed. Data passing is handled 
 * entirely by the output ports automatically passing values to
 * input ports via streams
 * 
 */
export class Graph {
  private nodeAdjacencies: Map<Node, Node[]> = new Map();

  addNode<I extends NodeInputs, K extends I[number]["name"], O extends InputTypeLabelByKey<I, K>>
    (node: Node<any, O>,
      connections: { targetNode: Node<I>, targetInputLabel: K }[] = []) {
    if (this.nodeAdjacencies.get(node)) {
      throw Error("Attempted to add node to graph, but it already has been added!")
    }
    this.nodeAdjacencies.set(node, []);

    connections.forEach(({ targetNode, targetInputLabel }) => {
      this.connectNodes(node, targetNode, targetInputLabel);
    });
  }

  addNodes(nodes: Node[]) {
    nodes.forEach((node) => this.addNode(node))
  }

  // Connect output of one node to input of another node
  connectNodes<T extends NodeInputs, K extends T[number]["name"], O extends InputTypeLabelByKey<T, K>>
    (sourceNode: Node<any, O>, targetNode: Node<T>, targetInputLabel: K) {
    const sourceNodeAdjacencies = this.nodeAdjacencies.get(sourceNode)
    if (sourceNodeAdjacencies == undefined) {
      throw Error("Source node not present in graph");
    }
    if (!this.nodeAdjacencies.has(targetNode)) {
      throw Error("Target node not present in graph");
    }
    if (sourceNode.outputType != targetNode.getInputType(targetInputLabel)) {
      throw Error(`Attempted to connect nodes of type (source, output: ${sourceNode.outputType} )and (target, input: ${targetNode.getInputType(targetInputLabel)})`)
    }
    // Add target node to the adjacency list of the source node
    sourceNodeAdjacencies.push(targetNode);

    // Subscribe the output of the source node to the input of the target node
    sourceNode.outputPort$.subscribe(output => {
      const targetInput = targetNode.inputStreams[targetInputLabel]
      if (targetInput === undefined) {
        throw Error(`Attempted to access input label ${targetInputLabel} on node ${targetNode} `)
      }
      targetInput.next(output);
    });
  }

  getConnectedNodes(node: Node) {
    return this.nodeAdjacencies.get(node)
  }
}
