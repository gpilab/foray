import {
  DefaultToolbar, DefaultToolbarContent,
  TLUiAssetUrlOverrides,
  TLUiComponents, TLUiOverrides, Tldraw,
  TldrawUiMenuItem, useIsToolSelected, useTools
} from 'tldraw'

import 'tldraw/tldraw.css'
import './App.css'
import { WireShapeUtil } from './shapes/wire/WireShapeUtil'
import { WireBindingUtil } from './shapes/wire/WireBindingUtil'
import { WireTool } from './shapes/wire/WireTool'
import { NodeShapeUtil } from './shapes/node/nodeShapeUtil'
import { NodeStylePanel } from './shapes/node/nodeStylePanel'
import { MathTextShapeUtil } from './shapes/math/MathShapeUtil'
import { MathShapeTool } from './shapes/math/MathShapeTool'

export default function GPI() {
  return (
    <Tldraw
      persistenceKey="basicTldrawGraph"
      inferDarkMode
      shapeUtils={[WireShapeUtil, NodeShapeUtil, MathTextShapeUtil]}
      bindingUtils={[WireBindingUtil]}
      tools={[WireTool, MathShapeTool]}
      overrides={overrides}
      components={components}
      assetUrls={customAssetURLs}
    />
  )
}

export const customAssetURLs: TLUiAssetUrlOverrides = {
  icons: {
    'pi-symbol': 'pi-symbol.svg',
    'network': 'network.svg',
    'wire': 'wire.svg',
  }
}

const overrides: TLUiOverrides = {
  tools(editor, tools) {
    tools.mathText = {
      id: 'math-text',
      icon: 'pi-symbol',
      label: 'Math',
      kbd: 'm',
      onSelect: () => {
        editor.setCurrentTool('math-text')
      },
    }
    tools.wire = {
      id: 'wire',
      label: 'wire',
      icon: 'network',
      kbd: 'w',
      onSelect: () => {
        editor.setCurrentTool('wire')
      },
    }
    return tools
  },
}

const components: TLUiComponents = {
  StylePanel: NodeStylePanel,
  Toolbar: (...props) => {
    const tools = useTools()
    const wire = tools.wire
    const math = tools.mathText
    const isWireSelected = useIsToolSelected(wire)
    const isMathSelected = useIsToolSelected(math)
    return (
      <DefaultToolbar {...props}>
        <TldrawUiMenuItem {...wire} isSelected={isWireSelected} />
        <TldrawUiMenuItem {...math} isSelected={isMathSelected} />
        <DefaultToolbarContent />
      </DefaultToolbar>
    )
  },
}
