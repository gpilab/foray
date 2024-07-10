import { track } from "tldraw"
import { NodeShapeProps } from "../nodeShapeUtil"
import { NumericInput } from "../../components/numericInput"
import { rangeDef } from "../nodeDefinitions"
import { ReactNode } from "react"
import { useNodeConfig } from "./NodeContent"

export const RangeNode = track((props: { updateNode: (updatedProps: Partial<NodeShapeProps>) => void }) => {

  const { updateNode } = props
  const config = useNodeConfig().config as typeof rangeDef.state.config

  const updateConfig = (configUpdate: Partial<typeof config>) => {
    updateNode({
      config:
        { ...config, ...configUpdate }
    })
  }

  return <div style={{ width: "100%", height: "100%", display: "flex", alignItems: "center" }}>
    <RangeInput >
      start
      <NumericInput value={config.start} setValue={
        (value: number) => updateConfig({ start: value })
      } />
    </RangeInput >
    <RangeInput >
      end
      <NumericInput value={config.end} setValue={
        (value: number) => updateConfig({ end: value })
      } />
    </RangeInput >
    <RangeInput >
      step
      <NumericInput value={config.step} setValue={
        (value: number) => updateConfig({ step: value })
      } validator={(input) => parseFloat(input) > 0} />
    </RangeInput >
  </div >
})

export function RangeInput(props: { children: ReactNode }) {
  return <div style={{ display: "flex", flexDirection: "column", alignItems: "center" }}>
    {props.children}
  </div>

}
