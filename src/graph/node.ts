type CustomType = {
  a: string
  b: boolean
}

/** `(Label: Type)` pairs for all possible data types that can be passed through ports
 * `Label`s are used as generic strings for creating types
 */
export interface PortDataTypes {
  number: number
  vec: number[]
  string: string
  customType: CustomType
}

export type PortDataTypesTuple = [
  number,
  number[],
  string,
  CustomType,
]


// An actual instance of data that can be on a port 
// The keys of PortDataType are removed, leaving just a union of the data type
// not currently needed
type PortDataInstance = PortDataTypes[keyof PortDataTypes]

/** Common values that input and output ports share
 */
interface BasePort<T extends keyof PortDataTypes> {
  label: string
  dataType: T
  heldValue?: PortDataTypes[T]
}

/** Defines the shape of values that a  node can accept as input
 */
interface inputPort
  <T extends keyof PortDataTypes> extends BasePort<T> {
  ioType: "in"
}

/** Defines the shape of values that a the a node can produce
 */
interface outputPort
  <T extends keyof PortDataTypes> extends BasePort<T> {
  ioType: "out"
}


export function createInput
  <T extends keyof PortDataTypes>
  (label: string, dataType: T): inputPort<T> {
  return { ioType: "in", label: label, dataType: dataType }
}

export function createOutput
  <T extends keyof PortDataTypes>
  (label: string, dataType: T): outputPort<T> {
  return { ioType: "out", label: label, dataType: dataType }
}

/** Tests if two ports can be wired together
  */
export function isCompatiblePort
  (output: outputPort<any>, input: inputPort<any>) {
  //TODO: account for cyclic graphs/self assignment (those might be same thing?)
  return output.dataType === input.dataType
}




export interface BaseNode {
  label: string
  // inputPorts: Parameters<T>
  // outputPorts: ReturnType<T>
  compute: nodeCompute
}

function multiply(...l: number[]) {
  return l.reduce((p, c) => (p * c))
}
console.log(multiply(1, 2, 3))

//type addable = number | number[] | string

// type myType = string | number | number[]
// type myCompute<T extends myType, U extends myType> = (...a: T[]) => U
//
// const a: myCompute<number, number> = (a: number, b: number) => {
//   return a + b
// }
//
// type myCompute2 = (...a: (string | number)[]) => string | number
//
// const a2: myCompute2 = (a: number, b: number) => {
//   return a + b
// }


type NodeInOut = string | number | number[] | boolean

type ComputeInput = {} | { [key: string]: NodeInOut }
type ComputeOutput = NodeInOut | { [key: string]: NodeInOut }

type Compute<Input extends ComputeInput, Output extends ComputeOutput> = (input: Input) => Output

type add_input = {
  a: number,
  b: number
}

type NodeTemplate<Input extends ComputeInput, Output extends ComputeOutput> = {
  label: string
  compute: Compute<Input, Output>
}

function createNode<Input extends ComputeInput, Output extends NodeInOut>(label: string, f: (i: Input) => Output): NodeTemplate<Input, Output> {
  return {
    label: label,
    compute: f
  }
}


const a3 = ({ a, b }: add_input) => {
  return a + b
}


const add_node: NodeTemplate<add_input, number> = { label: "add", compute: a3 }

const not = createNode("not", (a: boolean) => { return !a })


const a4 = () => {
  return false
}

const falseNode: NodeTemplate<Parameters<typeof a4>, ReturnType<typeof a4>> = { label: "false", compute: a4 }
const falseNode2 = createNode("false", a4)





// old implementation
export interface addNode extends BaseNode {
  compute: typeof a3
}

const addInA = createInput("a", "number")
addInA.heldValue = 7
const addInB = createInput("b", "number")
addInB.heldValue = 4
const addOutSum = createOutput("sum", "number")

const addNodeInstance: addNode<[inputPort<"number">, inputPort<"number">], [outputPort<"number">]> = {
  label: "add",
  inputPorts: [addInA, addInB],
  outputPorts: [addOutSum],
  compute: (inputs: [inputPort<"number">, inputPort<"number">]) => {
    console.log(inputs)
    const a = inputs[0].heldValue
    const b = inputs[1].heldValue
    if (a == undefined || b == undefined) {
      throw Error("inputs are still undefined")
    }
    addOutSum.heldValue = a + b
    return [addOutSum]
  }
}

console.log(addNodeInstance.compute([addInA, addInB]))

