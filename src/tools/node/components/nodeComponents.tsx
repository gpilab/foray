import { ReactNode } from "react"
import { Port } from "../../../graph/node"
import { NodeType } from "../../../graph/nodeDefinitions"
import { DefaultNode } from "./defaultNode"

const colorMap = {
  "string": "green",
  "number": "purple",
  "numberArray": "blue",
  "boolean": "yellow",
}


type PortsProps = {
  ports: Port[],
  portIO: "in" | "out",
  currentValue?: string
}

function Ports({ ports, portIO, currentValue }: PortsProps) {
  return <div style={{
    position: "absolute",
    top: portIO == "in" ? "-5px" : "",
    bottom: portIO == "out" ? "-5px" : "",
  }}>
    {ports.map((e: Port) => {
      return <span
        key={e.name}
        style={{
          width: 50,
          height: 25,
          borderRadius: "15px",
          background: colorMap[e.portType],
          padding: "5px 10px 5px 10px",
          marginLeft: "5px",
          marginRight: "5px"
        }}>
        {e.name}{currentValue ? ": " + currentValue : ""}
      </span>
    })
    }
  </div>
}

type NodeBaseProps = {
  width: number
  height: number
  inputPorts: Port[]
  outputPort: Port //TODO improve names/type specificity for in/out ports
  nodeType: NodeType
  nodeId: string
  currentValue: string //TODO make this more specifc?
  handleValueUpdate: (val: number) => void
}

export function NodeBase(p: NodeBaseProps) {
  const { inputPorts, outputPort, currentValue } = p
  return <div>
    <Ports portIO="in" ports={inputPorts} />
    {chooseComponent(p)}
    <Ports portIO="out" ports={[outputPort]} currentValue={currentValue} />
  </div>
}

function chooseComponent(p: NodeBaseProps): ReactNode {
  const { nodeId, nodeType, currentValue, handleValueUpdate } = p
  switch (p.nodeType) {
    case ("Constant"): {
      return <DefaultNode {...{ ...p, width: 90 }}>
        <input style={{ width: "80%", height: "80%", fontSize: "25px", margin: "auto" }}
          id={nodeId + "input"}
          inputMode="numeric"
          value={isNaN(parseInt(currentValue)) ? "" : currentValue}
          onChange={(e) => handleValueUpdate(parseInt(e.target.value))}>
        </input>
      </DefaultNode >
    }
    default: {
      return <DefaultNode {...p} >
        <div style={{ fontSize: "14px" }}>{nodeType}</div>
        <div style={{ color: "grey" }}>{nodeId}    </div>
      </DefaultNode>
    }
  }
}

