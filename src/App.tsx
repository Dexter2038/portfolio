import { ThemeProvider } from "@/components/theme-provider"
import { LanguageProvider } from "@/components/language-provider"
import { Main } from "@/components/main"

function App() {
  return (
    <ThemeProvider>
      <LanguageProvider>
        <Main />
      </LanguageProvider>
    </ThemeProvider >
  )
}

export default App;
