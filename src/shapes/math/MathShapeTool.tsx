import { StateNode } from 'tldraw'
import { Idle } from './states/Idle'
import { Pointing } from './states/Pointing'

export class MathShapeTool extends StateNode {
  static override id = 'math-text'
  static override initial = 'idle'
  static override children = () => [Idle, Pointing]
  override shapeType = 'math-text'
}
