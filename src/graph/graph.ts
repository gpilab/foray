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
type InPort = readonly [string, PortTypeKeys]
type NodeInputs = readonly InPort[]


type InputTypes<T extends NodeInputs> = {
  [K in keyof T]: T[K] extends readonly [string, infer U] ? (U extends PortTypeKeys ? ValidPortTypes[U] : never) : never;
};

// type InputTypeLabels<T extends NodeInputs> = {
//   [K in keyof T]: T[K] extends readonly [string, infer U] ? (U extends PortTypeKeys ? U : never) : never;
// };
//
type InputKeys<T extends NodeInputs> = {
  [K in keyof T]: T[K] extends readonly [infer U, PortTypeKeys] ? (U extends string ? U : never) : never;
};

type InputTypesUnion<T extends NodeInputs> = InputTypes<T>[number]
//type InputTypeLabelsUnion<T extends NodeInputs> = InputTypeLabels<T>[number]
type InputKeysUnion<T extends NodeInputs> = InputKeys<T>[number]

type InputTypeLabelByKey<T extends NodeInputs, K extends string> = Extract<T[number], readonly [K, any]>[1];
type InputTypeByKey<T extends NodeInputs, K extends string> = ValidPortTypes[Extract<T[number], readonly [K, any]>[1]];

type InputSubjectMap<T extends NodeInputs> = {
  [K in T[number][0]]: ReplaySubject<Extract<T[number], [K, any]>[1]>;
};

export class Node<I extends NodeInputs = any, O extends PortTypeKeys = any> {
  public id: string
  public inputStreams: InputSubjectMap<I>
  public outputStream$: Observable<ValidPortTypes[O]>
  readonly inputMap: I
  //public inputTypes: Map<InputKeysUnion<I>, InputTypeLabelsUnion<I>>
  public currentValue: ValidPortTypes[O] | undefined

  private computeInputToOutput: (...args: InputTypes<I>) => ValidPortTypes[O]

  constructor(
    computeInputToOutput: (...args: InputTypes<I>) => ValidPortTypes[O],
    inputMap: I,
    public outputType: O,
    id: string = "default_node_id"
  ) {
    this.id = id
    this.inputMap = inputMap
    this.computeInputToOutput = computeInputToOutput;

    this.inputStreams = {} as any

    const inputSubjects: ReplaySubject<InputTypesUnion<I>>[] = []//combineLatest need an array, not sure if this is necessarily the best method
    inputMap.forEach((label) => {
      const key: InputKeysUnion<I> = label[0] as InputKeysUnion<I>
      const subject = new ReplaySubject<InputTypesUnion<I>>(1)
      this.inputStreams[key] = subject

      inputSubjects.push(subject)
    })

    this.outputStream$ = combineLatest(inputSubjects).pipe(
      map(inputs => this.computeInputToOutput(...inputs as unknown as InputTypes<I>)),
      tap(output => this.currentValue = output)
    );
  }

  getInputStream<T extends I, K extends T[number][0]>(key: K): ReplaySubject<InputTypeByKey<T, K>> {
    return this.inputStreams[key]
  }

  getInputType<T extends I, K extends T[number][0]>(key: K): InputTypeLabelByKey<T, K> {
    const inputType = this.inputMap.find(input => input[0] === key)
    if (inputType === undefined) {
      throw Error(`Key ${key} is not present in input [${this.inputMap}]`)
    }
    return inputType[1]
  }
}



export class Graph {
  private nodeAdjacencies: Map<Node, Node[]> = new Map();

  // TODO: add output type restriction
  addNode<T extends NodeInputs, K extends T[number][0], O extends InputTypeLabelByKey<T, K>>
    (node: Node<any, O>, connections: { targetNode: Node<T>, targetInputLabel: K }[] = []) {
    this.nodeAdjacencies.set(node, []);
    connections.forEach(({ targetNode, targetInputLabel }) => {
      this.connectNodes(node, targetNode, targetInputLabel);
    });
  }
  addNodes(nodes: Node[]) {
    nodes.forEach((node) => this.addNode(node))
  }

  // Connect output of one node to input of another node
  connectNodes<T extends NodeInputs, K extends T[number][0]>
    (sourceNode: Node, targetNode: Node<T>, targetInputLabel: K) {
    const sourceNodeAdjacencies = this.nodeAdjacencies.get(sourceNode)
    if (sourceNodeAdjacencies == undefined) {
      throw Error("Source node not present in graph");
    }
    if (!this.nodeAdjacencies.has(targetNode)) {
      throw Error("Target node not present in graph");
    }
    if (sourceNode.outputType != targetNode.getInputType(targetInputLabel)) {
      throw Error(`Attempted to connect nodes of type (source, output: ${sourceNode.outputType} )and (target, input: ${targetNode.getInputType(targetInputLabel)})`)
    }
    // Add target node to the adjacency list of the source node
    sourceNodeAdjacencies.push(targetNode);

    // Subscribe the output of the source node to the input of the target node
    sourceNode.outputStream$.subscribe(output => {
      const input = targetNode.inputStreams[targetInputLabel]
      if (input === undefined) {
        throw Error(`Attempted to access input label ${targetInputLabel} on node ${node} `)
      }
      input.next(output);
    });
  }

  getConnections(node: Node) {
    return this.nodeAdjacencies.get(node)
  }
}
