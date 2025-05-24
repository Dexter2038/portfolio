import React from "react";
import { ThemeProvider } from "@/components/theme-provider"
import { LanguageProvider } from "@/components/language-provider"

function App() {
  return (
    <ThemeProvider>
      <LanguageProvider>
        <h1>Empty</h1>
      </LanguageProvider>
    </ThemeProvider >
  )
}

export default App;
