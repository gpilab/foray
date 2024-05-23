

// Base Types


type DataType = 'number' | 'boolean';

interface BaseIO {
  label: string;
  type: DataType;
  value?: any; // Optional value for computation
  ioType?: "in" | "out"
}

interface Input extends BaseIO {
  ioType?: "in"
}

interface Output extends BaseIO {
  ioType?: "out"
}


interface PortConnection {
  toNodeId: string
  toPortId: string
}
type PortConnectionMap = Record<string, PortConnection>



interface FunctionNodeBase<I extends BaseIO[], O extends BaseIO[]> {
  id: string;
  type: string;
  inputs: I;
  outputs: O;
  inputConnections: PortConnectionMap;
  outputConnections: PortConnectionMap;
  compute: (...args: any[]) => any; // Function to compute outputs from inputs
}


// Function Types



interface AddFunction extends FunctionNodeBase<
  [{ label: 'a', type: 'number' }, { label: 'b', type: 'number' }],
  [{ label: 'sum', type: 'number', value?: number }]
> {
  type: 'add';
  compute: (a: number, b: number) => number;
}

interface NotFunction extends FunctionNodeBase<
  [{ label: 'a', type: 'boolean' }],
  [{ label: 'not_a', type: 'boolean', value?: number }]
> {
  type: 'not';
  compute: (a: boolean) => boolean;
}

interface ConstantNumberFunction extends FunctionNodeBase<
  [],
  [{ label: 'c', type: 'number', value?: number }]
> {
  type: 'constant_number';
  compute: () => number;
}

type FunctionNode = AddFunction | NotFunction | ConstantNumberFunction;


// function Factories


const createAddFunction = (id: string): AddFunction => ({
  id,
  type: 'add',
  inputs: [{ label: 'a', type: 'number' }, { label: 'b', type: 'number' }],
  outputs: [{ label: 'sum', type: 'number' }],
  inputConnections: {},
  outputConnections: {},
  compute: (a: number, b: number) => a + b
});

const createNotFunction = (id: string): NotFunction => ({
  id,
  type: 'not',
  inputs: [{ label: 'a', type: 'boolean' }],
  outputs: [{ label: 'not_a', type: 'boolean' }],
  inputConnections: {},
  outputConnections: {},
  compute: (a: boolean) => !a
});

const createConstantNumberFunction = (id: string, constant: number): ConstantNumberFunction => ({
  id,
  type: 'constant_number',
  inputs: [],
  outputs: [{ label: 'c', type: 'number', value: constant }],
  inputConnections: {},
  outputConnections: {},
  compute: () => constant
});


const connectNodes = <
  ON extends FunctionNodeBase<any, any>,
  IN extends FunctionNodeBase<any, any>,
  OL extends ON['outputs'][number]['label'],
  IL extends IN['inputs'][number]['label'],
>(
  outputNode: ON,
  outputLabel: OL,
  inputNode: IN,
  inputLabel: IL
): void => {
  console.log("in connect")
  console.log(`Connecting output ${outputNode.id}, label: ${outputLabel},\nto ${inputNode.id}, label: ${inputLabel}`)
  const output = outputNode.outputs.find((o: Output) => o.label === outputLabel);
  const input = inputNode.inputs.find((i: Input) => i.label === inputLabel);

  if (!output || !input) {
    throw new Error(`Label not found on node. This should never happen. ${outputLabel}, ${inputLabel}`);
  }

  if (output.type !== input.type) {
    throw new Error(`Type mismatch: cannot connect ${output.type} to ${input.type}`);
  }

  // Add connections
  // if (!outputNode.outputConnections.find(o => o.label == outputLabel)) {
  outputNode.outputConnections[outputLabel] = {
    toNodeId: inputNode.id,
    toPortId: inputLabel
  }
  // }
  //outputNode.outputConnections[outputLabel]?.push(input);

  inputNode.inputConnections[inputLabel] = {
    toNodeId: outputNode.id,
    toPortId: outputLabel
  }
  // inputNode.inputConnections[inputLabel] = output;
  // input.value = output.value; // Propagate initial value
};

function fill_inputs(node: FunctionNode, all_nodes: FunctionNode[]) {

  Object.entries(node.inputConnections).map(([label, portConnection]) => {
    console.log("Port Connection", portConnection.toNodeId)
    const portLabel = portConnection.toPortId
    const connectedNode = all_nodes.find(n => n.id == portConnection.toNodeId) as FunctionNode

    const output = connectedNode.outputs.find(o => o.label == portLabel) as Output
    console.log(`grabbing value for ${node.id}, label: ${label}\n This node: ${node} \n looking for value from node ${connectedNode.id}, label ${portLabel}`)
    console.log(output)
    node.inputs.find(i => i.label == label).value = output.value
  })
}

const runCompute = (node: FunctionNode, all_nodes: FunctionNode[]): void => {
  fill_inputs(node, all_nodes)
  const inputs = node.inputs.map((input, label) => { label: input.value })
  const outputValues = node.compute(...inputs)

  node.outputs.forEach((output, index) => {
    output.value = outputValues[index];
  });
};
// Usage


const addNode = createAddFunction('add1');
const notNode = createNotFunction('not1');
const constantNumberNode = createConstantNumberFunction('const_num1', 5);
const constantNumberNode2 = createConstantNumberFunction('const_num2', 7);
const all_nodes = [addNode, constantNumberNode, constantNumberNode2]
// Connect the nodes
console.log("before connect")
connectNodes(constantNumberNode, 'c', addNode, 'a'); // Connect constant number output to add input 'a'
connectNodes(constantNumberNode2, 'c', addNode, 'b'); // Connect constant number output to add input 'a'

// Set a value for the second input of the add node
//addNode.inputs.find(input => input.label === 'b')!.value = 10;

// Compute the outputs
console.log("running constant")
runCompute(constantNumberNode, all_nodes);
runCompute(constantNumberNode2, all_nodes);
console.log("running add")
runCompute(addNode, all_nodes);

console.log(addNode.outputs.find(output => output.label === 'sum')!.value); // Should output 15
