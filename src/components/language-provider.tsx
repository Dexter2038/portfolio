import React, { createContext, useContext, useEffect, useState } from "react"

type Language = "russian" | "english" | "system"

type LanguageProviderProps = {
  children: React.ReactNode
  defaultLanguage?: Language
  storageKey?: string
}

type LanguageProviderState = {
  language: Language
  setLanguage: (language: Language) => void
}

const initialState: LanguageProviderState = {
  language: "system",
  setLanguage: () => null
}

const LanguageProviderContext = createContext<LanguageProviderState>(initialState)

export function LanguageProvider({
  children,
  defaultLanguage = "system",
  storageKey = "vite-ui-language",
  ...props
}: LanguageProviderProps) {
  const [language, setLanguage] = useState<Language>(
    () => (localStorage.getItem(storageKey) as Language) || defaultLanguage
  )

  useEffect(() => {
    // Do something with language
  }, [language])

  const value = {
    language,
    setLanguage: (language: Language) => {
      localStorage.setItem(storageKey, language)
      setLanguage(language)
    },
  }

  return (
    <LanguageProviderContext.Provider {...props} value={value}>
      {children}
    </LanguageProviderContext.Provider>
  )
}


export const useLanguage = () => {
  const context = useContext(LanguageProviderContext)

  if (context === undefined)
    throw new Error("useLanguage must be used within a ThemeProvider")

  return context
}

