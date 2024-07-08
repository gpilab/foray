# MathTextShape
## [MathTextTool](./MathShapeTool.tsx)
Handles Shape creation

- [x] toolbar item
- [x] custom icon
- [x] item should be placed next to text tool
## [MathShapeUtil](./MathShapeUtil.tsx)
Handles shape state and behaviour

## States and Behavior
Behavior should match default text and shape behavior as closely as possible, so that intuition/muscle memory can be 
carried over
### Idle

```
select.idle
editor.selection == shape.id
```

Visual
- [x] rendered equation

Behavior
- [x] Can be drag and dropped
- [x] Double click enters editing, all text selected
### Selected
```
select.idle
editor.selection == shape.id
```

Visual
- [x] rendered equation
- [x] resize handles
- [x] bounds are determined dynamically, and accurately surround the equation

Behavior
- [x] Can be drag and dropped
- [x] Single click enters editing cursor at end of text
- [x] Double click enters editing, selects all text
### Editing

```
select.editing_shape
editor.editing_shape == shape.id
```

Visual
- [x] equation
- [x] src editor text above

Behavior
- [x] text input is focused
- [x] Double click on src text selects all
- [x] Double click on shape selects all 
- [x] On new shape creation, Editing mode should be entered
    - [x] All text should be selected
    - [x] On shape duplication, shape should just be selected
- [x] Clicking on other editable shape immediately jumps to insert mode
    - [x] Double Click selects all text
- [ ] "Enter" exits edit mode and goes to select mode


- [?] Can be drag and dropped when render equation is dragged
    - Compare how built in shapes handle this
    - Frame object doesn't do this, so maybe it's not really necessary
- [?] Multi line edits?
    - [?] new line = rendered line break?

## Styles
Color 
- [x] Color of equation
    - [x] also changes src text color
- [x] size categories, match textShape size
- [?] text alignemnt (left, center, right). Right now, width is fixed to the minimum size. 
    - This would make using alignment options like "distribute horizontally" work better. right now it just pushes the text all the way to the right.
- [?] custom hand written rendering style?


## Misc Features
- [?] export to svg
- [?] copy as latex source
- [?] pasting latex source creates new shape
- [?] on syntax errors, src text is shown in red
- [?] show specific syntax error messages to aid user debugging

## Implementation Details

- [x] text input as a component associated with the shape
    - previously, a global component was being moved around to appear next to each shape
    - this worked fine, but is messy. It increases state sharing, and is clunky to react to state changes


