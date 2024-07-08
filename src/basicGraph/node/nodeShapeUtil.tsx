import {
  Circle2d, DefaultColorStyle, Geometry2d,
  Group2d, HTMLContainer, RecordPropsType,
  Rectangle2d, SVGContainer, ShapeUtil, T,
  TLOnBeforeUpdateHandler, TLShapeUtilFlag,
  Vec, VecLike, getDefaultColorTheme, track,
  useEditor, useIsDarkMode
} from 'tldraw'

import { TLBaseShape } from 'tldraw'
import { portColorMap } from '../../graph/nodeDefinitions'
import { useHover } from 'usehooks-ts'
import { useRef } from 'react'
import { NodeContent } from './NodeContent'
import { NodeTypeStyle, nodeTypeStyle } from './nodeStylePanel'
import { InPort, OutPort, Port } from './portDefinition'
import { addNodeDefinition, checkAllPortsPopulated, Config, nodeCompute, NodeInputs, NodeOutputs, NodeType, } from './nodeDefinitions'


const TLOutPort = T.object({
  name: T.string,
  ioType: T.literal("out"),
  dataType: T.literalEnum("boolean", "number", "numberArray"),
  value: T.optional(T.any)
})
const TLInPort = T.object({
  name: T.string,
  ioType: T.literal("in"),
  dataType: T.literalEnum("boolean", "number", "numberArray"),
  //TODO figure out a better way to make this type, I'm not sure how to use TLDraw's
  //type utility to make the union we need here
  value: T.optional(T.any)
})

const porta: InPort = { name: "a", ioType: "in", dataType: "number", value: undefined } as const
const portb: InPort = { name: "b", ioType: "in", dataType: "number", value: undefined } as const
const portd: OutPort = { name: "d", ioType: "out", dataType: "boolean", value: undefined } as const
const algebra_inputs = { a: porta, b: portb }
const algebra_output = { out: portd }



const nodeShapeProps = {
  width: T.nonZeroNumber,
  height: T.nonZeroNumber,
  inputs: T.dict(T.string, TLInPort),
  output: T.object({ "out": TLOutPort }),
  config: T.dict(T.string, T.any),
  nodeType: nodeTypeStyle,
  color: DefaultColorStyle
}

export type NodeShapeProps = RecordPropsType<typeof nodeShapeProps>

export type NodeShape = TLBaseShape<'node', NodeShapeProps>

export class NodeShapeUtil extends ShapeUtil<NodeShape> {
  static override type = 'node' as const

  /**
   * Unsure why this needs to be overriden, we already passed this info in.
   * Without the override TL validation fails
   */
  static override props = nodeShapeProps

  override canResize: TLShapeUtilFlag<NodeShape> = () => false
  override canEdit: TLShapeUtilFlag<NodeShape> = () => false
  override isAspectRatioLocked: TLShapeUtilFlag<NodeShape> = () => false
  override hideSelectionBoundsBg = () => true
  override hideSelectionBoundsFg = () => true
  override hideRotateHandle = () => true
  override hideResizeHandles = () => true

  getDefaultProps(): NodeShape['props'] {
    return {
      width: 200,
      height: 100,
      //not sure why this complains, maybe T.any type from TLDraw typing?
      // eslint-disable-next-line   
      inputs: addNodeDefinition.state.inputs,
      // eslint-disable-next-line 
      output: addNodeDefinition.state.output,
      nodeType: "Add",
      config: {},
      color: "black"
    }
  }

  //TODO Make this more like a reducer, so that we make sure we are handling all possible state changes, 
  //especially when multiple changes need to be made
  //TODO make sure ports with multiple children upate correctly! i.e. one change doesn't overwrite the other
  override onBeforeUpdate: TLOnBeforeUpdateHandler<NodeShape> = (prev: NodeShape, next: NodeShape) => {

    //handle updates to port type
    if (prev.props.nodeType != next.props.nodeType) {
      console.log("setting ports to default")
      const { inputs, output, config } = getPortDefaults(next.props.nodeType)

      //delete all bindings
      //TODO keep bindings that still fulfill data type
      this.editor.deleteBindings(this.editor.getBindingsInvolvingShape(prev))

      const newNext = { ...next, props: { ...next.props, inputs, output } }
      const newOutput = this.computeNodeValue(next.props.nodeType, inputs, output, config)

      return { ...newNext, props: { ...next.props, inputs, output: newOutput } }
    }

    //handle updates to inputs
    if (JSON.stringify(next.props.inputs) !== JSON.stringify(prev.props.inputs)
      || JSON.stringify(next.props.config) !== JSON.stringify(prev.props.config)
    ) {
      const newOutput = this.computeNodeValue(next.props.nodeType, next.props.inputs, next.props.output, next.props.config)
      return { ...next, props: { ...next.props, output: newOutput } }

    }

    return next
  }



  computeNodeValue(nodeType: NodeType, inputs: NodeInputs, output: NodeOutputs, config: Config) {
    //don't compute if there are any undefined inputs

    if (checkAllPortsPopulated(inputs)) {
      const populatedInputs = inputs
      //TODO call nodedef compute func
      const node = {
        type: nodeType,
        inputs: populatedInputs,
        output: output,
        config: config,
      }

      const nextValue = nodeCompute(node)
      // console.log("New Output Value: ", nextValue)

      return { "out": { ...output["out"], value: nextValue } }
    } else {
      console.log(`Encountered undefined port value when calculating output for node: ${nodeType}`)
      return output
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

  static portLocations(ioType: "in" | "out", shape: NodeShape): { port: Port, geometry: Circle2d }[] {
    const { inputs, output } = shape.props
    const radius = portDiameter / 2 + portSpacing / 2 // we add some extra padding here, to give a larger hitbox
    if (ioType === "in") {
      return Object.values(inputs).map((port, i) => ({
        port,
        geometry: new Circle2d({
          radius: radius,
          isFilled: true,
          x: getPortXPosition(i) - radius,
          y: -radius
        })
      })
      )
    } else {
      return [{
        port: output["out"],
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

    return nearestLocation?.port
    // if (nearestLocation !== undefined) {
    //   //specific port was selected
    //   return nearestLocation.port
    // }
    //TODO intelligently select default port if specific port wasn't clicked
    //return portLocations[0].port
  }

  /**
   * Get port location relative to the node's transform
   * TODO support arbitrary inport names (right now, it only works with < 2 inports, 1 of which is named "b")
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
    const portIndex = portName === "b" ? 1 : 0//shape.props.inputs[portName] //TODO sort port names alphabetically and get portName's index from that list 

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
      strokeOpacity={.5}
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

  const inputs = Object.values(shape.props.inputs)
  const output = shape.props.output["out"]

  const { width, height } = shape.props
  const nodeColor = theme[shape.props.color].solid
  const backgroundColor = theme["background"]
  return <HTMLContainer>
    <div>
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
          <rect rx={5} strokeWidth={nodeStrokeWidth} fill={backgroundColor} fillOpacity={.7}
            strokeOpacity={1}
            width={width} height={height} />
        </g>
      </SVGContainer>
    </div>
    <div style={{ width: width, height: height, position: "absolute", }} id="nodeContent">
      <NodeContent nodeShape={shape} />
    </div>
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
    <text
      textAnchor="end"
      strokeWidth="0"
      fill={theme["grey"].solid}
      y={portDiameter * 0.2 * (port.ioType === "in" ? -1 : 2)}
      x={-portDiameter * 2 / 4}>
      {//TODO handle all data types
        typeof port.value == "number"
          ? parseFloat(port.value.toPrecision(12))
          : "..."
      }
    </text>
  </g>
})

function getPortXPosition(portIndex: number) {
  return (portStartOffset + portDiameter / 2) + (portIndex * (portDiameter + portSpacing))
}

function getPortDefaults(nodeType: NodeTypeStyle) {
  if (nodeType === "Constant") {
    return {
      inputs: {},
      output: algebra_output,
      config: { value: 7 }
    }
  }
  return {
    inputs: algebra_inputs,
    output: algebra_output,
    config: {}
  }
}
