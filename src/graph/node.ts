import { Observable, ReplaySubject, combineLatest } from 'rxjs';
import { map } from 'rxjs/operators';
import { NodeAttributes, defaultNodeAtrributes } from "./nodeDefinitions.ts"


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
export interface ValidPortTypes {
  "string": string
  "number": number
  "numberArray": number[]
  "boolean": boolean
}

/** union of all valid port type */
export type PortTypeKey = keyof ValidPortTypes
export type PortTypes = ValidPortTypes[keyof ValidPortTypes]

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

//type InputTypesUnion<T extends NodeInputs> = ComputInputParams<T>[number]
//type InputTypeLabelsUnion<T extends NodeInputs> = InputTypeLabels<T>[number]
type InputKeysUnion<T extends NodeInputs> = InputKeys<T>[number]

export type InputTypeLabelByKey<T extends NodeInputs, K extends string> = Extract<T[number], { name: K, portType: any }>["portType"];
type InputTypeByKey<T extends NodeInputs, K extends string> = ValidPortTypes[Extract<T[number], { name: K, portType: any }>["portType"]];



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

    public nodeAttributes: NodeAttributes = defaultNodeAtrributes
  ) {
    this.computeInputToOutput = computeInputToOutput;
    this.inputStreams = {} as any


    inputPorts.forEach((input) => {
      const key: InputKeysUnion<I> = input.name as InputKeysUnion<I>
      this.inputStreams[key] = new ReplaySubject<typeof input.portType>(1)
    })

    this.outputPort$ = combineLatest(this.inputStreams).pipe(
      map((inputs) => {
        return this.computeInputToOutput(...Object.values(inputs) as unknown as ComputInputParams<I>)
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

  pushValue<T extends I, K extends T[number]["name"]>(value: InputTypeByKey<I, K>, portName: K) {
    this.getInputStream(portName).next(value)
  }
}
