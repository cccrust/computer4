import { useAppContext } from './AppContext';
import FileExplorer from './components/FileExplorer';
import TerminalApp from './components/TerminalApp';
import Editor from './components/Editor';
import BrowserApp from './components/BrowserApp';

export default function Apps() {
  const { activeWindows, currentWindow, closeApp } = useAppContext();

  const renderWindow = (id, Component, title, icon) => {
    const isOpen = activeWindows.has(id);
    const isFront = currentWindow === id;
    
    // An iOS like window on mobile: full screen. On desktop: floating
    return (
      <div 
        className={`window ${isOpen ? 'open' : ''} ${isFront ? 'front' : ''} app-window`}
        id={`win-${id}`}
      >
        <div className="win-titlebar">
          <div className="win-title">
            <span className="win-title-icon">{icon}</span>
            <span>{title}</span>
          </div>
          <div className="win-controls">
            <button className="win-btn close" onClick={() => closeApp(id)}>✕</button>
          </div>
        </div>
        <div className="win-body">
          <Component isOpen={isOpen} isFront={isFront} />
        </div>
      </div>
    );
  };

  return (
    <>
      {renderWindow('files', FileExplorer, '檔案總管', '📁')}
      {renderWindow('terminal', TerminalApp, '終端機', '💻')}
      {renderWindow('editor', Editor, '編輯器', '📝')}
      {renderWindow('browser', BrowserApp, '瀏覽器', '🌐')}
    </>
  );
}
