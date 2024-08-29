import { PI } from "tldraw"
import { Config, createNodeDef, NodeInputs, NodeOutputs, NodeType } from "./nodeType"
import { binaryOpInputs, PortDataType, singleInput, singleOutput } from "./portDefinition"
import { invoke } from "@tauri-apps/api"
import { range } from "../../util/array"

export const algebraNodes = ["Add", "Subtract", "Multiply", "Constant", "pyAdd", "DynamicNode"] as const

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

export const pyAddDef = createNodeDef({
  state: {
    type: "pyAdd",
    inputs: binaryOpInputs("number"),
    output: singleOutput("number"),
    config: { formula: "+ (py)" },
  },
  compute: async ({ a, b }) => {
    const val = await invoke<number>('py_add', { a: a, b: b })
    return val
  }
})

export const arrayNodes = ["Range", "cos", "sin", "sinc", "fft", "Plot", "ArrayAdd", "ArrayMult", "PyAddArray"] as const


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

export const pyAddArrayDef = createNodeDef({
  state: {
    type: "PyAddArray",
    inputs: binaryOpInputs("numberArray"),
    output: singleOutput("numberArray"),
    config: { formula: "+ (py array)" },
  },
  compute: async ({ a, b }) => {
    const val = await invoke<number[]>('py_add_array', { a: a, b: b })
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




export const createDynamicNode = (config: Config,
  inputs: NodeInputs,
  output: NodeOutputs) => {

  return createNodeDef({
    state: {
      type: "DynamicNode",
      inputs,
      output,
      config
    },
    compute: async ({ a, b }) => {
      const dynamic_message = {
        message: {
          node_type: a,
          inputs: b
        }
      }
      /// list of input `Values` to pass to `node_type`'s python "compute" function
      const val = await invoke<PortDataType>('dynamic_command', dynamic_message)
      return val
    }

  })
}


export const defaultDynamicNodeDef = createNodeDef({
  state: {
    type: "DynamicNode",
    inputs: binaryOpInputs("number"),
    output: singleOutput("number"),
    config: {}
  },
  compute: async ({ a, b }) => {
    const dynamic_message = {
      message: {
        node_type: "add_int",
        inputs: [{ Integer: a }, { Integer: b }]
      }
    }
    console.log(dynamic_message)
    /// list of input `Values` to pass to `node_type`'s python "compute" function
    const val = await invoke<{ Integer: number }>('dynamic_command', dynamic_message)
    console.log("value", val)
    return val.Integer
  }
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
  "pyAdd": pyAddDef,
  "PyAddArray": pyAddArrayDef,
  "DynamicNode": defaultDynamicNodeDef,
}

export const getDefaultNodeDefinition = (nodeType: NodeType) => {
  return nodeDefaultDefinitions[nodeType]
}

