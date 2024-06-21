import { Editor, StateNode, TLEventHandlers, TLShape, TLShapeId, createShapeId } from "tldraw"



export class WireTool extends StateNode {
  static override id = 'wire'

  static override initial = 'idle'
  static override children = () => [Idle, ConnectingNodes]
}

class Idle extends StateNode {
  static override id = 'idle'


  override onEnter = () => {
    this.editor.setCursor({ type: 'cross', rotation: 0 })
  }

  override onCancel = () => {
    this.editor.setCurrentTool('select')
  }

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor)

    if (target === undefined) {
      return
    }

    //create the wire, and bind it to target
    const wireId = createShapeId()
    this.editor.createShape({
      id: wireId,
      type: 'wire',
    })

    this.editor.createBinding({
      type: 'wire',
      fromId: wireId,
      toId: target.id,
      props: {
        terminal: "start"
      },
    })
    this.editor.setSelectedShapes([wireId])
    this.parent.transition("connecting_nodes", wireId)
  }
}

class ConnectingNodes extends StateNode {
  static override id = 'connecting_nodes'
  currentWireId?: TLShapeId

  onEnter = (wireId: TLShapeId) => {
    //keep track of the current wire we are working with
    this.currentWireId = wireId
  }

  override onPointerMove: TLEventHandlers['onPointerMove'] = (_info) => {
    this.editor.updateShape({
      id: this.currentWireId!,
      type: "wire",
      props: { end: this.editor.inputs.currentPagePoint.toJson() }
    })

  }

  override onPointerUp: TLEventHandlers['onPointerUp'] = (_info) => {
    const target = getShapeAtCursor(this.editor, this.currentWireId)
    if (this.checkValidBind(target)) {
      this.bindEndWire(target)
      this.editor.setCurrentTool('select')

      this.editor.setSelectedShapes([])
      this.parent.transition('idle')
    }
  }

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor, this.currentWireId)
    if (this.checkValidBind(target)) {
      this.bindEndWire(target)

      this.editor.setSelectedShapes([])
      this.parent.transition('idle')
    }
    else {
      this.editor.deleteShape(this.currentWireId!)
      this.editor.setSelectedShapes([])
      this.parent.transition('idle')
    }
  }


  override onCancel: TLEventHandlers['onCancel'] = () => {
    this.cancel()
  }

  override onComplete: TLEventHandlers['onComplete'] = () => {
    this.cancel()
  }

  override onInterrupt: TLEventHandlers['onInterrupt'] = () => {
    this.cancel()
  }

  cancel() {
    this.currentWireId = undefined
    this.editor.setSelectedShapes([])
    this.parent.transition('idle')
  }

  /**
   * Wire can (currently) bind to any shape other than the start shape
  */
  checkValidBind(target: TLShape | undefined): target is TLShape {
    if (target === undefined) {
      return false
    }
    const wireShape = this.editor.getShape(this.currentWireId!)!
    const startShapeId = this.editor.getBindingsFromShape(wireShape, 'wire')[0].toId

    const targetIsStartShape = target?.id == startShapeId

    return !targetIsStartShape
  }

  bindEndWire(target: TLShape) {

    this.editor.createBinding({
      type: 'wire',
      fromId: this.currentWireId!,
      toId: target.id,
      props: {
        terminal: "end"
      },
    })

  }

}

function getShapeAtCursor(editor: Editor, excludeId?: TLShapeId): TLShape | undefined {
  const target = editor.getShapeAtPoint(editor.inputs.currentPagePoint, {
    hitInside: true,
    filter: (potentialTarget) =>
      editor.canBindShapes({ fromShape: { type: "wire" }, toShape: potentialTarget, binding: 'wire' })
      && potentialTarget.id !== excludeId

  })
  return target
}
