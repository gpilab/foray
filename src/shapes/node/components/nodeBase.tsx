import { HTMLContainer, SVGContainer, track, useEditor } from "tldraw"
import { getPortXPosition, NodeShape } from "../nodeShapeUtil"
import { NodeContent } from "./NodeContent"
import { Port, portColorMap } from "../portDefinition"
import { ReactNode, useRef } from "react"
import { useHover } from "usehooks-ts"
import { useTheme } from "../../../util/useTheme"
import { nodeUIConfig } from "../nodeConstants"


const { portDiameter, portSpacing, nodeStrokeWidth, portStrokeWidth } = nodeUIConfig

export const NodeBase = track(({ shape }: { shape: NodeShape }) => {
  const theme = useTheme()
  const { nodeType, inputs, output, width, height, color } = shape.props

  console.log(`Rendering node ${nodeType}`)

  const backgroundColor = theme.id == "dark" ? "#000000dd" : "#ffffffdd"
  return <HTMLContainer>
    <SVGContainer>
      <g id="entire_node" fill="none" stroke={theme[color]} >
        <NodeBackground blur={false} id={shape.id} width={width} height={height} backgroundColor={backgroundColor}>
          <InputPorts ports={Object.values(inputs)} />
          <OutputPorts ports={[output["out"]]} height={height} />
        </NodeBackground>

      </g>
    </SVGContainer>

    <div style={{
      width: width, height: height, position: "absolute", fontSize: "20px",
      color: theme.text
    }} id="nodeContent">
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
          stroke={"none"}
          fill={"none"} />
        <rect id="cover_node_frame" rx={5}
          width={width}
          height={height}
          stroke={theme[color]}
          strokeOpacity={.7}
          fill={"none"}
          fillOpacity={1}
        />
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
  const port_empty = port.value == undefined
  const stroke_color = theme[portColorMap[dataType]]
  const [fill_color, fill_opacity] = (port_empty && port.ioType === "in")
    ? [theme["red"], .6]
    : [stroke_color, .2]
  const stroke_opacity = .8
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
      stroke={stroke_color} fill={fill_color}
      fillOpacity={
        isHover
          ? .9 : fill_opacity
      }
      strokeOpacity={isHover ? 1 : stroke_opacity}
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
  console.log(JSON.stringify(port))
  if (port.value === undefined) {
    return ""
  }
  switch (port.value[0]) {
    case "Integer": {
      const value = port.value[1] as number
      return value
    }
    case "Real": {
      const value = port.value[1] as number
      return parseFloat(value.toPrecision(12))
    }
    case "String": {
      return port.value[1] as string
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
            {props.children}
          </feComponentTransfer>
          <feBlend mode="normal" in="opaqueBlur" in2="SourceGraphic" />
        </filter>
      </defs>
      {props.children}
      <g id="ports"
        filter={`url(#blurry${props.id})`}
        width={props.width} height={props.height}>

        <rect rx={5}
          width={props.width} height={props.height}
          stroke="none" fill={props.backgroundColor} />
      </g>
    </>
  }
  else {
    return <>{props.children}
      <rect rx={5}
        width={props.width} height={props.height}
        stroke="none" fill={props.backgroundColor} />

    </>
  }
}
