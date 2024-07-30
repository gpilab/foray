import { useState } from "react"
import { track, useEditor } from "tldraw"
import { useTheme } from "../../util/useTheme"

export const NumericInput = track((
  props: {
    value: number,
    setValue: (value: number) => void,
    validator?: (value: string) => boolean
    textAlign: "start" | "end" | "center"
  }) => {
  const { value, setValue, validator = () => (true), textAlign } = props
  const editor = useEditor()
  const theme = useTheme()

  const [unvalidatedValue, setUnvalidatedState] = useState(value?.toString())
  const [inputError, setInputError] = useState(false)

  const handleOnChange: React.ChangeEventHandler<HTMLInputElement> = (e) => {
    const amount = e.target.value
    setUnvalidatedState(amount)
    if (amount !== "" && amount.match(/^-{0,1}\d*(\.\d*)?$/)) {
      if (validator(amount)) {
        setInputError(false)
        setValue(parseFloat(amount))
        return
      }
    }
    setInputError(true)
  }


  return < div style={{ padding: "5px", paddingBottom: "9px" }}>
    <div>
      < input style={{
        width: "100%",
        fontSize: "22px",
        pointerEvents: "all",
        textAlign: textAlign,
        border: "none",
        color: theme["text-high-contrast"],
        textDecoration: inputError ? "underline" : "",
        backgroundColor: unvalidatedValue?.toString().length < 1 ? theme.red : "transparent",
        textDecorationColor: inputError ? theme.red : theme.text,
        borderRadius: "5px"
      }}
        autoComplete="off"
        value={unvalidatedValue}
        onClick={(_) => { editor.cancelDoubleClick() }}
        onChange={handleOnChange}>
      </input >
    </div>
  </div >

})
