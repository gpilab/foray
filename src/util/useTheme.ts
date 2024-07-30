import { getDefaultColorTheme, useIsDarkMode } from "tldraw"

export const useTheme = () => {
  const theme = getDefaultColorTheme({ isDarkMode: useIsDarkMode() })
  return {
    id: theme.id,
    background: theme.background,
    text: theme.id == "dark" ? "#cccccc" : "#444444",//theme.text,
    "text-high-contrast": theme.id == "dark" ? "#dddddd" : "#333333",//theme.text,
    black: theme.black.solid,
    white: theme.white.solid,
    grey: theme.grey.solid,
    "light-violet": theme["light-violet"].solid,
    "violet": theme["violet"].solid,
    "light-blue": theme["light-blue"].solid,
    blue: theme["blue"].solid,
    yellow: theme["yellow"].solid,
    orange: theme["orange"].solid,
    "light-green": theme["light-green"].solid,
    green: theme["green"].solid,
    "light-red": theme["light-red"].solid,
    red: theme["red"].solid,
  }
}
