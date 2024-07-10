import { range } from "../util/array"
import { createNodeDef, NodeType } from "./nodeType"
import { binaryOpInputs, singleInput, singleOutput } from "./portDefinition"

export const algebraNodes = ["Add", "Subtract", "Multiply", "Constant"] as const

export const addNodeDefinition = createNodeDef({
  state: {
    type: "Add",
    inputs: binaryOpInputs("number"),
    output: singleOutput("number"),
    config: {}
  },
  compute: ({ a, b }) => a + b
})

const subtractDef = createNodeDef({
  state: {
    type: "Subtract",
    inputs: binaryOpInputs("number"),
    output: singleOutput("number"),
    config: {}
  },
  compute: ({ a, b }) => a - b
})

const multiplyDef = createNodeDef({
  state: {
    type: "Multiply",
    inputs: binaryOpInputs("number"),
    output: singleOutput("number"),
    config: {}
  },
  compute: ({ a, b }) => a * b
})

const constantDef = createNodeDef({
  state: {
    type: "Constant",
    inputs: {},
    output: singleOutput("number"),
    config: { value: 10 }
  },
  compute: (_, config) => config.value
})

export const arrayNodes = ["Range", "Sin", "Plot"] as const //, "Cos"] as const

export const rangeDef = createNodeDef({
  state: {
    type: "Range",
    inputs: {},
    output: singleOutput("numberArray"),
    config: {
      start: -10,
      end: 10,
      step: .1
    }
  },
  compute: (_, { start, end, step }) => {
    console.log({ start, end, step })
    return range(start, end, step)
  }
})

export const sinDef = createNodeDef({
  state: {
    type: "Sin",
    inputs: singleInput("numberArray"),
    output: singleOutput("numberArray"),
    config: {}
  },
  compute: ({ a }) => a.map(e => Math.sin(e))
})

export const plotDef = createNodeDef({
  state: {
    type: "Plot",
    inputs: singleInput("numberArray"),
    output: singleOutput("numberArray"),
    config: {}
  },
  compute: ({ a }) => a
})

export const nodeDefaultDefinitions = {
  "Add": addNodeDefinition,
  "Subtract": subtractDef,
  "Multiply": multiplyDef,
  "Constant": constantDef,
  "Range": rangeDef,
  "Sin": sinDef,
  "Plot": plotDef,
}

export const getDefaultNodeDefinition = (nodeType: NodeType) => {
  return nodeDefaultDefinitions[nodeType]
}

