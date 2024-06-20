import React from 'react'
import ReactDOM from 'react-dom/client'
import './index.css'
import BasicTldrawGraph from './basicGraph/basicTldrawGraph'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <div style={{ height: "100vh" }}>
      <BasicTldrawGraph />
    </div>
  </React.StrictMode >,
)
