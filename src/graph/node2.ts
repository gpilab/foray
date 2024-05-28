
function createInput(nodeId: string, portName: string, portDataType: PortDataType): IOPort {
  return { nodeId, portName, portDataType };
}

function createOutput(nodeId: string, portName: string, portDataType: PortDataType): IOPort {
  return { nodeId, portName, portDataType };
}
interface AddNodeInputs {
  a: number;
  b: number;
}

interface AddNodeOutputs {
  sum: number;
}

interface MultiplyNodeInputs {
  x: number;
  y: number;
}

interface MultiplyNodeOutputs {
  product: number;
}

interface ConstantNodeOutputs<T> {
  value: T;
}
type NodeComputeFunction<InputTypes, OutputTypes> = (inputs: InputTypes) => OutputTypes;

type PortDataType = number | number[] | string

interface IOPort {
  nodeId: string;
  portName: string;
  portDataType: PortDataType;
}

interface MNode<InputTypes, OutputTypes> {
  id: string;
  inputs: IOPort[]
  outputs: IOPort[]
  compute: NodeComputeFunction<InputTypes, OutputTypes>;
}

const createNode = <InputTypes, OutputTypes>(
  id: string,
  inputs: IOPort[],
  outputs: IOPort[],
  compute: NodeComputeFunction<InputTypes, OutputTypes>
): MNode<InputTypes, OutputTypes> => ({
  id,
  inputs,
  outputs,
  compute
});

const createConstantNode = <T>(id: string, value: T): MNode<{}, ConstantNodeOutputs<T>> => createNode(
  id,
  [],
  [{ portName: "constOut", nodeId: id, portDataType: typeof value }],
  () => ({ value })
);


const addNode = createNode<AddNodeInputs, AddNodeOutputs>(
  'addNode1',
  [createInput('addNode1', 'a', 'number'),
  createInput('addNode1', 'b', 'number')],
  [createOutput('addNode1', 'sum', 'number')
  ],
  (inputs) => {
    return { sum: inputs.a + inputs.b };
  }
);

const multiplyNode = createNode<MultiplyNodeInputs, MultiplyNodeOutputs>(
  'multiplyNode1',
  [createInput('multiplyNode1', 'x', 'number'),
  createInput('multiplyNode1', 'y', 'number')],
  [createOutput('multiplyNode1', 'product', 'number')],
  (inputs) => {
    return { product: inputs.x * inputs.y };
  }
);

class MGraph {
  nodes: MNode<any, any>[] = [];
  edges: { from: IOPort; to: IOPort }[] = [];

  addNode<InputTypes, OutputTypes>(node: MNode<InputTypes, OutputTypes>) {
    this.nodes.push(node);
  }

  connectNodes(from: IOPort, to: IOPort) {
    if (from.portDataType !== to.portDataType) {
      throw new Error('Type mismatch');
    }

    this.edges.push({ from, to });
  }

  topologicalSort(): MNode<any, any>[] {
    const inDegree: { [key: string]: number } = {};
    const adjList: { [key: string]: string[] } = {};

    for (const node of this.nodes) {
      inDegree[node.id] = 0;
      adjList[node.id] = [];
    }

    for (const edge of this.edges) {
      const fromId = edge.from.nodeId;
      const toId = edge.to.nodeId;
      inDegree[toId] = (inDegree[toId] || 0) + 1;
      adjList[fromId].push(toId);
    }

    const queue: string[] = [];
    for (const nodeId in inDegree) {
      if (inDegree[nodeId] === 0) {
        queue.push(nodeId);
      }
    }

    const sortedNodes: MNode<any, any>[] = [];
    while (queue.length > 0) {
      const nodeId = queue.shift()!;
      const node = this.nodes.find(n => n.id === nodeId)!;
      sortedNodes.push(node);

      for (const neighbor of adjList[nodeId]) {
        inDegree[neighbor]--;
        if (inDegree[neighbor] === 0) {
          queue.push(neighbor);
        }
      }
    }

    if (sortedNodes.length !== this.nodes.length) {
      throw new Error('Graph has a cycle');
    }

    return sortedNodes;
  }

  evaluate() {
    const nodeValues: { [nodeId: string]: any } = {};

    const getNodeValue = (port: IOPort) => {
      if (!nodeValues.hasOwnProperty(port.nodeId)) {
        throw new Error(`Value for node ${port.nodeId} not computed yet`);
      }
      return nodeValues[port.nodeId][port.portName];
    };

    //const sortedNodes = this.topologicalSort();
    const sortedNodes = this.nodes

    for (const node of sortedNodes) {
      console.log(node)
      const inputValues: any = {};
      for (const input of node.inputs) {
        console.log("starting eval of input:", input)
        inputValues[input.portName] = getNodeValue(input);
        console.log(inputValues[input.portName])
      }

      const outputValues = node.compute(inputValues);
      nodeValues[node.id] = outputValues;
    }

    return nodeValues;
  }
}

const graph = new MGraph();

// Create constant nodes
const constantNode1 = createConstantNode('aaconstantNode1', 3);
const constantNode2 = createConstantNode('constantNode2', 4);
const constantNode3 = createConstantNode('constantNode3', 5);

// Add all nodes to the graph
graph.addNode(constantNode1);
graph.addNode(constantNode2);
graph.addNode(constantNode3);
graph.addNode(addNode);
graph.addNode(multiplyNode);

// Connect constant nodes to the add node
graph.connectNodes(constantNode1.outputs[0], addNode.inputs[0]);
graph.connectNodes(constantNode2.outputs[0], addNode.inputs[1]);

// Connect add node to the multiply node
graph.connectNodes(addNode.outputs[0], multiplyNode.inputs[0]);

// Connect another constant node to the multiply node
graph.connectNodes(constantNode3.outputs[0], multiplyNode.inputs[1]);

console.log(graph);

const result = graph.evaluate();
console.log(result); // This will log the final values of all nodes
