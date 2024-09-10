import { ReactNode, useContext, useRef } from "react"
import { createShapeId, EASINGS, track, useEditor } from "tldraw"
import './custom-ui.css'
import { useHover } from "usehooks-ts"
import { GPIContext } from "../gpi"
import { Idle, WireTool } from "../shapes/wire/WireTool"

export const NodeSelect = track(() => {
  const editor = useEditor()
  const gpi_state = useContext(GPIContext)

  let node_names = gpi_state.NodeDefinitions.map(n => n.state.type)

  let active = editor.getCurrentToolId() === 'wire'

  const handle_select = (node_name: string) => {
    console.log("Selected create node: ", node_name)
    const { center: view_center } = editor.getViewportPageBounds()
    //TODO: get the actual node dimensions
    const node_width = 100
    const node_height = 50
    const node_location = {
      x: view_center.x,
      y: view_center.y + node_height * 3
    }
    //create the wire, and bind it to target
    const nodeId = createShapeId()

    editor.mark(`creating: ${nodeId}`)
    editor.createShape({
      id: nodeId,
      type: 'node',
      x: node_location.x - node_width / 2,
      y: node_location.y - node_height / 2,
      props: { nodeType: node_name }
    }).centerOnPoint(node_location, {
      animation: {
        duration: 800,
        easing: EASINGS.easeOutCubic
      }
    })
    // run wire tool complete code
    let tool = editor.getCurrentTool().getCurrent() as Idle;
    tool.onComplete()
  }

  return <>
    {active ? <div className="node-select-frame">
      < GridSelect items={node_names} onSelectionChange={(name) => { handle_select(name) }} />
    </div >
      : <></>}
  </>
})


interface GridSelectProps {
  items: string[]; // Array of items to display
  onSelectionChange: (selectedItem: string) => void;
}

export const GridSelect = ({ items, onSelectionChange }: GridSelectProps) => {
  // Handler for selecting/deselecting items
  return (
    <div className="grid">
      {items.map((item, index) => (
        <div
          key={index}
          className={`grid-item`}
          onPointerDown={() => onSelectionChange(item)}
          style={{ pointerEvents: "all" }}
        >
          <Hoverable>{item}</Hoverable>
        </div>
      ))}
    </div>
  );
}

export const Hoverable = ({ children }: { children: ReactNode }) => {
  const divRef = useRef<HTMLDivElement>(null);
  const isHovered = useHover(divRef);
  return <div ref={divRef} className={isHovered ? 'hover' : ''}>
    {children}
  </div>
}
