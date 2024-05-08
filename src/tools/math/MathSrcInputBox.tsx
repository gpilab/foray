import { LABEL_FONT_SIZES, TLShapeId, getDefaultColorTheme, stopEventPropagation, useEditor } from "tldraw"
import { MathTextShape } from "./MathShapeUtil"
import { RefObject } from "react"

export const MyComponent = function MyComponent({
  id,
  text,
  type,
  inputRef
}: {
  id: TLShapeId
  text: string
  type: "math-text"
  inputRef: RefObject<HTMLInputElement>
}) {
  const editor = useEditor()
  const shape = editor.getShape(id)

  if (shape == null || shape.type != 'math-text') return null

  const theme = getDefaultColorTheme({ isDarkMode: editor.user.getIsDarkMode() })

  const mShape = shape as MathTextShape
  const color = mShape.props.color
  const font_height = LABEL_FONT_SIZES['m']

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!mShape) return null

    const new_text = e.currentTarget.value
    if (text === new_text) return null

    editor.updateShapes([
      {
        id,
        type: type,
        props: { text: new_text },
      },
    ])
  }
  function handleClick(e: React.MouseEvent<HTMLInputElement>) {
    (e.target as HTMLInputElement).select()
    stopEventPropagation(e)
  }
  return (
    <div
      style={{
        pointerEvents: 'all',
        position: 'absolute',
        bottom: '100%',
        left: '0%',
        transformOrigin: "bottom left",
        padding: '5px',
        transform: 'scale(var(--tl-scale)) translateX(calc(-1 * var(--space-3))',
      }}
      onPointerDown={stopEventPropagation}>
      <input id='math-text-input'
        ref={inputRef}
        value={mShape.props.text}
        style={{
          height: 'auto',
          //width: 'fitContent',
          width: Math.max(5, text.length + 3) + 'ch',
          color: theme[color].solid,
          fontSize: font_height,
          backgroundColor: 'transparent',
          border: 'none',

        }}
        onChange={handleChange}
        onDoubleClick={(e) => handleClick(e)}
      />
    </div >
  )
}
