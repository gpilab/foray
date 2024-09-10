import {
  DefaultColorThemePalette,
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
import { invoke } from '@tauri-apps/api'

DefaultColorThemePalette.darkMode.background = "#00000000"
DefaultColorThemePalette.lightMode.background = "#ffffff00"



import { NodeDefinition } from './shapes/node/nodeType'
import React, { useState } from 'react'
import { parse_nodes, SerializedPythonNode } from './util/parse_tauri'
import { GPI_UI } from './ui/gpi_ui'
import { nodeDefaultDefinitions } from './shapes/node/nodeDefinitions'
//import { NodeSelect } from './ui/NodeSelect'
//import { GPI_UI } from './ui/gpi_ui'

type GPIState = {
  NodeDefinitions: NodeDefinition<any, any, any>[]
}
const initGPIState: GPIState = {
  NodeDefinitions: []
}
export const GPIContext = React.createContext(initGPIState)
export let GPI_Nodes: NodeDefinition<any, any, any>[] = []


export default function GPI() {
  const [gpiState, setGpiState] = useState(initGPIState)
  return (
    <GPIContext.Provider value={gpiState}>
      <div className="tldraw__editor">
        <Tldraw
          persistenceKey="basicTldrawGraph"
          inferDarkMode
          shapeUtils={[WireShapeUtil, NodeShapeUtil, MathTextShapeUtil]}
          bindingUtils={[WireBindingUtil]}
          tools={[WireTool, MathShapeTool]}
          overrides={overrides}
          components={components}
          assetUrls={customAssetURLs}
          onMount={() => {
            //// Initialize python nodes from server
            invoke<SerializedPythonNode[]>('get_python_nodes').then(
              parse_nodes).then(nodes => {
                // Trying out just having this be global state
                const all_nodes = nodes.concat(Object.values(nodeDefaultDefinitions))
                GPI_Nodes = all_nodes
                setGpiState({ ...gpiState, NodeDefinitions: all_nodes })
              }
              ).catch((e) => {
                console.error("Failed to load nodes", e)
              })

          }}
        >
          <GPI_UI />
        </Tldraw>
      </div>
    </GPIContext.Provider >
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
