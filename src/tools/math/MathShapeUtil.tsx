import {
  DefaultColorStyle,
  Rectangle2d, ShapePropsType,
  ShapeUtil,
  T,
  TLBaseShape, TLOnResizeHandler,
  getDefaultColorTheme,
  LABEL_FONT_SIZES,
  DefaultSizeStyle,
  HTMLContainer,
  TLOnBeforeCreateHandler,
  useEditor,
  Vec,
} from 'tldraw';
import 'katex/dist/katex.min.css';
import { BlockMath } from 'react-katex';
import { createRef, useEffect } from 'react';
import { MyComponent } from './MathSrcInputBox';



const mathTextShapeProps = {
  text: T.string,
  color: DefaultColorStyle,
  size_style: DefaultSizeStyle,
  scale: T.number,
  // Shape width
  w: T.number,
  // Shape height
  h: T.number,
}

export type MathTextShapeProps = ShapePropsType<typeof mathTextShapeProps>

export type MathTextShape = TLBaseShape<'math-text', MathTextShapeProps>

export class MathTextShapeUtil extends ShapeUtil<MathTextShape> {
  static override type = 'math-text' as const
  static override props = mathTextShapeProps
  // TODO handle migration

  override isAspectRatioLocked = (_shape: MathTextShape) => false
  override canResize = (_shape: MathTextShape) => true
  override canBind = (_shape: MathTextShape) => true
  override canEdit = (_shape: MathTextShape) => true

  initialText = "a^2+b^2 = c^2"

  getDefaultProps(): MathTextShape['props'] {
    return {
      text: this.initialText,
      size_style: "m",
      w: 140,
      h: 60,
      scale: 1,
      color: 'red',
    }
  }

  getGeometry(shape: MathTextShape) {
    return new Rectangle2d({
      width: shape.props.w,
      height: shape.props.h,
      // should hitbox be edges only, or filled to include center
      isFilled: true,
      // make shape editable with a single click if it is already selected
      // causes problems if enabled when the shape is editable
      isLabel: true,
    })
  }

  indicator(shape: MathTextShape) {
    return <rect width={shape.props.w} height={shape.props.h} />
  }

  override onResize: TLOnResizeHandler<MathTextShape> = (shape, info) => {

    const {
      initialBounds,
      scaleX,
      scaleY,
      newPoint,
    } = info

    const scaleDelta = Math.max(0.01, (Math.abs(scaleX) + Math.abs(scaleY)) / 2)

    // Compute the offset (if flipped X or flipped Y)
    const offset = new Vec(0, 0)

    if (scaleX < 0) {
      offset.x = -(initialBounds.width * scaleDelta)
    }
    if (scaleY < 0) {
      offset.y = -(initialBounds.height * scaleDelta)
    }

    // Apply the offset to the new point
    const { x, y } = Vec.Add(newPoint, offset.rot(shape.rotation))

    const next = {
      x,
      y,
      props: {
        scale: scaleDelta * shape.props.scale,
      },
    }
    return {
      id: shape.id,
      type: shape.type,
      ...next
    }
  }

  override onBeforeCreate: TLOnBeforeCreateHandler<MathTextShape> = () => {
    //this.focusTextBox(true)
  }


  component(shape: MathTextShape) {
    const {
      props: { text, color, size_style, scale, w, h }
    } = shape
    const theme = getDefaultColorTheme({ isDarkMode: this.editor.user.getIsDarkMode() })
    const editor = useEditor()
    const isEditing = editor.getEditingShapeId() == shape.id

    //use this to determine what the rendered equation size is once it is rendered
    const mathTextRef = createRef<HTMLDivElement>()
    const inputRef = createRef<HTMLInputElement>()

    //first render only
    useEffect(() => {
      if (isEditing) {
        if (text == this.initialText) {
          // highlight 
          focusTextBox(true)
        } else {
          focusTextBox(false)
        }
      }
    }, [])

    //check for updated size
    useEffect(() => {
      if (!mathTextRef.current) return

      const renderedWidth = mathTextRef.current.offsetWidth * scale
      const renderedHeight = mathTextRef.current.offsetHeight * scale

      if (renderedWidth != w || renderedHeight != h) {
        this.editor.updateShape<MathTextShape>({
          id: shape.id,
          type: 'math-text',
          props: {
            w: renderedWidth,
            h: renderedHeight
          },
        })
      }
    }, [text, scale, size_style, w, h])

    function focusTextBox(selectAllText: boolean) {
      const input = inputRef.current
      if (input == null) return

      input.focus()
      if (selectAllText) {
        input.select()
      }
    }

    return (<HTMLContainer>
      {isEditing ? <MyComponent id={shape.id} type={shape.type} text={text} inputRef={inputRef} /> : null}
      <HTMLContainer style={{
        color: theme[color].solid,
        fontSize: LABEL_FONT_SIZES[size_style],
        transform: `scale(${scale})`,
      }}
      >
        <div
          ref={mathTextRef}
          style={{
            pointerEvents: 'all',
            width: 'fit-content',
            height: 'fit-content',
          }}
          onDoubleClick={() => {
            focusTextBox(true)
          }}
          onClick={() => {
            focusTextBox(false)
          }}
        >
          <BlockMath math={text} ></BlockMath>
        </div>
      </ HTMLContainer >
    </ HTMLContainer >)
  }
}
