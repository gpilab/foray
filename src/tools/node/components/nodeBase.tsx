import { ReactNode } from "react"
import { Port, PortTypes } from "../../../graph/node"
import { NodeAttributes } from "../../../graph/nodeDefinitions"
import { DefaultNode } from "./defaultNode"
import "./nodeStyle.css"
import { ConstantNode } from "./constantNode"
import { ConstantBool } from "./digitalLogic/constantBool"

export type NodeBaseProps = {
  width: number
  height: number
  //width and height from tldraw should be used as the source of truth
  nodeAttributes: Omit<NodeAttributes, "width" | "height">
  inputPorts: Port[]
  outputPort: Port
  nodeId: string
  currentValue: PortTypes
  handleValueUpdate: (val: any, portLabel: string) => void // can this be well typed?
}


export function NodeBase(p: NodeBaseProps) {
  return <div>
    {chooseComponent(p)}
  </div>
}

function chooseComponent(nodeData: NodeBaseProps): ReactNode {
  const { nodeAttributes, } = nodeData
  if (nodeAttributes == undefined) {
    return <div>Child Placing...</div>
  }
  switch (nodeAttributes.type) {
    case ("Constant"): {
      return <ConstantNode nodeData={nodeData} />
    }
    case ("Boolean"): {

      return <ConstantBool nodeData={nodeData} />
    }
    default: {
      return <DefaultNode nodeData={nodeData} />
    }
  }
}

