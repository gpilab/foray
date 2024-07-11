import { HTMLContainer, SVGContainer, track, useEditor } from "tldraw"
import { getPortXPosition, NodeShape } from "../nodeShapeUtil"
import { NodeContent } from "./NodeContent"
import { Port, portColorMap } from "../portDefinition"
import { ReactNode, useRef } from "react"
import { useHover } from "usehooks-ts"
import { useTheme } from "../../util/useTheme"
import { nodeUIConfig } from "../nodeConstants"




const { portDiameter, portSpacing, nodeStrokeWidth, portStrokeWidth } = nodeUIConfig

export const NodeBase = track(({ shape }: { shape: NodeShape }) => {
  const theme = useTheme()
  const { inputs, output, width, height, color } = shape.props

  const backgroundColor = theme["background"]
  return <HTMLContainer>
    <SVGContainer>
      <g id="entire_node" fill="none" stroke={theme[color]} >

        <NodeBackground blur={true} id={shape.id} width={width} height={height} backgroundColor={backgroundColor}>
          <InputPorts ports={Object.values(inputs)} />
          <OutputPorts ports={[output["out"]]} height={height} />
        </NodeBackground>

        <rect rx={5}
          width={width} height={height}
          strokeWidth={nodeStrokeWidth}
          fill={backgroundColor} fillOpacity={.7} />
      </g>
    </SVGContainer>

    <div style={{ width: width, height: height, position: "absolute", fontSize: "20px" }} id="nodeContent">
      <NodeContent nodeShape={shape} />
    </div>

    <SVGContainer
    >
      <g
        strokeWidth={nodeStrokeWidth}
      >
        <rect id="cover_node_frame" rx={5}
          width={width}
          height={height}
          stroke={theme["black"]}
          fill={"none"} />
        <rect id="cover_node_frame" rx={5}
          width={width}
          height={height}
          stroke={theme[color]}
          strokeOpacity={.7}
          fill={"none"} />
      </g>
    </SVGContainer>
  </HTMLContainer >
})


const InputPorts = track(({ ports }: { ports: Port[] }) => {
  return <g id="inputs">
    {ports.map((port, i) =>
      <g key={port.name} transform={`translate(${getPortXPosition(i)})`}>
        <IOPort port={port} />
      </g>
    )}
  </g>
})
const OutputPorts = track((props: { ports: Port[], height: number }) => {
  return <g id="output"
    transform={`translate(${getPortXPosition(0)},${props.height})`}>
    <IOPort port={props.ports[0]} />
  </g>
})


const IOPort = track(({ port }: { port: Port }) => {
  const editor = useEditor()
  const { dataType } = port
  const ref = useRef(null)
  const isHover = useHover(ref)


  const theme = useTheme()
  const color = theme[portColorMap[dataType]]
  const paddedDiameter = portDiameter + portSpacing / 4


  return <g id="portOuterBound"
    strokeWidth={portStrokeWidth}>
    <circle
      ref={ref}
      style={{ pointerEvents: "all" }}
      stroke="none" fill="none" r={paddedDiameter / 2}
      onPointerDown={() => { editor.setCurrentTool("wire") }}
    />

    <circle r={portDiameter / 2}
      stroke={color} fill={color} fillOpacity={isHover ? .7 : .2}
    />
    <text
      x={-portDiameter * 2 / 4}
      y={portDiameter * 0.2 * (port.ioType === "in" ? -1 : 2)}
      textAnchor="end"
      strokeWidth="0"
      fill={theme.grey}>
      {displayPortValue(port)}
    </text>
  </g>
})

function displayPortValue(port: Port) {
  if (port.value === undefined) {
    return ""
  }
  switch (port.dataType) {
    case "number": {
      const value = port.value as number
      return parseFloat(value.toPrecision(12))
    }
    default: "..."
  }
}


function NodeBackground(props: { blur: boolean, children: ReactNode, id: string, width: number, height: number, backgroundColor: string }) {
  if (props.blur) {
    return <>
      <defs>
        <filter id={`blurry${props.id}`} x="0%" y="0%" height={"100%"} width={"100%"} primitiveUnits="userSpaceOnUse">
          <feGaussianBlur x={nodeStrokeWidth / 2} y={nodeStrokeWidth / 2} width={props.width - nodeStrokeWidth / 2} height={props.height - nodeStrokeWidth / 2} stdDeviation="5" in="SourceGraphic" result="blurSquares" />
          <feComponentTransfer in="blurSquares" result="opaqueBlur">
            <feFuncA type="linear" intercept="1" />
          </feComponentTransfer>
          <feBlend mode="normal" in="opaqueBlur" in2="SourceGraphic" />
        </filter>
      </defs>
      <g id="ports"
        filter={`url(#blurry${props.id})`}
        width={props.width} height={props.height}>

        <rect rx={5}
          width={props.width} height={props.height}
          stroke="none" fill={props.backgroundColor} />
        {props.children}
      </g>
    </>
  }
  else {
    return props.children
  }
}
