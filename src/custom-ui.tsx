import { useEffect } from 'react'
import { track, useEditor } from 'tldraw'
import './custom-ui.css'

export default track(() => {
  const editor = useEditor()
  useEffect(() => {
    const handleKeyUp = (e: KeyboardEvent) => {
      if (!editor.getCurrentTool().getIsActive()) {
        switch (e.key) {
          case 'Delete':
          case 'Backspace': {
            editor.deleteShapes(editor.getSelectedShapeIds())
            break
          }
          case 'v': {
            editor.setCurrentTool('select')
            break
          }
          case 'e': {
            editor.setCurrentTool('eraser')
            break
          }
          case 'x':
          case 'p':
          case 'b':
          case 'd': {
            editor.setCurrentTool('draw')
            break
          }
        }
      }
    }

    window.addEventListener('keyup', handleKeyUp)
    return () => {
      window.removeEventListener('keyup', handleKeyUp)
    }
  })

  return (
    <div className="custom-layout">
      <div className="custom-toolbar">
        <button
          className="custom-button"
          data-isactive={editor.getCurrentToolId() === 'select'}
          onClick={() => editor.setCurrentTool('select')}
        >
          Select
        </button>
        <button
          className="custom-button"
          data-isactive={editor.getCurrentToolId() === 'geo'}
          onClick={() => editor.setCurrentTool('geo')}
        >
          Node
        </button>
        <button
          className="custom-button"
          data-isactive={editor.getCurrentToolId() === 'arrow'}
          onClick={() => editor.setCurrentTool('arrow')}
        >
          Wire
        </button>
        <button
          className="custom-button"
          data-isactive={editor.getCurrentToolId() === 'text'}
          onClick={() => editor.setCurrentTool('text')}
        >
          Comment
        </button>
        <button
          className="custom-button"
          data-isactive={editor.getCurrentToolId() === 'draw'}
          onClick={() => editor.setCurrentTool('draw')}
        >
          Draw
        </button>
        <button
          className="custom-button"
          data-isactive={editor.getCurrentToolId() === 'eraser'}
          onClick={() => editor.setCurrentTool('eraser')}
        >
          Eraser
        </button>
      </div>
    </div>
  )
})
