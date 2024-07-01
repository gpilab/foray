import {
  BindingOnCreateOptions,
  BindingOnDeleteOptions,
  BindingOnShapeChangeOptions,
  BindingOnShapeIsolateOptions,
  BindingUtil, RecordPropsType, T, TLBaseBinding,
} from 'tldraw'

const wireBindingProps = {
  terminal: T.literalEnum("start", "end"),
  portName: T.optional(T.string)
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
    this.propogate(options.binding)
  }

  //start terminals propogate changes  FROM node TO wires
  onAfterChangeToShape(options: BindingOnShapeChangeOptions<WireBinding>): void {
    if (options.binding.props.terminal == "start") {
      this.propogate(options.binding)
    }
  }

  //end terminals propogate changes TO nodes FROM wires
  onAfterChangeFromShape(options: BindingOnShapeChangeOptions<WireBinding>): void {
    if (options.binding.props.terminal == "end") {
      this.propogate(options.binding)
    }
  }

  private propogate(binding: WireBinding) {
    const { parentShape, childShape } = this.getParentChild(binding)

    if (!("color" in parentShape.props && "color" in childShape.props)) {
      return
    }
    const newColor = parentShape.props.color
    const oldColor = childShape.props.color

    if (newColor == oldColor) {
      return
    }

    console.log(`updating color from ${childShape.props.color} to `, newColor)
    window.setTimeout(() => this.editor.updateShape({ id: childShape.id, type: childShape.type, props: { ...childShape.props, color: newColor } }), 100)
  }

  private getParentChild(binding: WireBinding) {
    const { parentId, childId } = {
      parentId: binding.props.terminal == "start" ? binding.toId : binding.fromId,
      childId: binding.props.terminal == "start" ? binding.fromId : binding.toId
    }
    return {
      parentShape: this.editor.getShape(parentId)!,
      childShape: this.editor.getShape(childId)!
    }

  }
  //cleanup wire shape when binding is deleted.
  onAfterDelete(options: BindingOnDeleteOptions<WireBinding>): void {
    console.log("after delete binding", options)
    this.editor.deleteShape(options.binding.fromId)
  }

  onBeforeIsolateToShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
    console.log("isolating to", options)
    this.editor.updateShape({ id: options.binding.fromId, type: "wire", isLocked: false })
  }

  onBeforeIsolateFromShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
    console.log("isolating to", options)
    this.editor.updateShape({ id: options.binding.fromId, type: "wire", isLocked: false })
  }

}
