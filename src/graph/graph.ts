import { Observable, ReplaySubject, combineLatest } from 'rxjs';
import { map, tap } from 'rxjs/operators';

// type DataType = number | string;
// type DataTypeLabel = "number" | "string";

// type DataMap = {
//   number: "number"
//   string: "string"
// }


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

type DataType = Values<PortDataTypes>
type DataTypeLabel = Labels<PortDataTypes>


export type ComputeInput<T> = { [key: string]: T }
export type ComputeOutput<T> = { [key: string]: T }


type InputTypes = DataType[]
type MyInputValues<T extends InputTypes> = T

let myInputs: MyInputValues<[number, number, string]>
myInputs = [5, 6, "abc"]
//@ts-expect-error
myInputs = [5, "123", "456"]


type InputTypeLabels = DataTypeLabel[]
type MyInputTypeLabels<T extends InputTypeLabels> = T
let myInputTypeLabels: MyInputTypeLabels<["number", "number", "string"]>
myInputTypeLabels = ["number", "number", "string"]

type InputLabels = string[]
type MyInputLabels<T extends InputLabels> = T
let myInputLabels: MyInputLabels<["x", "y", "z"]>
myInputLabels = ["x", "y", "z"]

// type InferedTypes<T extends InputTypeLabels> = keyof T[number] extends DataType ? T[number][] : never
// let exInferred: InferedTypes<typeof myInputTypeLabels>
// exInferred = [1, 2, "a"]
type InferedTypes<I extends PortDataTypes, T extends keyof I, U extends keyof T> = { [K in keyof U]: U[K] }
let exInferred: InferedTypes<typeof myInputTypeLabels>
exInferred = [1, 2, "a"]



let targetInffered: (string | number)[]
targetInffered = [1, 2, "a"]







//
// type NodeDef<T extends (keyof PortDataTypes)[], Output extends keyof PortDataTypes> = {//, K extends keyof T = keyof T> = {
//   f: (...args: { [k in keyof T]: PortDataTypes[T[k]] }) => PortDataTypes[Output]
//   inputTypes: { [k in keyof T]: T[k] }
//   inputLabels: { [k in keyof T]: string }
//   outputType: Output
// }
//
//
// let n: NodeDef<["number", "number", "string"], "number">
// n = {
//   f: (a: number, b: number, c: string) => {
//     console.log(a, b, c);
//     return 1
//   },
//   inputTypes: ["number", "number", "string"],
//   inputLabels: ["x", "y", "z"],
//   outputType: "number"
// }


type NodeDef<T extends [string, Labels<PortDataTypes>][], Output extends keyof PortDataTypes> = {//, K extends keyof T = keyof T> = {
  compute: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => PortDataTypes[Output]
  inputs: { [k in keyof T]: T[k] }
  outputType: Output
}

function createNode<T extends [string, Labels<PortDataTypes>][]
  , Output extends keyof PortDataTypes>
  (compute: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => PortDataTypes[Output]
    , inputs: { [k in keyof T]: T[k] }
    , outputType: Output): NodeDef<T, Output> {
  return { compute, inputs, outputType }
}

let n2: NodeDef<[["a", "number"], ["b", "number"], ["c", "string"]], "number">
n2 = {
  compute: (ab: number, b: number, c: string) => {
    console.log(ab, b, c);
    return 1
  },
  inputs: [["a", "number"], ["b", "number"], ["c", "string"]],
  outputType: "number"
}

const n3 = createNode(
  (ab: number, b: number, c: string) => {
    console.log(ab, b, c);
    return 1
  },
  [["a", "number"], ["b", "number"], ["c", "string"]],
  "number"
)




type InputMap<T extends Labels<PortDataTypes>> = { [s: string]: T }
let im: InputMap<"number" | "string">
im = { x: "number", y: "string" } as const

type InputMapValues<U extends Labels<PortDataTypes>, T extends InputMap<U>> = keyof T
let im2: InputMapValues<"number" | "string", typeof im>
im2 = { x: "number", y: "string" } as const

type InputArgsMap<U extends InputMap<Labels<PortDataTypes>>, T extends Labels<U>> = T extends Labels<U> ? Record<T, any> : never
let iam: InputArgsMap<typeof im, Labels<typeof im>>
iam = { d: "number", y: "string" }



type Args<T extends Labels<PortDataTypes>> = [string: InputArgsMap<T>[string]]




type Func<T extends Labels<PortDataTypes>> = (...args: Args<T>) => any

const myFunc: Func<{ x: number, y: number }> = (x: number, y: number) => x + y



export class Node<T extends InputMap<DataTypeLabel> = InputMap<DataTypeLabel>, U extends DataTypeLabel = DataTypeLabel> {
  public currentValue: DataType | undefined
  public outputStream$: Observable<DataType>
  public inputStreams: Map<keyof T, ReplaySubject<DataType>>;
  private inputMap: Map<keyof T, DataTypeLabel>
  private computeInputToOutput: (...args: Args<T>) => DataType

  constructor(computeInputToOutput: (...args: Args<T>) => DataType,
    inputMap: T,
    public outputType: U,
    public id: string = "default") {
    this.computeInputToOutput = computeInputToOutput;
    //const numInputs = computeInputToOutput.length;
    const inputArray: ReplaySubject<any>[] = []
    this.inputStreams = new Map<keyof T, ReplaySubject<DataType>>()
    this.inputMap = new Map<keyof T, DataTypeLabel>()
    Object.keys(inputMap).forEach((key: string) => {
      const subject = new ReplaySubject<any>(1)
      this.inputStreams.set(key, subject)
      this.inputMap.set(key, inputMap[key])
      inputArray.push(subject)
    }
    )
    //Array.from({ length: numInputs }, () => new ReplaySubject<any>(1));

    this.outputStream$ = combineLatest(inputArray).pipe(
      map(inputs => {
        const value = this.computeInputToOutput(...inputs);
        //console.log(`Processing (${inputs}) through ${computeInputToOutput}....${value}`);
        return value;
      }), tap((output) => this.currentValue = output));
    this.currentValue = undefined
  }

  getInputStream(key: keyof T): ReplaySubject<DataType> {
    return this.inputStreams.get(key)!
  }
  getInputType(key: keyof T): DataType {
    return this.inputMap.get(key)!
  }
}



export class Graph {
  private nodeAdjacencies: Map<Node<any>, Node<any>[]> = new Map();

  addNode<T extends InputMap, U extends keyof T>(node: Node<any>, connections: { targetNode: Node<T>, targetInputLabel: U }[] = []) {
    this.nodeAdjacencies.set(node, []);
    connections.forEach(({ targetNode, targetInputLabel }) => {
      this.connectNodes(node, targetNode, targetInputLabel);
    });
  }
  addNodes(nodes: Node<any>[]) {
    nodes.forEach((node) => this.addNode(node))
  }

  // Connect output of one node to input of another node
  connectNodes<T extends InputMap, U extends keyof T>(sourceNode: Node<any>, targetNode: Node<T>, targetInputLabel: U) {
    const sourceNodeAdjacencies = this.nodeAdjacencies.get(sourceNode)
    if (sourceNodeAdjacencies == undefined) {
      throw Error("Source node not present in graph");
    }
    if (!this.nodeAdjacencies.has(targetNode)) {
      throw Error("Target node not present in graph");
    }
    if (sourceNode.outputType != targetNode.getInputType(targetInputLabel)) {
      throw Error("Attempted to connect nodes of type ${sourceNode.outputType} and ${targetNode.inputMap.get(targetInputLabel))}")
    }
    // Add target node to the adjacency list of the source node
    sourceNodeAdjacencies.push(targetNode);

    // Subscribe the output of the source node to the input of the target node
    sourceNode.outputStream$.subscribe(output => {
      const input = targetNode.inputStreams.get(targetInputLabel)
      if (input === undefined) {
        throw Error("Attempted to access input label ${targetInputLabel} on node ${node}")
      }
      input.next(output);
    });
  }

  getConnections(node: Node<any>) {
    return this.nodeAdjacencies.get(node)
  }
}
