import {
  BindingOnCreateOptions,
  BindingOnDeleteOptions,
  BindingOnShapeChangeOptions,
  BindingUtil, TLBaseBinding,
} from 'tldraw'


export type WireBinding = TLBaseBinding<'wire', {
  terminal: "start" | "end"
}>

/**
 * Determines how wire should behave when bound shapes change
 */
export class WireBindingUtil extends BindingUtil<WireBinding> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return {
      terminal: "start" as const
    }
  }

  onAfterCreate(options: BindingOnCreateOptions<WireBinding>): void {
    this.propogate(options.binding)
  }

  //start terminals need to propogate changes to wires
  onAfterChangeToShape(options: BindingOnShapeChangeOptions<WireBinding>): void {
    if (options.binding.props.terminal == "start") {
      this.propogate(options.binding)
    }
  }

  //end terminals propogate changes from wires to bound shapes
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
    this.editor.updateShape({ id: childShape.id, type: childShape.type, props: { ...childShape.props, color: newColor } })
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
    this.editor.deleteShape(options.binding.fromId)
  }

}
