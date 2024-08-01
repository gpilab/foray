import {
  BindingOnCreateOptions,
  BindingOnDeleteOptions,
  BindingOnShapeChangeOptions,
  BindingUtil, RecordPropsType, T, TLBaseBinding,
  TLShapeId,
} from 'tldraw'
import { NodeShape } from '../node/nodeShapeUtil'
import { PortDataType } from '../node/portDefinition'

const wireBindingProps = {
  terminal: T.literalEnum("start", "end"),
  portName: T.string
}

type WireBindingProps = RecordPropsType<typeof wireBindingProps>

/**
 * Bindings relate two shapes. The wire bindings relate nodes to wires.
 * Each wire has a start and an end binding that connects each terminal 
 * of the wire to a specific node port.
 *
 *
 * Each connection between nodes deals with the following tldraw Records:
 * ## parentNode
 * - inputPorts
 * - outputPort
 *   - !!The start terminal binding will refer to this port
 *
 * ## childNode
 * - inputPorts
 *   - !!The end terminal binding will refer to ONE of these ports
 * - outputPort
 *
 * ## Start terminal binding
 * - fromId: wireShapeId
 * - toId: parentNodeId
 * - portName: !!Name of output port on parentNode, currently always named "out"
 *
 * ## End terminal binding
 * - fromId: wireShapeId
 * - toId: childNodeId
 * - portName: !!Name of input port on childNode
 *
 * Using this information, data is passed from parent to child whenever the parent shape 
 * set's it's output port value
 */
export type WireBinding = TLBaseBinding<'wire', WireBindingProps>

/**
 * Determines how wire should behave when bound shapes change
 * Each wire has 2 bindings, 1 for the start terminal, one for the end terminal
 * (technically a wire only has a start binding while the wire is being created(dragging out) by the user)
 */
export class WireBindingUtil extends BindingUtil<WireBinding> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return {
      terminal: "start" as const,// start bindings bind to the parent's output port 
      portName: "out" as const // default name for output ports
    }
  }

  onAfterCreate(options: BindingOnCreateOptions<WireBinding>): void {
    if (options.binding.props.terminal == "end") {
      console.log("initial propogation for end binding")
      this.propogate(options.binding.fromId)
    }
  }

  onAfterChangeToShape(options: BindingOnShapeChangeOptions<WireBinding>): void {
    if (options.binding.props.terminal == "start") {
      //the parentNode has changed, propogate changes
      this.propogate(options.binding.fromId)
    }
  }

  private propogate(wireShapeId: TLShapeId) {
    //pass data from parentNode's output to childNode's input
    const wireShape = this.editor.getShape(wireShapeId)!
    const bindings = this.editor.getBindingsInvolvingShape<WireBinding>(wireShape)

    const startBinding = bindings.find(b => b.props.terminal == "start")
    const endBinding = bindings.find(b => b.props.terminal == "end")

    if (startBinding === undefined) {
      throw new Error(`Failed to find startBindng for wire: ${wireShapeId}, bindings: ${bindings.toString()}`)
    }
    if (endBinding === undefined) {
      throw new Error(`Failed to find endBinding for wire: ${wireShapeId}, bindings: ${bindings.toString()}`)
    }

    const parentNodeId = startBinding.toId
    const childNodeId = endBinding.toId
    const parentNode = this.editor.getShape<NodeShape>(parentNodeId)
    const childNode = this.editor.getShape<NodeShape>(childNodeId)

    if (parentNode === undefined) {
      throw new Error(`Failed to find parent node for wire: ${wireShapeId}`)
    }
    if (childNode === undefined) {
      throw new Error(`Failed to find child node for wire: ${wireShapeId}`)
    }

    const parentOutPort = parentNode.props.output["out"]
    const childInPort = childNode.props.inputs[endBinding.props.portName]


    if (parentOutPort.value == childInPort.value) {
      // ports are already synced, no need to updated
      return
    }

    // artificial delay for testing 
    window.setTimeout(() => {
      this.updatePort(childNodeId, childInPort.name, parentOutPort.value)
    }, 0)
  }

  /**
   * Update a node's port value
   * WARNING, seems like this can cause some race conditions
  */
  private updatePort(nodeId: TLShapeId, portName: string, portValue: PortDataType | undefined) {
    // grab a new object to make sure we have the most up-to-date shape
    const upToDateChild = this.editor.getShape<NodeShape>(nodeId)
    if (upToDateChild === undefined) {
      console.log("attempted to update port that no longer exists!")
      return
    }
    const updatedInputs = structuredClone(upToDateChild.props.inputs)
    updatedInputs[portName] = { ...upToDateChild.props.inputs[portName], value: portValue }

    this.editor.updateShape({
      id: nodeId,
      type: "node",
      props: { inputs: updatedInputs }
    })
  }


  /**
   * Perform any necessary cleanup after a binding is deleted
  */
  onAfterDelete(options: BindingOnDeleteOptions<WireBinding>): void {
    const { fromId } = options.binding
    const { terminal, portName } = options.binding.props
    console.log("after delete binding", options)
    this.editor.deleteShape(fromId)
    if (terminal == "start" && portName == "out") {
      //clear the connected child node port
      this.updatePort(fromId, portName, undefined)
    }

  }

  // May be necessary for handling edge cases with operations on groups of nodes like copy/paste 
  // onBeforeIsolateToShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
  //   console.log("isolating to", options)
  //   this.editor.updateShape({ id: options.binding.fromId, type: "wire", isLocked: false })
  // }
  //
  // onBeforeIsolateFromShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
  //   console.log("isolating to", options)
  //   this.editor.updateShape({ id: options.binding.fromId, type: "wire", isLocked: false })
  // }
}
