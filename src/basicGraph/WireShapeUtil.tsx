import {
  DefaultColorStyle,
  Mat,
  Polyline2d, RecordPropsType, SVGContainer, ShapeUtil, T, TLBaseShape,
  TLOnResizeEndHandler,
  TLOnResizeHandler,
  TLOnResizeStartHandler,
  TLOnRotateEndHandler,
  TLOnRotateHandler,
  TLOnRotateStartHandler,
  TLOnTranslateEndHandler,
  TLOnTranslateHandler,
  TLOnTranslateStartHandler,
  Vec, VecModel,
  getDefaultColorTheme, useIsDarkMode,
  vecModelValidator
} from 'tldraw'
import { WireBinding } from './WireBindingUtil'

export const wireShapeProps = {
  color: DefaultColorStyle,
  isPlacing: T.boolean,
  start: vecModelValidator,
  end: vecModelValidator,
}

type WireShapeProps = RecordPropsType<typeof wireShapeProps>
type WireShape = TLBaseShape<'wire', WireShapeProps>


/**
 * Determines how the wire shape behaves under various scenarios
 */
export class WireShapeUtil extends ShapeUtil<WireShape> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return {
      color: 'black' as const,
      start: { x: 0, y: 0 },
      end: { x: 50, y: 100 },
      isPlacing: true
    }
  }

  canSnap = () => false

  override canBind() {
    return true //TODO make specific binding requirements
  }
  override canEdit = () => false
  override canResize = () => false
  override hideRotateHandle = () => true
  override isAspectRatioLocked = () => false
  override canBeLaidOut = () => false
  override hideSelectionBoundsBg = () => true
  override hideSelectionBoundsFg = () => true


  //Don't allow user mutations, shape will be calculated only using bound shapes
  override onRotateStart: TLOnRotateStartHandler<WireShape> = (shape) => shape
  override onRotateEnd: TLOnRotateEndHandler<WireShape> = (initial, _current) => initial
  override onRotate: TLOnRotateHandler<WireShape> = (initial, _current) => initial

  override onTranslate: TLOnTranslateHandler<WireShape> = (initial, _current) => initial
  override onTranslateStart: TLOnTranslateStartHandler<WireShape> = (shape) => shape
  override onTranslateEnd: TLOnTranslateEndHandler<WireShape> = (initial, _current) => initial

  override onResize: TLOnResizeHandler<WireShape> = (initial, _current) => initial
  override onResizeStart: TLOnResizeStartHandler<WireShape> = (shape) => shape
  override onResizeEnd: TLOnResizeEndHandler<WireShape> = (initial, _current) => initial



  override getGeometry(shape: WireShape) {
    // const points = [Vec.From(shape.props.start)
    //   , Vec.From(shape.props.end)]
    const { start, end } = this.getLineShape(shape)
    return new Polyline2d({ points: [Vec.From(start), Vec.From(end)] })
  }

  override component(shape: WireShape) {
    const isDarkMode = useIsDarkMode()
    const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })
    const { start, end } = this.getLineShape(shape)


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
    const { start, end } = this.getLineShape(shape)

    return <LineComponent color={theme.blue.solid}
      shape={shape}
      end={end}
      start={start}
      strokeWidth={1}
    />
  }

  /**
   * Calcualte a start and end wire position using the shapes bound to the wire
   */
  getLineShape(shape: WireShape) {
    const bindings = this.editor.getBindingsFromShape<WireBinding>(shape, "wire")
    const startBinding = bindings.find(b => b.props.terminal === "start")
    if (startBinding === undefined) {
      console.log("rendering line without start!")
      console.log(bindings)
      return {
        start: { x: 0, y: 0 },
        end: { x: 0, y: 0 }
      }
    }

    const start = this.getTerminalInWireSpace(shape, startBinding)

    const endBinding = bindings.find(b => b.props.terminal === "end")
    if (endBinding === undefined) {
      // If there isn't an end yet, use current mouse position as the end point
      return {
        start: start,
        end: this.editor.inputs.currentPagePoint
      }
    }

    const end = this.getTerminalInWireSpace(shape, endBinding)
    return { start: start, end: end }

  }

  /**
   * Inspiration from tldraw straight arrow calculation
   */
  getTerminalInWireSpace(wireShape: WireShape, binding: WireBinding) {
    const { point, size } = this.editor.getShapeGeometry(binding.toId).bounds
    const shapePoint = Vec.Add(
      point,
      Vec.MulV({ x: 0.5, y: 0.5 }, size)
    )

    const arrowPageTransform = this.editor.getShapePageTransform(wireShape)!
    const pagePoint = Mat.applyToPoint(this.editor.getShapePageTransform(binding.toId)!, shapePoint)
    const linePoint = Mat.applyToPoint(Mat.Inverse(arrowPageTransform), pagePoint)

    return linePoint
  }
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
    <SVGContainer id={shape.id} style={{ pointerEvents: "none" }}>
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
