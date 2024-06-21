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
    // const wireAnchor = calcShapeAnchor(this.editor, Mat.Identity(), target, { x: .5, y: .5 })
    this.editor.createShape({
      id: wireId,
      type: 'wire',
      // props: {
      //   start: wireAnchor,
      //   end: this.editor.inputs.currentPagePoint.toJson()
      // }
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
      // this.exit(info, 'wire')
      this.editor.setSelectedShapes([])
      this.parent.transition('idle')
      // this.exit(info, 'wire')
    }
  }

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor, this.currentWireId)
    if (this.checkValidBind(target)) {
      this.bindEndWire(target)
      this.editor.setSelectedShapes([])
      this.parent.transition('idle')
      // this.exit(info, 'wire')
    }
    else {
      this.editor.deleteShape(this.currentWireId!)
      this.editor.setSelectedShapes([])
      this.parent.transition('idle')
      // this.exit(info, 'wire')
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
    console.log(this.currentWireId)
    const wireShape = this.editor.getShape(this.currentWireId!)!
    console.log(wireShape)
    const boundShapes = this.editor.getBindingsFromShape(wireShape, 'wire')
    console.log(boundShapes)
    console.log(boundShapes[0])
    console.log(boundShapes[0].toId)
    const startShapeId = this.editor.getBindingsFromShape(wireShape, 'wire')[0].toId

    const targetIsStartShape = target?.id == startShapeId

    return !targetIsStartShape
  }

  bindEndWire(target: TLShape) {
    // const wireAnchor = calcShapeAnchor(this.editor, Mat.Identity(), target, { x: .5, y: .5 })

    this.editor.updateShape({
      id: this.currentWireId!,
      type: "wire",
      // props: { end: wireAnchor }
    })

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
