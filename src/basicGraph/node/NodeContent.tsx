import { getDefaultColorTheme, useIsDarkMode } from "tldraw";
import { NodeTypeStyle } from "./nodeStylePanel";

export function NodeContent(props: { nodeType: NodeTypeStyle }) {
  const isDarkMode = useIsDarkMode()
  const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })

  const infoColor = theme["grey"].solid
  return <div style={{ width: "100%", height: "100%", display: "flex", justifyContent: "end" }}>
    <div style={{ color: infoColor, padding: "5px" }}>
      {props.nodeType}
    </div>
  </div>
}
