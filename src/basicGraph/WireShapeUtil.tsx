import {
  DefaultColorStyle,
  Polyline2d, RecordPropsType, SVGContainer, ShapeUtil, TLBaseShape,
  TLOnResizeHandler,
  TLOnRotateHandler,
  TLOnTranslateHandler,
  Vec, VecModel,
  getDefaultColorTheme, useIsDarkMode,
  vecModelValidator
} from 'tldraw'

export const wireShapeProps = {
  color: DefaultColorStyle,
  start: vecModelValidator,
  end: vecModelValidator,
}

export type WireShapeProps = RecordPropsType<typeof wireShapeProps>
export type WireShape = TLBaseShape<'wire', WireShapeProps>


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
  override onRotate: TLOnRotateHandler<WireShape> = (initial, _current) => initial
  override onTranslate: TLOnTranslateHandler<WireShape> = (initial, _current) => initial
  override onResize: TLOnResizeHandler<WireShape> = (initial, _current) => initial

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
