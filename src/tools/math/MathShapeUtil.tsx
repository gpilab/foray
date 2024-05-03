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
  TLOnBeforeCreateHandler
} from 'tldraw';
import 'katex/dist/katex.min.css';
import { BlockMath } from 'react-katex';



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

// const sizeCache = new WeakMapCache<MathTextShape['props'], { height: number; width: number }>()

export class MathTextShapeUtil extends ShapeUtil<MathTextShape> {
  static override type = 'math-text' as const
  static override props = mathTextShapeProps
  // TODO handle migration

  override isAspectRatioLocked = (_shape: MathTextShape) => false
  override canResize = (_shape: MathTextShape) => true

  //determines if arrows can bind to shape
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
    const width = shape.props.w
    const height = shape.props.h

    //const { width, height } = getTextSize(this.editor, shape.props)
    return new Rectangle2d({
      width: width,
      height: height,
      // should hitbox be edges only, or filled to include center
      isFilled: true,
      // make shape editable with a single click if it is already selected
      // Only works well with TextLabel labels
      // causes problems if enabled when the shape is editable
      isLabel: true,
    })
  }


  indicator(shape: MathTextShape) {
    const width = shape.props.w
    const height = shape.props.h
    return <rect width={width} height={height} />
  }
  // TODO calculate shape size dynamically
  // getMinDimensions(shape: MathTextShape) {
  //   return sizeCache.get(shape.props, (props) => this.getTextSize(this.editor, props))
  // }
  //
  // getTextSize(editor: Editor, props: MathTextShape['props']) {
  //   const { text, sizeStyle: font_size, w: w } = props
  //
  //   const minWidth = Math.max(16, w)
  //   const fontSize = LABEL_FONT_SIZES[font_size]
  //
  //   const cw = Math.floor(Math.max(minWidth, w))
  //
  //   const result = editor.textMeasure.measureText(text, {
  //     ...TEXT_PROPS,
  //     fontFamily: FONT_FAMILIES["mono"],
  //     fontSize: fontSize,
  //     maxWidth: cw,
  //   })
  //
  //   return {
  //     width: Math.max(minWidth, result.w),
  //     height: Math.max(fontSize, result.h),
  //   }
  // }

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

  focusTextBox(selectAllText: boolean) {
    const input = window.document.getElementById("math-text-input") as HTMLInputElement
    if (input == null) return

    input.focus()

    if (selectAllText) {
      input.select()
    }
  }

  override onBeforeCreate: TLOnBeforeCreateHandler<MathTextShape> = () => {
    this.focusTextBox(true)
  }

  component(shape: MathTextShape) {
    const {
      props: { text, color, sizeStyle: font_size, scale }
    } = shape

    const theme = getDefaultColorTheme({ isDarkMode: this.editor.user.getIsDarkMode() })

    // const isSelected = id === this.editor.getOnlySelectedShapeId()
    // const isEditing = id === this.editor.getEditingShapeId()
    //const { width, height } = getTextSize(this.editor, shape.props)


    return (<HTMLContainer style={{
      color: theme[color].solid,
      fontSize: LABEL_FONT_SIZES[font_size],
      transform: `scale(${scale})`,
      transformOrigin: 'top center',
      width: shape.props.w,
      height: shape.props.h,
    }}
    >
      <div
        style={{
          pointerEvents: 'all',
        }}
        onDoubleClick={() => {
          console.log("double click!")
          this.focusTextBox(true)
        }
        }
        onClick={() => {
          console.log("single click!")
          this.focusTextBox(false)
        }
        }
      >
        <BlockMath math={text} ></BlockMath>
      </div>
    </ HTMLContainer >)

  }
}

