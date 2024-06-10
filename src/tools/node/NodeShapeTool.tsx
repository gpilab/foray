import { BaseBoxShapeTool } from 'tldraw'

export class NodeShapeTool extends BaseBoxShapeTool {
  static override id = 'node'
  static override initial = 'idle'
  override shapeType = 'node'
}
