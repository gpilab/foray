import { PI } from "tldraw"
import { createNodeDef, NodeType } from "./nodeType"
import { binaryOpInputs, singleInput, singleOutput } from "./portDefinition"
import { invoke } from "@tauri-apps/api"
import { range } from "../../util/array"

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
    ...addNodeDefinition.state,
    type: "Subtract",
  },
  compute: ({ a, b }) => a - b
})

const multiplyDef = createNodeDef({
  state: {
    ...addNodeDefinition.state,
    type: "Multiply",
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

export const arrayNodes = ["Range", "cos", "sin", "sinc", "fft", "Plot", "ArrayAdd", "ArrayMult"] as const


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
    type: "sin",
    inputs: singleInput("numberArray"),
    output: singleOutput("numberArray"),
    config: { amplitude: 1, phaseOffset: 0, frequency: 4 }
  },
  compute: ({ a }, { amplitude, phaseOffset, frequency }) =>
    a.map(e =>
      amplitude * Math.sin(e * frequency + phaseOffset)
    )
})

export const cosDef = createNodeDef({
  state: {
    ...sinDef.state,
    type: "cos",
  },
  compute: ({ a }, { amplitude, phaseOffset, frequency }) =>
    a.map(e =>
      amplitude * Math.cos(e * frequency + phaseOffset)
    )
})

export const sincDef = createNodeDef({
  state: {
    ...sinDef.state,
    type: "sinc",
  },
  compute: ({ a }, { amplitude, phaseOffset, frequency }) =>
    a.map(e => {
      const x = e * frequency + phaseOffset

      if (x == 0) {
        return amplitude * 1
      }

      return amplitude * Math.sin(PI * x) / (PI * x)
    }
    )
})

export const arrayAddDef = createNodeDef({
  state: {
    type: "ArrayAdd",
    inputs: binaryOpInputs("numberArray"),
    output: singleOutput("numberArray"),
    config: { formula: "\\textbf{+}" }
  },
  compute: ({ a, b }) => a.map((e, i) => e + b[i])
})

export const arrayMultiplyDef = createNodeDef({
  state: {
    type: "ArrayMult",
    inputs: binaryOpInputs("numberArray"),
    output: singleOutput("numberArray"),
    config: { formula: "\\times" }
  },
  compute: ({ a, b }) => a.map((e, i) => e * b[i])
})

export const fftDef = createNodeDef({
  state: {
    type: "fft",
    inputs: singleInput("numberArray"),
    output: singleOutput("numberArray"),
    config: { formula: "\\mathcal{F}\\{f(x)\\}" },
  },
  compute: async ({ a }) => {
    const val = await invoke<number[]>('fft', { signal: a })
    console.log(Math.max(...val))
    return val
  }
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
  "sin": sinDef,
  "cos": cosDef,
  "sinc": sincDef,
  "fft": fftDef,
  "ArrayAdd": arrayAddDef,
  "ArrayMult": arrayMultiplyDef,
  "Plot": plotDef,
}

export const getDefaultNodeDefinition = (nodeType: NodeType) => {
  return nodeDefaultDefinitions[nodeType]
}

