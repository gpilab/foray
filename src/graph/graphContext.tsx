import { Dispatch, createContext, useContext, useReducer } from 'react'
import { Node } from './node.ts'
import { Graph } from './graph.ts'

export interface GraphUI {
  graph: Graph
}

const GraphContext = createContext<GraphUI | null>(null)
const GraphDispatchContext = createContext<Dispatch<ACTIONTYPE> | null>(null)


type ACTIONTYPE =
  | { type: "addNode"; node: Node }
  | { type: "removeNode"; node: Node; }
  | { type: "addEdge"; from: string; to: string }

function graphReducer(graphUI: GraphUI, action: ACTIONTYPE): GraphUI {
  switch (action.type) {
    case "addNode": {
      graphUI.graph.addNode(action.node)
      return { ...graphUI, graph: graphUI.graph }
    }
    // case "removeNode": {
    //   const newNodes = graphUI.nodes.filter((n) => n.nodeId != action.nodeId)
    //   return { ...graphUI, nodes: newNodes }
    // }
    default:
      throw new Error()
  }
}
// TODO: Test and implement this reducer!
type GraphProviderProps = {
  children: React.ReactNode,
  initialGraphUI: GraphUI
}
export function GraphProvider({ children, initialGraphUI }: GraphProviderProps) {
  const [graph, dispatch] = useReducer(graphReducer, initialGraphUI)

  return (
    <GraphContext.Provider value={graph} >
      <GraphDispatchContext.Provider value={dispatch}>
        {children}
      </ GraphDispatchContext.Provider>
    </ GraphContext.Provider>
  )
}

export function useGraph() {
  const graphContext = useContext(GraphContext)

  if (graphContext === null) {
    throw Error("useGraphDispatch Must be used inside a child of GraphProvider ")
  }

  return graphContext
}

export function useGraphDispatch() {
  const graphDispatch = useContext(GraphDispatchContext)

  if (graphDispatch === null) {
    throw Error("useGraphDispatch Must be used inside a child of GraphProvider ")
  }

  return graphDispatch
}

