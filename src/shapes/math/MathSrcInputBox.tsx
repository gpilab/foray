import { LABEL_FONT_SIZES, TLShapeId, stopEventPropagation, useEditor } from "tldraw"
import { MathTextShape } from "./MathShapeUtil"
import { Ref } from "react"
import { useTheme } from "../../util/useTheme"

export const MathSrcInputBox = function MathSrcInputBox({
  id,
  text,
  type,
  isEditing,
  inputRef
}: {
  id: TLShapeId
  text: string
  type: "math-text"
  isEditing: boolean
  inputRef: Ref<HTMLInputElement>
}) {
  if (!isEditing) return null
  const editor = useEditor()
  const shape = editor.getShape<MathTextShape>(id)

  if (!shape) return null

  const theme = useTheme()

  const color = shape.props.color
  const font_height = LABEL_FONT_SIZES['m']

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
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

  function handleKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    if (e.key == "Enter") {
      editor.setCurrentTool('select.idle')
      stopEventPropagation(e)
    }
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
        value={shape.props.text}
        ref={inputRef}
        style={{
          height: 'auto',
          //width: 'fitContent',
          width: Math.max(5, text.length + 3) + 'ch',
          color: theme[color],
          fontSize: font_height,
          backgroundColor: 'transparent',
          border: 'none',

        }}
        onKeyDown={handleKeyDown}
        onChange={handleChange}
        onDoubleClick={handleClick}
      />
    </div >
  )
}
