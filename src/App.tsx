import 'tldraw/tldraw.css'
import './App.css'
import _startShape from "./assets/zeno.json"
import { GraphProvider } from './graph/graphContext.tsx'
import { TldrawCanvas } from './tldrawCanvas.tsx'

export default function CustomUiExample() {
  return (
    <GraphProvider>
      <TldrawCanvas />
    </GraphProvider >
  )
}

