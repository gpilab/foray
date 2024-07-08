import { getDefaultColorTheme, track, useEditor, useIsDarkMode } from "tldraw";
import { NodeTypeStyle } from "./nodeStylePanel";
import { Config } from "./nodeDefinitions";
import { NodeShape, NodeShapeProps } from "./nodeShapeUtil";
import { useState } from "react";

export const NodeContent = track((props: { nodeShape: NodeShape }) => {
  const { nodeShape } = props
  const { nodeType } = nodeShape.props
  const editor = useEditor()


  const updateNode = (updatedProps: Partial<NodeShapeProps>) => {
    editor.updateShape({
      id: nodeShape.id,
      type: "node",
      props: { ...nodeShape.props, ...updatedProps }
    })
  }

  switch (nodeType) {
    case ("Constant"):
      return <ConstantNode nodeShape={nodeShape} updateNode={updateNode} config={{ value: 10 }} />
    default:
      return <DefaultNode nodeType={nodeType} />

  }
})


const DefaultNode = track((props: { nodeType: NodeTypeStyle }) => {
  const isDarkMode = useIsDarkMode()
  const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })

  const infoColor = theme["grey"].solid

  return <div style={{ width: "100%", height: "100%", display: "flex", justifyContent: "end" }}>
    <div style={{ color: infoColor, padding: "5px" }}>
      {props.nodeType}
    </div>
  </div>

})


const ConstantNode = track((props: { config: Config, nodeShape: NodeShape, updateNode: (updatedProps: Partial<NodeShapeProps>) => void }) => {
  const editor = useEditor()
  const { nodeShape, updateNode } = props
  const { output } = nodeShape.props

  const [unvalidatedValue, setUnvalidatedState] = useState(output.out.value.toString())
  const [inputError, setInputError] = useState(false)
  const isDarkMode = useIsDarkMode()
  const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })

  const infoColor = theme["grey"].solid

  return <div style={{ width: "100%", height: "100%", display: "flex", alignItems: "center" }}>

    <div style={{ color: infoColor, padding: "5px" }}>
      {unvalidatedValue?.toString().length > 0
        ? ""
        : <div style={{
          position: "absolute",
          left: "0",
          right: "0",
          marginLeft: "auto",
          marginRight: "auto",
          width: 40,
          height: 50,
          backgroundColor: theme.red.solid,
          borderRadius: 5
        }} />}
      <input style={{
        position: "relative",
        width: "100%",
        fontSize: "40px",
        pointerEvents: "all",
        textAlign: "center",
        border: "none",
        color: theme.black.solid,
        textDecoration: inputError ? "underline" : "",
        backgroundColor: "transparent",
        textDecorationColor: inputError ? theme.red.solid : theme.text,
        borderRadius: "5px"
      }}
        autoComplete="off"
        id={nodeShape.id + "input"}
        value={unvalidatedValue}
        onClick={(_) => { editor.cancelDoubleClick() }}
        onChange={(e) => {
          const amount = e.target.value
          setUnvalidatedState(amount)
          console.log(amount)
          if (amount.match(/^-{0,1}\d+(\.\d*)?$/)) {
            setInputError(false)
            updateNode({ config: { value: parseFloat(amount) } })
          } else {
            setInputError(true)
          }
        }
        }>
      </input>
    </div>
  </div>
})

