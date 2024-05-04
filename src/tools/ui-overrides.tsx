import {
  ArrowDownToolbarItem,
  ArrowLeftToolbarItem,
  ArrowRightToolbarItem,
  ArrowToolbarItem,
  ArrowUpToolbarItem,
  AssetToolbarItem,
  CheckBoxToolbarItem,
  CloudToolbarItem,
  DefaultToolbar,
  DiamondToolbarItem,
  DrawToolbarItem,
  EllipseToolbarItem,
  EraserToolbarItem,
  FrameToolbarItem,
  HandToolbarItem,
  HexagonToolbarItem,
  HighlightToolbarItem,
  LaserToolbarItem,
  LineToolbarItem,
  NoteToolbarItem,
  OvalToolbarItem,
  RectangleToolbarItem,
  RhombusToolbarItem,
  SelectToolbarItem,
  StarToolbarItem,
  TextToolbarItem,
  TrapezoidToolbarItem,
  TriangleToolbarItem,
  XBoxToolbarItem,
} from 'tldraw'

import {
  DefaultKeyboardShortcutsDialog,
  DefaultKeyboardShortcutsDialogContent,
  LABEL_FONT_SIZES,
  TLComponents,
  TLUiAssetUrlOverrides,
  TLUiOverrides,
  TldrawUiMenuItem,
  getDefaultColorTheme,
  stopEventPropagation,
  track,
  useEditor,
  useIsToolSelected,
  useTools,
} from 'tldraw'
import { MathTextShape } from './math/MathShapeUtil'

export const MyComponentInFront = track(function MyComponent() {
  const editor = useEditor()
  const selectionRotatedPageBounds = editor.getSelectionRotatedPageBounds()
  if (!selectionRotatedPageBounds) return null

  const shape = editor.getEditingShape()

  if (shape == null || shape.type != 'math-text') return null

  const theme = getDefaultColorTheme({ isDarkMode: editor.user.getIsDarkMode() })

  const mShape = shape as MathTextShape
  const text = mShape.props.text
  const id = mShape.id
  const type = mShape.type
  const color = mShape.props.color
  const font_size = mShape.props.sizeStyle
  const font_height = LABEL_FONT_SIZES[font_size]


  const pageCoordinates = editor.pageToViewport(selectionRotatedPageBounds.point)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!mShape) return null

    const new_text = e.currentTarget.value
    if (text === new_text) return null

    editor.updateShapes([
      {
        id,
        type: type,
        props: { text: new_text },
      },
    ])
  }
  function handleClick(e: React.MouseEvent<HTMLInputElement>) {
    (e.target as HTMLInputElement).select()
    stopEventPropagation(e)
  }
  return (
    <div
      style={{
        pointerEvents: 'all',
        position: 'absolute',
        top: Math.max(32, pageCoordinates.y - (font_height + 20)),
        left: Math.max(32, pageCoordinates.x - 20),
      }}
      onPointerDown={stopEventPropagation}>
      <input id='math-text-input'
        value={mShape.props.text}
        style={{
          height: 'auto',
          width: Math.max(5, text.length + 3) + 'ch',
          color: theme[color].solid,
          fontSize: font_height,
          backgroundColor: 'transparent',
          border: 'none',
          padding: '5px 20px 5px 20px',
        }}
        onChange={handleChange}
        onDoubleClick={(e) => handleClick(e)}
      />
    </div >
  )
})

export const customAssetURLs: TLUiAssetUrlOverrides = {
  icons: {
    'pi-symbol': 'pi-symbol.svg',
  }
}

export const uiOverrides: TLUiOverrides = {
  tools(editor, tools) {
    tools.mathText = {
      id: 'math-text',
      icon: 'pi-symbol',
      label: 'Math',
      kbd: 'm',
      onSelect: () => {
        editor.setCurrentTool('math-text')
      },
    }
    return tools
  },
}

export const components: TLComponents = {
  InFrontOfTheCanvas: MyComponentInFront,
  Toolbar: (props) => {
    const tools = useTools()
    const isMathSelected = useIsToolSelected(tools['mathText'])

    return (
      <DefaultToolbar {...props}>
        <SelectToolbarItem />
        <HandToolbarItem />
        <DrawToolbarItem />
        <EraserToolbarItem />
        <ArrowToolbarItem />
        <TextToolbarItem />
        <TldrawUiMenuItem {...tools['mathText']} isSelected={isMathSelected} />
        <RectangleToolbarItem />
        <LineToolbarItem />
        <TriangleToolbarItem />
        <EllipseToolbarItem />
        <OvalToolbarItem />
        <DiamondToolbarItem />
        <TrapezoidToolbarItem />
        <RhombusToolbarItem />
        <HexagonToolbarItem />
        <CloudToolbarItem />
        <StarToolbarItem />
        <XBoxToolbarItem />
        <CheckBoxToolbarItem />
        <ArrowLeftToolbarItem />
        <ArrowUpToolbarItem />
        <ArrowDownToolbarItem />
        <ArrowRightToolbarItem />
        <HighlightToolbarItem />
        <NoteToolbarItem />
        <AssetToolbarItem />
        <FrameToolbarItem />
        <LaserToolbarItem />
      </DefaultToolbar>
    )
  },
  KeyboardShortcutsDialog: (props) => {
    const tools = useTools()
    return (
      <DefaultKeyboardShortcutsDialog {...props}>
        <TldrawUiMenuItem {...tools['mathText']} />
        <DefaultKeyboardShortcutsDialogContent />
      </DefaultKeyboardShortcutsDialog>
    )
  },
}
