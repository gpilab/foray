import {
  BindingOnDeleteOptions,
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
  //cleanup wire shape when binding is deleted.
  onAfterDelete(options: BindingOnDeleteOptions<WireBinding>): void {
    this.editor.deleteShape(options.binding.fromId)
  }
}
