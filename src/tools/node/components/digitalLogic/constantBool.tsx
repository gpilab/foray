import { stopEventPropagation } from "tldraw"
import { NodeBaseProps } from "../nodeBase"
import { IOPort } from "../Ports"

export function ConstantBool({ nodeData }: { nodeData: NodeBaseProps }) {
  const { nodeId, outputPort, currentValue, handleValueUpdate
  } = nodeData
  return <IOPort port={outputPort} width={nodeData.width} height={nodeData.height}>
    <div id={nodeId + "bool"}
      style={{ pointerEvents: "all" }}
      onPointerDown={(e) => {
        stopEventPropagation(e)
        return handleValueUpdate(!currentValue, "none")
      }}
    >
      <ToggleIndicator state={currentValue as boolean} />
    </div>
  </IOPort >
}

function ToggleIndicator({ state }: { state: boolean }) {
  const style = state ?
    { backgroundColor: "limegreen", borderColor: "white" }
    : { backgroundColor: "darkgreen", borderColor: "grey" }
  return <div style={{
    ...style,
    width: "20px",
    height: "20px",
    borderWidth: "2px",
    borderStyle: "solid",
    borderRadius: "2px",
  }}>
  </div>
}
