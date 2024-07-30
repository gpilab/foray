import { SVGContainer, track } from "tldraw"
import { useNodeConfig } from "./NodeContent"
import { Plot, PolyLine } from "../../../util/svg_util"
import { linspace } from "../../../util/array"
import { useTheme } from "../../../util/useTheme"


export const PlotNode = track(
  () => {
    const theme = useTheme()
    const { inputs, color } = useNodeConfig()
    const value = inputs.a.value as number[]

    const dataWidth = 4
    const dataHeight = 3
    const x = linspace(-dataWidth / 2, dataWidth / 2, value?.length)

    return <SVGContainer >
      <svg id="plot-svg"
        viewBox={`${-dataWidth / 2} ${-dataHeight / 2} ${dataWidth} ${dataHeight}`}
        strokeWidth={.03}
      >
        <g strokeWidth={.005} >
          <Plot color={theme.black} scale={1} />
        </g>
        {value === undefined ? ""
          : <g stroke={theme[color] ?? theme["blue"]}
            transform="scale(1,-1)"
            fill={"none"}
            strokeLinejoin="round">
            <PolyLine points={value.map((y, i) => ({ x: x[i], y: y }))} />
          </g>
        }
      </svg>
    </SVGContainer>
  })
