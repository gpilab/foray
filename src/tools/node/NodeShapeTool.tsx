import { StateNode, TLPointerEventInfo, TLShapeId, ZERO_INDEX_KEY, createShapeId, getIndexAbove } from 'tldraw'
import { createAddNode, createConstantNode, createMultiplyNode, createSubtractNode } from '../../graph/nodeDefinitions'
import { createNodeShapeProps } from './NodeShapeUtil'

export class NodeShapeTool extends StateNode {
  static override id = 'node'
  override shapeType = 'node'

  static children = () => [Choose, Idle, Pointing]
  static override initial = 'choose'
}

class Choose extends StateNode {
  static override id = 'choose'
}

class Idle extends StateNode {
  static override id = 'idle'



  enter = (info: string) => {
    const currentShapeId = createShapeId()
    let node = null
    if (info == "add") {
      node = createAddNode(currentShapeId)
    } else if (info == "constant") {
      node = createConstantNode(currentShapeId, 0)
    }
    else if (info == "subtract") {
      node = createSubtractNode(currentShapeId)
    }
    else if (info == "multiply") {
      node = createMultiplyNode(currentShapeId)
    }
    else {
      console.log("couldn't find node from info!")
      node = createAddNode(currentShapeId)
    }


    console.log("creating a node with given info:", info)

    console.log("(currently hardcoded to just create constant)")

    this.editor.createShape({
      id: currentShapeId,
      index: getIndexAbove(ZERO_INDEX_KEY),
      type: "node",
      x: this.editor.inputs.currentPagePoint.x,
      y: this.editor.inputs.currentPagePoint.y,
      props: createNodeShapeProps(node, true)
    })


    console.log("created ", currentShapeId)
    this.parent.transition("pointing", currentShapeId)
  }


  override onEnter = () => {
    this.editor.setCursor({ type: 'cross', rotation: 0 })
  }
}

class Pointing extends StateNode {
  static override id = 'pointing'
  currentShapeId: TLShapeId | null = null

  enter = (tlShapeId: TLShapeId) => {
    console.log("entered pointing")
    this.currentShapeId = tlShapeId
    this.editor.updateShape({ id: this.currentShapeId, type: "node", props: { placed: true } })
  }

  override onPointerMove = (info: TLPointerEventInfo) => {
    // console.log(info.point)
    // console.log(this.currentShapeId)
    if (this.currentShapeId) {
      this.editor.updateShape({ id: this.currentShapeId, type: "node", x: info.point.x, y: info.point.y, props: { placed: false } })
    }
  }

  override onPointerDown = (info: TLPointerEventInfo) => {
    console.log(info.point)
    console.log(this.currentShapeId)
    if (this.currentShapeId) {
      this.editor.updateShape({ id: this.currentShapeId, type: "node", props: { placed: true } })
    }

    this.parent.parent.transition('select')
  }

  onExit = () => {
    console.log("exit pointing")
  }

}

