import {
  DefaultToolbar, DefaultToolbarContent,
  TLUiAssetUrlOverrides,
  TLUiComponents, TLUiOverrides, Tldraw,
  TldrawUiMenuItem, useIsToolSelected, useTools
} from 'tldraw'

import 'tldraw/tldraw.css'
import '../App.css'
import { WireShapeUtil } from './WireShapeUtil'
import { WireBindingUtil } from './WireBindingUtil'
import { WireTool } from './WireTool'

export default function BasicTldrawGraph() {
  return (
    <Tldraw
      persistenceKey="basicTldrawGraph"
      inferDarkMode
      onMount={(editor) => {
        ; (window as any).editor = editor
      }}
      shapeUtils={[WireShapeUtil]}
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
