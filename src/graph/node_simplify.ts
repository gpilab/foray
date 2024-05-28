

// Base Types


type DataType = 'number' | 'boolean';

interface PortConnection {
  toNodeId: string,
  toPortId: string
}

interface BaseIO {
  type: DataType;
}

interface Input extends BaseIO {
  value?: any;
}

interface Output extends BaseIO {
  value?: any;
  portConnection?: PortConnection
}

// key is the label for the port
type Ports<T extends BaseIO> = Record<string, T>

// interface Input extends BaseIO {
//   ioType?: "in"
// }
//
// interface Output extends BaseIO {
//   ioType?: "out"
// }





interface FunctionNodeBase<I extends Ports<Input>, O extends Ports<Output>> {
  id: string
  type: string
  inputs: I
  outputs: O
  compute: (...args: any[]) => any
}


// Function Types



interface AddFunction extends FunctionNodeBase<
  { a: { type: 'number', value?: number }, b: { type: 'number', value?: number } },
  { sum: { type: 'number', value?: number } }> {
  type: 'add'
  inputs: { a: { type: 'number', value?: number }, b: { type: 'number', value?: number } },
  outputs: { sum: { type: 'number', value?: number } },
  compute: (a: number, b: number) => number;
}

const createAddFunction = (id: string): AddFunction => ({
  id,
  type: 'add',
  inputs: { a: { type: 'number' }, b: { type: 'number' } },
  outputs: { sum: { type: 'number' } },
  compute: (a: number, b: number) => a + b
});


interface NotFunction extends FunctionNodeBase<
  { a: { type: 'boolean', value?: boolean } },
  { not_a: { type: 'boolean', value?: boolean } }> {
  type: 'not'
  inputs: { a: { type: 'boolean' } },
  outputs: { not_a: { type: 'boolean' } },
  compute: (a: boolean) => boolean;
}

const createNotFunction = (id: string): NotFunction => ({
  id,
  type: 'not',
  inputs: { a: { type: 'boolean' } },
  outputs: { not_a: { type: 'boolean' } },
  compute: (a: boolean) => !a
});


interface ConstantNumberFunction extends FunctionNodeBase<
  {},
  { c: { type: 'number', value: number } }> {
  type: 'constant_number'
  inputs: {},
  outputs: { c: { type: 'number', value: number } },
  compute: () => number
}

const createConstantNumberFunction = (id: string, constant: number): ConstantNumberFunction => ({
  id,
  type: 'constant_number',
  inputs: {},
  outputs: { c: { type: 'number', value: constant } },
  compute: () => constant
});


type FunctionNode = AddFunction | NotFunction | ConstantNumberFunction;

// Node Utilities

const connectNodes = <
  ON extends FunctionNode,
  IN extends FunctionNode,
  OL extends keyof ON['outputs'],
  IL extends keyof IN['inputs']
>(
  outputNode: ON,
  outputLabel: OL,
  inputNode: IN,
  inputLabel: IL
): void => {
  console.log(outputNode.outputs)
  outputNode.outputs[outputLabel].portConnection = {
    toNodeId: inputNode.id,
    toPortId: inputLabel
  }
  console.log(outputNode.outputs)
};

const setInputValue = (nodeId: string, portLabel: string, value: any, all_nodes: FunctionNode[]) => {
  const node = all_nodes.find(n => n.id == nodeId)
  if (!node) {
    throw Error(`Could not find nodeId:${nodeId}`)
  }
  const input = node.inputs[portLabel]

  if (!input) {
    throw Error(`Could not find portLabel,${portLabel}, on the inputs of node: ${nodeId}`)
  }
  input.value = value
}

const runCompute = (node: FunctionNode, all_nodes: FunctionNode[]): void => {
  const computeInputs = Object.entries(node.inputs).map<Input>(([_label, input]) => input.value)
  console.log(computeInputs)
  const outputValues = node.compute(...computeInputs)
  console.log(outputValues)


  Object.entries(node.outputs).forEach(([_index, o]) => {
    console.log(_index, o)
    const output = o as Output
    output.value = outputValues
    if (output.portConnection) {
      setInputValue(output.portConnection.toNodeId, output.portConnection.toPortId, outputValues, all_nodes)
    }
  });
};

// Usage


const add_Node = createAddFunction('add1');
const add_Node2 = createAddFunction('add2');
const add_Node3 = createAddFunction('add3');
const notNode = createNotFunction('not1');
const constantNumberNode = createConstantNumberFunction('const_num1', 5);
const constantNumberNode2 = createConstantNumberFunction('const_num2', 7);
const constantNumberNode3 = createConstantNumberFunction('const_num1', 13);
const constantNumberNode4 = createConstantNumberFunction('const_num2', 17);
const all_nodes = [add_Node, add_Node2, add_Node3, notNode, constantNumberNode, constantNumberNode2, constantNumberNode3, constantNumberNode4]
// Connect the nodes
console.log("before connect")
connectNodes(constantNumberNode, 'c', add_Node, 'a');
connectNodes(constantNumberNode2, 'c', add_Node, 'b');
connectNodes(constantNumberNode3, 'c', add_Node2, 'a');
connectNodes(constantNumberNode4, 'c', add_Node2, 'b');

connectNodes(add_Node, 'sum', add_Node3, 'a');
connectNodes(add_Node2, 'sum', add_Node3, 'b');

// Set a value for the second input of the add node
//addNode.inputs.find(input => input.label === 'b')!.value = 10;

// Compute the outputs
console.log("running constant")
runCompute(constantNumberNode, all_nodes);
runCompute(constantNumberNode2, all_nodes);
runCompute(constantNumberNode3, all_nodes);
runCompute(constantNumberNode4, all_nodes);
console.log("running add")
runCompute(add_Node, all_nodes);
runCompute(add_Node2, all_nodes);
runCompute(add_Node3, all_nodes);
console.log(constantNumberNode)
console.log(constantNumberNode2)
console.log(add_Node)
console.log(add_Node.outputs["sum"].value);
console.log(add_Node2.outputs["sum"].value);
console.log(add_Node3.outputs["sum"].value); 
