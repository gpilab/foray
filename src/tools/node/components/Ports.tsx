import { Port, PortTypes } from "../../../graph/node"
import { portColorMap } from "../../../graph/nodeDefinitions"

type PortsProps = {
  ports: Port[],
  portIO: "in" | "out",
  currentValue?: PortTypes
}

export function Ports({ ports, portIO, currentValue }: PortsProps) {
  return <div style={{
    position: "absolute",
    top: portIO == "in" ? "-10px" : "",
    bottom: portIO == "out" ? "-10px" : "",
    left: portIO == "out" ? "50%" : "",
    transform: portIO == "out" ? "translate(-50%, 0)" : "",
    display: "flex",
    marginLeft: "5px"
  }}>
    {ports.map((e: Port) => {
      if (e.name == "none") {
        return null
      }
      return <IOPort key={e.name} port={e}>
        {portIO == "in" ?
          <>{e.name}</> :
          <>{currentValue !== undefined ?
            <span style={{ fontSize: "16px" }} > {currentValue.toString()}</span> :
            "N/A"}
          </>
        }
      </IOPort>
    })
    }
  </div >
}

export function IOPort({ children, port, width, height }: { children: React.ReactNode, port: Port, width?: number, height?: number }) {
  return <div
    key={port.name}
    style={{
      width: width,
      height: height,
      borderRadius: "12px",
      background: portColorMap[port.portType],
      padding: "5px 10px 5px 10px",
      marginRight: "5px",
      display: "flex",
      placeContent: "center",
    }}>
    {children}
  </div>
}
