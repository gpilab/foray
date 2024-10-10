import { track } from "tldraw"
import { useNodeConfig } from "./NodeContent"
import { useTheme } from "../../../util/useTheme"
import { useEffect, useRef } from "react"


export const ImageNode = track(
  () => {
    const theme = useTheme()
    const { inputs, color, showPlotGrid } = useNodeConfig()

    console.log("img inputs:", inputs)
    const value = inputs.a?.value?.[1] as { "Image": number[][] }
    //const value = inputs.a?.value?.[1] as number[][]

    //const data = value
    const data = value?.map(row => row["Array"].map(pixel => pixel["Real"]))
    const height = data.length
    const width = data[0].length
    console.log("img data:", data)


    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
      const canvas = canvasRef.current;
      const ctx = canvas?.getContext('2d');
      if (ctx == null) {
        return
      }

      for (let i = 0; i < (data?.length ?? 100); i++) {
        for (let j = 0; j < (data ? data[i].length : 100); j++) {
          const value = data ? data[i][j] : 0;
          const value2 = Math.round(Math.abs(value))

          ctx.fillStyle = `rgb(${value2}, ${value2}, ${value2})`;
          ctx.fillRect(j, i, 1, 1); // Each cell is 1x1 pixel

        }
      }
    }, [data]);

    return <div style={{ width: "100%", height: "100%", aspectRatio: 1, display: "flex", flexDirection: "row", justifyContent: "center" }}>
      <canvas ref={canvasRef} width={width} height={height}

      ></canvas>
    </div>
  })
