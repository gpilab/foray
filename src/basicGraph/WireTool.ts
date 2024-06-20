import { Editor, Mat, StateNode, TLEventHandlers, TLShape, TLShapeId, createShapeId } from "tldraw"
import { calcShapeAnchor } from "./WireBindingUtil"



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

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor)
    if (target === undefined) {
      console.log("wire target not found")
      return
    }
    //create the wire, and bind it to target
    const wireId = createShapeId()

    const wireAnchor = calcShapeAnchor(this.editor, Mat.Identity(), target, { x: .5, y: .5 })

    this.editor.createShape({
      id: wireId,
      type: 'wire',
      // x: this.editor.inputs.currentPagePoint.x,
      // y: this.editor.inputs.currentPagePoint.y,
      props: {
        start: wireAnchor,
        end: this.editor.inputs.currentPagePoint.toJson()
      }
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
    this.currentWireId = wireId
  }

  override onPointerMove: TLEventHandlers['onPointerMove'] = (_info) => {
    this.editor.updateShape({
      id: this.currentWireId!,
      type: "wire",
      props: { end: this.editor.inputs.currentPagePoint.toJson() }
    })

  }

  override onPointerDown: TLEventHandlers['onPointerDown'] = (_info) => {
    const target = getShapeAtCursor(this.editor, this.currentWireId)
    if (target === undefined) {
      console.log("wire target not found")
      return
    }

    const wireAnchor = calcShapeAnchor(this.editor, Mat.Identity(), target, { x: .5, y: .5 })

    this.editor.updateShape({
      id: this.currentWireId!,
      type: "wire",
      // isLocked: true,
      props: { end: wireAnchor }
    })

    this.editor.createBinding({
      type: 'wire',
      fromId: this.currentWireId!,
      toId: target.id,
      props: {
        terminal: "end"
      },
    })
    this.editor.setSelectedShapes([])

    this.editor.setCurrentTool('select')

  }

}

function getShapeAtCursor(editor: Editor, excludeId?: TLShapeId): TLShape | undefined {
  const target = editor.getShapeAtPoint(editor.inputs.currentPagePoint, {
    hitInside: true,
    filter: (potentialTarget) =>
      editor.canBindShapes({ fromShape: { type: "wire" }, toShape: potentialTarget, binding: 'wire' })
      && potentialTarget.id !== excludeId

  })
  console.log("wire target", target)

  return target
}
