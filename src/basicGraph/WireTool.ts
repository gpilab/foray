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
  override onComplete = () => {
    this.editor.setCurrentTool('select')
  }

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor)

    if (target === undefined) {
      return
    }

    //create the wire, and bind it to target
    const wireId = createShapeId()

    this.editor.mark(`creating: ${wireId}`)
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
    this.parent.transition("connecting_nodes", wireId)
  }
}

class ConnectingNodes extends StateNode {
  static override id = 'connecting_nodes'
  currentWireId?: TLShapeId

  onEnter = (wireId: TLShapeId) => {
    console.log("onEnter")
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
      this.complete()
    }
  }

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor, this.currentWireId)
    if (this.checkValidBind(target)) {
      this.bindEndWire(target)
      this.complete()
    }
    else {
      this.cancel()
    }
  }

  cancel = () => {
    console.log("myCancel")
    this.editor.deleteShape(this.currentWireId!)
    this.complete()
  }

  complete = () => {
    console.log("myComplete")
    this.editor.mark(`creating: ${this.currentWireId}`)
    this.currentWireId = undefined

    if (this.editor.getInstanceState().isToolLocked) {
      this.parent.transition('idle')
    } else {
      this.editor.setCurrentTool('select.idle')
    }
  }

  override onExit = () => {
    console.log("onExit")
    if (this.currentWireId !== undefined) {
      console.log("Wire not cleaned up yet!")
      this.editor.deleteShape(this.currentWireId!)
      this.currentWireId = undefined
    }
  }

  /**
   * Runs when right click during action. CONFUSINGLY NAMED!(to me)
   */
  override onComplete: TLEventHandlers['onComplete'] = () => {
    console.log("onComplete")
    this.cancel()
  }

  /**
   * Runs when right click during action. CONFUSINGLY NAMED!(to me)
   */
  override onCancel = () => {
    console.log("onCancel")
    this.cancel()
  }

  /**
   * 
   */
  override onInterrupt = () => {
    console.log("onInterupt")
    this.cancel()
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
    this.editor.sendToBack([this.currentWireId!]) //wires in the back looks better
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
