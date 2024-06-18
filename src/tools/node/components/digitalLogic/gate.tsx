import { BlockMath } from "react-katex"
import { NodeBaseProps } from "../nodeBase"
import { Ports } from "../Ports"

type DefaultNodeProps = {
  showInfo?: boolean,
  nodeData: NodeBaseProps,
  children?: React.ReactNode
}

export function Gate({ showInfo = true, nodeData, children }: DefaultNodeProps) {
  const { width, height, nodeAttributes, nodeId, inputPorts, outputPort, currentValue } = nodeData
  return <div>
    <Ports portIO="in" ports={inputPorts} />
    <div
      style={{
        width: `${width}px`,
        height: `${height}px`,
        pointerEvents: "all",
        border: "2px solid #aaa",
        borderRadius: "4px",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        flexDirection: "column"
      }}>
      {showInfo &&
        <div className="nodeInfo">
          <div className="nodeType">{nodeAttributes.type}</div>
          <div className="nodeId">{nodeId} </div>
        </div>
      }
      {nodeAttributes.equation &&
        <div style={{ fontSize: "30px" }}>
          <BlockMath math={nodeAttributes.equation} />
        </div>
      }

      {children}
    </div >
    <div style={{ width: "100%", display: "flex", justifyItems: "center", alignItems: "center" }}>
      <Ports portIO="out" ports={[outputPort]} currentValue={currentValue} />
    </div >
  </div >
}
