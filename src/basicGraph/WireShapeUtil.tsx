import {
  DefaultColorStyle,
  Mat,
  Polyline2d, RecordPropsType, SVGContainer, ShapeUtil, T, TLBaseShape,
  TLDefaultColorStyle,
  TLOnResizeEndHandler,
  TLOnResizeHandler,
  TLOnResizeStartHandler,
  TLOnRotateEndHandler,
  TLOnRotateHandler,
  TLOnRotateStartHandler,
  TLOnTranslateEndHandler,
  TLOnTranslateHandler,
  TLOnTranslateStartHandler,
  Vec,
  getDefaultColorTheme,
  useEditor,
  useIsDarkMode,
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
      isPlacing: true,
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
    const { start, end } = this.getLineShape(shape)

    return new Polyline2d({ points: [Vec.From(start), Vec.From(end)] })
  }

  override component(shape: WireShape) {
    const editor = useEditor()
    const { start, end } = this.getLineShape(shape)

    return <LineComponent color={shape.props.color}
      shape={shape}
      end={end}
      start={start}
      strokeWidth={3}
      debug={true}
      isSelected={editor.getSelectedShapeIds()
        .indexOf(shape.id) != -1
      }
    />
  }

  override indicator(shape: WireShape) {
    const { start, end } = this.getLineShape(shape)

    return <LineComponent color={"blue"}
      shape={shape}
      end={end}
      start={start}
      strokeWidth={1}
      debug={false} />
  }

  /**
   * Calcualte a start and end wire position using the shapes bound to the wire
   */
  getLineShape(shape: WireShape): { start: Vec, end: Vec } {
    const bindings = this.editor.getBindingsFromShape<WireBinding>(shape, "wire")
    const startBinding = bindings.find(b => b.props.terminal === "start")

    if (startBinding === undefined) {
      console.log("rendering line without start!")
      console.log(bindings)
      return {
        start: Vec.From({ x: 0, y: 0 }),
        end: Vec.From({ x: 0, y: 0 })
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
    const pagePoint = Mat.applyToPoint(
      this.editor.getShapePageTransform(binding.toId)!,
      shapePoint
    )
    const linePoint = Mat.applyToPoint(
      Mat.Inverse(arrowPageTransform),
      pagePoint
    )

    return linePoint
  }
}


type LineProps = {
  start: Vec,
  end: Vec,
  strokeWidth: number,
  color: TLDefaultColorStyle,
  shape: WireShape,
  debug?: boolean
  isSelected?: boolean
}

function LineComponent({ shape, start, end, color, strokeWidth, debug = false, isSelected = false }: LineProps) {
  const isDarkMode = useIsDarkMode()
  const theme = getDefaultColorTheme({ isDarkMode: isDarkMode })
  const colorVal = theme[color].solid

  const startToEnd = Vec.Sub(end, start).toJson()
  const center = Vec.Div(startToEnd, 2).add(start).toJson()
  const arrowLegUnit = Vec.Uni(startToEnd).mul(10).toJson()
  const radians = Math.PI * (3 / 4)
  const arrowLeg1 = Vec.Rot(arrowLegUnit, radians).add(center).toJson()
  const arrowLeg2 = Vec.Rot(arrowLegUnit, -radians).add(center).toJson()

  return (
    <SVGContainer id={shape.id} style={{ pointerEvents: "none" }}>
      <g stroke={colorVal}
        fill="none"
        strokeWidth={strokeWidth}
        strokeLinejoin="round"
        strokeLinecap="round"
        pointerEvents="none"
      >
        <path
          d={`M ${start.x} ${start.y} L${end.x} ${end.y}`} />
        <g strokeWidth={strokeWidth * .75} opacity={.1}>
          <path id="dir_ind_1"
            d={`M ${center.x} ${center.y} L${arrowLeg1.x} ${arrowLeg1.y}`} />
          <path id="dir_ind_2"
            d={`M ${center.x} ${center.y} L${arrowLeg2.x} ${arrowLeg2.y}`} />
        </g>
        {debug && isSelected ? <g strokeWidth={.5} opacity={.5}>
          <path stroke="#00cccc" id="start"
            d={`M 0 0 L ${start.x} ${start.y}`} />
          <path stroke="#cccc00" id="end"
            d={`M 0 0 L ${end.x} ${end.y}`} />
          <path stroke="#44cc44" id="arrow_direction_indicator_1"
            d={`M 0 0 L${center.x} ${center.y}`} />
          <path stroke="#cc0000" id="dir_ind_1"
            d={`M ${center.x} ${center.y} L${arrowLeg1.x} ${arrowLeg1.y}`} />
          <path stroke="#5555ee" id="dir_ind_2"
            d={`M ${center.x} ${center.y} L${arrowLeg2.x} ${arrowLeg2.y}`} />
        </g> : ""}
      </g>
    </SVGContainer>
  )
}
