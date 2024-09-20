import { TLDefaultColorStyle } from "tldraw"

//export const PortTypeLabels = ["IntegerVec", "RealVec", "ComplexVec", "StringVec", "StructVec", "FlagVec", "Primitive",] as const
//export type PortDataTypeLabel = typeof PortTypeLabels[number]
//

export type VecContain = number | string | boolean | object
type Complex = [number, number]

//// Primitive

export const PortTypeLabels = ["Integer", "Real", "Complex", "String", "Flag", "Array", "Struct"] as const
export type PortDataTypeLabel = typeof PortTypeLabels[number]

export type PortTypeMap =
  | ["Integer", number]
  | ["Real", number]
  | ["Complex", Complex]
  | ["String", string]
  | ["Flag", Boolean]
  | ["Struct", Record<PortDataTypeLabel, [PortTypeMap[1]]>]
  | ["Array", PortTypeMap[1][]]

//export type PortTypeMap = {
//  "Integer": number
//  "Real": number
//  "Complex": Complex
//  "String": string
//  "Flag": Boolean
//  "Array": PortDataType[]
//}

//export type PrimitiveType = Primitive[PortDataTypeLabel]
//export type PortDataType = PortTypeMap[1]

////// PrimitiveVec
//
//export const PrimitiveVecLabels = ["IntegerVec", "RealVec", "ComplexVec", "StringVec", "FlagVec"] as const
//
//export type PrimitiveVecTypeLabel = typeof PrimitiveVecLabels[number]
//
//type PrimitiveOrVec<T> = T | PrimitiveOrVec<T>[] | PrimitiveStruct
//type PrimitiveVec = PrimitiveOrVec<PrimitiveType>[]
//
//const myNestedVec: PrimitiveVec = [[1, 2], [1, 2], { a: 1 }, { b: 2, c: [1, 2, 3] }]
//
//
//type PrimitiveOrStruct<T> = T | { [key: string]: PrimitiveOrStruct<T> | PrimitiveVec }
//type PrimitiveStruct = PrimitiveOrStruct<PrimitiveType>
//
//
//const myRecord: Record<string, number> = { a: 1 }
//const myNestedStruct: PrimitiveStruct = {
//  a: 1, b: { a: false }, c: [1, 2, 3], d: [[1, 2], [3, 4]]
//}
//
//
//type GPI_DATA = PrimitiveStruct | PrimitiveVec
//
//const myData: GPI_DATA = { a: 1, b: 2, c: [1, 2, 3] }
//
////TODO:  Simplify this down by starting with a type...
//
//type gpi_data =
//  | string
//  | number
//  | Complex
//  | gpi_data[]
//  | { [property: string]: gpi_data }
//
//
//export type PrimitiveVecExplicit = {
//  "IntegerVec": Primitive["Integer"][]
//  "RealVec": Primitive["Real"][]
//  "ComplexVec": Primitive["Complex"][]
//  "StringVec": Primitive["String"][]
//  "FlagVec": Primitive["Flag"][]
//}
//
//export type PrimitiveVecType = PrimitiveVecExplicit[PrimitiveVecTypeLabel]
//
////// Compound
//
//export type Compound = {
//  "Vec2": PrimitiveVecType[]
//}





//export type PortTypeMap = {
//  "IntegerVec": number[]
//  "RealVec": number[]
//  "ComplexVec": Complex[]
//  "StringVec": string[]
//  "FlagVec": Boolean[]
//  "StructVec": Record<string, VecContain>[]
//}
//
//export type VectorTypes = {
//  "IntegerVec": number[]
//  "RealVec": number[]
//  "ComplexVec": Complex[]
//  "StringVec": string[]
//  "FlagVec": Boolean[]
//}
//
//
//export type PortDataType = PortTypeMap[PortDataTypeLabel]

export function isPortDataTypeLabel(maybeLabel: unknown): maybeLabel is PortDataTypeLabel {
  return PortTypeLabels.includes(maybeLabel as PortDataTypeLabel)
}

export const portColorMap: Record<PortDataTypeLabel, TLDefaultColorStyle> = {
  "Struct": "violet",
  "String": "blue",
  "Complex": "green",
  "Real": "yellow",
  "Integer": "orange",
  "Flag": "red",
  "Array": "light-violet",
}

export type Port<K extends PortDataTypeLabel = PortDataTypeLabel> = {
  name: string
  ioType: "in" | "out"
  dataType: K
  value?: PortTypeMap
}


export type InPort<K extends PortDataTypeLabel = PortDataTypeLabel> = Port<K> & { ioType: "in" }
export type OutPort<K extends PortDataTypeLabel = PortDataTypeLabel> = Port<K> & { ioType: "out" }

export const singleOutput = <T extends PortDataTypeLabel>(dataType: T): {
  out: { name: "out", ioType: "out", dataType: T, value?: PortTypeMap },
} => {
  return {
    out: {
      name: "out" as const,
      ioType: "out" as const,
      dataType: dataType,
    }
  }
}
export const singleInput = <T extends PortDataTypeLabel>(dataType: T): {
  a: { name: "a", ioType: "in", dataType: T, value?: PortTypeMap[T] },
} => {
  return {
    a: {
      name: "a" as const,
      ioType: "in" as const,
      dataType: dataType,
    }
  }
}

export const binaryOpInputs = <T extends PortDataTypeLabel>(dataType: T): {
  a: { name: "a", ioType: "in", dataType: T, value?: PortTypeMap[T] },
  b: { name: "b", ioType: "in", dataType: T, value?: PortTypeMap[T] }
} => {
  return {
    a: {
      name: "a" as const,
      ioType: "in" as const,
      dataType: dataType,
    },
    b: {
      name: "b" as const,
      ioType: "in" as const,
      dataType: dataType,
    }
  }
}



//Test Types

let testIn: InPort
//@ts-expect-error bad type example
testIn = { name: "tin", ioType: "out", dataType: "Real" }
//@ts-expect-no-error
testIn = { name: "tin", ioType: "in", dataType: "Real" }

let testOut: OutPort
//@ts-expect-error bad type example
testOut = { name: "tout", ioType: "in", dataType: "Real" }
//@ts-expect-no-error
testOut = { name: "tout", ioType: "out", dataType: "Real" }

