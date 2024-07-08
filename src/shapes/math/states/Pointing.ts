import { StateNode, TLEventHandlers, BaseBoxShapeTool, createShapeId, TLBaseBoxShape, Vec } from "tldraw"

export class Pointing extends StateNode {
  static override id = 'pointing'

  markId = ''

  wasFocusedOnEnter = false

  override onEnter = () => {
    this.wasFocusedOnEnter = !this.editor.getIsMenuOpen()
  }

  override onPointerUp: TLEventHandlers['onPointerUp'] = () => {
    this.complete()
  }

  override onCancel: TLEventHandlers['onCancel'] = () => {
    this.cancel()
  }

  override onComplete: TLEventHandlers['onComplete'] = () => {
    this.complete()
  }

  override onInterrupt: TLEventHandlers['onInterrupt'] = () => {
    this.cancel()
  }

  complete() {
    const { originPagePoint } = this.editor.inputs

    if (!this.wasFocusedOnEnter) {
      return
    }

    this.editor.mark(this.markId)

    const shapeType = (this.parent as BaseBoxShapeTool)!.shapeType as TLBaseBoxShape['type']

    const id = createShapeId()

    this.editor.mark(this.markId)

    this.editor.createShapes<TLBaseBoxShape>([
      {
        id,
        type: shapeType,
        x: originPagePoint.x,
        y: originPagePoint.y,
      },
    ])

    const shape = this.editor.getShape<TLBaseBoxShape>(id)!
    const { w, h } = this.editor.getShapeUtil(shape).getDefaultProps() as TLBaseBoxShape['props']
    const delta = new Vec(w / 2, h / 2)

    const parentTransform = this.editor.getShapeParentTransform(shape)
    if (parentTransform) delta.rot(-parentTransform.rotation())

    this.editor.updateShapes<TLBaseBoxShape>([
      {
        id,
        type: shapeType,
        x: shape.x - delta.x,
        y: shape.y - delta.y,
      },
    ])

    this.editor.setSelectedShapes([id])

    if (this.editor.getInstanceState().isToolLocked) {
      this.parent.transition('idle')
    } else {
      this.editor.setEditingShape(id)
      this.editor.setCurrentTool('select.editing_shape')
    }
  }

  cancel() {
    this.parent.transition('idle')
  }
}
