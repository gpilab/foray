import {
  DefaultToolbar, DefaultToolbarContent,
  TLUiAssetUrlOverrides,
  TLUiComponents, TLUiOverrides, Tldraw,
  TldrawUiMenuItem, useIsToolSelected, useTools
} from 'tldraw'

import 'tldraw/tldraw.css'
import '../App.css'
import { WireShapeUtil } from './wire/WireShapeUtil'
import { WireBindingUtil } from './wire/WireBindingUtil'
import { WireTool } from './wire/WireTool'
import { NodeShapeUtil } from './node/nodeShapeUtil'
import { NodeStylePanel } from './node/nodeStylePanel'

export default function BasicTldrawGraph() {
  return (
    <Tldraw
      persistenceKey="basicTldrawGraph"
      inferDarkMode
      shapeUtils={[WireShapeUtil, NodeShapeUtil]}
      bindingUtils={[WireBindingUtil]}
      tools={[WireTool]}
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
  tools(editor, schema) {
    schema['wire'] = {
      id: 'wire',
      label: 'wire',
      icon: 'wire',
      kbd: 'w',
      onSelect: () => {
        editor.setCurrentTool('wire')
      },
    }
    return schema
  },
}

const components: TLUiComponents = {
  StylePanel: NodeStylePanel,
  Toolbar: (...props) => {
    const wire = useTools().wire
    const isWireSelected = useIsToolSelected(wire)
    return (
      <DefaultToolbar {...props}>
        <TldrawUiMenuItem {...wire} isSelected={isWireSelected} />
        <DefaultToolbarContent />
      </DefaultToolbar>
    )
  },
}
