import { TLShape, Tldraw } from 'tldraw'
import CustomUi from './custom-ui'

import 'tldraw/tldraw.css'

// aquired via "copy as JSON", which is an option when debug
// mode is on
import _startShape from './assets/init_snapshot.json'
const startShapes = _startShape.shapes as Array<TLShape>

export default function CustomUiExample() {
  return (
    <Tldraw
      inferDarkMode
      persistenceKey='gpi_v2'
      onMount={(editor) => {
        editor.updateInstanceState({ isFocusMode: true })
        editor.createShapes(startShapes)
      }}>
      <CustomUi />
    </Tldraw>
  )
}

