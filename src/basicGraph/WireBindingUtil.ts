import {
  BindingOnShapeDeleteOptions, BindingOnShapeIsolateOptions,
  BindingUtil, TLBaseBinding
} from 'tldraw'


export type WireBinding = TLBaseBinding<'wire', {
  terminal: "start" | "end"
}>

/**
 * Determines how wire should behave when bound shapes change
 */
export class WireBindingUtil extends BindingUtil<WireBinding> {
  static override type = 'wire' as const

  override getDefaultProps() {
    return {
      terminal: "start" as const
    }
  }

  // when the thing we're stuck to is deleted, delete the wire too 
  override onBeforeDeleteToShape({ binding }: BindingOnShapeDeleteOptions<WireBinding>): void {
    this.editor.deleteShape(binding.fromId)
  }

  override onBeforeIsolateFromShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
    this.editor.deleteShape(options.binding.fromId)
  }

  override onBeforeIsolateToShape(options: BindingOnShapeIsolateOptions<WireBinding>): void {
    this.editor.deleteShape(options.binding.fromId)
  }
}
