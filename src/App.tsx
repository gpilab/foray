import { TLShapeId, Tldraw, ZERO_INDEX_KEY, createShapeId, getIndexAbove } from 'tldraw'
import { uiOverrides, customAssetURLs } from './tools/ui-overrides'
import { components } from './tools/ui-overrides'
import { MathTextShapeUtil } from './tools/math/MathShapeUtil'
import { MathShapeTool } from './tools/math/MathShapeTool'

import 'tldraw/tldraw.css'
import './App.css'
import _startShape from "./assets/zeno.json"
import { NodeShapeUtil } from './tools/node/NodeShapeUtil.tsx'
import { NodeShapeTool } from './tools/node/NodeShapeTool.tsx'
import { GraphProvider, GraphUI } from './graph/graphContext.tsx'
import { Graph } from './graph/graph.ts'
import { Node, port, port2 } from './graph/node.ts'
// import { NodeContext, useNode } from './graph/useNode.ts'



const createConstantNode = (id: string) => new Node(port("x", "number"), "number", (x: number) => x, id, "Constant");
const createSumNode = (id: string) => new Node(port2("x", "number", "y", "number"), "number", (x: number, y: number) => { return x + y }, id, "Sum");
const createMultiplyNode = (id: string) => new Node(port2("x", "number", "y", "number"), "number", (x: number, y: number) => { return x * y }, id, "Multiply");

const c1 = createConstantNode("c1")
const c2 = createConstantNode("c2")
const c3 = createConstantNode("c3")
const c4 = createConstantNode("c4")
//const outSink = createConstantNode("outSink", 0)
const s1 = createSumNode("s1")
const s2 = createSumNode("s2")

const m1 = createMultiplyNode("m1")

const initialGraph: Graph = new Graph()
initialGraph.addNode(c2)
initialGraph.addNode(c3)

initialGraph.addNode(s1)
initialGraph.connectNodes(c3, s1, "x")
initialGraph.connectNodes(c2, s1, "y")

initialGraph.addNode(c1)
initialGraph.addNode(c4)
//initialGraph.addNode(outSink)
initialGraph.addNode(s2)
initialGraph.connectNodes(c1, s2, "x")
initialGraph.connectNodes(s1, s2, "y")

initialGraph.addNode(m1)
initialGraph.connectNodes(c4, m1, "y")
initialGraph.connectNodes(s2, m1, "x")
//initialGraph.connectNodes(sumNode, outSink, "x")

c4.getInputStream("x").next(1)
c1.getInputStream("x").next(2)
c2.getInputStream("x").next(3)
c3.getInputStream("x").next(4)

//TODO why isn't sum updating in UI? is the node itself updated?

const initialGraphUI: GraphUI = { graph: initialGraph }



export default function CustomUiExample() {
  return (
    <GraphProvider initialGraphUI={initialGraphUI} >
      <Tldraw
        shapeUtils={[MathTextShapeUtil, NodeShapeUtil]}
        tools={[MathShapeTool, NodeShapeTool]}
        overrides={uiOverrides}
        components={components}
        assetUrls={customAssetURLs}

        inferDarkMode
        persistenceKey='gpi_v2'
        onMount={(editor) => {
          editor
            .selectAll()
            .deleteShapes(editor.getSelectedShapeIds())
          //
          const nodes = initialGraphUI.graph.getNodes()
          const nodeCreationData = nodes.map((n) => {
            return {
              node: n,
              TLID: createShapeId(),
              props: {
                nodeId: n.id,
                nodeType: n.nodeType,
                inputTypes: n.inputPorts, //TODO use more consistent names across node and shape
                outputType: n.outputType,
                w: 200,
                h: 100
              }
            }
          })

          nodeCreationData.forEach((fromNode, i) => {
            editor.createShape({
              id: fromNode.TLID, index: getIndexAbove(ZERO_INDEX_KEY), type: "node", x: 300 + 150 * (i % 2 == 0 ? 1 : -1), y: 200 + i * 130, props: fromNode.props
            })
          })

          nodeCreationData.forEach((fromNode) => {
            //create connections
            const connectedNodeInfo = initialGraphUI.graph.getConnectedNodeInfo(fromNode.node.id)
            if (connectedNodeInfo != undefined) {
              const connectedNodeInfoTLIDs = connectedNodeInfo.map(
                (nodeInfo) => {
                  const toNodeData = nodeCreationData.find((n) => n.node.id == nodeInfo.nodeId)
                  return { ...nodeInfo, tlID: toNodeData!.TLID }
                })
              connectedNodeInfoTLIDs.forEach(({ port, tlID, portIndex }) => {
                if (tlID === undefined) {
                  throw Error("Connection could not be made between nodes")
                }
                editor.createShape(createArrow(fromNode.TLID, tlID, port.name, portIndex))
              })
            }
          })



        }}
      >
      </Tldraw >
    </GraphProvider >
  )
}

function createArrow(startId: TLShapeId, endId: TLShapeId, _label: string, portIndex: number) {
  const arrowId = createShapeId()
  return {
    id: arrowId,
    index: ZERO_INDEX_KEY,
    type: 'arrow',
    x: 100,
    y: 100,
    props: {
      start: {
        type: 'binding',
        isExact: true,
        boundShapeId: startId,
        normalizedAnchor: { x: .15, y: 1.08 },
        isPrecise: true,
      },
      end: {
        type: 'binding',
        isExact: true,
        boundShapeId: endId,
        normalizedAnchor: { x: .1 + .18 * portIndex, y: -.05 },
        isPrecise: true,
      },
      //text: label,
      size: "s"
    },
  }
}
