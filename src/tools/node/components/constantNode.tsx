import { useState } from "react"
import { NodeBaseProps } from "./nodeBase"
import { IOPort } from "./Ports"

export function ConstantNode({ nodeData }: { nodeData: NodeBaseProps }) {
  const { nodeId, outputPort, currentValue, handleValueUpdate } = nodeData
  const [unvalidatedValue, setUnvalidatedState] = useState(currentValue?.toString() ? currentValue?.toString() : "")
  const [inputError, setInputError] = useState(false)
  return <IOPort port={outputPort} width={nodeData.width} height={nodeData.height}>
    <input style={{
      width: "100%", fontSize: "20px",
      pointerEvents: "all",
      textAlign: "center",
      border: "none",
      background: unvalidatedValue?.toString().length > 0 ? "none" : "#777",
      color: "white",
      textDecoration: "underline",
      textDecorationColor: inputError ? "red" : "white",
    }}
      autoComplete="off"
      id={nodeId + "input"}
      //inputMode="decimal"
      value={unvalidatedValue}
      onChange={(e) => {
        const amount = e.target.value
        setUnvalidatedState(amount)
        if (!amount || amount.match(/^\d{1,}(\.\d{0,4})?$/)) {
          setInputError(false)
          return handleValueUpdate(parseFloat(amount), "none")
        } else {
          setInputError(true)
        }
      }
      }>
    </input>
  </IOPort >
}
