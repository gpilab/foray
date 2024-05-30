import { Observable, ReplaySubject, combineLatest } from 'rxjs';
import { map, tap } from 'rxjs/operators';


interface PortDataTypes {
  "string": string
  "number": number
  "numberArray": number[]
  "boolean": boolean
}

// type DataTypeLabel = keyof PortDataTypes
type DataType = PortDataTypes[keyof PortDataTypes]

// type NodeDef<T extends [string, Labels<PortDataTypes>][], Output extends keyof PortDataTypes> = {//, K extends keyof T = keyof T> = {
//   compute: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => PortDataTypes[Output]
//   inputs: { [k in keyof T]: T[k] }
//   outputType: Output
// }
//
// function createNode<T extends [string, Labels<PortDataTypes>][]
//   , Output extends keyof PortDataTypes>
//   (compute: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => PortDataTypes[Output]
//     , inputs: { [k in keyof T]: T[k] }
//     , outputType: Output): NodeDef<T, Output> {
//   return { compute, inputs, outputType }
// }

// let n2: NodeDef<[["a", "number"], ["b", "number"], ["c", "string"]], "number">
// n2 = {
//   compute: (ab: number, b: number, c: string) => {
//     console.log(ab, b, c);
//     return 1
//   },
//   inputs: [["a", "number"], ["b", "number"], ["c", "string"]],
//   outputType: "number"
// }
//
// const n3 = createNode(
//   (ab: number, b: number, c: string) => {
//     console.log(ab, b, c);
//     return 1
//   },
//   [["a", "number"], ["b", "number"], ["c", "string"]],
//   "number"
// )
// n3

type KeyValueTuple = [string, Labels<PortDataTypes>][]

export class Node<T extends KeyValueTuple = any
  , OutputType extends keyof PortDataTypes = any
  , Output extends PortDataTypes[OutputType] = any
  , InLabels extends T[number][0] = string
  , InTypes extends PortDataTypes[T[number][1]] = any> {
  public currentValue: Output | undefined
  public outputStream$: Observable<Output>
  public inputStreams: Map<InLabels, ReplaySubject<DataType>>;
  private inputMap: Map<InLabels, InTypes>
  private computeInputToOutput: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => Output

  constructor(computeInputToOutput: (...args: { [k in keyof T]: PortDataTypes[T[k][1]] }) => Output,
    inputMap: { [k in keyof T]: T[k] },
    public outputType: OutputType,
    public id: string = "default") {
    this.computeInputToOutput = computeInputToOutput;
    //const numInputs = computeInputToOutput.length;
    const inputArray: ReplaySubject<any>[] = []

    this.inputStreams = new Map<InLabels, ReplaySubject<DataType>>()
    this.inputMap = new Map<InLabels, InTypes>()
    inputMap.forEach(([label, type]) => {
      const subject = new ReplaySubject<any>(1)
      this.inputStreams.set(label as InLabels, subject)
      this.inputMap.set(label as InLabels, type as InTypes)
      inputArray.push(subject)
    }
    )
    //Array.from({ length: numInputs }, () => new ReplaySubject<any>(1));

    this.outputStream$ = combineLatest(inputArray).pipe(
      map(inputs => {
        const value = this.computeInputToOutput(...inputs as { [k in keyof T]: PortDataTypes[T[k][1]]; }); // TODO fix as
        //console.log(`Processing (${inputs}) through ${computeInputToOutput}....${value}`);
        return value;
      }), tap((output) => this.currentValue = output));
    this.currentValue = undefined
  }

  getInputStream(key: InLabels): ReplaySubject<DataType> {
    return this.inputStreams.get(key)!
  }
  getInputType(key: InLabels): InTypes {
    return this.inputMap.get(key)!
  }
}



export class Graph {
  private nodeAdjacencies: Map<Node, Node[]> = new Map();

  // add output type restriction
  addNode/**<T extends KeyValueTuple, U extends keyof T>**/(node: Node, connections: { targetNode: Node/**<T>**/, targetInputLabel: string/**keyof KeyValueTuple/**U**/ }[] = []) {
    this.nodeAdjacencies.set(node, []);
    connections.forEach(({ targetNode, targetInputLabel }) => {
      this.connectNodes(node, targetNode, targetInputLabel);
    });
  }
  addNodes(nodes: Node[]) {
    nodes.forEach((node) => this.addNode(node))
  }

  // Connect output of one node to input of another node
  connectNodes/**<T extends KeyValueTuple, U extends keyof T>**/(sourceNode: Node, targetNode: Node/**<T>**/, targetInputLabel: string/**U**/) {
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
