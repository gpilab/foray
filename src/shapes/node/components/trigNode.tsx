import { track } from "tldraw"
import { NodeShapeProps } from "../nodeShapeUtil"
import { NumericInput } from "../../components/numericInput"
import { sinDef } from "../nodeDefinitions"
import { ReactNode } from "react"
import { useNodeConfig } from "./NodeContent"
import { BlockMath } from "react-katex"

export const TrigNode = track((props: { updateNode: (updatedProps: Partial<NodeShapeProps>) => void }) => {

  const { updateNode } = props
  const type = useNodeConfig().nodeType
  const config = useNodeConfig().config as typeof sinDef.state.config

  const updateConfig = (configUpdate: Partial<typeof config>) => {
    updateNode({
      config:
        { ...config, ...configUpdate }
    })
  }

  return <div style={{ height: "100%", display: "flex", flexDirection: "column", alignItems: "stretch", justifyContent: "space-around" }}>
    <div style={{ padding: "5px" }} >
      <BlockMath math={`A *\\rm{${type}}(f x+\\phi)`} />
    </div>
    <div style={{ width: "100%", display: "flex", flexDirection: "row", alignItems: "center", justifyContent: "flex-end", paddingLeft: "5px" }}>
      <TrigInput>
        <div style={{ color: "grey" }}>
          <BlockMath math="A" />
        </div>
        <NumericInput value={config.amplitude} setValue={
          (value: number) => updateConfig({ amplitude: value })}
          textAlign="start"
        />
      </TrigInput>
      <TrigInput>
        <div style={{ color: "grey" }}>
          <BlockMath math="f" />
        </div>
        <NumericInput value={config.frequency} setValue={
          (value: number) => updateConfig({ frequency: value })}
          textAlign="start"
        />
      </TrigInput>
      <TrigInput>
        <div style={{ color: "grey" }}>
          <BlockMath math="\phi" />
        </div>
        <NumericInput value={config.phaseOffset} setValue={
          (value: number) => updateConfig({ phaseOffset: value })}
          textAlign="start"
        />
      </TrigInput>
    </div >
  </div>
})

export function TrigInput(props: { children: ReactNode }) {
  return <div style={{ display: "flex", alignItems: "center" }}>
    {props.children}
  </div>

}
