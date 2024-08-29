import { TLDefaultColorStyle } from "tldraw"

export const PortTypeLabels = ["number", "string", "boolean", "numberArray"] as const
export type PortDataTypeLabel = typeof PortTypeLabels[number]

export type PortTypeMap = {
  "number": number
  "string": string
  "boolean": boolean
  "numberArray": number[]
}

export type PortDataType = PortTypeMap[PortDataTypeLabel]

export function isPortDataTypeLabel(maybeLabel: unknown): maybeLabel is PortDataTypeLabel {
  return PortTypeLabels.includes(maybeLabel as PortDataTypeLabel)
}

export const portColorMap: Record<PortDataTypeLabel, TLDefaultColorStyle> = {
  "number": "violet",
  "numberArray": "blue",
  "string": "green",
  "boolean": "yellow",

}

export type Port<K extends PortDataTypeLabel = PortDataTypeLabel> = {
  name: string
  ioType: "in" | "out"
  dataType: K
  value?: PortTypeMap[K]
}


export type InPort<K extends PortDataTypeLabel = PortDataTypeLabel> = Port<K> & { ioType: "in" }
export type OutPort<K extends PortDataTypeLabel = PortDataTypeLabel> = Port<K> & { ioType: "out" }

export const singleOutput = <T extends PortDataTypeLabel>(dataType: T): {
  out: { name: "out", ioType: "out", dataType: T, value?: PortTypeMap[T] },
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
testIn = { name: "tin", ioType: "out", dataType: "number" }
//@ts-expect-no-error
testIn = { name: "tin", ioType: "in", dataType: "number" }

let testOut: OutPort
//@ts-expect-error bad type example
testOut = { name: "tout", ioType: "in", dataType: "number" }
//@ts-expect-no-error
testOut = { name: "tout", ioType: "out", dataType: "number" }

