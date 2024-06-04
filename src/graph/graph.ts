import { InputTypeLabelByKey, NodeInputs, Node } from "./node";


export class Graph {
  private nodeAdjacencies: Map<Node, Node[]> = new Map();

  // TODO: add output type restriction
  addNode<I extends NodeInputs, K extends I[number]["name"], O extends InputTypeLabelByKey<I, K>>
    (node: Node<any, O>, connections: { targetNode: Node<I>, targetInputLabel: K }[] = []) {
    this.nodeAdjacencies.set(node, []);
    connections.forEach(({ targetNode, targetInputLabel }) => {
      this.connectNodes(node, targetNode, targetInputLabel);
    });
  }

  addNodes(nodes: Node[]) {
    nodes.forEach((node) => this.addNode(node))
  }

  // Connect output of one node to input of another node
  connectNodes<T extends NodeInputs, K extends T[number]["name"]>
    (sourceNode: Node, targetNode: Node<T>, targetInputLabel: K) {
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
    sourceNode.outputStream$.subscribe(output => {
      const input = targetNode.inputStreams[targetInputLabel]
      if (input === undefined) {
        throw Error(`Attempted to access input label ${targetInputLabel} on node ${node} `)
      }
      input.next(output);
    });
  }

  getConnections(node: Node) {
    return this.nodeAdjacencies.get(node)
  }
}
