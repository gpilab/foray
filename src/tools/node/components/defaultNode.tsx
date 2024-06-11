
export function DefaultNode({ width, height, children }: { width: number, height: number, children?: React.ReactNode }) {
  return <div
    style={{
      width: `${width}px`,
      height: `${height}px`,
      border: "2px solid white",
      borderRadius: "4px",
      padding: "15px 10px",
      pointerEvents: "all",
      display: "flex",
      alignItems: "center",
      justifyContent: "center"
    }}>
    {children}
  </div >
}
