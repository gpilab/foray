import { TLShape, Tldraw } from 'tldraw'
import CustomUi from './custom-ui'
import { CardShapeUtil } from './tools/card/CardShapeUtil'
import { CardShapeTool } from './tools/card/CardShapeTool'
import { components, uiOverrides, } from './tools/card/ui-overrides'
import 'tldraw/tldraw.css'

// aquired via "copy as JSON", which is an option when debug
// mode is on
import _startShape from './assets/init_snapshot.json'


export default function CustomUiExample() {
  return (
    <Tldraw
      shapeUtils={[CardShapeUtil]}
      tools={[CardShapeTool]}
      overrides={uiOverrides}
      components={components}

      inferDarkMode
      persistenceKey='gpi_v2'
      onMount={(editor) => {
        editor.updateInstanceState({ isFocusMode: true })
        editor.createShapes((_startShape.shapes as Array<TLShape>))
      }}>
      <CustomUi />
    </Tldraw>
  )
}
