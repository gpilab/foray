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
  TLComponents,
  TLUiAssetUrlOverrides,
  TLUiOverrides,
  TldrawUiMenuItem,
  useIsToolSelected,
  useTools,
} from 'tldraw'

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
    tools.nodeShape = {
      id: 'node',
      icon: 'pi-symbol',
      label: 'Node',
      kbd: 'n',
      onSelect: () => {
        editor.setCurrentTool('node')
      },
    }
    return tools
  },
}

export const components: TLComponents = {
  Toolbar: (props) => {
    const tools = useTools()
    const isMathSelected = useIsToolSelected(tools['mathText'])
    const isNodeSelected = useIsToolSelected(tools['nodeShape'])

    return (
      <DefaultToolbar {...props}>
        <SelectToolbarItem />
        <HandToolbarItem />
        <DrawToolbarItem />
        <EraserToolbarItem />
        <ArrowToolbarItem />
        <TextToolbarItem />
        <TldrawUiMenuItem {...tools['mathText']} isSelected={isMathSelected} />
        <TldrawUiMenuItem {...tools['nodeShape']} isSelected={isNodeSelected} />
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
        <TldrawUiMenuItem {...tools['nodeShape']} />
        <DefaultKeyboardShortcutsDialogContent />
      </DefaultKeyboardShortcutsDialog>
    )
  },
}
