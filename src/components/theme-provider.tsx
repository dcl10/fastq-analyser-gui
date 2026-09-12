"use client";

import * as React from "react";

type Mode = "light" | "dark" | "system";

const KEY = "fastq-analyser.theme";

const ThemeContext = React.createContext<{ mode: Mode; setMode: (m: Mode) => void }>({
  mode: "system",
  setMode: () => {},
});

export function useTheme() {
  return React.useContext(ThemeContext);
}

export function ThemeProvider({ children }: { children: React.ReactNode }) {
  const [mode, setMode] = React.useState<Mode>("system");

  // Static export: read the stored choice after mount, once, so the first client
  // render still matches the server-rendered "system" default and hydration never
  // mismatches.
  React.useEffect(() => {
    const stored = window.localStorage.getItem(KEY) as Mode | null;
    if (stored === "light" || stored === "dark" || stored === "system") {
      // eslint-disable-next-line react-hooks/set-state-in-effect -- one-time restore from localStorage, not a reactive sync
      setMode(stored);
    }
  }, []);

  React.useEffect(() => {
    window.localStorage.setItem(KEY, mode);
    const mq = window.matchMedia("(prefers-color-scheme: dark)");
    const apply = () => {
      const dark = mode === "dark" || (mode === "system" && mq.matches);
      document.documentElement.classList.toggle("dark", dark);
    };
    apply();
    mq.addEventListener("change", apply);
    return () => mq.removeEventListener("change", apply);
  }, [mode]);

  return <ThemeContext.Provider value={{ mode, setMode }}>{children}</ThemeContext.Provider>;
}
