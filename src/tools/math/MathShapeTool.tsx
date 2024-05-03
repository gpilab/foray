import { BaseBoxShapeTool } from 'tldraw'

export class MathShapeTool extends BaseBoxShapeTool {
  static override id = 'math-text'
  static override initial = 'idle'
  override shapeType = 'math-text'
}

