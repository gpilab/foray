import {
  BindingOnChangeOptions,
  BindingOnShapeDeleteOptions, BindingOnShapeIsolateOptions, BindingUtil,
  Editor, Mat, TLBaseBinding,
  TLShape,
  VecModel,
  lerp,
} from 'tldraw'
import { WireShape } from './WireShapeUtil'


export type WireBinding = TLBaseBinding<'wire', {
  anchor: VecModel,
  terminal: "start" | "end"
}>

/**
 * Determines how wire should behave when bound shapes change
 */
export class WireBindingUtil extends BindingUtil<WireBinding> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return {
      anchor: { x: 0.5, y: 0.5 },
      terminal: "start" as const
    }
  }

  // when the BOUND SHAPE changes, update the wire's position
  // override onAfterChangeToShape({ binding, shapeAfter }: BindingOnShapeChangeOptions<WireBinding>): void {
  // console.log("onAfterChangeToShape")
  // const wireShape = this.editor.getShape<WireShape>(binding.fromId)!

  //     console.log(` 
  // binding id: ${binding.id}
  // wireShape : ${wireShape.id}
  // binding fromId: ${binding.fromId}
  // binding toId: ${binding.toId}
  // shapeAfter: ${shapeAfter.id}
  // wireShape parent: ${wireShape.parentId}
  // shapeAfter parent: ${shapeAfter.parentId} 
  // `)
  //     //reparent if objects are grouped
  //     if (wireShape.parentId != shapeAfter.parentId) {
  //       const parent = this.editor.getShape(shapeAfter.parentId)!
  //       console.log("reparenting to ", parent.id)
  //       const newUpdate = {
  //         id: binding.id,
  //         type: "wire",
  //         fromId: wireShape.id,
  //         toId: parent.id,
  //         props: wireShape.props,
  //         typeName: "binding"
  //       }
  //       //this.editor.updateBinding(newUpdate as any)
  //       console.log("new parent:", this.editor.getBinding(binding.id)?.toId)
  //       return
  //     }
  //
  // updateWirePos(this.editor, binding, wireShape)

  // }
  // override onOperationComplete(): void {
  //   console.log("operation complete")
  // }

  // when the WIRE SHAPE changes. This will happen if a group selection contains a wire. 
  // Individual wires can't be modified by the user
  // override onAfterChangeFromShape({ binding, shapeAfter, }: BindingOnShapeChangeOptions<WireBinding>) {
  //   // console.log("onAfterChangeFromShape")
  //   // const wireShape = shapeAfter as WireShape
  //   // updateWirePos(this.editor, binding, wireShape)
  // }

  // when the thing we're stuck to is deleted, delete the wire too 
  override onBeforeDeleteToShape({ binding }: BindingOnShapeDeleteOptions<WireBinding>): void {
    this.editor.deleteShape(binding.fromId)
  }

  override onBeforeIsolateFromShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
    console.log("onBeforeIsolateFromShape: ", options.binding.fromId)
    this.editor.deleteShape(options.binding.fromId)
  }

  override onBeforeIsolateToShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
    console.log("onBeforeIsolateToShape: ", options.binding.fromId)
    this.editor.deleteShape(options.binding.fromId)
  }
  override onAfterChange(_options: BindingOnChangeOptions<WireBinding>): void {
    console.log("after binding change")

  }
  override onBeforeChange(_options: BindingOnChangeOptions<WireBinding>): void {
    console.log("before binding change")

  }
}

/** 
  * Determine where a wire should be anchored on a given a shape
  * */
export function calcShapeAnchor(editor: Editor, parentTransform: Mat, baseShape: TLShape, anchor: VecModel) {

  const shapeBounds = editor.getShapeGeometry(baseShape)!.bounds
  const shapeAnchor = {
    x: lerp(shapeBounds.minX, shapeBounds.maxX, anchor.x),
    y: lerp(shapeBounds.minY, shapeBounds.maxY, anchor.y),
  }
  const pageAnchor = editor.getShapePageTransform(baseShape).applyToPoint(shapeAnchor)

  return parentTransform.invert().applyToPoint(pageAnchor).toJson()
}

/** 
 * given a binding type (start or end binding), update the wire to be placed correctly
 * relative to the bound shape
 * */
// function updateWirePos(editor: Editor, binding: WireBinding, wireShape: WireShape) {
//   const boundShape = editor.getShape(binding.toId)!
//
//   const isStartChange = binding.props.terminal == "start"
//
//   const pastAnchor = isStartChange ? wireShape.props.start : wireShape.props.end
//   const newAnchor = calcShapeAnchor(editor, editor.getShapePageTransform(wireShape), boundShape, binding.props.anchor)
//   const delta = Vec.From(pastAnchor).sub(Vec.From(newAnchor))
//
//   if (delta.len() > 0.0001) {
//     // console.log(`Wire change: ${binding.props.terminal}
//     //             old start: ${JSON.stringify(wireShape.props.start)}
//     //             old end: ${JSON.stringify(wireShape.props.end)}
//     //             New WireAnchor: ,${JSON.stringify(newAnchor)}
//     //             delta: ${delta}
//     //             `)
//
//     const startAnchor = isStartChange ? newAnchor : wireShape.props.start
//     const endAnchor = isStartChange ? wireShape.props.end : newAnchor
//     editor.updateShape({
//       id: wireShape.id,
//       type: 'wire',
//       x: 0,
//       y: 0,
//       props: {
//         start: startAnchor,
//         end: endAnchor
//       }
//     })
//   }
// }



export interface WireBindings {
  start: WireBinding | undefined
  end: WireBinding | undefined
}

export function getArrowBindings(editor: Editor, shape: WireShape): WireBindings {
  const bindings = editor.getBindingsFromShape<WireBinding>(shape, 'wire')
  return {
    start: bindings.find((b) => b.props.terminal === 'start'),
    end: bindings.find((b) => b.props.terminal === 'end'),
  }
}
