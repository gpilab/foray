import { Observable, ReplaySubject, combineLatest } from 'rxjs';
import { map, tap } from 'rxjs/operators';

interface ValidPortTypes {
  "string": string
  "number": number
  "numberArray": number[]
  "boolean": boolean
}

/** union of all valid port type */
export type PortTypeKey = keyof ValidPortTypes

/** Node inputs are described by a unique label and a data type. Data types are defined as the keys of ValidPortTypes */
export type InPort = {
  readonly name: string
  readonly portType: PortTypeKey
}

/** A node's list of input ports*/
export type NodeInputs = readonly InPort[]


/** converts nodeinputs into a form that can be used to 
 * constrain the input of the compute function
 * - This might be able to be further constrained by requiring
 *   the input param name to match the node input name */
export type ComputInputParamsO<T extends NodeInputs> = {
  [K in keyof T]: T[K] extends [string, infer U] ? (U extends PortTypeKey ? ValidPortTypes[U] : never) : never;
};

export type ComputInputParams<T extends ReadonlyArray<InPort>> = {
  [K in keyof T]: ValidPortTypes[T[K]["portType"]] //extends { name: infer N, portKey: infer P } ? (T[K]["portType"] extends PortTypeKey ? ValidPortTypes[T[K]["portType"]] : never) : never
}


type InputKeys<T extends NodeInputs> = {
  [K in keyof T]: T[K] extends { name: infer U, portKey: PortTypeKey } ? (U extends string ? U : never) : never;
};

type InputTypesUnion<T extends NodeInputs> = ComputInputParams<T>[number]
//type InputTypeLabelsUnion<T extends NodeInputs> = InputTypeLabels<T>[number]
type InputKeysUnion<T extends NodeInputs> = InputKeys<T>[number]

export type InputTypeLabelByKey<T extends NodeInputs, K extends string> = Extract<T[number], { name: K, portType: any }>["portType"];
type InputTypeByKey<T extends NodeInputs, K extends string> = ValidPortTypes[Extract<T[number], { name: K, portType: any }>["portType"]];

type InputSubjectMap<T extends NodeInputs> = {
  [K in T[number]["name"]]: ReplaySubject<Extract<T[number], { name: K, portType: T[number]["portType"] }>["portType"]>;
};

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

export class Node<I extends NodeInputs = any, O extends PortTypeKey = PortTypeKey, C extends (...args: ComputInputParams<I>) => ValidPortTypes[O] = any> {
  public inputStreams: InputSubjectMap<I>
  public outputStream$: Observable<ValidPortTypes[O]>
  public currentValue: ValidPortTypes[O] | undefined

  private computeInputToOutput: C
  constructor(
    public inputPorts: I,
    public outputType: O,
    computeInputToOutput: C,
    public id: string = "default_node_id",
  ) {
    this.computeInputToOutput = computeInputToOutput;

    this.inputStreams = {} as any

    //const inputSubjects: ReplaySubject<InputTypesUnion<I>>[] = []//combineLatest need an array, not sure if this is necessarily the best method
    inputPorts.forEach((input) => {
      const key: InputKeysUnion<I> = input.name as InputKeysUnion<I>
      const subject = new ReplaySubject<typeof input.portType>(1)
      this.inputStreams[key] = subject
      //inputSubjects.push(subject)
    })

    //coarse inputStreams into the format that combine latest needs
    const inputSubjects = Object.values(this.inputStreams) as unknown as ReplaySubject<InputTypesUnion<I>>[]
    this.outputStream$ = combineLatest(inputSubjects).pipe(
      map(inputs => this.computeInputToOutput(...inputs as unknown as ComputInputParams<I>)),
      tap(output => this.currentValue = output)
    );
  }

  getInputStream<T extends I, K extends T[number]["name"]>(key: K): ReplaySubject<InputTypeByKey<T, K>> {
    return this.inputStreams[key]
  }

  getInputType<T extends I, K extends T[number]["name"]>(key: K): InputTypeLabelByKey<T, K> {
    const inputType = this.inputPorts.find(input => input.name === key)
    if (inputType === undefined) {
      throw Error(`Key ${key} is not present in input [${this.inputPorts}]`)
    }
    return inputType.portType
  }
}
