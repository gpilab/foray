import {
  DefaultStylePanel, DefaultStylePanelContent,
  StyleProp, T,
  useEditor, useRelevantStyles
} from "tldraw"
import { nodeTypes } from "./nodeType"
import { useTheme } from "../../util/useTheme"


/**
 * TLDraw Styles let us manipulate multiple shapes at once.
 * Node types are implemented as styles so that we can edit
 * multiple nodes at once based on their shared properties,
 * i.e. adjusting visual settings on multiple "Plot" components
 */
export const nodeTypeStyle = StyleProp.defineEnum('gpi:node_type', {
  defaultValue: "Add",
  values: nodeTypes
})
export const showPlotGridStyle = StyleProp.defineEnum('gpi:show_plot_grid', {
  defaultValue: true,
  values: [true, false]
})

export type NodeTypeStyle = T.TypeOf<typeof nodeTypeStyle>

export function NodeStylePanel() {
  const editor = useEditor()
  const theme = useTheme()
  const styles = useRelevantStyles()
  if (!styles) {
    return null
  }

  const nodeType = styles.get(nodeTypeStyle)
  const showPlotGrid = styles.get(showPlotGridStyle)

  return (
    <DefaultStylePanel>
      <DefaultStylePanelContent styles={styles} />
      {nodeType !== undefined && (
        <div style={{
          padding: "5px",
          paddingBottom: "10px"
        }}>
          <select
            style={{
              width: '100%',
              padding: "5px",
              color: theme.text,
              backgroundColor: theme.grey,
              borderRadius: "8px",
              borderColor: theme.grey
            }}
            value={nodeType.type === 'mixed' ? '' : nodeType.value}
            onChange={(e) => {
              editor.mark('changing nodeType')
              const value = nodeTypeStyle.validate(e.currentTarget.value)
              editor.setStyleForSelectedShapes(nodeTypeStyle, value)
            }}
          >
            {nodeType.type === 'mixed' ? <option value="">Mixed</option> : null}
            {nodeTypes.map(n =>
              <option key={n} value={n}>{n}</option>
            )}
          </select>
        </div>
      )
      }
      {showPlotGrid !== undefined && showPlotGrid.type !== 'mixed' && (
        <div style={{ padding: "5px" }}>
          <label>
            <input type="checkbox"
              checked={showPlotGrid.value}

              onChange={() => {
                editor.mark('changing nodeType')
                const value = showPlotGridStyle.validate(!showPlotGrid.value)
                editor.setStyleForSelectedShapes(showPlotGridStyle, value)
              }}
            />

            Show Grid
          </label>
        </div>
      )
      }
    </DefaultStylePanel >
  )
}
