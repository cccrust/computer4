import { useState, useEffect } from 'react';
import { useAppContext } from '../AppContext';

const detectLang = (name) => {
  const ext = name.split('.').pop().toLowerCase();
  const langs = {
    rs: 'rust', js: 'javascript', ts: 'typescript', jsx: 'jsx', tsx: 'tsx',
    py: 'python', go: 'go', rb: 'ruby', java: 'java', c: 'c', cpp: 'cpp',
    html: 'html', css: 'css', json: 'json', toml: 'toml', md: 'markdown', 
    sh: 'shell', txt: 'text'
  };
  return langs[ext] || 'text';
};

export default function Editor({ isOpen, isFront }) {
  const { editorFile } = useAppContext();
  const [tabs, setTabs] = useState([]);
  const [activeTab, setActiveTab] = useState(null);

  useEffect(() => {
    if (editorFile) {
      openFile(editorFile);
    }
  }, [editorFile]);

  const openFile = async (path) => {
    const existing = tabs.find(t => t.path === path);
    if (existing) {
      setActiveTab(existing.id);
      return;
    }
    
    try {
      const res = await fetch('/api/fs/read?path=' + encodeURIComponent(path));
      const data = await res.json();
      if (!data.ok) throw new Error(data.error);

      const name = path.split('/').pop();
      const id = 'tab_' + Date.now();
      const tab = {
        id, path, name,
        content: data.data.content,
        modified: false,
        lang: detectLang(name)
      };
      setTabs(t => [...t, tab]);
      setActiveTab(id);
    } catch(e) {
      alert("Failed to load file: " + e.message);
    }
  };

  const closeTab = (id, e) => {
    e.stopPropagation();
    const idx = tabs.findIndex(t => t.id === id);
    if (idx === -1) return;
    const tab = tabs[idx];
    if (tab.modified && !confirm(`「${tab.name}」有未儲存的變更，確定關閉？`)) return;
    
    setTabs(prev => {
      const next = prev.filter(t => t.id !== id);
      if (activeTab === id) {
        if (next.length > 0) {
          setActiveTab(next[Math.max(0, idx - 1)].id);
        } else {
          setActiveTab(null);
        }
      }
      return next;
    });
  };

  const updateContent = (content) => {
    setTabs(prev => prev.map(t => {
      if (t.id === activeTab) {
        return { ...t, content, modified: true };
      }
      return t;
    }));
  };

  const saveFile = async () => {
    const tab = tabs.find(t => t.id === activeTab);
    if (!tab) return;
    try {
      const res = await fetch('/api/fs/write', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({ path: tab.path, content: tab.content })
      });
      const data = await res.json();
      if (!data.ok) throw new Error(data.error);
      
      setTabs(prev => prev.map(t => {
        if (t.id === activeTab) return { ...t, modified: false };
        return t;
      }));
    } catch(e) {
      alert("Save failed: " + e.message);
    }
  };

  const activeTabData = tabs.find(t => t.id === activeTab);
  
  return (
    <div className="editor-container">
      <div className="editor-tabs">
        {tabs.map(t => (
          <div key={t.id} className={`editor-tab ${t.id === activeTab ? 'active' : ''}`} onClick={() => setActiveTab(t.id)}>
            <span className="tab-name">{t.name}{t.modified ? ' •' : ''}</span>
            <button className="tab-close" onClick={(e) => closeTab(t.id, e)}>✕</button>
          </div>
        ))}
      </div>
      
      {activeTabData ? (
        <>
          <div className="editor-toolbar">
            <span className="editor-lang">{activeTabData.lang}</span>
            <button className="editor-btn save" onClick={saveFile}>💾 儲存</button>
            <div style={{flex:1}}></div>
            <span className={`editor-status ${activeTabData.modified ? 'unsaved' : ''}`}>
              {activeTabData.modified ? '● 未儲存' : '已儲存'}
            </span>
          </div>
          <div className="editor-area">
            <textarea
              className="editor-content"
              value={activeTabData.content}
              onChange={e => updateContent(e.target.value)}
              spellCheck="false" autoCorrect="off" autoCapitalize="off"
            />
          </div>
        </>
      ) : (
        <div className="editor-no-file">
          <div className="editor-no-file-icon">📄</div>
          <div className="editor-no-file-text">尚未開啟任何檔案</div>
        </div>
      )}
    </div>
  );
}
