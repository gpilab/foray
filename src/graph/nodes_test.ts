//import { createOutput, createInput, isCompatiblePort } from './node'
import { createInput, createNodeTemplate, createOutput, isCompatiblePort } from './node'

const testOutPortA = createOutput("myLabelOut", "number")
const testInPortA = createInput("myLabelIn", "number")
const testOutPortC = createOutput("myCustomOut", "numberArray")

testInPortA.heldValue = 1
testInPortA.heldValue = 2
testOutPortC.heldValue = [1, 2, 3]

console.log(testOutPortA, testOutPortA.dataType)
console.log(testInPortA)
console.log(isCompatiblePort(testOutPortA, testInPortA))

console.log(testOutPortC)
console.log(testInPortA)
console.log(isCompatiblePort(testOutPortC, testInPortA))

const add_input = {
  a: "number",
  b: "number"
}



const a3 = ({ a, b }: typeof add_input) => {
  return a + b
}


// const add_node: NodeTemplate<add_input, number> = { label: "add", compute: a3 }
// console.log(add_node)
const add_node2 = createNodeTemplate(a3)
console.log(add_node2)

function not(a: boolean) {
  return !a
}

const notNode = createNodeTemplate(not)
console.log(notNode)

const a4 = () => {
  return false
}

//const falseNode: NodeTemplate<Parameters<typeof a4>, ReturnType<typeof a4>> = { label: "false", compute: a4 }
//console.log(falseNode)
const falseNode2 = createNodeTemplate(a4, "false")
console.log(falseNode2)


//TODO setup jest testing

// the interface can get updated over time, or by API consumers?
// interface PortDataTypes {
//   coolType: boolean
// }


