
type FlowType = number | number[]

type Port<T> = {
  //id: string
  dataType?: T
}

type GPI_Node<InputTypes, OutputTypes> = {
  label: string
  id: string
  inputs: { [K in keyof InputTypes]: Port<InputTypes[K]> }
  outputs: { [K in keyof OutputTypes]: Port<OutputTypes[K]> }
  compute: (inputs: InputTypes) => OutputTypes
}

type constIn = {}
type constOut<T> = {
  a: T
}

type BinaryIn<T> = {
  a: T
  b: T
}


const add: GPI_Node<BinaryIn<number>, number> = {
  label: "add",
  id: "add1",
  inputs: {
    a: { portName: "number" }, b: { dataType: "number" }
  },
  compute: (inputs) => {
    console.log("add", inputs.a, inputs.b);
    return inputs.a + inputs.b
  }

}

const constNode7: GPI_Node = {
  label: "constant 7",
  id: "const7",
  inputs: [],
  outputs: [{ id: "a" }],
  compute: (a, b) => 7
}
const constNode4: GPI_Node = {
  label: "constant 4",
  id: "const4",
  inputs: [],
  outputs: [{ id: "a" }],
  compute: (a, b) => 4
}



const subtract: GPI_Node = {
  label: "subtract",
  id: "add2",
  inputs:
    outputs: [{ id: "c" }],
  compute: (inputs) => {
    console.log("subtract", inputs.a, inputs.b);
    return a - b
  }
}

type Graph = {
  nodes: GPI_Node[],
  wires: Wire[]
}

const g: Graph = {
  nodes: [constNode7, constNode4, add],
  wires: [{ startNode: constNode7, startPortId: "a", endNode: add, endPortId: "a" }
    , { startNode: constNode4, startPortId: "a", endNode: add, endPortId: "b" }]
}


function getParentNodes(node: GPI_Node, wires: Wire[]) {
  let parents: GPI_Node[] = []
  node.inputs.forEach((input) => {
    let wire = wires.find((w) => (w.endNode == node && w.endPortId == input.id))
    if (wire) {
      parents.push(wire.startNode)
    }
  })
  return parents
}

function evalNode(node: GPI_Node, processedNodes: GPI_Node[], wires: Wire[]) {
  if (node.inputs.length == 0) {
    console.log("reached leaf!", node)
    processedNodes.push(node)
    return node.compute(0, 0)
  }
  const parents = getParentNodes(node, wires)
  parents.forEach((p) => evalNode(p, processedNodes, wires))

  processedNodes.push(node)
  return node.compute(node.inputs)

}

function evalGraph(g: Graph, startNodeIndex: number) {
  let currentNode = g.nodes[startNodeIndex]
  let processedNodes = []

  while (processedNodes.length < g.nodes.length) {
    const inputs = currentNode.inputs
    const inputValues = new Array<number>
    inputs.forEach((input) => {
      let wire = g.wires.find((w) => (w.endNode == currentNode && w.endPortId == input.id))
      if (wire) {
        const valueForInput = wire.startNode.compute(0, 0)
        inputValues.push(valueForInput)
        processedNodes.push(wire.startNode)
      }
    })
    const value = currentNode.compute(inputValues[0], inputValues[1])
    console.log(value)
    processedNodes.push(currentNode)

  }

}

evalGraph(g, 2)



