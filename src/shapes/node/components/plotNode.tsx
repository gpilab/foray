import { SVGContainer, track } from "tldraw"
import { useNodeConfig } from "./NodeContent"
import { Plot, PolyLine } from "../../util/svg_util"
import { linspace } from "../../util/array"
import { useTheme } from "../../util/useTheme"


export const PlotNode = track(
  () => {
    const theme = useTheme()
    const { width, height, inputs } = useNodeConfig()
    const value = inputs.a.value as number[]
    const worldWidth = width / 100 + 1
    const worldHeight = worldWidth * height / width


    const dataWidth = 20
    const x = linspace(-dataWidth / 2, dataWidth / 2, value?.length)

    return <SVGContainer >
      <svg id="plot-svg"
        viewBox={`${-worldWidth / 2} ${-worldHeight / 2} ${worldWidth} ${worldHeight}`}
        strokeWidth={worldWidth / 80}
      >
        <g strokeWidth={worldWidth / 200} >
          <Plot color={theme.black} worldWidth={dataWidth} worldHeight={dataWidth * height / width} />
        </g>
        {value === undefined ? ""
          :
          <g stroke={theme.blue}
            transform="scale(1,-1)"
            fill={"none"}
            strokeLinejoin="round">
            <PolyLine points={value.map((y, i) => ({ x: x[i], y: y }))} />
          </g>
        }
      </svg>
    </SVGContainer>
  })
