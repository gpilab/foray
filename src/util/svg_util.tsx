import { useNodeConfig } from "../shapes/node/components/NodeContent"
import { linspace } from "./array"

type Point = { x: number, y: number }

export function PolyLine(props: { points: Point[], closed?: boolean }) {
  const { points, closed } = props
  return <path d={points.map(
    (p, i) =>
      `${i == 0 ? "M" : "L"} ${p.x} ${p.y}`)
    .join(" ")
    .concat(closed ? "Z" : "")
  } />
}

export function Plot(props: { color: string, scale: number }) {
  const { scale = 1 } = props
  return <g stroke={props.color}>
    <Axis scale={scale} />
    <GridLines scale={scale} />
    <g transform="rotate(90)">
      <Axis scale={scale} />
      <GridLines scale={scale} />
    </g>
  </g>
}

function Axis(props: { scale: number }) {
  const { width: containerWidth, height: containerHeight } = useNodeConfig()
  const { scale } = props

  const xTicks = getSymetricTicks(containerWidth, scale)
  const tickHeight = scale / 10
  return <g >
    <path d={`M ${-containerHeight / 2} 0 L ${containerHeight / 2} 0`} />
    {xTicks.map((x, i) => <path key={i} d={`M ${x} ${-tickHeight} L ${x} ${tickHeight}`} />)}
  </g>
}

function GridLines(props: { scale: number }) {
  const { width: containerWidth, height: containerHeight } = useNodeConfig()
  const { scale } = props

  const xTicks = getSymetricTicks(containerWidth, scale)

  return <g opacity={.2}>
    <path d={`M ${-containerHeight / 2} 0 L ${containerHeight / 2} 0`} />
    {xTicks.map((x, i) => <path key={i} d={`M ${x} ${-containerHeight} L ${x} ${containerHeight}`} />)}
  </g>
}

function getSymetricTicks(width: number, scale: number) {
  const container_end = (Math.ceil(width / 2))
  const end = container_end % 2 == 0 ? container_end : container_end + 1
  const start = -end
  const numTicks = Math.ceil((end - start) / scale) + 1

  return linspace(start, end, numTicks)
}
