import { TLShape, Tldraw } from 'tldraw'
import { uiOverrides, customAssetURLs } from './tools/ui-overrides'
import { components } from './tools/ui-overrides'
import { MathTextShapeUtil } from './tools/math/MathShapeUtil'
import { MathShapeTool } from './tools/math/MathShapeTool'

import 'tldraw/tldraw.css'
import './App.css'
import _startShape from "./assets/zeno.json"

export default function CustomUiExample() {
  return (
    <Tldraw
      shapeUtils={[MathTextShapeUtil]}
      tools={[MathShapeTool]}
      overrides={uiOverrides}
      components={components}
      assetUrls={customAssetURLs}

      inferDarkMode
      persistenceKey='gpi_v2'
      onMount={(editor) => {
        //editor.updateInstanceState({ isFocusMode: true })
        editor.setCurrentTool('hand')
        editor.createShapes((_startShape.shapes as Array<TLShape>))
      }}
    >
    </Tldraw >
  )
}
