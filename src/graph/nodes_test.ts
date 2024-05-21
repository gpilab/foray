import { createOutput, createInput, isCompatiblePort } from './node'

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

//TODO setup jest testing

// the interface can get updated over time, or by API consumers?
// interface PortDataTypes {
//   coolType: boolean
// }


