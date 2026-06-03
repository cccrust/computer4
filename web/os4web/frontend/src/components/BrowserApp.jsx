import { useState, useEffect } from 'react';

export default function BrowserApp({ isOpen, isFront }) {
  const [urlInput, setUrlInput] = useState('https://example.com');
  const [currentUrl, setCurrentUrl] = useState('https://example.com');
  
  // Navigation stack (for Back/Forward)
  const [navStack, setNavStack] = useState(['https://example.com']);
  const [navIndex, setNavIndex] = useState(0);

  // Persistent Bookmarks and History
  const [bookmarks, setBookmarks] = useState([]);
  const [globalHistory, setGlobalHistory] = useState([]);
  
  // Views: 'none', 'bookmarks', 'history'
  const [activeView, setActiveView] = useState('none');
  const [useProxy, setUseProxy] = useState(false);

  useEffect(() => {
    const b = localStorage.getItem('os4web_bookmarks');
    if (b) try { setBookmarks(JSON.parse(b)); } catch(e) {}
    
    const h = localStorage.getItem('os4web_history');
    if (h) try { setGlobalHistory(JSON.parse(h)); } catch(e) {}
  }, []);

  const saveToHistory = (url) => {
    let newH = [...globalHistory].filter(x => x.url !== url);
    newH.unshift({ url, time: Date.now() });
    if (newH.length > 50) newH = newH.slice(0, 50);
    setGlobalHistory(newH);
    localStorage.setItem('os4web_history', JSON.stringify(newH));
  };

  const visitUrl = (url) => {
    const newStack = navStack.slice(0, navIndex + 1);
    newStack.push(url);
    setNavStack(newStack);
    setNavIndex(newStack.length - 1);
    
    setCurrentUrl(url);
    setUrlInput(url);
    setActiveView('none');
    saveToHistory(url);
  };

  const navigate = (e) => {
    e.preventDefault();
    let url = urlInput.trim();
    if (url) {
      if (!url.startsWith('http://') && !url.startsWith('https://')) {
        url = 'https://' + url;
      }
      visitUrl(url);
    }
  };

  const navDirect = () => {
    let url = urlInput.trim();
    if (url) {
      if (!url.startsWith('http://') && !url.startsWith('https://')) {
        url = 'https://' + url;
      }
      window.location.href = url;
    }
  };

  const goBack = () => {
    if (navIndex > 0) {
      const idx = navIndex - 1;
      setNavIndex(idx);
      const url = navStack[idx];
      setCurrentUrl(url);
      setUrlInput(url);
      setActiveView('none');
      saveToHistory(url);
    }
  };

  const goForward = () => {
    if (navIndex < navStack.length - 1) {
      const idx = navIndex + 1;
      setNavIndex(idx);
      const url = navStack[idx];
      setCurrentUrl(url);
      setUrlInput(url);
      setActiveView('none');
      saveToHistory(url);
    }
  };

  const toggleBookmark = () => {
    const isB = bookmarks.some(b => b.url === currentUrl);
    let updated;
    if (isB) updated = bookmarks.filter(b => b.url !== currentUrl);
    else updated = [...bookmarks, { title: currentUrl, url: currentUrl }];
    setBookmarks(updated);
    localStorage.setItem('os4web_bookmarks', JSON.stringify(updated));
  };

  const openLink = (url) => {
    visitUrl(url);
  };

  const isBookmarked = bookmarks.some(b => b.url === currentUrl);

  const getEffectiveUrl = () => {
    if (useProxy) {
      return '/api/proxy?url=' + encodeURIComponent(currentUrl);
    }
    return currentUrl;
  };

  const clearHistory = () => {
    setGlobalHistory([]);
    localStorage.setItem('os4web_history', '[]');
  };

  return (
    <div className="browser-container">
      <div className="browser-toolbar">
        <div className="browser-nav-btns">
          <button className="browser-btn hide-text" disabled={navIndex === 0} onClick={goBack} title="上一頁">‹</button>
          <button className="browser-btn hide-text" disabled={navIndex >= navStack.length - 1} onClick={goForward} title="下一頁">›</button>
          <button className="browser-btn hide-text" onClick={() => visitUrl(currentUrl)} title="重新整理">↻</button>
        </div>
        <form onSubmit={navigate} className="browser-address-form">
          <input 
            type="text" 
            className="browser-input" 
            value={urlInput} 
            onChange={(e) => setUrlInput(e.target.value)}
            placeholder="搜尋或輸入網址" 
          />
          <button type="submit" className="browser-btn hide-sm">前往</button>
        </form>
        <button 
          className="browser-btn hide-sm" 
          onClick={navDirect}
          title="在新分頁/整頁開啟，當使用上一頁時，您即可回到原先 os4web 狀態"
        >
          🚀 跳轉
        </button>
        <label className="browser-proxy-label hide-sm" title="透過代理繞過 X-Frame-Options">
          <input type="checkbox" checked={useProxy} onChange={e => setUseProxy(e.target.checked)} />
          代理
        </label>
        <button 
          className={`browser-btn hide-text ${isBookmarked ? 'bookmarked' : ''}`} 
          onClick={toggleBookmark}
          title={isBookmarked ? '移除書籤' : '加入書籤'}
        >
          {isBookmarked ? '★' : '☆'}
        </button>
        <button 
          className={`browser-btn hide-text ${activeView === 'bookmarks' ? 'active-icon' : ''}`} 
          onClick={() => setActiveView(activeView === 'bookmarks' ? 'none' : 'bookmarks')}
          title="書籤"
        >
          🔖
        </button>
        <button 
          className={`browser-btn hide-text ${activeView === 'history' ? 'active-icon' : ''}`} 
          onClick={() => setActiveView(activeView === 'history' ? 'none' : 'history')}
          title="歷史紀錄"
        >
          🕒
        </button>
      </div>
      
      <div className="browser-content-area">
        {activeView === 'bookmarks' && (
          <div className="browser-sidepanel">
            <h3 className="panel-title">我的書籤</h3>
            {bookmarks.length === 0 ? <div className="panel-empty">尚未加入任何書籤</div> : 
              <div className="panel-list">
                {bookmarks.map((b, i) => (
                  <div key={i} className="panel-item" onClick={() => openLink(b.url)}>
                    <div className="panel-icon">🌐</div>
                    <div className="panel-url">{b.url}</div>
                  </div>
                ))}
              </div>
            }
          </div>
        )}

        {activeView === 'history' && (
          <div className="browser-sidepanel">
            <div className="panel-header">
              <h3 className="panel-title">歷史紀錄</h3>
              <button className="browser-btn" onClick={clearHistory}>清除</button>
            </div>
            {globalHistory.length === 0 ? <div className="panel-empty">無歷史紀錄</div> : 
              <div className="panel-list">
                {globalHistory.map((h, i) => (
                  <div key={i} className="panel-item" onClick={() => openLink(h.url)}>
                    <div className="panel-icon">🕒</div>
                    <div className="panel-url">{h.url}</div>
                  </div>
                ))}
              </div>
            }
          </div>
        )}

        {activeView === 'none' && (
          <iframe 
            src={getEffectiveUrl()} 
            className="browser-iframe" 
            title="Browser Content"
            sandbox="allow-same-origin allow-scripts allow-popups allow-forms"
          />
        )}
      </div>
    </div>
  );
}
