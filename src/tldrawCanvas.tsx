
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
import { useGraph } from './graph/graphContext.tsx'

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

      const nodes = graphUI.graph.getNodes()

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
          },
          connections: graphUI.graph.getConnectedNodeInfo(n.id)
        }
      })

      nodeCreationData.forEach((fromNode, i) => {
        editor.createShape({
          id: fromNode.TLID, index: getIndexAbove(ZERO_INDEX_KEY), type: "node", x: 300 + 150 * (i % 2 == 0 ? 1 : -1), y: 200 + i * 130, props: fromNode.props
        })
      })

      nodeCreationData.forEach((fromNode) => {
        //create connections
        const connectedNodeInfo = graphUI.graph.getConnectedNodeInfo(fromNode.node.id)
        if (connectedNodeInfo != undefined) {
          const connectedNodeInfoTLIDs = connectedNodeInfo.map(
            (nodeInfo) => {
              const toNodeData = nodeCreationData.find((n) => n.node.id == nodeInfo.nodeId)
              return { ...nodeInfo, tlID: toNodeData!.TLID }
            })
          connectedNodeInfoTLIDs.forEach(({ port, tlID, nodeId, portIndex }) => {
            if (tlID === undefined) {
              throw Error("Connection could not be made between nodes")
            }
            console.log("creating connection ", fromNode.node.id, " ", nodeId)
            editor.createShape(createArrow(fromNode.TLID, tlID, port.name, portIndex))
          })
        }
      })
    }
    }
  >
  </Tldraw >
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
