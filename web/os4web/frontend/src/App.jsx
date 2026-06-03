import { useState, useEffect } from 'react';
import { AppProvider, useAppContext } from './AppContext';
import Apps from './Apps';
import './index.css';

function Taskbar() {
  const { activeWindows, currentWindow, toggleApp, goHome } = useAppContext();
  
  const tasks = [
    { id: 'files', icon: '📁', label: '檔案' },
    { id: 'terminal', icon: '💻', label: '終端機' },
    { id: 'editor', icon: '📝', label: '編輯器' },
    { id: 'browser', icon: '🌐', label: '瀏覽器' }
  ];

  return (
    <div id="taskbar">
      {tasks.map(t => (
        <button 
          key={t.id} 
          className={`task-btn ${activeWindows.has(t.id) ? 'active' : ''} ${currentWindow === t.id ? 'current' : ''}`}
          onClick={() => toggleApp(t.id)}
        >
          <span className="task-icon">{t.icon}</span>
          {/* <span className="task-label">{t.label}</span> */}
        </button>
      ))}
      <button className="task-home-btn" onClick={goHome}>⊞</button>
    </div>
  );
}

function Homescreen() {
  const { openApp, activeWindows } = useAppContext();
  const [dateStr, setDateStr] = useState('');
  
  useEffect(() => {
    const update = () => {
      const now = new Date();
      const days = ['星期日','星期一','星期二','星期三','星期四','星期五','星期六'];
      setDateStr(`${now.getMonth()+1}月${now.getDate()}日　${days[now.getDay()]}`);
    };
    update();
    const timer = setInterval(update, 60000);
    return () => clearInterval(timer);
  }, []);

  // Hide homescreen if any window is open
  const isHidden = activeWindows.size > 0;

  return (
    <div id="homescreen" className={isHidden ? 'hidden' : ''}>
      <div className="home-greeting">歡迎回來 👋</div>
      <div className="home-subtitle">{dateStr}</div>
      <div className="app-grid">
        <div className="app-icon app-files" onClick={() => openApp('files')}>
          <div className="app-icon-inner">📁</div>
          <span className="app-icon-label">檔案</span>
        </div>
        <div className="app-icon app-terminal" onClick={() => openApp('terminal')}>
          <div className="app-icon-inner">💻</div>
          <span className="app-icon-label">終端機</span>
        </div>
        <div className="app-icon app-editor" onClick={() => openApp('editor')}>
          <div className="app-icon-inner">📝</div>
          <span className="app-icon-label">編輯器</span>
        </div>
        <div className="app-icon app-browser" onClick={() => openApp('browser')}>
          <div className="app-icon-inner">🌐</div>
          <span className="app-icon-label">瀏覽器</span>
        </div>
      </div>
    </div>
  );
}

function Statusbar() {
  const [time, setTime] = useState('');
  useEffect(() => {
    const update = () => {
      const now = new Date();
      setTime(now.getHours().toString().padStart(2,'0') + ':' + now.getMinutes().toString().padStart(2,'0'));
    };
    update();
    const timer = setInterval(update, 1000);
    return () => clearInterval(timer);
  }, []);

  return (
    <div id="statusbar">
      <div className="sb-left">
        <span className="sb-badge">os4web</span>
        <span id="sb-hostname">localhost</span>
      </div>
      <div className="sb-right">
        <span>{time}</span>
        <span>🌐</span>
      </div>
    </div>
  );
}

export default function App() {
  return (
    <AppProvider>
      <Statusbar />
      <div id="desktop">
        <Homescreen />
        <Apps />
      </div>
      <Taskbar />
    </AppProvider>
  );
}
