import { useEditor } from "tldraw";
import { NodeSelect } from "./NodeSelect"
import { GPIContext } from "../gpi";
import { useContext, useEffect } from "react";
import { NodeShape } from "../shapes/node/nodeShapeUtil";


export const GPI_UI = () => {
  const editor = useEditor();
  const gpi_context = useContext(GPIContext);
  useEffect(() => {
    console.log("Should I reload all the nodes now??");
    const nodes = editor.getCurrentPageShapes().filter(shape => shape.type == "node"
    ) as NodeShape[]

    console.log(nodes);
    nodes.filter(node => Object.entries(node.props.inputs).length == 0)
      .map(node => editor.updateShape(
        {
          id: node.id,
          type: "node",
          props: {
            ...node.props,
            config: {
              ...node.props.config,
              version: (node.props.config.version ?? 1) + 1
            }
          }
        }))
  }, [gpi_context]);


  return <div className="gpi-ui-layout" style={{ pointerEvents: "none" }}>
    <NodeSelect />
  </div>
}

