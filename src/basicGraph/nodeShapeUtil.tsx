import { Circle2d, Geometry2d, Group2d, HTMLContainer, RecordPropsType, Rectangle2d, SVGContainer, ShapeUtil, T, TLShapeUtilFlag, Vec, VecLike, getDefaultColorTheme, track, useEditor, useIsDarkMode } from 'tldraw'

import { TLBaseShape } from 'tldraw'
import { portColorMap } from '../graph/nodeDefinitions'
import { useHover } from 'usehooks-ts'
import { useRef } from 'react'

type PortType = {
  "number": number
  "boolean": boolean
  "numberArray": number[]
}



type PortTypeLabel = keyof PortType

type Port<K extends PortTypeLabel = PortTypeLabel> = {
  name: string
  ioType: "in" | "out"
  dataType: K
  value?: PortType[K]
}


const port1: Port = { name: "a", ioType: "in", dataType: "number" }
const port2: Port = { name: "b", ioType: "in", dataType: "boolean" }
const port3: Port = { name: "c", ioType: "out", dataType: "numberArray" }
const inputs: Port[] = [port1, port2]

const TLPort = T.object({
  name: T.string,
  ioType: T.literalEnum("in", "out"),
  dataType: T.literalEnum("boolean", "number", "numberArray"),
  value: T.optional(T.any)
})
const nodeShapeProps = {
  width: T.nonZeroNumber,
  height: T.nonZeroNumber,
  inputs: T.arrayOf(TLPort),
  output: TLPort
}

type NodeShapeProps = RecordPropsType<typeof nodeShapeProps>

export type NodeShape = TLBaseShape<'node', NodeShapeProps>

export class NodeShapeUtil extends ShapeUtil<NodeShape> {
  static override type = 'node' as const

  override canResize: TLShapeUtilFlag<NodeShape> = () => false
  override canEdit: TLShapeUtilFlag<NodeShape> = () => false
  override isAspectRatioLocked: TLShapeUtilFlag<NodeShape> = () => false
  override hideSelectionBoundsBg = () => true
  override hideSelectionBoundsFg = () => true
  override hideRotateHandle = () => true
  override hideResizeHandles = () => true

  getDefaultProps(): NodeShape['props'] {
    console.log("returning default props ")
    console.log(inputs)
    return {
      width: 200,
      height: 100,
      inputs: inputs,
      output: port3
    }
  }

  getGeometry(shape: NodeShape) {
    const inputCircles = NodeShapeUtil.portLocations("in", shape).map(location => location.geometry)
    const outputCircles = NodeShapeUtil.portLocations("out", shape).map(location => location.geometry)

    return new Group2d({
      children: [...inputCircles, ...outputCircles,
      new Rectangle2d({
        width: shape.props.width,
        height: shape.props.height,
        isFilled: true,
      }) as Geometry2d]
    })
  }

  static portLocations(ioType: "in" | "out", shape: NodeShape) {
    const { inputs, output } = shape.props
    const radius = portDiameter / 2 + portSpacing / 2 // we add some extra padding here, to give a larger hitbox
    if (ioType === "in") {
      return inputs.map((port, i) => ({
        port,
        geometry: new Circle2d({
          radius: radius,
          isFilled: true,
          x: getPortXPosition(i) - radius,
          y: -radius
        })
      }
      ))
    } else {
      return [{
        port: output,
        geometry: new Circle2d({
          radius: radius,
          isFilled: true,
          x: getPortXPosition(0) - radius,
          y: shape.props.height - radius

        })
      }]
    }
  }

  /**
   * Get port location relative to the node's location
   */
  static getNearestPortFromPoint(shape: NodeShape, ioType: "in" | "out" | "all", relativePoint: VecLike) {
    const portLocations = ioType != "all" ? this.portLocations(ioType, shape) : this.portLocations("in", shape).concat(this.portLocations("out", shape))

    console.log(relativePoint)
    console.log(portLocations)

    const nearestLocation = portLocations.find((portLocation) =>
      portLocation.geometry.hitTestPoint(Vec.From(relativePoint)))

    if (nearestLocation !== undefined) {
      //specific port was selected
      return nearestLocation.port
    }
    //TODO intelligently select default port if specific port wasn't clicked
    return portLocations[0].port

  }
  /**
   * Get port location relative to the node's transform
   */
  static getRelativePortLocation(shape: NodeShape, ioType: "in" | "out", portName?: string) {
    if (ioType == "out") {
      //only support single output for now
      return new Vec(getPortXPosition(0), shape.props.height + portDiameter / 2)
    }

    // portName is undefined while port is being created. 
    // the line is drawn to the cursor, so no offset is needed 
    if (portName === undefined) {
      return new Vec(0, 0)
    }

    // calculate offset based on which input index corresponds to portName
    const portIndex = shape.props.inputs.findIndex((port) => port.name == portName)
    if (portIndex === undefined) {
      throw Error(`Tried to find offset for portName: ${portName}, which doesn't exist on ${shape.id}.
                This shape's ports are: ${shape.props.inputs}`)
    }

    return new Vec(getPortXPosition(portIndex), -portDiameter / 2)
  }

  component(shape: NodeShape) {
    return <HTMLContainer>
      <NodeSvg shape={shape} />
    </HTMLContainer>
  }

  indicator(shape: NodeShape) {
    return <rect rx={5}
      strokeWidth={nodeStrokeWidth}
      strokeOpacity={.2}
      width={shape.props.width}
      height={shape.props.height} />
  }
}
const portStartOffset = 10
const portDiameter = 30
const portSpacing = 12.5
const nodeStrokeWidth = 2
const portStrokeWidth = 2

const NodeSvg = track(({ shape }: { shape: NodeShape }) => {
  const isDarkMode = useIsDarkMode()
  const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })

  const inputs = shape.props.inputs as Port[]
  const output = shape.props.output as Port
  const { width, height } = shape.props
  const nodeColor = theme["grey"].solid
  const backgroundColor = theme["background"]
  return <HTMLContainer>
    <SVGContainer>
      <defs>
        <filter id="blurry" x="0%" y="0%" height="100%" width="100%" primitiveUnits="userSpaceOnUse">
          <feGaussianBlur x={nodeStrokeWidth / 2} y={nodeStrokeWidth / 2} width={width - nodeStrokeWidth / 2} height={height - nodeStrokeWidth / 2} stdDeviation="5" in="SourceGraphic" result="blurSquares" />
          <feComponentTransfer in="blurSquares" result="opaqueBlur">
            <feFuncA type="linear" intercept="1" />
          </feComponentTransfer>
          <feBlend mode="normal" in="opaqueBlur" in2="SourceGraphic" />
        </filter>
      </defs>
      <g id="entire_node" fill="none" stroke={nodeColor} strokeWidth={portStrokeWidth} >

        <g id="ports" strokeWidth={portStrokeWidth} filter="url(#blurry)" width={width}>
          <rect rx={5} stroke="none" fill={backgroundColor} fillOpacity={1}
            width={width} height={height} />
          <InputPorts ports={inputs} />
          <g id="output" transform={`translate(${getPortXPosition(0)},${height})`}>
            <IOPort port={output} />
          </g>
        </g>
        <g id="node_content" >
        </g>
        <rect rx={5} strokeWidth={nodeStrokeWidth} fill={backgroundColor} fillOpacity={.7}
          strokeOpacity={1}
          width={width} height={height} />
      </g>
    </SVGContainer>
  </HTMLContainer>
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


const IOPort = track(({ port }: { port: Port }) => {
  const editor = useEditor()
  const { dataType } = port
  const isDarkMode = useIsDarkMode()
  const ref = useRef(null)
  const isHover = useHover(ref)
  const hoverDiameter = portDiameter + portSpacing / 4


  const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })
  const color = theme[portColorMap[dataType]].solid
  return <g id="portOuterBound">
    <circle
      ref={ref}
      style={{ pointerEvents: "all" }}
      stroke="none" fill="none" r={hoverDiameter / 2}
      onPointerDown={(e) => {
        if (e.button == 2) {
          console.log("port pointer right down")
          editor.setCurrentTool("wire")
        }
        else if (e.button == 0) {
          console.log("port pointer left")
          editor.setCurrentTool("wire")
        }
      }}
    />
    <g stroke={color} fill={color} strokeLinecap="butt" fillOpacity={isHover ? .7 : .2}>
      return <circle r={portDiameter / 2} />
    </g>
  </g>
})

function getPortXPosition(portIndex: number) {
  return (portStartOffset + portDiameter / 2) + (portIndex * (portDiameter + portSpacing))
}

