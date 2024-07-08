import {
  BindingOnCreateOptions,
  BindingOnDeleteOptions,
  BindingOnShapeChangeOptions,
  BindingOnShapeIsolateOptions,
  BindingUtil, RecordPropsType, T, TLBaseBinding,
  TLShapeId,
} from 'tldraw'
import { NodeShape } from '../node/nodeShapeUtil'

const wireBindingProps = {
  terminal: T.literalEnum("start", "end"),
  portName: T.string
}

type WireBindingProps = RecordPropsType<typeof wireBindingProps>

export type WireBinding = TLBaseBinding<'wire', WireBindingProps>

/**
 * Determines how wire should behave when bound shapes change
 * Each wire has 2 bindings, 1 for the start terminal, one for the end terminal
 * (technically a wire only has a start binding while the wire is being created(dragging out) by the user)
 *
 * Each connection between nodes deals with the following RecordPropsType
 *
 * ## parentNode
 *
 * ## childNode
 *
 * ## wireShape
 *
 *
 * ## Start terminal binding
 * - fromId: wireShapeId
 * - toId: parentNodeId
 * - portName: name of output port on parentNode
 *
 * ## End terminal binding
 * - fromId: wireShapeId
 * - toId: childNodeId
 * - portName: name of input port on childNode
 *
 * Binding relationship:
 * ```
 * parent <-toId-- `startBinding` --fromId-> wire
 * child <-toId-- `endBinding` --fromId-> wire
 * ```
 *
 * Data propogates as follows:
 * ```
 * parent -> wire -> child
 * ```
 *
 */
export class WireBindingUtil extends BindingUtil<WireBinding> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return {
      terminal: "start" as const, //
      portName: "out" as const // default name for output ports
    }
  }

  onAfterCreate(options: BindingOnCreateOptions<WireBinding>): void {
    console.log("onAfterCreate")
    if (options.binding.props.terminal == "end") {
      console.log("initial propogation for end binding")
      this.propogate(options.binding.fromId)
    }
  }

  //start terminals propogate changes  FROM node TO wires
  onAfterChangeToShape(options: BindingOnShapeChangeOptions<WireBinding>): void {
    if (options.binding.props.terminal == "start") {
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
      return
    }



    console.log(`updating port from ${childInPort.value} to ${parentOutPort.value}`)
    // artificial delay for testing 
    window.setTimeout(() => {
      const upToDateChild = this.editor.getShape<NodeShape>(childNodeId)!
      const updatedInputs = structuredClone(upToDateChild.props.inputs)
      updatedInputs[childInPort.name] = { ...childInPort, value: parentOutPort.value }

      this.editor.updateShape({
        id: childNode.id,
        type: "node",
        props: { inputs: updatedInputs }
      })
    }, 100)

  }


  //cleanup wire shape when binding is deleted.
  onAfterDelete(options: BindingOnDeleteOptions<WireBinding>): void {
    console.log("after delete binding", options)
    this.editor.deleteShape(options.binding.fromId)
  }

  _onBeforeIsolateToShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
    console.log("isolating to", options)
    this.editor.updateShape({ id: options.binding.fromId, type: "wire", isLocked: false })
  }

  onBeforeIsolateFromShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
    console.log("isolating to", options)
    this.editor.updateShape({ id: options.binding.fromId, type: "wire", isLocked: false })
  }

}
