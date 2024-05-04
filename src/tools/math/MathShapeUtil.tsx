import {
  DefaultColorStyle,
  Rectangle2d, ShapePropsType,
  ShapeUtil,
  T,
  TLBaseShape, TLOnResizeHandler,
  getDefaultColorTheme, resizeBox,
  LABEL_FONT_SIZES,
  DefaultSizeStyle,
  HTMLContainer,
  TLOnBeforeCreateHandler,
} from 'tldraw';
import 'katex/dist/katex.min.css';
import { BlockMath } from 'react-katex';
import { createRef, useEffect } from 'react';



const mathTextShapeProps = {
  text: T.string,
  color: DefaultColorStyle,
  sizeStyle: DefaultSizeStyle,
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

  getDefaultProps(): MathTextShape['props'] {
    return {
      text: "a^2+b^2 = c^2",
      sizeStyle: "m",
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
    const next = resizeBox(shape, info)
    const is_init = info.initialBounds.h == 1

    const delta_scale = is_init ? 1 : shape.props.scale * info.scaleY

    this.editor.updateShape<MathTextShape>({
      id: shape.id,
      type: 'math-text',
      props: { scale: delta_scale },
    })

    return next
  }

  override onBeforeCreate: TLOnBeforeCreateHandler<MathTextShape> = () => {
    this.focusTextBox(true)
  }

  focusTextBox(selectAllText: boolean) {
    const input = window.document.getElementById("math-text-input") as HTMLInputElement
    if (input == null) return

    input.focus()
    if (selectAllText) {
      input.select()
    }
  }


  component(shape: MathTextShape) {
    const {
      props: { text, color, sizeStyle: font_size, scale, w, h }
    } = shape
    const theme = getDefaultColorTheme({ isDarkMode: this.editor.user.getIsDarkMode() })
    //use this to determine what the rendered equation size is once it is rendered
    const mathTextRef = createRef<HTMLDivElement>()

    useEffect(() => {
      if (!mathTextRef.current) return

      const renderedWidth = mathTextRef.current.offsetWidth
      const renderedHeight = mathTextRef.current.offsetHeight

      if (renderedWidth != w || renderedHeight != h) {
        this.editor.updateShape<MathTextShape>({
          id: shape.id,
          type: 'math-text',
          props: {
            w: (mathTextRef.current.offsetWidth ?? 200) * scale,
            h: (mathTextRef.current.offsetHeight ?? 200) * scale
          },
        })
      }
    })

    return (<HTMLContainer style={{
      color: theme[color].solid,
      fontSize: LABEL_FONT_SIZES[font_size],
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
          this.focusTextBox(true)
        }}
        onClick={() => {
          this.focusTextBox(false)
        }}
      >
        <BlockMath math={text} ></BlockMath>
      </div>
    </ HTMLContainer >)
  }
}

