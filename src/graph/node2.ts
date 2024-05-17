type CustomType = {
  a: string
  b: boolean
}


// All possible data types that can be passed through ports
type PortDataType =
  {
    number: number
    vec: number[]
    string: string
    customType: CustomType
  }

// An actual instance of data that can be on a port 
// The keys of PortDataType are removed, leaving just a union of the data type
type PortDataInstance<T extends keyof PortDataType> = PortDataType[T]

type PortDataInstance2 = number | number[] | string | CustomType
type PortDataLabels = "number" | "number[]" | "string" | "CustomType"



type PortDataType2<T extends PortDataLabels> =
  {
    type: T
  }
// interface MPort {
//   ioType: "in" | "out"
//   label: string
//   dataType: keyof PortDataType
// }


interface inputPort2<T extends keyof PortDataType> {
  ioType: "in"
  label: string
  dataType: T
  heldValue?: PortDataInstance<T>
}
interface outputPort2<T extends keyof PortDataType> {
  ioType: "out"
  label: string
  dataType: T
  heldValue?: PortDataInstance<T>
}




function createInput2<T extends keyof PortDataType>(label: string, dataType: T): inputPort2<T> {
  return { ioType: "in", label: label, dataType: dataType } as const
}

function createOutput2<T extends keyof PortDataType>(label: string, dataType: T): outputPort2<T> {
  return { ioType: "out", label: label, dataType: dataType } as const
}

function isCompatiblePort2(output: outputPort2<any>, input: inputPort2<any>) {
  return output.dataType === input.dataType
}

const testOutPortA = createOutput2("myLabelOut", "number")
const testInPortA = createInput2("myLabelIn", "number")
const testOutPortC = createOutput("myCustomOut", "vec")

testInPortA.heldValue = 1
testInPortA.heldValue = 2
testOutPortC.heldValue = [1, 2, 3]

console.log(testOutPortA)
console.log(testInPortA)
console.log(isCompatiblePort2(testOutPortA, testInPortA))
console.log(testOutPortC)
console.log(testInPortA)
console.log(isCompatiblePort2(testOutPortC, testInPortA))

const testPort: inputPort<"number", number> = {
  ioType: "in",
  label: "abc",
  dataType: "number"
}




interface outputPort<U extends keyof PortDataType, T extends PortDataInstance<U>> {
  ioType: "out"
  label: string
  dataType: U
  heldValue?: T
}
interface inputPort<U extends keyof PortDataType, T extends PortDataInstance<U>> {
  ioType: "in"
  label: string
  dataType: U
  heldValue?: T
}

function createInput<U extends keyof PortDataType, T extends PortDataInstance<U>>(label: string, dataType: U): inputPort<U, T> {
  return { ioType: "in", label: label, dataType: dataType } as const
}
function createOutput<U extends keyof PortDataType, T extends PortDataInstance<U>>(label: string, dataType: U): outputPort<U, T> {
  return { ioType: "out", label: label, dataType: dataType } as const
}
// function createInput<T extends PortDataType2>(label: string, dataType: keyof T): inputPort<T> {
//   return { ioType: "in", label: label, dataType: dataType } as const
// }
//
// function createOutput(label: string, dataType: keyof PortDataType): outputPort {
//   return { ioType: "out", label: label, dataType: dataType } as const
// }

// function isCompatiblePort(output: outputPort<any>, input: inputPort<any, any>) {
//   return output.dataType === input.dataType
// }
//
// const constO = createOutput("seven", "number")
// const const1 = createOutput("four", "number")
// const a = const1.dataType
// const1.heldValue = "abc"
//
//
// const addA = createInput("a", "number")
// const addB = createInput("b", "number")
// const addO = createInput("sum", "number")
//
// const customA = createInput("a", "customType")
//
// addO.heldValue = { a: "abc", b: false }
// addO.heldValue = "abc"
// addO.heldValue = 1
//
//
// console.log("add data type:", addO.dataType)
// console.log("add data instance:", addO.heldValue)
// console.log("custom data type:", customA.dataType)
//
//
//
// const areCompatible1 = isCompatiblePort(constO, addA)
// console.log(areCompatible1)
// const areCompatible2 = isCompatiblePort(constO, customA)
// console.log(areCompatible2)
//
//
//
