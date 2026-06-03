import React, { createContext, useContext } from 'react';
import { useSessionState } from './useSessionState';

const AppContext = createContext();

export function AppProvider({ children }) {
  const [activeWindowsArr, setActiveWindowsArr] = useSessionState('os_activeWindows', []);
  const [currentWindow, setCurrentWindow] = useSessionState('os_currentWindow', null);

  const activeWindows = new Set(activeWindowsArr);

  const openApp = (app) => {
    setActiveWindowsArr(prev => {
      if (!prev.includes(app)) return [...prev, app];
      return prev;
    });
    setCurrentWindow(app);
  };
  
  const closeApp = (app) => {
    setActiveWindowsArr(prev => prev.filter(x => x !== app));
    setCurrentWindow(prev => prev === app ? null : prev);
  };

  const toggleApp = (app) => {
    if (activeWindows.has(app)) {
      if (currentWindow === app) {
        closeApp(app);
      } else {
        openApp(app); // Bring to front
      }
    } else {
      openApp(app);
    }
  };

  const goHome = () => {
    setCurrentWindow(null); // Just hide windows
  };

  // Global state for sharing between apps (e.g. opening editor from files)
  const [editorFile, setEditorFile] = useSessionState('os_editorFile', null);
  const [terminalDir, setTerminalDir] = useSessionState('os_terminalDir', null);

  return (
    <AppContext.Provider value={{
      activeWindows, currentWindow, openApp, closeApp, toggleApp, goHome,
      editorFile, setEditorFile,
      terminalDir, setTerminalDir
    }}>
      {children}
    </AppContext.Provider>
  );
}

export const useAppContext = () => useContext(AppContext);
