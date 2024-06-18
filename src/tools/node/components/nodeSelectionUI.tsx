import { useState } from 'react';
import Select, { SingleValue } from 'react-select';
import { stopEventPropagation, track, useEditor } from 'tldraw';
type NodeOptions = { value: string, label: string }
const options: NodeOptions[] =
  [
    { value: 'constant', label: 'Constant' },
    { value: 'add', label: 'Add' },
    { value: 'subtract', label: 'Subtract' },
    { value: 'multiply', label: 'Multiply' },
  ];

export const NodeSelectionUI = track(() => {
  const editor = useEditor()
  const [selectedOption, setSelectedOption] = useState<SingleValue<NodeOptions> | null>(null);

  if (!editor.isIn('node')) {
    return
  }

  return (
    <div style={{
      width: "200px",
      position: "absolute",
      left: "50%",
      top: "50%",
      backgroundColor: "grey",
      pointerEvents: "all"
    }}
      onPointerDown={stopEventPropagation}>
      <Select
        styles={{
          option: (baseStyles) => ({ ...baseStyles, color: "black" })
        }}
        defaultValue={selectedOption}
        onChange={(e) => {
          console.log(e)
          setSelectedOption(e)
          editor.setCurrentTool('node.idle', e?.value)
        }}
        options={options}

      />
    </div>
  );
})
