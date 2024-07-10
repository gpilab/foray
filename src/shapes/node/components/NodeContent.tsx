import { TLShapeId, track, useEditor } from "tldraw";
import { NodeShape, NodeShapeProps } from "../nodeShapeUtil";
import { ConstantNode } from "./constantNode";
import { PlotNode } from "./plotNode";
import { RangeNode } from "./rangeNode";
import { useTheme } from "../../util/useTheme";
import { createContext, useContext } from "react";


export type NodeConfig = {
  id: TLShapeId,
} & NodeShapeProps


const NodeConfigContext = createContext<NodeConfig | null>(null)

export const useNodeConfig = () => {
  const context = useContext(NodeConfigContext)
  if (context === null) {
    throw Error("useNodeConfig must be called from inside NodeConfigContext.Provider")
  }
  return context
}
export const NodeContent = track((props: { nodeShape: NodeShape }) => {
  const { nodeShape } = props
  const { nodeType } = nodeShape.props
  const editor = useEditor()
  const nodeConfig = {
    id: nodeShape.id,
    ...nodeShape.props
  }


  const updateNode = (updatedProps: Partial<NodeShapeProps>) => {
    editor.updateShape({
      id: nodeShape.id,
      type: "node",
      props: { ...nodeShape.props, ...updatedProps }
    })
  }

  return <NodeConfigContext.Provider value={nodeConfig}>
    {(() => {
      switch (nodeType) {
        case ("Constant"):
          return <ConstantNode updateNode={updateNode} />
        case ("Plot"):
          return <PlotNode />
        case ("Range"):
          return <RangeNode updateNode={updateNode} />
        default:
          return <DefaultNode />
      }
    })()}
  </NodeConfigContext.Provider >
})


const DefaultNode = track(() => {
  const nodeType = useNodeConfig().nodeType
  const theme = useTheme()
  const infoColor = theme.grey

  return <div style={{ width: "100%", height: "100%", display: "flex", justifyContent: "end" }}>
    <div style={{ color: infoColor, padding: "5px" }}>
      {nodeType}
    </div>
  </div>
})

