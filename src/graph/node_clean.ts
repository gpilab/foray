/** Key:Label
 */
type PortKeyLabel = {
  number: "number"
  numberArray: "numberArray"
  boolean: "boolean"
}
/** Key:Type
 */
type PortKeyType = {
  number: number
  numberArray: number[]
  boolean: boolean
}



type IOLabel = {
  [portType: string]: PortKeyLabel[keyof PortKeyLabel]
}

/** takes a label (type that extends IOLabel), and converts it to something that extends GenericInputType
 */
type IOType<L extends IOLabel> =
  { [k in keyof L]: PortKeyType[L[k]] }

type f<IL extends IOLabel, OL extends IOLabel> = { (inputs: IOType<IL>): IOType<OL> }

interface Binary_Num_Op_Labels extends IOLabel {
  a: "number"
  b: "number"
}

const Binary_Num_Op_Labels_Inst: Binary_Num_Op_Labels = {
  a: "number",
  b: "number"
}


interface Num_Result_Label extends IOLabel {
  result: "number"
}
const Num_Result_Label_Inst: Num_Result_Label = {
  result: "number"
}

type Binary_Input_Types = IOType<Binary_Num_Op_Labels>

const add = (inputs: Binary_Input_Types) => {
  return { result: inputs.a + inputs.b }
}

const subtract: f<Binary_Num_Op_Labels, Num_Result_Label> = (inputs: Binary_Input_Types) => {
  return { result: inputs.a - inputs.b }
}

type NodeTemplate<IL extends IOLabel, OL extends IOLabel> = {
  inputs: IL
  outputs: OL
  compute: f<IL, OL>
}

const add_node: NodeTemplate<Binary_Num_Op_Labels, Num_Result_Label> = {
  inputs: Binary_Num_Op_Labels_Inst,
  outputs: Num_Result_Label_Inst,
  compute: add
}

function createNodeTemplate<IL extends IOLabel, OL extends IOLabel>
  (inputs: IL, outputs: OL, compute: f<IL, OL>): NodeTemplate<IL, OL> {

  return {
    inputs: inputs,
    outputs: outputs,
    compute: compute
  }
}

// interface Bad_Bin extends IOLabel {
//   a: "number"
//   c: "numberArray"
// }
//
// const Bad_Bin_Inst: Bad_Bin = {
//   a: "number",
//   c: "numberArray"
// }
//
//
// const Dup_Infer_Inst = { a: "number", b: "number" }
// interface Dup extends IOLabel = typeof Dup_Infer_Inst
// interface DRes extends IOLabel { result: "number" }
// const DRes_Infer_Inst = { result: "number" }
//
// const add_dup = (inputs: { a: number, b: number }) => {
//   return { result: inputs.a + inputs.b }
// }

//maybe this is fine for now
interface Bin extends IOLabel { a: "number", b: "number" }
interface Res extends IOLabel { result: "number" }
const Bin_Infer_Inst: Bin = { a: "number", b: "number" }
const Res_Infer_Inst: Res = { result: "number" }

const add_infer = (inputs: { a: number, b: number }) => {
  return { result: inputs.a + inputs.b }
}

const newAdd = createNodeTemplate(Binary_Num_Op_Labels_Inst, Num_Result_Label_Inst, add) // This seems to work!!
// const newAdd_fail = createNodeTemplate(Bad_Bin_Inst, Num_Result_Label_Inst, add)
// const newAdd_dup = createNodeTemplate<Dup, DRes>(Dup_Infer_Inst, DRes_Infer_Inst, add_dup)
const newAdd_infer = createNodeTemplate(Bin_Infer_Inst, Res_Infer_Inst, add_infer)// maybe this is fine for now


//maybe this is fine for now
interface Single_Bool extends IOLabel { a: "boolean" }
interface Bin_Bool extends IOLabel { a: "boolean", b: "boolean" }
interface Result_Bool extends IOLabel { result: "boolean" }
const Single_Bool_Inst: Single_Bool = { a: "boolean" }
const Bin_Bool_Inst: Bin_Bool = { a: "boolean", b: "boolean" }
const Result_Bool_Inst: Result_Bool = { result: "boolean" }

const and = (inputs: { a: boolean, b: boolean }) => {
  return { result: inputs.a && inputs.b }
}
const andTemplate = createNodeTemplate(Bin_Bool_Inst, Result_Bool_Inst, and)

const or = (inputs: { a: boolean, b: boolean }) => {
  return { result: inputs.a || inputs.b }
}
const orTemplate = createNodeTemplate(Bin_Bool_Inst, Result_Bool_Inst, or)

const not = (inputs: { a: boolean }) => {
  return {
    result: !inputs.a
  }
}
const notTemplate = createNodeTemplate(Single_Bool_Inst, Result_Bool_Inst, not)
console.log(andTemplate)
//TODO instantiate template

