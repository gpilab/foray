import { Observable, ReplaySubject, combineLatest } from 'rxjs';
import { map, tap } from 'rxjs/operators';


interface ValidPortTypes {
  "string": string
  "number": number
  "numberArray": number[]
  "boolean": boolean
}

/** union of all valid  PortType */
type PortTypeKeys = keyof ValidPortTypes

/** union of all valid data types */
type PortTypes = ValidPortTypes[keyof ValidPortTypes]

/** Node inputs are described by a unique label and a data type. Data types are defined as the keys of ValidPortTypes*/
type InPort = [string, PortTypeKeys]
type NodeInputs = InPort[]

export class Node<T extends NodeInputs = any
  , OutputType extends keyof ValidPortTypes = any
  , Output extends ValidPortTypes[OutputType] = any
  , InLabels extends T[number][0] = string
  , InTypes extends ValidPortTypes[T[number][1]] = any> {

  public currentValue: Output | undefined
  public outputStream$: Observable<Output>
  public inputStreams: Map<InLabels, ReplaySubject<PortTypes>>;
  private inputMap: Map<InLabels, InTypes>
  private computeInputToOutput: (...args: { [k in keyof T]: ValidPortTypes[T[k][1]] }) => Output

  constructor(computeInputToOutput: (...args: { [k in keyof T]: ValidPortTypes[T[k][1]] }) => Output,
    inputMap: { [k in keyof T]: T[k] },
    public outputType: OutputType,
    public id: string = "default") {
    this.computeInputToOutput = computeInputToOutput;
    //const numInputs = computeInputToOutput.length;
    const inputArray: ReplaySubject<PortTypes>[] = []
    //Array.from({ length: numInputs }, () => new ReplaySubject<any>(1));
    //
    this.inputStreams = new Map<InLabels, ReplaySubject<PortTypes>>()
    this.inputMap = new Map<InLabels, InTypes>()
    inputMap.forEach(([label, type]) => {
      const subject = new ReplaySubject<PortTypes>(1)
      this.inputStreams.set(label as InLabels, subject)
      this.inputMap.set(label as InLabels, type as InTypes)
      inputArray.push(subject)
    }
    )


    this.outputStream$ = combineLatest(inputArray).pipe(
      map(inputs => {
        const value = this.computeInputToOutput(...inputs as { [k in keyof T]: ValidPortTypes[T[k][1]]; }); // TODO fix as
        //console.log(`Processing (${inputs}) through ${computeInputToOutput}....${value}`);
        return value;
      }), tap((output) => this.currentValue = output));
    this.currentValue = undefined
  }

  getInputStream(key: InLabels): ReplaySubject<PortTypes> {
    return this.inputStreams.get(key)!
  }
  getInputType(key: InLabels): InTypes {
    return this.inputMap.get(key)!
  }
}



export class Graph {
  private nodeAdjacencies: Map<Node, Node[]> = new Map();

  // TODO: add output type restriction
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

  getConnections(node: Node) {
    return this.nodeAdjacencies.get(node)
  }
}
