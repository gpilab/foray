import { Dispatch, createContext, useContext, useReducer } from 'react'
import { Port, Node } from './node.ts'
import { Graph } from './graph.ts'
import { createAddNode, createAndNode, createBoolNode, createConstantNode, createDefaultNode, createMultiplyNode, createNotNode, createOrNode, createXorNode } from './nodeDefinitions.ts'

export interface GraphUI {
  graph: Graph
}

const GraphContext = createContext<GraphUI | null>(null)
const GraphDispatchContext = createContext<Dispatch<ACTIONTYPE> | null>(null)

export function useGraph() {
  const graphContext = useContext(GraphContext)

  if (graphContext === null) {
    throw Error("useGraphDispatch must be used inside a child of GraphProvider ")
  }
  return graphContext
}

export function useGraphDispatch() {
  const graphDispatch = useContext(GraphDispatchContext)

  if (graphDispatch === null) {
    throw Error("useGraphDispatch must be used inside a child of GraphProvider ")
  }
  return graphDispatch
}

type ACTIONTYPE =
  | { type: "initializeGraph"; }
  | { type: "addNode"; node: Node; }
  | { type: "fireNode"; nodeId: string; port: Port, value: number }
  | { type: "removeNode"; node: Node; }
  | { type: "addEdge"; from: string; to: string }

function graphReducer(graphUI: GraphUI, action: ACTIONTYPE): GraphUI {
  console.log("In Graph Reducer for action", action.type)
  switch (action.type) {
    case "initializeGraph": {
      return { ...graphUI, graph: initializeGraph() }
    }
    case "addNode": {
      console.log("adding from reducer")
      if (graphUI.graph.getNode(action.node.nodeId) !== undefined) {
        console.log("In reducer, node already exists in graph")
        return graphUI
      }
      graphUI.graph.addNode(action.node)
      return { ...graphUI, graph: graphUI.graph }
    }
    case "fireNode": {
      const node = graphUI.graph.getNode(action.nodeId)
      node?.getInputStream(action.port.name).next(action.value)
      return { ...graphUI, graph: graphUI.graph }
    }
    default:
      throw new Error("Unhandled graphDispatch action")
  }
}


type GraphProviderProps = {
  children: React.ReactNode,
  //initialGraphUI: GraphUI
}

export function GraphProvider({ children }: GraphProviderProps) {
  const initialGraphUI = { graph: initializeGraph() } // for now use hardcoded starting graph
  // if (initialGraphUI === undefined) {
  //   console.log("initialGraph undefined")
  //   initialGraphUI = { graph: initializeGraph() } // for now use hardcoded starting graph
  // }

  const [graph, dispatch] = useReducer(graphReducer, initialGraphUI)

  return (
    <GraphContext.Provider value={graph} >
      <GraphDispatchContext.Provider value={dispatch}>
        {children}
      </ GraphDispatchContext.Provider>
    </ GraphContext.Provider>
  )
}


function initializeGraph(): Graph {
  console.log("initialzing graph")

  const d1 = createDefaultNode("d1")
  const c1 = createConstantNode("c1")
  const c2 = createConstantNode("c2")
  const c3 = createConstantNode("c3")
  const c4 = createConstantNode("c4")

  const a1 = createAddNode("a1")
  const a2 = createAddNode("a2")

  const m1 = createMultiplyNode("m1")

  const initialGraph: Graph = new Graph()

  initialGraph.addNode(c1)
  initialGraph.addNode(c2)
  initialGraph.addNode(a1)
  initialGraph.connectNodes(c1, a1, "y")
  initialGraph.connectNodes(c2, a1, "x")

  initialGraph.addNode(c3)
  initialGraph.addNode(a2)
  initialGraph.connectNodes(c3, a2, "x")
  initialGraph.connectNodes(a1, a2, "y")

  initialGraph.addNode(c4)
  initialGraph.addNode(m1)
  initialGraph.connectNodes(c4, m1, "x")
  initialGraph.connectNodes(a2, m1, "y")


  c1.getInputStream("none").next(1)
  c2.getInputStream("none").next(2)
  c3.getInputStream("none").next(3)
  c4.getInputStream("none").next(4)

  initialGraph.addNode(d1)


  const b1 = createBoolNode("b1", false)
  const b2 = createBoolNode("b2", false)
  const b3 = createBoolNode("b3", false)
  const b4 = createBoolNode("b4", false)

  const and = createAndNode("and1")
  const or = createOrNode("or1")
  const xor = createXorNode("xor1")
  const not = createNotNode("not1")

  initialGraph.addNodes([b1, b2, not, and, b3, b4])
  initialGraph.addNodes([or, xor])
  initialGraph.connectNodes(b1, not, "a")
  initialGraph.connectNodes(b2, and, "a")
  initialGraph.connectNodes(b1, and, "b")
  initialGraph.connectNodes(b2, or, "a")
  initialGraph.connectNodes(b1, or, "b")
  initialGraph.connectNodes(b2, xor, "a")
  initialGraph.connectNodes(b1, xor, "b")

  return initialGraph
}
