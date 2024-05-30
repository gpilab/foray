
/** label:Type
 */
type PortKeyType = {
  number: number
  numberArray: number[]
  boolean: boolean
}
type PortTemplateField = {
  number: PortNumber
  numberArray: PortNumberArray
  boolean: PortBoolean
}

type PortNumber = "number"
type PortNumberArray = "numberArray"
type PortBoolean = "boolean"
type PortKeyLiteral = PortNumber | PortNumberArray | PortBoolean


/** Instances of this type define a port's type, it doesn't hold the value of the port, 
 *
 * ex:
 * ``` ts
 * const addInput: PortTemplate = {
 *   a: "number",
 *   b: "number"
 * }
 * const addOutput: PortTemplate = {
 *   sum: "number"
 * }
 * ```
 */
type PortTemplate = {
  [portId: string]: PortKeyLiteral
}

const addInput: PortTemplate = {
  a: "number",
  b: "number"
}
const addOutput: PortTemplate = {
  sum: "number"
}

/** takes a type that extends PortTemplate, and converts it to an object with the same keys
 *
 * ex:
 * ``` ts
 * const addInputTemplate: PortTemplate = {
 *   a: "number",
 *   b: "number"
 * }
 * 
 * const specificAddInput: PortInstance<typeof addInputTemplate> = {
 *   a: 7,
 *   b: 11
 * }
 * ```
 */
// type PortObjectInstance<L extends PortTemplate> =
//   { [portId in keyof L]: PortKeyType[L[portId]] }
// type PortInstance<L extends PortKeyLiteral | PortTemplate> =
//   L extends PortKeyLiteral ? L : PortObjectInstance<Exclude<L, PortKeyLiteral>>
type PortInstance<L extends PortTemplate> =
  { [portId in keyof L]: PortKeyType[L[portId]] }

const addInputTemplate: PortTemplate = {
  a: "number",
  b: "number"
}

const specificAddInput: PortInstance<typeof addInputTemplate> = {
  a: 7,
  b: 11
}

/** Specifies a function that will process inputs and produce outputs
 */
type NodeCompute<IT extends PortTemplate, OT extends PortTemplate> =
  { (inputs: PortInstance<IT>): PortInstance<OT> }


type NodeTemplate<IT extends PortTemplate, OT extends PortTemplate> = {
  inputs: IT
  outputs: OT
  compute: NodeCompute<IT, OT>
}

function createNodeTemplate<IT extends PortTemplate, OT extends PortTemplate>
  (inputs: IT, outputs: OT, compute: NodeCompute<IT, OT>): NodeTemplate<IT, OT> {
  return {
    inputs: inputs,
    outputs: outputs,
    compute: compute
  }
}

const bi = { a: "number" as PortNumber, b: "number" as PortNumber }
type BI = typeof bi
type BI_Inst = PortInstance<BI>

const testInput: BI_Inst = {
  a: 5,
  b: 7
}


const nr = { result: "number" as PortNumber }
type NR = typeof nr
type NR_Inst = PortInstance<NR>

const testOutput: NR_Inst = {
  result: 5,
}



const add = ({ a, b }: BI_Inst): NR_Inst => {
  return { result: a + b }
}

const add2 = (inputs: PortInstance<{ a: "number", b: "number" }>): PortInstance<{ result: "number" }> => {
  return { result: inputs.a + inputs.b }
}

const add3 = (inputs: PortInstance<typeof bi>): PortInstance<typeof nr> => {
  return { result: inputs.a + inputs.b }
}

const add4 = (inputs: PortInstance<typeof bi>): number => {
  return inputs.a + inputs.b
}

const subtract = (inputs: BI_Inst): NR_Inst => {
  return { result: inputs.a - inputs.b }
}

const add_node: NodeTemplate<BI, NR> = {
  inputs: bi,
  outputs: nr,
  compute: add
}
const add_node2: NodeTemplate<typeof bi, typeof nr> = {
  inputs: bi,
  outputs: nr,
  compute: add2
}
const add_node3: NodeTemplate<typeof bi, typeof nr> = {
  inputs: bi,
  outputs: nr,
  compute: add3
}

interface NodeInstance<IT extends PortTemplate = any, OT extends PortTemplate = any> extends NodeTemplate<IT, OT> {
  id: string
  heldValue?: PortInstance<OT>
}




type PortReference = {
  nodeId: string
  portId: string
}

type Edge = {
  from: PortReference
  to: PortReference
}


type Graph = {
  nodes: NodeInstance[]
  edges: Edge[]
}



// Helper to build adjacency list from the edges.
// key is a node Id, the value is a list of connected node Ids
function buildAdjacencyList(nodes: NodeInstance[], edges: Edge[]): Record<string, string[]> {
  const adjacencyList: Record<string, string[]> = {};

  nodes.forEach(node => {
    adjacencyList[node.id] = [];
  });

  // Populate the list with directed edges
  edges.forEach(edge => {
    adjacencyList[edge.from.nodeId].push(edge.to.nodeId);
  });

  return adjacencyList;
}
// Helper to build adjacency list from the edges.
// key is a node Id, the value is a list of connected node Ids
function buildDependencyList(nodes: NodeInstance[], edges: Edge[]): Record<string, string[]> {
  const dependencyList: Record<string, string[]> = {};

  nodes.forEach(node => {
    dependencyList[node.id] = [];
  });

  // Populate the list with directed edges
  edges.forEach(edge => {
    dependencyList[edge.to.nodeId].push(edge.from.nodeId);
  });

  return dependencyList;
}

// visit nodes via depth first search
function dfs(nodeId: string, adjacencyList: Record<string, string[]>, visited: Set<string>, stack: string[]): void {
  visited.add(nodeId);

  // Process all adjacent nodes first
  adjacencyList[nodeId].forEach(child => {
    if (!visited.has(child)) {
      dfs(child, adjacencyList, visited, stack);
    }
  });

  // Push current node to stack after processing all dependencies
  stack.push(nodeId);
}

// Main function to perform topological sort
function topologicalSort(graph: Graph): NodeInstance[] {
  const nodes = graph.nodes;
  const edges = graph.edges;
  const adjacencyList = buildAdjacencyList(nodes, edges);
  const visited = new Set<string>();
  const stack: string[] = [];

  // Visit all nodes and perform DFS from unvisited nodes
  nodes.forEach(node => {
    if (!visited.has(node.id)) {
      dfs(node.id, adjacencyList, visited, stack);
    }
  });

  // Return reverse of stack to get correct topological order
  const sortedIds = stack.reverse();
  let sortedNodes: NodeInstance[] = []
  sortedIds.forEach((id) => {
    sortedNodes.push(nodes.find((n) => n.id == id) as NodeInstance)
  })
  return sortedNodes
}
//console.log("g topologically sorted:", topologicalSort(g));
//const gr = { ...g, nodes: g.nodes.reverse() }
//console.log("g reverse sorted:", topologicalSort(gr));

function evaluateGraph(g: Graph) {
  const nodes = topologicalSort(g)
  nodes.forEach((node) => {
    const dependancyEdges = g.edges.filter((edge) => edge.to.nodeId == node.id)
    let input: PortInstance<any> = {}
    dependancyEdges.forEach((edge) => {
      const fromNode = nodes.find((n) => n.id == edge.from.nodeId)
      if (!fromNode) {
        throw `${node} depends on ${edge.from.nodeId}, but it was not found!`
      }
      const dependantValue = fromNode.heldValue
      if (dependantValue === undefined) {
        throw `${node} depends on ${fromNode}, but it has no computed value!`
      }
      input[edge.to.portId] = fromNode.heldValue![edge.from.portId]
    })
    // if (node.inputs != typeof input) {
    //   throw `Expected input for ${JSON.stringify(node)} does not match received input ${JSON.stringify(typeof input)}`
    // }
    node.heldValue = node.compute(input)
    console.log("processed", node.id, "with input", input, "and computed", node.heldValue.result)
  })
}

const constNode = createNodeTemplate({}, nr, () => { return { result: 5 } })

const myNodes: NodeInstance<any, any>[] = [
  { ...constNode, id: "c1" },
  { ...constNode, id: "c2" },
  { ...constNode, id: "c3" },
  { ...add_node, id: "a1" },
  { ...add_node, id: "a2" },
  { ...add_node, id: "a3" },
]
const edges: Edge[] = [
  {
    from: { nodeId: "c1", portId: "result" },
    to: { nodeId: "a1", portId: "a" }
  },
  {
    from: { nodeId: "c2", portId: "result" },
    to: { nodeId: "a1", portId: "b" }
  },
  {
    from: { nodeId: "c3", portId: "result" },
    to: { nodeId: "a2", portId: "a" }
  },
  {
    from: { nodeId: "c2", portId: "result" },
    to: { nodeId: "a2", portId: "b" }
  },
  {
    from: { nodeId: "a1", portId: "result" },
    to: { nodeId: "a3", portId: "a" }
  },
  {
    from: { nodeId: "a2", portId: "result" },
    to: { nodeId: "a3", portId: "b" }
  }
]

const g: Graph = {
  nodes: myNodes,
  edges: edges
}
try {
  evaluateGraph(g)
} catch (error) {
  console.log(error)
}



// interface Bad_Bin extends IOTabel {
//   a: "number"
//   c: "numberArray"
// }
//
// const Bad_Bin_Inst: Bad_Bin = {
//   a: "number",
//   c: "numberArray"
// }
//
//
// const Dup_Infer_Inst = { a: "number", b: "number" }
// interface Dup extends IOTabel = typeof Dup_Infer_Inst
// interface DRes extends IOTabel { result: "number" }
// const DRes_Infer_Inst = { result: "number" }
//
// const add_dup = (inputs: { a: number, b: number }) => {
//   return { result: inputs.a + inputs.b }
// }

//maybe this is fine for now
interface Bin extends PortTemplate { a: "number", b: "number" }
interface Res extends PortTemplate { result: "number" }
const Bin_Infer_Inst: Bin = { a: "number", b: "number" }
const Res_Infer_Inst: Res = { result: "number" }

const add_infer = (inputs: { a: number, b: number }) => {
  return { result: inputs.a + inputs.b }
}

const newAdd = createNodeTemplate(bi, nr, add) // This seems to work!!
// const newAdd_fail = createNodeTemplate(Bad_Bin_Inst, Num_Result_Label_Inst, add)
// const newAdd_dup = createNodeTemplate<Dup, DRes>(Dup_Infer_Inst, DRes_Infer_Inst, add_dup)
const newAdd_infer = createNodeTemplate(Bin_Infer_Inst, Res_Infer_Inst, add_infer)// maybe this is fine for now


//maybe this is fine for now
interface Single_Bool extends PortTemplate { a: "boolean" }
interface Bin_Bool extends PortTemplate { a: "boolean", b: "boolean" }
interface Result_Bool extends PortTemplate { result: "boolean" }
const Single_Bool_Inst: Single_Bool = { a: "boolean" }
const Bin_Bool_Inst: Bin_Bool = { a: "boolean", b: "boolean" }
const Result_Bool_Inst: Result_Bool = { result: "boolean" }



const and = (inputs: { a: boolean, b: boolean }) => {
  return { result: inputs.a && inputs.b }
}
const andTemplate = createNodeTemplate(Bin_Bool_Inst, Result_Bool_Inst, and)

const or = (inputs: { a: boolean, b: boolean }) => {
  return { result: inputs.a || inputs.b }
}
const orTemplate = createNodeTemplate(Bin_Bool_Inst, Result_Bool_Inst, or)

const not = (inputs: { a: boolean }) => {
  return {
    result: !inputs.a
  }
}
const notTemplate = createNodeTemplate(Single_Bool_Inst, Result_Bool_Inst, not)
//console.log(andTemplate)
//TODO instantiate template

