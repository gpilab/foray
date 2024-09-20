import {
  Circle2d, DefaultColorStyle, Geometry2d,
  Group2d, HTMLContainer, RecordPropsType,
  Rectangle2d, ShapeUtil, T,
  TLOnBeforeCreateHandler,
  TLOnBeforeUpdateHandler, TLOnResizeHandler, TLShapeUtilFlag,
  Vec, VecLike
} from 'tldraw'

import { TLBaseShape } from 'tldraw'
import { showPlotGridStyle } from './nodeStylePanel'
import { InPort, Port } from './portDefinition'
import { addNodeDefinition, getDefaultNodeDefinition } from './nodeDefinitions'
import { checkAllPortsPopulated, Config, nodeCompute, NodeInputs, NodeOutputs } from './nodeType'
import { nodeUIConfig } from './nodeConstants'
import { NodeBase } from './components/nodeBase'
import { WireBinding } from '../wire/WireBindingUtil'

/// tldraw types
/// requried for tldraw to properly perform 
/// validation when serializing shapes

const TLPortValue = T.any


const TLBasePort = {
  name: T.string,
  dataType: TLPortValue,//T.literalEnum(...PortTypeLabels),
  value: T.optional(T.any)
}

const TLOutPort = T.object({
  ...TLBasePort,
  ioType: T.literal("out"),
})

const TLInPort = T.object({
  ...TLBasePort,
  ioType: T.literal("in"),
})

const nodeShapeProps = {
  // UI props
  width: T.nonZeroNumber,
  height: T.nonZeroNumber,
  color: DefaultColorStyle,

  // Node behaviour props
  nodeType: T.string,
  showPlotGrid: showPlotGridStyle,
  inputs: T.dict(T.string, TLInPort),
  output: T.object({ "out": TLOutPort }),
  config: T.dict(T.string, T.any),
  inFlightCalc: T.boolean
}

export const defaultNodeProps: NodeShapeProps = {
  width: 200,
  height: 100,
  inputs: addNodeDefinition.state.inputs,
  output: addNodeDefinition.state.output,
  nodeType: "_Add",
  showPlotGrid: false,
  config: {},
  color: "black",
  inFlightCalc: false
}
const { portStartOffset, portDiameter, portSpacing, nodeStrokeWidth } = nodeUIConfig

export function getPortXPosition(portIndex: number) {
  return (portStartOffset + portDiameter / 2) + (portIndex * (portDiameter + portSpacing))
}


export type NodeShapeProps = RecordPropsType<typeof nodeShapeProps>
export type NodeShape = TLBaseShape<'node', NodeShapeProps>


export class NodeShapeUtil extends ShapeUtil<NodeShape> {
  static override type = 'node' as const
  static override props = nodeShapeProps

  override canResize: TLShapeUtilFlag<NodeShape> = () => true
  //override hideResizeHandles = () => false
  override canEdit: TLShapeUtilFlag<NodeShape> = () => false
  override isAspectRatioLocked: TLShapeUtilFlag<NodeShape> = () => false
  //override hideSelectionBoundsBg = () => true
  //override hideSelectionBoundsFg = () => true
  override hideRotateHandle = () => true

  getDefaultProps(): NodeShapeProps {
    //using spread so that props aren't considered readonly.
    //tldraw needs to make changes to this object
    return { ...defaultNodeProps }
  }

  onBeforeCreate: TLOnBeforeCreateHandler<NodeShape> = (next) => {
    //TODO: this is a mess...
    const { inputs, output, config
    } = getDefaultNodeDefinition(next.props.nodeType).state
    const defaultInputs = inputs as Record<string, InPort>


    //compute output for the new nodeType
    this.computeNodeValue(next.props.nodeType, defaultInputs, output, config)
      .then((out) => {
        this.editor.updateShape({
          ...next,
          props: {
            ...next.props,
            inputs: defaultInputs, output: out, config, inFlightCalc: false
          }
        })
      }).catch((onrejected) => {
        console.log("got rejected")
        //TODO set node to an error state with useful info for user
        throw onrejected
      })
    //return what we can for now. The callback will update the new value when it's done
    return {
      ...next,
      props: {
        ...next.props,
        inputs: defaultInputs, output, config, inFlightCalc: true
      }

    }
  }

  //TODO Make this more like a reducer, so that we make sure we are handling all possible state changes
  //especially when multiple changes need to be made
  //TODO make sure ports with multiple children upate correctly! i.e. one change doesn't overwrite the other
  override onBeforeUpdate: TLOnBeforeUpdateHandler<NodeShape> = (prev: NodeShape, next: NodeShape) => {
    // If we just got back the new computed output, just return that
    if (prev.props.inFlightCalc && !next.props.inFlightCalc) {
      return next
    }
    // handle when the port type is updated
    if (prev.props.nodeType != next.props.nodeType) {
      console.log("Port Type updated! Getting default node definition")
      const { inputs, output, config
      } = getDefaultNodeDefinition(next.props.nodeType).state
      const defaultInputs = inputs as Record<string, InPort>


      const validPortNames = Object.keys(defaultInputs).filter(
        portName =>
          defaultInputs[portName].dataType == prev.props.inputs[portName]?.dataType)      // if input is the same data type, copy it over
      const mergedInput = Object.keys(defaultInputs).reduce((merged: Record<string, InPort>, portName) => {
        merged[portName] = {
          ...defaultInputs[portName],
          value: prev.props.inputs[portName]?.value
        }
        return merged
      }, {})

      //Delete any bindings that don't have a matching name/data type
      const bindings = this.editor.getBindingsInvolvingShape<WireBinding>(prev)
      const invalidBindings = bindings.filter(b => !validPortNames.concat(
        output.out.dataType == next.props.output.out.dataType ? ["out"] : []
      ).includes(b.props.portName))
      this.editor.deleteBindings(invalidBindings)

      //compute output for the new nodeType
      this.computeNodeValue(next.props.nodeType, mergedInput, output, config)
        .then((out) => {
          this.editor.updateShape({
            ...next,
            props: {
              ...next.props,
              inputs: mergedInput, output: out, config, inFlightCalc: false
            }
          })
        }).catch((onrejected) => {
          console.log("got rejected")
          //TODO set node to an error state with useful info for user
          throw onrejected
        })
      //return what we can for now. The callback will update the new value when it's done
      return {
        ...next,
        props: {
          ...next.props,
          inputs: mergedInput, output, config, inFlightCalc: true
        }
      }
    }

    //handle updates to inputs
    if (JSON.stringify(next.props.inputs) !== JSON.stringify(prev.props.inputs)
      || JSON.stringify(next.props.config) !== JSON.stringify(prev.props.config)) {

      console.log("updating input to type:", next.props.nodeType, prev.props.nodeType)

      this.computeNodeValue(next.props.nodeType, next.props.inputs, next.props.output, next.props.config)
        .then((out) => {
          this.editor.updateShape({
            ...next,
            props: {
              ...next.props,
              output: out, inFlightCalc: false
            }
          })
        }).catch((onrejected) => {
          console.log("got rejected")
          throw onrejected
        })
      return {
        ...next, props: { ...next.props, inFlightCalc: true }
      }
    }
    return next
  }

  onResize: TLOnResizeHandler<NodeShape> = (shape: NodeShape, info) => {
    const { width, height } = info.initialShape.props
    const minWidth = portDiameter * 3
    const minHeight = portDiameter * 1.5
    return {
      id: shape.id, type: "node",
      props: {
        width: Math.max(width * info.scaleX, minWidth),
        height: Math.max(height * info.scaleY, minHeight)
      }
    }
  }

  async computeNodeValue(nodeType: string, inputs: NodeInputs, output: NodeOutputs, config: Config) {
    //don't compute if there are any undefined inputs
    if (checkAllPortsPopulated(inputs)) {
      const populatedInputs = inputs
      const node = {
        type: nodeType,
        inputs: populatedInputs,
        output: output,
        config: config,
      }
      const nextValue = await nodeCompute(node)
      return { "out": { ...output["out"], value: nextValue } }
    } else {
      console.log(`Encountered undefined port value when calculating output for node: ${nodeType}`)
      //Force the output to be undefined if some inputs are undefined
      return { "out": { ...output["out"], value: undefined } }
    }
  }

  /**
   * The visual boudary shape of the node, used for 
   * things like collision detection
   */
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


  /** Determines the relative positions for ports
   */
  static portLocations(ioType: "in" | "out", shape: NodeShape): { port: Port, geometry: Circle2d }[] {
    const { inputs, output } = shape.props
    const radius = portDiameter / 2 + portSpacing / 2 // we add some extra padding here, to give a larger hitbox
    if (ioType === "in") {
      return Object.values(inputs)
        .sort((a, b) => a.name.localeCompare(b.name))
        .map((port, i) => ({
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
   * Get nearest port to the `relativePoint`
   */
  static getNearestPortFromPoint(shape: NodeShape, ioType: "in" | "out" | "all", relativePoint: VecLike) {
    const portLocations = ioType != "all" ? this.portLocations(ioType, shape) : this.portLocations("in", shape).concat(this.portLocations("out", shape))


    const nearestLocation = portLocations.find((portLocation) =>
      portLocation.geometry.hitTestPoint(Vec.From(relativePoint)))

    return nearestLocation?.port
    // if (nearestLocation !== undefined) {
    //   //specific port was selected
    //   return nearestLocation.port
    // }
    // TODO intelligently select default port if specific port wasn't clicked
    // return portLocations[0].port
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
    // TODO sort port names alphabetically and get portName's index from that list 
    const portIndex = portName === "b" ? 1 : 0//shape.props.inputs[portName] 

    return new Vec(getPortXPosition(portIndex), -portDiameter / 2)
  }
  component(shape: NodeShape) {
    return <HTMLContainer>
      <NodeBase shape={shape} />
    </HTMLContainer >
  }

  indicator(shape: NodeShape) {
    return <rect rx={5}
      strokeWidth={nodeStrokeWidth}
      strokeOpacity={.5}
      width={shape.props.width}
      height={shape.props.height} />
  }
}



