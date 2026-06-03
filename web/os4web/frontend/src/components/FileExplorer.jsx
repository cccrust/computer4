import { useState, useEffect } from 'react';
import { useAppContext } from '../AppContext';

const getFileIcon = (e) => {
  if (e.is_dir) return '📁';
  const ext = (e.name || '').split('.').pop().toLowerCase();
  const icons = {
    rs: '🦀', js: '📜', ts: '📘', jsx: '⚛️', tsx: '⚛️', py: '🐍',
    html: '🌐', css: '🎨', json: '📋', toml: '⚙️', md: '📖', txt: '📄',
    png: '🖼️', jpg: '🖼️',
  };
  return icons[ext] || '📄';
};

export default function FileExplorer({ isOpen, isFront }) {
  const [path, setPath] = useState('/');
  const [history, setHistory] = useState([]);
  const [entries, setEntries] = useState([]);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(false);
  const { setEditorFile, openApp, terminalDir, setTerminalDir } = useAppContext();
  
  const [menuFor, setMenuFor] = useState(null); // Which entry has menu open

  const navigateTo = async (newPath) => {
    if (!newPath) newPath = '/';
    setLoading(true);
    setError(null);
    try {
      const res = await fetch('/api/fs/list?path=' + encodeURIComponent(newPath));
      const data = await res.json();
      if (!data.ok) throw new Error(data.error);
      setPath(newPath);
      setEntries(data.data);
      localStorage.setItem('lastPath', newPath);
    } catch (e) {
      setError(e.message);
      setEntries([]);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (isOpen && entries.length === 0) {
      navigateTo(localStorage.getItem('lastPath') || '/');
    }
  }, [isOpen]);

  const handleBack = () => {
    if (history.length > 0) {
      const prev = history[history.length - 1];
      setHistory(h => h.slice(0, -1));
      navigateTo(prev);
    }
  };

  const handleHome = () => {
    setHistory(h => [...h, path]);
    navigateTo('/');
  };

  const itemClick = (e) => {
    if (e.is_dir) {
      setHistory(h => [...h, path]);
      navigateTo(e.path);
    } else {
      setEditorFile(e.path);
      openApp('editor');
    }
  };

  const formatSize = (b) => {
    if (b < 1024) return b + 'B';
    if (b < 1024*1024) return (b/1024).toFixed(1) + 'KB';
    return (b/1024/1024).toFixed(1) + 'MB';
  };
  
  const ctxTerm = (e) => {
    const dir = e.is_dir ? e.path : (e.path.substring(0, e.path.lastIndexOf('/')) || '/');
    setTerminalDir(dir);
    openApp('terminal');
    setMenuFor(null);
  };
  
  const ctxDel = async (e) => {
    if (!confirm(`確定要刪除「${e.name}」嗎？`)) return;
    try {
      await fetch('/api/fs/delete', { method: 'POST', headers: {'Content-Type':'application/json'}, body: JSON.stringify({path: e.path}) });
      navigateTo(path);
    } catch(err) {
      alert("Delete failed: " + err);
    }
    setMenuFor(null);
  };

  return (
    <div className="fe-container">
      <div className="fe-toolbar">
        <button className="fe-back-btn" onClick={handleBack} disabled={history.length === 0}>‹</button>
        <button className="fe-home-btn" onClick={handleHome}>⌂</button>
        <div className="fe-path-bar">{path}</div>
        <button className="fe-new-btn" onClick={() => {
          const name = prompt("Enter new name (with extension for file):");
          if (!name) return;
          const isDir = name.indexOf('.') === -1;
          const p = (path.endsWith('/') ? path : path + '/') + name;
          if (isDir) {
            fetch('/api/fs/mkdir', { method:'POST', headers:{'Content-Type':'application/json'}, body:JSON.stringify({path:p})}).then(()=>navigateTo(path));
          } else {
            fetch('/api/fs/write', { method:'POST', headers:{'Content-Type':'application/json'}, body:JSON.stringify({path:p, content:''})}).then(()=>navigateTo(path));
          }
        }}>+</button>
      </div>

      <div className="fe-list">
        {loading && <div className="fe-loading">載入中...</div>}
        {error && <div className="fe-empty">⚠️ {error}</div>}
        {!loading && !error && entries.length === 0 && <div className="fe-empty">空資料夾</div>}
        
        {entries.map(e => (
          <div key={e.path} className="fe-item">
            <div className="fe-item-main" onClick={() => itemClick(e)}>
              <div className="fe-item-icon">{getFileIcon(e)}</div>
              <div className="fe-item-info">
                <div className="fe-item-name">{e.name}</div>
                <div className="fe-item-meta">{!e.is_dir && formatSize(e.size)}</div>
              </div>
            </div>
            
            <button className="fe-menu-btn" onClick={() => setMenuFor(menuFor === e.path ? null : e.path)}>⋮</button>
            
            {menuFor === e.path && (
              <div className="fe-menu-dropdown">
                {!e.is_dir && <div onClick={() => { setEditorFile(e.path); openApp('editor'); setMenuFor(null); }}>📝 編輯</div>}
                <div onClick={() => ctxTerm(e)}>💻 開啟終端機</div>
                <div onClick={() => {
                  const newName = prompt("New name:", e.name);
                  if (newName && newName !== e.name) {
                    const dir = e.path.substring(0, e.path.lastIndexOf('/'));
                    const p = (dir || '') + '/' + newName;
                    fetch('/api/fs/rename', { method:'POST', headers:{'Content-Type':'application/json'}, body:JSON.stringify({from: e.path, to: p})}).then(()=>navigateTo(path));
                  }
                  setMenuFor(null);
                }}>✏️ 重新命名</div>
                <div className="danger" onClick={() => ctxDel(e)}>🗑️ 刪除</div>
              </div>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
