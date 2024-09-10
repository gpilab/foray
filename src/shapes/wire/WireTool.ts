import { Editor, StateNode, TLEventHandlers, TLShape, TLShapeId, VecLike, createShapeId } from "tldraw"
import { NodeShapeUtil, NodeShape } from "../node/nodeShapeUtil"
import { WireBinding } from "./WireBindingUtil"



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
    if (this.editor.getInstanceState().isToolLocked) {
      this.parent.transition('idle')
    } else {
      this.editor.setCurrentTool('select.idle')
    }
  }
  override onComplete = () => {
    if (this.editor.getInstanceState().isToolLocked) {
      this.parent.transition('idle')
    } else {
      this.editor.setCurrentTool('select.idle')
    }
  }
  override onDoubleClick: TLEventHandlers['onDoubleClick'] = (_info) => {
    const { inputs } = this.editor
    const { currentPagePoint } = inputs

    //create the wire, and bind it to target
    const nodeId = createShapeId()

    this.editor.mark(`creating: ${nodeId}`)
    this.editor.createShape({
      id: nodeId,
      type: 'node',
      x: currentPagePoint.x,
      y: currentPagePoint.y
    })

    //stop tldraw from inserting text shape
    this.editor.cancelDoubleClick();

    this.onComplete()
  }

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor)


    if (target?.type === undefined) {
      //TODO: create node selection popup
      //create_node_popup()
    }
    if (target?.type !== "node") {
      return
    }


    const pagePoint = this.editor.inputs.currentPagePoint
    const relativeShapePoint = this.editor.getPointInShapeSpace(target, pagePoint)
    const port = NodeShapeUtil.getNearestPortFromPoint(target as NodeShape, "all", relativeShapePoint)
    if (port === undefined) {
      return
    }
    //create the wire, and bind it to target
    const wireId = createShapeId()

    this.editor.mark(`creating: ${wireId}`)
    this.editor.createShape({
      id: wireId,
      type: 'wire',
      // line rendering currently expects this to start as x,y == 0,0. 
      // x,y are free to change after initial drawing is complete
      x: pagePoint.x,
      y: pagePoint.y
    })
    this.editor.sendToBack([wireId])
    this.editor.createBinding({
      type: 'wire',
      fromId: wireId,
      toId: target.id,
      props: {
        terminal: "start",
        portname: port.name  //TODO should be able to create ports starting from inputs
      },
    })
    this.parent.transition("connecting_nodes", wireId)
  }
  override onRightClick: TLEventHandlers['onRightClick'] = (_info) => {
    const target = getShapeAtCursor(this.editor)
    if (target?.type !== "node") {
      return
    }

    const pagePoint = this.editor.inputs.currentPagePoint
    const relativeShapePoint = this.editor.getPointInShapeSpace(target, pagePoint)
    const port = NodeShapeUtil.getNearestPortFromPoint(target as NodeShape, "all", relativeShapePoint)
    if (port !== undefined) {
      deleteBindingsAtPort(this.editor, target.id, port.name)
    }
    //TODO handle transition back to select idle better.
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

    if (!this.checkValidBind(target)) {
      return
    }

    this.bindEndWire(target, this.editor.inputs.currentPagePoint)
    this.complete()
  }

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor, this.currentWireId)
    if (!this.checkValidBind(target)) {
      this.cancel()
      return
    }

    this.bindEndWire(target, this.editor.inputs.currentPagePoint)
    this.complete()
  }

  cancel = () => {
    this.editor.deleteShape(this.currentWireId!)
    this.editor.setCurrentTool('select.idle')
  }

  complete = () => {
    this.editor.mark(`creating: ${this.currentWireId}`)
    this.currentWireId = undefined

    if (this.editor.getInstanceState().isToolLocked) {
      this.parent.transition('idle')
    } else {
      this.editor.setCurrentTool('select.idle')
    }
  }

  override onExit = () => {
    if (this.currentWireId !== undefined) {
      console.log("Wire not cleaned up yet!")
      this.editor.deleteShape(this.currentWireId)
      this.currentWireId = undefined
    }
  }

  /**
   * Runs when right click during action. CONFUSINGLY NAMED!(to me)
   */
  override onComplete: TLEventHandlers['onComplete'] = () => {
    this.cancel()
  }

  override onCancel = () => {
    this.cancel()
  }
  override onInterrupt = () => {
    this.cancel()
  }


  /**
   * Wire can (currently) bind to any shape other than the start shape
  */
  checkValidBind(target: TLShape | undefined): target is NodeShape {
    if (target?.type !== "node") {
      return false
    }
    const wireShape = this.editor.getShape(this.currentWireId!)!
    const startShapeId = this.editor.getBindingsFromShape(wireShape, 'wire')[0].toId

    const targetIsStartShape = target.id == startShapeId

    return !targetIsStartShape
  }

  bindEndWire(target: NodeShape, pagePoint: VecLike) {
    const relativeShapePoint = this.editor.getPointInShapeSpace(target, pagePoint)
    const port = NodeShapeUtil.getNearestPortFromPoint(target, "in", relativeShapePoint)
    if (port === undefined) {
      //no valid port found
      this.cancel()
      return
    }

    //get the port the pointer is over
    this.editor.sendToBack([this.currentWireId!]) //wires in background looks better
    this.editor.updateShape({
      id: this.currentWireId!,
      type: "wire",
    })

    //override any existing bindings on this input port
    deleteBindingsAtPort(this.editor, target.id, port.name)
    this.editor.createBinding({
      type: 'wire',
      fromId: this.currentWireId!,
      toId: target.id,
      props: {
        terminal: "end",
        portName: port.name
      },
    })
  }
}

function deleteBindingsAtPort(editor: Editor, nodeShapeId: TLShapeId, portName: string) {
  const nodeBindings = editor.getBindingsToShape<WireBinding>(nodeShapeId, "wire")
  const toDelete = nodeBindings.filter(b => b.props.portName == portName)

  //if we lock wires, we must unlock the wires so they can get deleted
  // const wireUpdates = toDelete.map(s => s.fromId)
  //   .map(wireShapeId => ({ id: wireShapeId, type: "wire", isLocked: false }))
  // editor.updateShapes(wireUpdates)

  editor.deleteBindings(toDelete)
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
