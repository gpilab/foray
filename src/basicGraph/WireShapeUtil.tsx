import {
  DefaultColorStyle,
  Editor,
  Mat,
  Polyline2d, RecordPropsType, SVGContainer, ShapeUtil, TLBaseShape,
  TLDefaultColorStyle,
  TLOnBeforeUpdateHandler,
  Vec,
  VecLike,
  getDefaultColorTheme,
  lerp,
  track,
  useEditor,
  useIsDarkMode,
} from 'tldraw'
import { WireBinding } from './WireBindingUtil'
import { NodeShapeUtil } from './nodeShapeUtil'
import { NodeShape } from '../tools/node/NodeShapeUtil'

export const wireShapeProps = {
  color: DefaultColorStyle,
}

type WireShapeProps = RecordPropsType<typeof wireShapeProps>
type WireShape = TLBaseShape<'wire', WireShapeProps>


/**
 * Determines how the wire shape behaves under various scenarios
 */
export class WireShapeUtil extends ShapeUtil<WireShape> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return { color: 'grey' as const, }
  }

  canSnap = () => false

  override canBind() {
    return true //TODO make specific binding requirements
  }
  override canEdit = () => false
  override isAspectRatioLocked = () => false
  override canBeLaidOut = () => false

  override hideSelectionBoundsBg = () => true
  override hideSelectionBoundsFg = () => true
  override hideRotateHandle = () => true
  override canResize = () => false

  override onBeforeUpdate: TLOnBeforeUpdateHandler<WireShape> = (_prev, next) => {
    // workaround to prevent the wire from blowing in some group resizing scenarios
    // wire rendering only depends start and end shapes, so we don't actually need x,y to update
    return { ...next, x: 0, y: 0 }
  }


  override getGeometry(wireShape: WireShape) {
    const { start, end } = WireShapeUtil.getLineShape(wireShape, this.editor)
    return new Polyline2d({ points: [Vec.From(start), Vec.From(end)] })
  }

  override component(wireShape: WireShape) {
    const editor = useEditor()
    const { start, end } = WireShapeUtil.getLineShape(wireShape, editor)

    return <LineComponent color={wireShape.props.color}
      wireShape={wireShape}
      end={end}
      start={start}
      strokeWidth={2}
      debug={false}
      isSelected={editor.getSelectedShapeIds()
        .indexOf(wireShape.id) != -1
      }
    />
  }

  override indicator(wireShape: WireShape) {
    const { start, end } = WireShapeUtil.getLineShape(wireShape, this.editor)

    return <LineComponent color={"blue"}
      wireShape={wireShape}
      end={end}
      start={start}
      strokeWidth={1}
      debug={false} />
  }

  /**
   * Calcualte a start and end wire position using the shapes bound to the wire
   */
  static getLineShape(wireShape: WireShape, editor: Editor): { start: Vec, end: Vec } {
    const bindings = editor.getBindingsFromShape<WireBinding>(wireShape, "wire")
    const startBinding = bindings.find(b => b.props.terminal === "start")

    if (startBinding === undefined) {
      console.log("rendering line without start!")
      console.log(bindings)
      return {
        start: Vec.From({ x: 0, y: 0 }),
        end: Vec.From({ x: 0, y: 0 })
      }
    }

    const start = WireShapeUtil.getTerminalInWireSpace(wireShape, startBinding, editor)
    const endBinding = bindings.find(b => b.props.terminal === "end")

    if (endBinding === undefined) {
      // If there isn't an end yet, use current mouse position as the end point
      const pointInShapeSpace = editor.getPointInShapeSpace(wireShape, editor.inputs.currentPagePoint)
      return {
        start: start,
        end: pointInShapeSpace
      }
    }

    const end = WireShapeUtil.getTerminalInWireSpace(wireShape, endBinding, editor)
    return { start: start, end: end }
  }

  /**
   * get the wire start/end point relative to the wire's transform
   */
  static getTerminalInWireSpace(wireShape: WireShape, binding: WireBinding, editor: Editor) {
    const boundShape = editor.getShape(binding.toId)! as NodeShape
    const ioType = binding.props.terminal === "start" ? "out" : "in"

    const portRelativeLoc = NodeShapeUtil.getRelativePortLocation(boundShape, ioType, binding.props.portName)

    //account for node's translation,rotation, and scale
    const boundShapeTransform = editor.getShapePageTransform(boundShape)
    const pagePoint = Mat.applyToPoint(boundShapeTransform, portRelativeLoc)

    const wirePoint = editor.getPointInShapeSpace(wireShape, pagePoint)

    return wirePoint
  }
}


type LineProps = {
  start: Vec,
  end: Vec,
  strokeWidth: number,
  color: TLDefaultColorStyle,
  wireShape: WireShape,
  debug?: boolean
  isSelected?: boolean
}

const LineComponent = track(({ wireShape, start, end, color, strokeWidth, debug = false, isSelected = false }: LineProps) => {
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
    <SVGContainer id={wireShape.id} style={{ pointerEvents: "none" }}>
      <g stroke={colorVal}
        fill="none"
        strokeWidth={strokeWidth}
        strokeLinejoin="round"
        strokeLinecap="round"
        pointerEvents="none"
      >
        <BezierS start={start} end={end} intensity={0.5} />
        {debug ?
          <g strokeWidth={strokeWidth * .5} opacity={.5}>
            <path id="dir_ind_1"
              d={`M ${center.x} ${center.y} L${arrowLeg1.x} ${arrowLeg1.y}`} />
            <path id="dir_ind_2"
              d={`M ${center.x} ${center.y} L${arrowLeg2.x} ${arrowLeg2.y}`} />
          </g>
          : ""}
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
})


function BezierS(props: { start: VecLike, end: VecLike, intensity?: number }) {
  const { start, end, intensity = .5 } = props
  const c1 = lerp(start.y, end.y, intensity)
  const c2 = lerp(start.y, end.y, 1 - intensity)
  return <path
    d={`M ${start.x} ${start.y} 
        C${start.x} ${c1}  
        ${end.x} ${c2}  
        ${end.x} ${end.y}`} />
}
