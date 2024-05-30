/** `(Label: Type)` pairs for all possible data types that can be passed through ports
 * `Label`s are used as generic strings for creating types
 */
interface PortDataTypes {
  "string": string
  "number": number
  "numberArray": number[]
  "boolean": boolean
}

// An actual instance of data that can be on a port 
// The keys of PortDataType are removed, leaving just a union of the data type
// not currently needed
//TODO should this be readonly? It would be preferrable if node computations were required to be pure
type Values<Type> = Type[keyof Type]
type Labels<Type> = keyof Type

type NodeInOutTypes<T> = Values<T>
type NodeInOutLabels<T> = Labels<T>

export type ComputeInput<T> = { [key: string]: T }
export type ComputeOutput<T> = { [key: string]: T }
export type InputRecord<T> = Record<string, Labels<T>>


interface allCompute<TL, TV, UL, UV> {
  inputRecord: { TL[number]: TV }
outputRecord: { [key: UL]: UV }
compute: (input: { [key: TL]: TV }) => { [key: UL]: UV }
}

const add = ({ a, b }: { a: "number", b: "number" }) => {
  return { sum: a + b }
}

type IR = { a: number, b: number }
type OR = { sum: number }
type IRV = Values<IR>
type IRL = Labels<IR>
type IRC = { a: "number", b: "number" }
type ORC = { sum: "number" }

const test: allCompute<IRC, ORC> = {
  inputRecord: { a: "number", b: "number" },
  outputRecord: { sum: "number" },
  compute: add

}


//type InOutLabels = keyof PortDataTypes

function getTypeLabel<T>(type: T): string {
  return typeof type
}


type Compute<IR extends Record<string>, ComputeInput, Output extends ComputeOutput> = (input: Input) => Output


export type NodeTemplate<Input extends ComputeInput, Output extends ComputeOutput> = {
  label: string
  compute: Compute<Input, Output>
  inputs: InputRecord
  outputs: InputRecord
}


export function createNodeTemplate
  <IR extends InputRecord, Input extends ComputeInput, Output extends ComputeOutput>
  (f: (i: Input) => Output, inputs: InputRecord, outputs: InputRecord, label: string): NodeTemplate<Input, Output> {
  if (label === undefined) {
    label = f.name
  }
  inputs
  //const string = f.arguments
  console.log(getTypeLabel(f))
  console.log(f)
  //console.log(f.arguments)
  console.log(f.prototype)
  return {
    label: label,
    inputs: inputs,
    outputs: outputs,
    compute: f
  }
}


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


const add_in_a = createInput("a", "number")
const add_in_b = createInput("b", "number")
//const add_out_sum = createOutput("sum", "number")

const const_num = createOutput("const1", "number")

add_in_a.heldValue = 1
add_in_b.heldValue = 2

console.log(isCompatiblePort(const_num, add_in_a))




const add_input2: Record<string, keyof PortDataTypes> = {
  a: "number",
  b: "number"
}
const add_output2: Record<string, keyof PortDataTypes> = {
  sum: "number",
}

const a3 = ({ a, b }: { a: number, b: number }) => {
  return { sum: a + b }
}

function aligns<Ti, To, U, V>(f: (input: Ti) => To, i: U, o: V) {
  return { f: f, i: i, o: o }
}
aligns(a3, add_input2, add_output2)

// const add_node: NodeTemplate<add_input, number> = { label: "add", compute: a3 }
// console.log(add_node)
const add_node2 = createNodeTemplate(a3, add_input2, add_output2, "add")
console.log(add_node2)



