import { InPort } from "../../graph/node"


type NodeBaseProps = {
  width: number
  height: number
  inputPorts: InPort[]
  outputPort: InPort //TODO improve names/type specificity for in/out ports
  nodeType: string
  nodeId: string
  currentValue: string //TODO make this more specifc?
  handleValueUpdate: (val: number) => void
}

const colorMap = {
  "string": "green",
  "number": "purple",
  "numberArray": "blue",
  "boolean": "yellow",
}


function Ports({ ports, portIO, currentValue }: { ports: InPort[], portIO: "in" | "out", currentValue?: string }) {
  return <div style={{
    position: "absolute",
    top: portIO == "in" ? "-5px" : "",
    bottom: portIO == "out" ? "-5px" : "",
    zIndex: 9999999
  }}>
    {
      ports.map((e: InPort) => {
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
          {e.name}{currentValue != null ? ": " + currentValue : ""}
        </span>
      })
    }
  </div >
}

export function NodeBase({ width, height, inputPorts, outputPort, nodeType, nodeId, currentValue, handleValueUpdate }: NodeBaseProps) {
  return <div>
    <Ports portIO="in" ports={inputPorts} />
    <div
      style={{
        width: `${width}px`,
        height: `${height}px`,
        border: "2px solid white",
        borderRadius: "4px",
        padding: "15px 10px",
        pointerEvents: "all"
      }}>
      <div style={{ fontSize: "14px" }}>{nodeType}</div>
      <div style={{ color: "grey" }}>{nodeId}    </div>
      {nodeType == "Constant" ?
        <input inputMode="numeric" value={isNaN(parseInt(currentValue)) ? "" : currentValue} onChange={(e) => handleValueUpdate(parseInt(e.target.value))}></input> : ""
      }
    </div >
    <Ports portIO="out" ports={[outputPort]} currentValue={currentValue} />
  </div>
}

