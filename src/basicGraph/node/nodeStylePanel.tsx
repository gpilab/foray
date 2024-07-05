import {
  DefaultStylePanel, DefaultStylePanelContent,
  StyleProp, T, getDefaultColorTheme,
  useEditor, useIsDarkMode, useRelevantStyles
} from "tldraw"


/**
 * TLDraw Styles let us manipulate multiple shapes at once.
 * Node types are implemented as styles so that we can edit
 * multiple nodes at once based on their shared properties,
 * i.e. adjusting visual settings on multiple "Plot" components
 */
export const nodeTypeStyle = StyleProp.defineEnum('gpi:node_type', {
  defaultValue: "Add",
  values: ["Add", "Subtract", "Multiply", "Constant"],
})

export type NodeTypeStyle = T.TypeOf<typeof nodeTypeStyle>

export function NodeStylePanel() {
  const editor = useEditor()
  const isDarkMode = useIsDarkMode()
  const theme = getDefaultColorTheme({ isDarkMode })
  const styles = useRelevantStyles()
  if (!styles) {
    return null
  }

  const rating = styles.get(nodeTypeStyle)

  return (
    <DefaultStylePanel>
      <DefaultStylePanelContent styles={styles} />
      {rating !== undefined && (
        <div style={{
          padding: "5px",
          paddingBottom: "10px"
        }}>
          <select
            style={{
              width: '100%',
              padding: "5px",
              color: theme["text"],
              backgroundColor: theme["grey"].semi,
              borderRadius: "8px",
              borderColor: theme["grey"].semi
            }}
            value={rating.type === 'mixed' ? '' : rating.value}
            onChange={(e) => {
              editor.mark('changing rating')
              const value = nodeTypeStyle.validate(e.currentTarget.value)
              editor.setStyleForSelectedShapes(nodeTypeStyle, value)
            }}
          >
            {rating.type === 'mixed' ? <option value="">Mixed</option> : null}
            <option value={"Add"}>Add</option>
            <option value={"Subtract"}>Subtract</option>
            <option value={"Multiply"}>Multiply</option>
            <option value={"Constant"}>Constant</option>
          </select>
        </div>
      )
      }
    </DefaultStylePanel >
  )
}
