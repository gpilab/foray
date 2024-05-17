

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


// An actual instance of data that can be on a port 
// The keys of PortDataType are removed, leaving just a union of the data type
// not currently needed
//type PortDataInstance<T extends keyof PortDataTypes> = PortDataTypes[T]

/** Common values that input and output ports share
 */
interface BasePort<T extends keyof PortDataTypes> {
  label: string
  dataType: T
  heldValue?: PortDataTypes[T]
}

/** Defines the shape of values that a  node can accept as input
 */
interface inputPort<T extends keyof PortDataTypes> extends BasePort<T> {
  ioType: "in"
}

/** Defines the shape of values that a the a node can produce
 */
interface outputPort<T extends keyof PortDataTypes> extends BasePort<T> {
  ioType: "out"
}


export function createInput<T extends keyof PortDataTypes>(label: string, dataType: T): inputPort<T> {
  return { ioType: "in", label: label, dataType: dataType }
}

export function createOutput<T extends keyof PortDataTypes>(label: string, dataType: T): outputPort<T> {
  return { ioType: "out", label: label, dataType: dataType }
}

/** Tests if two ports can be wired together
  */
export function isCompatiblePort(output: outputPort<any>, input: inputPort<any>) {
  //TODO: account for cyclic graphs/self assignment (those might be same thing?)
  return output.dataType === input.dataType
}

const testOutPortA = createOutput("myLabelOut", "number")
const testInPortA = createInput("myLabelIn", "number")
const testOutPortC = createOutput("myCustomOut", "vec")

testInPortA.heldValue = 1
testInPortA.heldValue = 2
testOutPortC.heldValue = [1, 2, 3]

console.log(testOutPortA, testOutPortA.dataType)
console.log(testInPortA)
console.log(isCompatiblePort(testOutPortA, testInPortA))

console.log(testOutPortC)
console.log(testInPortA)
console.log(isCompatiblePort(testOutPortC, testInPortA))


export interface BaseNode<T extends inputPort<any>[], U extends outputPort<any>[]> {
  label: string
  id: string
  inputPorts: T
  outputPorts: U
  compute: (inputs: T) => U
}


export interface addNode<T extends [inputPort<"number">, inputPort<"number">], U extends [sum: outputPort<"number">]> extends BaseNode<T, U> {
  label: "add"
  compute: (inputs: T) => U
}

const addInA = createInput("a", "number")
addInA.heldValue = 7
const addInB = createInput("b", "number")
addInB.heldValue = 4
const addOutSum = createOutput("sum", "number")

const addNodeInstance: addNode<[inputPort<"number">, inputPort<"number">], [outputPort<"number">]> = {
  label: "add",
  id: "abc123",
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

