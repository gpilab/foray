//This is the Node type paired down to it's most minimal form

type ExampleNodeType<T extends [string, Labels<PortDataTypes>][], Output extends keyof PortDataTypes> = {
  compute: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => PortDataTypes[Output]
  inputs: { [k in keyof T]: T[k] }
  outputType: Output
}

function createExampleNode<T extends [string, Labels<PortDataTypes>][]
  , Output extends keyof PortDataTypes>
  (compute: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => PortDataTypes[Output]
    , inputs: { [k in keyof T]: T[k] }
    , outputType: Output): ExampleNodeType<T, Output> {
  return { compute, inputs, outputType }
}

let n2: ExampleNodeType<[["a", "number"], ["b", "number"], ["c", "string"]], "number">
n2 = {
  compute: (ab: number, b: number, c: string) => {
    console.log(ab, b, c);
    return 1
  },
  inputs: [["a", "number"], ["b", "number"], ["c", "string"]],
  outputType: "number"
}

const n3 = createExampleNode(
  (ab: number, b: number, c: string) => {
    console.log(ab, b, c);
    return 1
  },
  [["a", "number"], ["b", "number"], ["c", "string"]],
  "number"
)
n3



const n = [["x", "number"], ["s", "string"]] as const

type N = typeof n

type NT = InputTypes<N>
let nt: NT
nt = [1, "a"]

type NTU = InputTypesUnion<N> // string | number
let ntu: NTU
ntu = 1
ntu = "a"
//@ts-expect-error
ntu = { a: 1 }

type NTL = InputTypeLabels<N> // ["number", "string"]
let ntl: NTL
ntl = ["number", "string"]
//@ts-expect-error
ntl = ["string", "number"]
//@ts-expect-error
ntl = ["number"]

type NTLU = InputTypeLabelsUnion<N> //  "string" | "number"
let ntlu: NTLU
ntlu = "number"
ntlu = "string"
//@ts-expect-error
ntlu = "boolean"

type NL = InputKeys<N> // ["x","s"]
let nl: NL
nl = ["x", "s"]
//@ts-expect-error
nl = ["s", "x"]

type NLU = InputKeysUnion<N> // "x" | "s"
let nlu: NLU
nlu = "s"
nlu = "x"
//@ts-expect-error
nlu = "z"
