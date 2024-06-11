import { Dispatch, createContext, useContext, useReducer } from 'react'
import { InPort, Node, port, port2 } from './node.ts'
import { Graph } from './graph.ts'

export interface GraphUI {
  graph: Graph
}

const GraphContext = createContext<GraphUI | null>({ graph: initializeGraph() })
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
  | { type: "fireNode"; nodeId: string; port: InPort, value: number }
  | { type: "removeNode"; node: Node; }
  | { type: "addEdge"; from: string; to: string }

function graphReducer(graphUI: GraphUI, action: ACTIONTYPE): GraphUI {
  switch (action.type) {
    case "initializeGraph": {
      return { ...graphUI, graph: initializeGraph() }
    }
    case "addNode": {
      graphUI.graph.addNode(action.node)
      return { ...graphUI, graph: graphUI.graph }
    }
    case "fireNode": {
      const node = graphUI.graph.getNode(action.nodeId)
      node?.getInputStream("x").next(action.value)
      return { ...graphUI, graph: graphUI.graph }
    }
    default:
      throw new Error("Unhandled graphDispatch action")
  }
}


type GraphProviderProps = {
  children: React.ReactNode,
  initialGraphUI?: GraphUI
}

export function GraphProvider({ children, initialGraphUI }: GraphProviderProps) {
  if (initialGraphUI === undefined) {
    initialGraphUI = { graph: initializeGraph() } // for now use hardcoded starting graph
  }

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
  const createConstantNode = (id: string) =>
    new Node(port("x", "number"), "number", (x: number) => x, id, "Constant");
  const createSumNode = (id: string) =>
    new Node(port2("x", "number", "y", "number"), "number", (x: number, y: number) => { return x + y }, id, "Sum");
  const createMultiplyNode = (id: string) =>
    new Node(port2("x", "number", "y", "number"), "number", (x: number, y: number) => { return x * y }, id, "Multiply");

  const c1 = createConstantNode("c1")
  const c2 = createConstantNode("c2")
  const c3 = createConstantNode("c3")
  const c4 = createConstantNode("c4")

  const s1 = createSumNode("s1")
  const s2 = createSumNode("s2")

  const m1 = createMultiplyNode("m1")

  const initialGraph: Graph = new Graph()

  initialGraph.addNode(c1)
  initialGraph.addNode(c2)
  initialGraph.addNode(s1)
  initialGraph.connectNodes(c1, s1, "y")
  initialGraph.connectNodes(c2, s1, "x")

  initialGraph.addNode(c3)
  initialGraph.addNode(s2)
  initialGraph.connectNodes(c3, s2, "x")
  initialGraph.connectNodes(s1, s2, "y")

  initialGraph.addNode(c4)
  initialGraph.addNode(m1)
  initialGraph.connectNodes(c4, m1, "x")
  initialGraph.connectNodes(s2, m1, "y")

  c1.getInputStream("x").next(1)
  c2.getInputStream("x").next(2)
  c3.getInputStream("x").next(3)
  c4.getInputStream("x").next(4)

  return initialGraph
}
