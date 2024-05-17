import { createOutput, createInput } from './node'

// the interface can get updated over time, or by API consumers?
// interface PortDataTypes {
//   coolType: boolean
// }
const out1 = createOutput("out1", "number")
const in1 = createInput("in1", "number")

////
