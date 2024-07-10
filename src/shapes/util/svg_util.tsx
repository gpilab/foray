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

export function Plot(props: { color: string, worldWidth: number, worldHeight: number }) {
  const { worldWidth, worldHeight } = props
  return <g stroke={props.color}>
    <Axis worldWidth={worldWidth} worldHeight={worldHeight} />
    <GridLines worldWidth={worldWidth} worldHeight={worldHeight} />
    <g transform="rotate(90)">
      <Axis worldWidth={worldWidth} worldHeight={worldHeight} />
      <GridLines worldWidth={worldWidth} worldHeight={worldHeight} />
    </g>
  </g>
}

function Axis(props: { worldWidth: number, worldHeight: number }) {
  const { worldWidth, worldHeight } = props
  const ticks = linspace(-worldWidth / 2, worldWidth / 2,
    Math.ceil(worldWidth)
  )
  const tickHeight = worldWidth / 100
  return <g >
    <path d={`M ${-worldWidth} 0 L ${worldHeight} 0`} />
    {ticks.map((x, i) => <path key={i} d={`M ${x} ${-tickHeight} L ${x} ${tickHeight}`} />)}
  </g>
}

function GridLines(props: { worldWidth: number, worldHeight: number }) {
  const { worldWidth, worldHeight } = props
  const marks = linspace(-worldWidth / 2, worldWidth / 2, Math.ceil(worldWidth))
  return <g opacity={.2}>
    {marks.map((x, i) => <path key={i} d={`M ${x} ${-worldHeight} L ${x} ${worldHeight}`} />)}
  </g>
}

