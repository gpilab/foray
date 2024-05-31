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
// type PortTypes = ValidPortTypes[keyof ValidPortTypes]

/** Node inputs are described by a unique label and a data type. Data types are defined as the keys of ValidPortTypes */
type InPort = [string, PortTypeKeys]
type NodeInputs = InPort[]

/** Compute outputs based on inputs */
// type ComputeInputs<T extends NodeInputs> = { [k in keyof T]: ValidPortTypes[T[k][1]] }


type InputDataTypes<T extends NodeInputs> = {
  [K in keyof T]: T[K] extends [string, infer U] ? (U extends PortTypeKeys ? ValidPortTypes[U] : never) : never;
};


export class Node<I extends NodeInputs = any, O extends PortTypeKeys = any> {
  public id: string
  public inputStreams: Map<string, ReplaySubject<ValidPortTypes[PortTypeKeys]>>
  public outputStream$: Observable<ValidPortTypes[O]>
  public inputTypes: Map<string, PortTypeKeys>
  public currentValue: ValidPortTypes[O] | undefined

  private computeInputToOutput: (...args: InputDataTypes<I>) => ValidPortTypes[O]

  constructor(
    computeInputToOutput: (...args: InputDataTypes<I>) => ValidPortTypes[O],
    inputMap: I,
    public outputType: O,
    id: string = "default_node_id"
  ) {
    this.id = id
    this.computeInputToOutput = computeInputToOutput;

    this.inputStreams = new Map()
    this.inputTypes = new Map()
    const inputSubjects: ReplaySubject<ValidPortTypes[PortTypeKeys]>[] = []
    inputMap.forEach(([label, type]) => {
      const subject = new ReplaySubject<ValidPortTypes[PortTypeKeys]>(1)
      this.inputStreams.set(label, subject)
      this.inputTypes.set(label, type)
      inputSubjects.push(subject)
    })

    this.outputStream$ = combineLatest(inputSubjects).pipe(
      map(inputs => this.computeInputToOutput(...inputs as unknown as InputDataTypes<I>)),
      tap(output => this.currentValue = output)
    );
  }

  getInputStream(key: string): ReplaySubject<ValidPortTypes[PortTypeKeys]> {
    return this.inputStreams.get(key)!
  }

  getInputType(key: string): PortTypeKeys {
    return this.inputTypes.get(key)!
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
