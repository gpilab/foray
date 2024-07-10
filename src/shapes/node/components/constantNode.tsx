import { track } from "tldraw"
import { NodeShapeProps } from "../nodeShapeUtil"
import { NumericInput } from "../../components/numericInput"
import { useNodeConfig } from "./NodeContent"

export const ConstantNode = track((props: { updateNode: (updatedProps: Partial<NodeShapeProps>) => void }) => {
  const { updateNode } = props
  const { output } = useNodeConfig()

  const updateConfig = (value: number) => { updateNode({ config: { value } }) }

  return <div style={{ width: "100%", height: "100%", display: "flex", alignItems: "center" }}>
    <NumericInput value={output.out.value} setValue={updateConfig} />
  </div>
})
