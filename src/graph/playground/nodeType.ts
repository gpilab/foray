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
