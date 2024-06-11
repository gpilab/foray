import { Node, outPort, port, port2 } from './node.ts';

export type AlgebraicExpression =
  | "Constant"
  | "Add"
  | "Subtract"
  | "Multiply"
  | "Divide"

export type BinaryLogic =
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

// Algebraic Expressions
const numOut = outPort("number")
const xyIn = port2("x", "number", "y", "number")
export const createConstantNode = (id: string) =>
  new Node(port("x", "number"), numOut,
    (x: number) => x,
    id, "Constant");

export const createAddNode = (id: string) =>
  new Node(xyIn, numOut,
    (x: number, y: number) => { return x + y },
    id, "Add");

export const createSubtractNode = (id: string) =>
  new Node(xyIn, numOut,
    (x: number, y: number) => { return x - y },
    id, "Add");

export const createMultiplyNode = (id: string) =>
  new Node(xyIn, numOut,
    (x: number, y: number) => { return x * y },
    id, "Multiply");

export const createDivideNode = (id: string) =>
  new Node(xyIn, numOut,
    (x: number, y: number) => { return x / y },
    id, "Divide");


// BinaryLogic
const boolOut = outPort("boolean")
const abIn = port2("a", "boolean", "b", "boolean")
export const createNotNode = (id: string) =>
  new Node(port("a", "boolean"), boolOut,
    (a: boolean) => !a,
    id, "Not");

export const createAndNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return a && b },
    id, "And");

export const createNandNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return !(a && b) },
    id, "And");

export const createOrNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return a || b },
    id, "Or");

export const createNorNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return !(a || b) },
    id, "Nor");

export const createXorNode = (id: string) =>
  new Node(abIn, boolOut,
    (a: boolean, b: boolean) => { return a !== b },
    id, "Xor");

