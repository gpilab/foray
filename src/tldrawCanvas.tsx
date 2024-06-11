import { Editor, TLShapeId, Tldraw, ZERO_INDEX_KEY, createShapeId, getIndexAbove } from 'tldraw'
import { uiOverrides, customAssetURLs } from './tools/ui-overrides'
import { components } from './tools/ui-overrides'
import { MathTextShapeUtil } from './tools/math/MathShapeUtil'
import { MathShapeTool } from './tools/math/MathShapeTool'

import 'tldraw/tldraw.css'
import './App.css'
import _startShape from "./assets/zeno.json"
import { NodeShapeUtil } from './tools/node/NodeShapeUtil.tsx'
import { NodeShapeTool } from './tools/node/NodeShapeTool.tsx'
import { useGraph } from './graph/graphContext.tsx'
import { Graph } from './graph/graph.ts'

export function TldrawCanvas() {
  const graphUI = useGraph()
  return < Tldraw
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

      createNodesAndArrows(editor, graphUI.graph)
    }}>
  </Tldraw >
}

function createNodesAndArrows(editor: Editor, graph: Graph) {
  const nodes = graph.getNodes()

  const nodeCreationData = nodes.map(node =>
  ({
    node: node,
    TLID: createShapeId(),
    props: {
      nodeId: node.nodeId,
      nodeType: node.nodeType,
      inputPorts: node.inputPorts,
      outputPort: node.outputPort,
      w: 200,
      h: 100
    },
    connections: graph.getConnectedNodeInfo(node.nodeId)
  }))

  //create the node shapes
  nodeCreationData.forEach((fromNode, i) => {
    editor.createShape({
      id: fromNode.TLID,
      index: getIndexAbove(ZERO_INDEX_KEY),
      type: "node",
      x: 300 + 150 * (i % 2 == 0 ? 1 : -1),
      y: 200 + i * 130,
      props: fromNode.props
    })
  })


  const portConnections = nodeCreationData.map(fromNode =>
    fromNode.connections.map(portInfo => ({
      ...portInfo,
      fromTLID: fromNode.TLID,
      toTLID: nodeCreationData.find((n) => n.node.nodeId == portInfo.nodeId)?.TLID
    }))).flat()

  //create arrows for each connection
  portConnections.forEach(({ port, fromTLID, toTLID, portIndex }) => {
    if (toTLID === undefined) {
      throw Error("Initial graph creation caused malformed node connection")
    }
    editor.createShape(createArrow(fromTLID, toTLID, port.name, portIndex))
  })
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
        normalizedAnchor: { x: .15, y: 1.08 }, // where the arrow starts relative to parent
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

