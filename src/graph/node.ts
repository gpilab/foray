import { Observable, ReplaySubject, combineLatest } from 'rxjs';
import { map } from 'rxjs/operators';
import { NodeType } from "./nodeDefinitions.ts"


// This file heavily uses complex types
//
// This provides a lot of guarantes that the 
// ports, nodes and compute functions have types
// that match.
//
// The downside is that it is more difficult to read, and make changes.
//
// The upside is taht once changes have been made, there are a lot fewer issues that
// can be introduced across the code base. (as long as everything still type checks!)
//
// Checkout typescript docs if you are unfamilar with any of the syntax below
// https://www.typescriptlang.org/docs/handbook/2/types-from-types.html



/** 
 * We want a node's ports to only alow connections between ports of the same type 
 *
 * This interface defines the set of valid port types, and a string that can uniquely
 * be used to make sure port types match at runtime
 *
 * Typescript removes type information in the transpiled js, so we need the string values
 * to have this info available at runtime.
 * 
 **/
interface ValidPortTypes {
  "string": string
  "number": number
  "numberArray": number[]
  "boolean": boolean
}

/** union of all valid port type */
export type PortTypeKey = keyof ValidPortTypes

/** Node inputs are described by a unique label and a data type. Data types are defined as the keys of ValidPortTypes */
export type Port<T extends PortTypeKey = PortTypeKey> = {
  readonly name: string
  readonly portType: T
}

/** A node's list of input ports*/
export type NodeInputs = readonly Port[]

/** 
 * Converts nodeinputs into a form that can be used to 
 * constrain the input of the compute function
 *
 * - This might be able to be further constrained by requiring
 *   the input param name to match the node input name */
export type ComputInputParams<T extends ReadonlyArray<Port>> = {
  [K in keyof T]: ValidPortTypes[T[K]["portType"]]
}

type InputKeys<T extends NodeInputs> = {
  [K in keyof T]: T[K] extends { name: infer U, portKey: PortTypeKey } ? (U extends string ? U : never) : never;
}

type InputTypesUnion<T extends NodeInputs> = ComputInputParams<T>[number]
//type InputTypeLabelsUnion<T extends NodeInputs> = InputTypeLabels<T>[number]
type InputKeysUnion<T extends NodeInputs> = InputKeys<T>[number]

export type InputTypeLabelByKey<T extends NodeInputs, K extends string> = Extract<T[number], { name: K, portType: any }>["portType"];
type InputTypeByKey<T extends NodeInputs, K extends string> = ValidPortTypes[Extract<T[number], { name: K, portType: any }>["portType"]];


/** Creates a node input with 1 port*/
export function outPort<
  T extends PortTypeKey>
  (type: T) {
  return { name: "out", portType: type }
}
/** Creates a node input with 1 port*/
export function port<
  S extends string,
  T extends PortTypeKey>
  (name: S, type: T) {
  return [{ name: name, portType: type }]
}

/** Creates a node input with 2 ports*/
export function port2<
  S1 extends string,
  T1 extends PortTypeKey,
  S2 extends string,
  T2 extends PortTypeKey>
  (name1: S1, type1: T1, name2: S2, type2: T2): [{ name: S1, portType: T1 }, { name: S2, portType: T2 }] {
  return [
    { name: name1, portType: type1 },
    { name: name2, portType: type2 }]
}

type InputSubjectMap<T extends NodeInputs> = {
  [K in T[number]["name"]]: ReplaySubject<Extract<T[number], { name: K, portType: T[number]["portType"] }>["portType"]>;
};


/**
 * Nodes transform data
 *
 * They are defined by their input ports, output port, 
 * and compute function that transforms inputs to outputs
 *
 * ### Input Ports
 * `inputPorts` is a fixed length array of `inPort` objects
 * `inPorts` have a unique name, and a data type. Data types are string literals
 * defined in the `ValidPortTypes` type
 *
 * ### Output Port
 * Each node can only have one output, so it doesn't need a name.
 * It is uniquely defined by its data type
 *
 * ### Compute Function
 * `computeInputToOutput` requires its inputs and outputs to match
 * the input and output ports
 *
 * ### Streams
 * inputs and ouputs are represented as streams of data
 * `computeInputToOutput` doesn't need to be called manually, whenever 
 * an input stream sends new data, computeInputToOutput will be called 
 * if data exists on all inputs
 **/
export class Node<I extends NodeInputs = any, O extends PortTypeKey = PortTypeKey, C extends (...args: ComputInputParams<I>) => ValidPortTypes[O] = any> {
  public inputStreams: InputSubjectMap<I>
  public outputPort$: Observable<ValidPortTypes[O]>
  public currentValue: ValidPortTypes[O] | undefined

  private computeInputToOutput: C

  constructor(
    public inputPorts: I,
    public outputPort: Port<O>,
    computeInputToOutput: C,
    public nodeId: string = "default_node_id",
    public nodeType: NodeType = "default_node_type",
  ) {
    this.computeInputToOutput = computeInputToOutput;
    this.inputStreams = {} as any

    inputPorts.forEach((input) => {
      const key: InputKeysUnion<I> = input.name as InputKeysUnion<I>
      const subject = new ReplaySubject<typeof input.portType>(1)
      this.inputStreams[key] = subject
    })

    //coaerce inputStreams into the format that combine latest needs
    const inputSubjects = Object.values(this.inputStreams) as unknown as ReplaySubject<InputTypesUnion<I>>[]
    this.outputPort$ = combineLatest(inputSubjects).pipe(
      map((inputs) => {
        return this.computeInputToOutput(...inputs as unknown as ComputInputParams<I>)
      })
    );

    //subscription for self
    this.outputPort$.subscribe((output) => {
      //console.log(`updating node ${this.nodeId} currentValue(${output}) because outputPort$ has fired`)
      this.currentValue = output
    })
  }

  getInputStream<T extends I, K extends T[number]["name"]>(key: K): ReplaySubject<InputTypeByKey<T, K>> {
    return this.inputStreams[key]
  }
  getInputPort<T extends I, K extends T[number]["name"]>(key: K): Port {//InputTypeLabelByKey<T, K> {
    const port = this.inputPorts.find(input => input.name === key)
    if (port === undefined) {
      throw Error(`Key ${key} is not present in input [${this.inputPorts}]`)
    }
    return port
  }

  getInputType<T extends I, K extends T[number]["name"]>(key: K): InputTypeLabelByKey<T, K> {
    const inputPort = this.inputPorts.find(input => input.name === key)
    if (inputPort === undefined) {
      throw Error(`Key ${key} is not present in input [${this.inputPorts}]`)
    }
    return inputPort.portType
  }

  getInPortIndex(port: Port) {
    return this.inputPorts.indexOf(port)
  }
}
