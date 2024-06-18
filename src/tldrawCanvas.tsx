import { Editor, TLShapeId, Tldraw, ZERO_INDEX_KEY, createShapeId, getIndexAbove } from 'tldraw'
import { uiOverrides, customAssetURLs } from './tools/ui-overrides'
import { components } from './tools/ui-overrides'
import { MathTextShapeUtil } from './tools/math/MathShapeUtil'
import { MathShapeTool } from './tools/math/MathShapeTool'

import 'tldraw/tldraw.css'
import './App.css'
import _startShape from "./assets/zeno.json"
import { NodeShape, NodeShapeUtil, createNodeShapeProps } from './tools/node/NodeShapeUtil.tsx'
import { NodeShapeTool } from './tools/node/NodeShapeTool.tsx'
import { useGraph, useGraphDispatch } from './graph/graphContext.tsx'
import { Graph } from './graph/graph.ts'
import { createAddNode, createConstantNode, createMultiplyNode, createSubtractNode } from './graph/nodeDefinitions'

export function TldrawCanvas() {
  console.log("rendering tldraw canvas")
  const graphUI = useGraph()
  const graphDispatch = useGraphDispatch()
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

      editor.sideEffects.registerBeforeCreateHandler('shape', (newShape) => {
        if (newShape.type === 'node') {
          const nodeShape = newShape as NodeShape
          if (graphUI.graph.getNode(newShape.id)) {
            return newShape
          }
          const nodeType = nodeShape.props.nodeType
          console.log("node Type: ", nodeType)
          let newNode = null
          if (nodeType == "Add") {
            newNode = createAddNode(nodeShape.id)
          } else if (nodeType == "Constant") {
            newNode = createConstantNode(nodeShape.id, 0)
          }
          else if (nodeType == "Subtract") {
            newNode = createSubtractNode(nodeShape.id)
          }
          else if (nodeType == "Multiply") {
            newNode = createMultiplyNode(nodeShape.id)
          }
          else {
            console.log("couldn't find node from info!")
            newNode = createAddNode(nodeShape.id)
          }

          console.log("dispatching from editor side effect")
          graphDispatch({ type: "addNode", node: createConstantNode(newShape.id) })
        }
        return newShape
      }
      )

      console.log("onMount")
      createNodesAndArrows(editor, graphUI.graph)
      console.log("done with onMount")
    }}>
  </Tldraw >
}

function createNodesAndArrows(editor: Editor, graph: Graph) {
  const nodes = graph.getNodes()

  const nodeCreationData = nodes.map(node => {
    return {
      node: node,
      TLID: createShapeId(),
      connections: graph.getConnectedNodeInfo(node.nodeId)
    }
  })

  //create the node shapes
  nodeCreationData.forEach((fromNode, i) => {
    editor.createShape({
      id: fromNode.TLID,
      index: getIndexAbove(ZERO_INDEX_KEY),
      type: "node",
      x: 300 + 150 * (i % 2 == 0 ? 1 : -1),
      y: 200 + i * 130,
      props: createNodeShapeProps(fromNode.node)
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
        normalizedAnchor: { x: .5, y: 1 }, // where the arrow starts relative to parent
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

