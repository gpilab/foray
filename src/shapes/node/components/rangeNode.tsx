import { track } from "tldraw"
import { NodeShapeProps } from "../nodeShapeUtil"
import { NumericInput } from "../../components/numericInput"
import { rangeDef } from "../nodeDefinitions"
import { ReactNode } from "react"
import { useNodeConfig } from "./NodeContent"
import { BlockMath } from "react-katex"

export const RangeNode = track((props: { updateNode: (updatedProps: Partial<NodeShapeProps>) => void }) => {

  const { updateNode } = props
  const config = useNodeConfig().config as typeof rangeDef.state.config

  const updateConfig = (configUpdate: Partial<typeof config>) => {
    updateNode({
      config:
        { ...config, ...configUpdate }
    })
  }

  return <div style={{
    height: "100%",
    display: "flex",
    flexDirection: "column",
    alignItems: "stretch",
    justifyContent: "space-around"
  }}>

    <div style={{ padding: "5px" }} >
      <BlockMath math="{\rm{range}}(a,b,\Delta x)" />
    </div>
    <div style={{
      width: "100%",
      display: "flex",
      flexDirection: "row",
      alignItems: "center",
      paddingLeft: "5px"
    }}>
      <RangeInput>
        <BlockMath math="a:" />
        <NumericInput value={config.start} setValue={
          (value: number) => updateConfig({ start: value })}
          textAlign="start"
        />
      </RangeInput>
      <RangeInput>
        <BlockMath math="b:" />
        <NumericInput value={config.end} setValue={
          (value: number) => updateConfig({ end: value })}
          textAlign="start"
        />
      </RangeInput>
      <RangeInput>
        <BlockMath math="\Delta x:" />
        <NumericInput value={config.step} setValue={
          (value: number) => updateConfig({ step: value })
        } validator={(input) => parseFloat(input) > 0}
          textAlign="start"
        />
      </RangeInput>
    </div >
  </div >
})

export function RangeInput(props: { children: ReactNode }) {
  return <div style={{ display: "flex", flexDirection: "row", alignItems: "center" }}>
    {props.children}
  </div>

}
