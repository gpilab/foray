type CustomType = {
  a: string
  b: boolean
}

// All possible data types that can be passed through ports
interface PortDataType {
  number: number
  vec: number[]
  string: string
  customType: CustomType
}
// the interface can get updated over time, or by API consumers?
interface PortDataType {
  coolType: boolean
}
// An actual instance of data that can be on a port 
// The keys of PortDataType are removed, leaving just a union of the data type
type PortDataInstance<T extends keyof PortDataType> = PortDataType[T]


interface inputPort<T extends keyof PortDataType> {
  ioType: "in"
  label: string
  dataType: T
  heldValue?: PortDataInstance<T>
}
interface outputPort<T extends keyof PortDataType> {
  ioType: "out"
  label: string
  dataType: T
  heldValue?: PortDataInstance<T>
}


function createInput<T extends keyof PortDataType>(label: string, dataType: T): inputPort<T> {
  return { ioType: "in", label: label, dataType: dataType }
}

function createOutput<T extends keyof PortDataType>(label: string, dataType: T): outputPort<T> {
  return { ioType: "out", label: label, dataType: dataType }
}

function isCompatiblePort(output: outputPort<any>, input: inputPort<any>) {
  return output.dataType === input.dataType
}

const testOutPortA = createOutput("myLabelOut", "number")
const testInPortA = createInput("myLabelIn", "number")
const testOutPortC = createOutput("myCustomOut", "vec")
const testOutPortD = createOutput("coolTypeLabel", "coolType")

testInPortA.heldValue = 1
testInPortA.heldValue = 2
testOutPortC.heldValue = [1, 2, 3]

console.log(testOutPortA, testOutPortA.dataType)
console.log(testInPortA)
console.log(isCompatiblePort(testOutPortA, testInPortA))

console.log(testOutPortC)
console.log(testInPortA)
console.log(isCompatiblePort(testOutPortC, testInPortA))


