import { ValidPortTypes, Node, PortTypeKey, Port } from './node.ts';

export type AlgebraicExpression =
  | "Constant"
  | "Add"
  | "Subtract"
  | "Multiply"
  | "Divide"

export type BinaryLogic =
  | "Boolean"
  | "And"
  | "Or"
  | "Xor"
  | "Not"
  | "Nand"
  | "Nor"

export type NodeType =
  | AlgebraicExpression
  | BinaryLogic
  | "default_node_type"


export type NodeAttributes = {
  type: NodeType,
  equation?: string
  width?: number
  height?: number
}

/** Creates a node input with 1 port*/
export function outPortFunction<
  T extends PortTypeKey>
  (type: T): Port<T> {
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

export const portColorMap: Record<keyof ValidPortTypes, string> = {
  "string": "goldenrod",
  "number": "purple",
  "numberArray": "blue",
  "boolean": "green",
}





export const defaultNodeAtrributes: NodeAttributes = {
  type: "default_node_type",
}

// Nodes with no inputs need to pass a sinlgle port with label "none"
export const createDefaultNode = (id: string) =>
  new Node(port("none", "number"), outPortFunction("number"), (x: number) => x, id)

// Algebraic Expressions
const numOut = outPortFunction("number")
const xyIn = port2("x", "number", "y", "number")
export const createConstantNode = (id: string, defaultValue?: number) => {
  const n = new Node(port("none", "number"), numOut,
    (x: number) => x,
    id, { type: "Constant", width: 100, height: 30 });

  if (defaultValue === undefined) {
    return n
  }
  else {
    n.pushValue(defaultValue, "none")
    return n
  }
}

export const createAddNode = (id: string) =>
  new Node(xyIn, numOut,
    (x: number, y: number) => { return x + y },
    id, { type: "Add", equation: "x+y" });

export const createSubtractNode = (id: string) =>
  new Node(xyIn, numOut,
    (x: number, y: number) => { return x - y },
    id, { type: "Subtract", equation: "x-y" });

export const createMultiplyNode = (id: string) =>
  new Node(xyIn, numOut,
    (x: number, y: number) => { return x * y },
    id, { type: "Multiply", equation: "x*y" });

export const createDivideNode = (id: string) =>
  new Node(xyIn, numOut,
    (x: number, y: number) => { return x / y },
    id, { type: "Divide", equation: "x/y" });


// BinaryLogic
const boolOut = outPortFunction("boolean")
export const createBoolNode = (id: string, defaultValue?: boolean) => {
  const n = new Node(port("none", "boolean"), boolOut,
    (a: boolean) => a,
    id, { type: "Boolean", width: 30, height: 30 });
  if (defaultValue === undefined) {
    return n
  }
  else {
    n.pushValue(defaultValue, "none")
    return n
  }
}

export const createNotNode = (id: string) =>
  new Node(port("a", "boolean"), boolOut,
    (a: boolean) => !a,
    id, { type: "Not", equation: "!a" });

const abIn = port2("a", "boolean", "b", "boolean")
export const createAndNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return a && b },
    id, { type: "And", equation: "a \\& b" });

export const createNandNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return !(a && b) },
    id, { type: "Nand", equation: "a!\\&b" });

export const createOrNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return a || b },
    id, { type: "Or", equation: "a\\parallel b" });

export const createNorNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return !(a || b) },
    id, { type: "Nor", equation: "a \\nparallel b" });

export const createXorNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return a !== b },
    id, { type: "Xor", equation: "a\\veebar b" });

