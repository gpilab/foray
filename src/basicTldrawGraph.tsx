import {
  BindingOnShapeChangeOptions, BindingUtil,
  DefaultColorStyle, DefaultToolbar, DefaultToolbarContent,
  Editor, Polyline2d, RecordPropsType, SVGContainer, ShapeUtil, StateNode, TLBaseBinding, TLBaseShape,
  TLEventHandlers,
  TLShape,
  TLShapeId,
  TLUiComponents, TLUiOverrides, Tldraw, TldrawUiMenuItem, Vec, VecModel,
  createShapeId, getDefaultColorTheme, useIsDarkMode,
  useIsToolSelected, useTools, vecModelValidator
} from 'tldraw'

import 'tldraw/tldraw.css'
import './App.css'

export const wireShapeProps = {
  color: DefaultColorStyle,
  start: vecModelValidator,
  end: vecModelValidator,
}

export type WireShapeProps = RecordPropsType<typeof wireShapeProps>
export type WireShape = TLBaseShape<'wire', WireShapeProps>


class WireShapeUtil extends ShapeUtil<WireShape> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return {
      color: 'black' as const,
      start: { x: 0, y: 0 },
      end: { x: 50, y: 100 },
    }
  }

  canSnap = () => false

  override canBind() {
    return true //TODO make specific binding requirements
  }
  override canEdit = () => false
  override canResize = () => false
  override hideRotateHandle = () => true
  override isAspectRatioLocked = () => true

  override getGeometry(shape: WireShape) {
    const points = [Vec.From(shape.props.start)
      , Vec.From(shape.props.end)]
    return new Polyline2d({ points })
  }

  override component(shape: WireShape) {
    const isDarkMode = useIsDarkMode()
    const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })
    const { start, end } = shape.props

    return <LineComponent color={theme.black.solid}
      shape={shape}
      end={start}
      start={end}
      strokeWidth={3}
    />
  }

  override indicator(shape: WireShape) {
    const isDarkMode = useIsDarkMode()
    const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })
    const { start, end } = shape.props

    return <LineComponent color={theme.blue.solid}
      shape={shape}
      end={end}
      start={start}
      strokeWidth={1}
    />
  }

  // override onTranslateStart: TLOnTranslateStartHandler<WireShape> = (shape) => {
  //   console.log("translating wire, clearing bindings")
  //   const bindings = this.editor.getBindingsFromShape(shape, 'wire')
  //   this.editor.deleteBindings(bindings)
  // }
  //
  // override onTranslateEnd: TLOnTranslateEndHandler<WireShape> = (_initial, wireShape) => {
  //   console.log("placed wire, maybe creating bindings")
  //   const pageAnchor = this.editor.getShapePageTransform(wireShape).applyToPoint({ x: 0, y: 0 })
  //   const target = this.editor.getShapeAtPoint(pageAnchor, {
  //     hitInside: true,
  //     filter: (potentialTarget) =>
  //       this.editor.canBindShapes({ fromShape: wireShape, toShape: potentialTarget, binding: 'wire' })
  //       && potentialTarget.id != wireShape.id
  //   })
  //   console.log("wire target", target)
  //   if (!target) return
  //
  //   const targetBounds = Box.ZeroFix(this.editor.getShapeGeometry(target)!.bounds)
  //   const pointInTargetSpace = this.editor.getPointInShapeSpace(target, pageAnchor)
  //
  //   const anchor = {
  //     x: invLerp(targetBounds.minX, targetBounds.maxX, pointInTargetSpace.x),
  //     y: invLerp(targetBounds.minY, targetBounds.maxY, pointInTargetSpace.y),
  //   }
  //   console.log("wire anchor: ", anchor)
  //   console.log("creating binding from ", wireShape.id, " to ", target.id)
  //   this.editor.createBinding({
  //     type: 'wire',
  //     fromId: wireShape.id,
  //     toId: target.id,
  //     props: {
  //       anchor: anchor,
  //     },
  //   })
  // }
}

type WireBinding = TLBaseBinding<'wire',
  {
    anchor: VecModel,
    terminal: "start" | "end"
  }>

class WireBindingUtil extends BindingUtil<WireBinding> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return {
      anchor: { x: 0.5, y: 0.5 },
      terminal: "start" as const
    }
  }

  // when the shape we're stuck to changes, update the wire's position
  override onAfterChangeToShape({
    binding,
    shapeAfter: changedBoundShape,
  }: BindingOnShapeChangeOptions<WireBinding>): void {

    const wireShape = this.editor.getShape<WireShape>(binding.fromId)!

    // const shapeBounds = this.editor.getShapeGeometry(changedBoundShape)!.bounds
    // const shapeAnchor = {
    //   x: lerp(shapeBounds.minX, shapeBounds.maxX, binding.props.anchor.x),
    //   y: lerp(shapeBounds.minY, shapeBounds.maxY, binding.props.anchor.y),
    // }
    // const pageAnchor = this.editor.getShapePageTransform(changedBoundShape).applyToPoint(shapeAnchor)
    //
    // const boundShapeAnchor = this.editor
    //   .getShapeParentTransform(wireShape)
    //   .invert()
    //   .applyToPoint(pageAnchor)

    const shapeLocation = { x: changedBoundShape.x, y: changedBoundShape.y }
    if (binding.props.terminal == "start") {
      this.editor.updateShape({
        id: wireShape.id,
        type: 'wire',
        x: 0,
        y: 0,
        props: {
          start: shapeLocation,
          end: wireShape.props.end
        }
      })

    } else {
      this.editor.updateShape({
        id: wireShape.id,
        type: 'wire',
        x: 0,
        y: 0,
        props: {
          start: wireShape.props.start,
          end: shapeLocation
        }
      })

    }

  }

  // override onAfterChangeFromShape({ binding, shapeAfter }: BindingOnShapeChangeOptions<WireBinding>) {
  //   console.log("FROM SHAPE(wire shape) CHANGED! IDK why this would happen other then on create and delete!! binding ", binding)
  //   console.log("shapeAfter", shapeAfter)
  // }

  // // when the thing we're stuck to is deleted, delete the wire too
  // override onBeforeDeleteToShape({ binding }: BindingOnShapeDeleteOptions<WireBinding>): void {
  //   this.editor.deleteShape(binding.fromId)
  // }
}

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


class WireTool extends StateNode {
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
    this.editor.createShape({
      id: wireId,
      type: 'wire',
      // x: this.editor.inputs.currentPagePoint.x,
      // y: this.editor.inputs.currentPagePoint.y,
      props: {
        start: this.editor.inputs.currentPagePoint.toJson(),
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

const overrides: TLUiOverrides = {
  tools(editor, schema) {
    schema['wire'] = {
      id: 'wire',
      label: 'wire',
      icon: 'heart-icon',
      kbd: 'p',
      onSelect: () => {
        editor.setCurrentTool('wire')
      },
    }
    return schema
  },
}

const components: TLUiComponents = {
  Toolbar: (...props) => {
    const wire = useTools().wire
    const isWireSelected = useIsToolSelected(wire)
    return (
      <DefaultToolbar {...props}>
        <TldrawUiMenuItem {...wire} isSelected={isWireSelected} />
        <DefaultToolbarContent />
      </DefaultToolbar>
    )
  },
}

export default function BasicTldrawGraph() {
  return (
    <Tldraw
      persistenceKey="basicTldrawGraph"
      inferDarkMode
      onMount={(editor) => {
        ; (window as any).editor = editor
      }}
      shapeUtils={[WireShapeUtil]}
      bindingUtils={[WireBindingUtil]}
      tools={[WireTool]}
      overrides={overrides}
      components={components}
    />
  )
}

type LineProps = {
  start: VecModel,
  end: VecModel,
  strokeWidth: number,
  color: string,
  shape: WireShape
}

function LineComponent({ shape, start, end, color, strokeWidth }: LineProps) {
  return (
    <SVGContainer id={shape.id} >
      <g stroke={color}
        fill="none"
        strokeWidth={strokeWidth}
        strokeLinejoin="round"
        strokeLinecap="round"

        pointerEvents="none"
      >
        <path d={`M ${start.x} ${start.y} L${end.x} ${end.y}`} />

      </g>
    </SVGContainer>
  )

}

// <LineShapeSvg shape={shape} />
