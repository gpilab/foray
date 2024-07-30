import { appWindow } from "@tauri-apps/api/window"
import { ReactNode, useRef } from "react";
import { useHover } from "usehooks-ts";



export function TitleBar(props: { showWindowControls: boolean }) {
  return (
    <div data-tauri-drag-region
      className="titlebar"
      style={{
        height: "30px",
        display: 'flex',
        justifyContent: 'flex-end'
      }}>
      {
        props.showWindowControls
          ? <WindowActionButtons />
          : ""
      }
    </div>
  )
}

export function Menu(props: { children: ReactNode }) {
  return <div style={{
    display: "flex",
    alignItems: "stretch",
  }}>
    {props.children}
  </div>
}

export function MenuItem(props: { children: ReactNode, onClick: () => void, selected?: boolean }) {
  const ref = useRef(null)

  return <>
    <div onClick={props.onClick} ref={ref} style={{
      display: "flex",
      alignItems: "end",
      cursor: "default",
      minWidth: "50px",
      paddingInline: "5px",
      paddingBottom: "2px",
    }}>
      {props.children}
    </div >
  </>
}

function WindowActionButtons() {
  return <div style={{
    display: "flex",
  }}>
    <TitlebarButton
      action={() => appWindow.minimize()}
      icon="minimize.svg"
      alt="minimize" />
    <TitlebarButton
      action={() => appWindow.toggleMaximize()}
      icon="maximize.svg"
      alt="maximize" />
    <TitlebarButton
      action={() => appWindow.close()}
      icon="close.svg"
      alt="close" />
  </div>
}

type TitleBarProps = {
  action: (() => Promise<void>) | (() => void),
  alt: string,
  icon?: string,
  svg?: ReactNode
}

function TitlebarButton(props: TitleBarProps) {
  const { action, alt, icon } = props

  const hoverRef = useRef(null)
  const isHover = useHover(hoverRef)

  return <div
    ref={hoverRef}
    style={{
      width: "32px",
      display: "flex",
      justifyContent: "center",
      alignContent: "center",
      backgroundColor: isHover ? "#ffffff11" : "",
      borderRadius: "8px",
    }}
    onClick={() => action}
  >
    <img src={icon} alt={alt} />
  </div>
}
