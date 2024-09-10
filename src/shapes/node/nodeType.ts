import {
  algebraNodes, arrayNodes,
  getDefaultNodeDefinition,
} from "./nodeDefinitions";
import {
  InPort, OutPort,
  binaryOpInputs, singleOutput
} from "./portDefinition"

export const nodeTypes = [
  ...algebraNodes,
  ...arrayNodes]

export type NodeType = typeof nodeTypes[number]

/**
 * A node's inputs are a set of named `InPort`s
 */
export type NodeInputs = Record<string, InPort>;

/**
 * A node's outputs are a set of named `OutPort`s
 * Currently restricted to only one OutPort named "out"
 */
export type NodeOutputs = Record<"out", OutPort>;

/**
 * Make the type's subTypes have required fields
 * This is specifically useful for requiring a 
 * Node's input ports to be populated with a value
 */
type Populated<T> = {
  [k in keyof T]: Required<T[k]>
}
type PopulatedInputs = Populated<NodeInputs>

/**
 * Node's can have arbitrary configuration dictionaries
 */
export type Config = Record<string, unknown>


/**
 * Persistent Node data
 * this data gets saved to disk
 * given a specific NodeState, the output calculation of the node 
 * will always be the same
 */
export type NodeState<I extends NodeInputs, O extends NodeOutputs, C extends Config> = {
  type: string,
  inputs: I,
  output: O,
  config: C,
}
type PopulatedNodeState = NodeState<PopulatedInputs, NodeOutputs, Config>

function createNodeState<
  I extends NodeInputs,
  O extends NodeOutputs,
  C extends Config>
  (type: NodeType, inputs: I, output: O, config: C): NodeState<I, O, C> {
  return {
    type, inputs, output, config
  }
}


const addState: NodeState<{
  a: {
    name: "a", ioType: "in", dataType: "Real", value?: number
  },
  b: { name: "b", ioType: "in", dataType: "Real", value?: number }
}, Record<"out", OutPort>, Record<string, never>>
  = createNodeState("_Add", binaryOpInputs("Real"), singleOutput("Real"), {})


//type InputValues<I extends NodeInputs> = Record<keyof Populated<I>, Populated<I>[keyof Populated<I>]["value"]>
//
type NodeCompute<
  I extends NodeInputs,
  O extends NodeOutputs,
  C extends Config
> = (inputs: Populated<I>,
  config: C
) => Promise<O["out"]["value"]> | O["out"]["value"]


export type NodeDefinition<
  I extends NodeInputs,
  O extends NodeOutputs,
  C extends Config
> = {
  state: NodeState<I, O, C>,
  compute: NodeCompute<I, O, C>
}

export function createNodeDef<
  I extends NodeInputs,
  O extends NodeOutputs,
  C extends Config>
  (def: {
    state: NodeState<I, O, C>,
    compute: NodeCompute<I, O, C>
  }): NodeDefinition<I, O, C> {
  return def
}

type addType = typeof addState
const addCompute: NodeCompute<addType["inputs"], addType["output"], addType["config"]>
  = (inputs, _config: Record<string, never>) => {
    return inputs.a.value + inputs.b.value
  }

const addTest: NodeDefinition<addType["inputs"], addType["output"], addType["config"]>
  = {
  state: addState,
  compute: addCompute
}
addTest


/**
 * For a given PopulatedNodeState, calculate the next output
 *
 * A NodeState can be checked/converted to a PopulatedNodeState
 * using `checkAllPortsPopulated`
 *
 * TODO: give an example (and make the interface easier, checkAllPortsPopulated 
 * operates on just inputs, and this function operates on the whole state.
 * Maybe both should work on the full state object?)
 */
export const nodeCompute = async <T extends PopulatedNodeState>(nodeState: T) => {
  const { type, inputs, config } = nodeState
  console.log("Calling compute for node: ", { type, inputs, config })
  const compute = getDefaultNodeDefinition(type).compute

  //const inputVals = flattenInputs(inputs) as InputValues<T["inputs"]>
  const outputValue = await compute(inputs, config)
  //TODO: make work for arbitary named outputs
  //
  if (outputValue["out"] != undefined) {
    return outputValue["out"]
  }
  else {
    return outputValue
  }
}

/**
 * TODO make this API work better with nodeCompute
 */
export const checkAllPortsPopulated = (ports: Record<string, InPort>)
  : ports is Populated<Record<string, InPort>> => {
  const isAnyUndefined = Object.values(ports).find(port => port.value === undefined) !== undefined
  return !isAnyUndefined
}

/**
 * make these types better/ create a cleaner way of iterating of inputs
 */
//function flattenInputs(inputs: Populated<NodeInputs>): Record<string, PortDataType> {
//  return Object.entries(inputs).reduce<Record<string, PortDataType>>((acc, [label, port]) => {
//    acc[label] = port.value
//    return acc
//  }, {})
//}
//
