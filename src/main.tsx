import React from 'react'
import ReactDOM from 'react-dom/client'
import GPI from './gpi'

import { TitleBar } from './util/titleBar'
import { os } from '@tauri-apps/api'

const platform = await os.platform()
ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <div>
      {window.__TAURI__
        ? <div id="tauri-frame"
          style={{
            position: 'absolute',
            inset: "30px 0px 0px 0px"
          }}>
          <TitleBar showWindowControls={platform != "darwin"} />
          <GPI />
        </div>
        // No title bar needed when running in browser
        : <div id="browser-frame">
          <GPI />
        </div>
      }
    </div>
  </React.StrictMode >
  ,)
