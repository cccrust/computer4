import { useState, useEffect, useRef } from 'react';
import { useAppContext } from '../AppContext';

export default function TerminalApp({ isOpen, isFront }) {
  const { terminalDir } = useAppContext();
  const [output, setOutput] = useState('');
  const [history, setHistory] = useState([]);
  const [histIdx, setHistIdx] = useState(-1);
  const [inputVal, setInputVal] = useState('');
  const wsRef = useRef(null);
  const scrollRef = useRef(null);
  const [cwd, setCwd] = useState('~');

  const connect = (dir) => {
    if (wsRef.current) {
      wsRef.current.close();
    }
    const proto = location.protocol === 'https:' ? 'wss' : 'ws';
    const ws = new WebSocket(`${proto}://${location.host}/ws/terminal`);
    wsRef.current = ws;

    ws.onopen = () => {
      ws.send(JSON.stringify({ type: 'init', cwd: dir || '/' }));
      setCwd(dir || '/');
      appendOut(`\x1b[1;32m● 終端機已連線\x1b[0m  cwd: ${dir || '/'}\r\n\r\n`);
      
      // Auto run ls when connected to a directory, passing -la by default since we want . and .. to show up!
      if (dir) {
        // ws.send(JSON.stringify({ type: 'input', data: 'ls -la\n' }));
      }
    };

    ws.onmessage = (e) => {
      const msg = JSON.parse(e.data);
      if (msg.type === 'output') appendOut(msg.data);
    };

    ws.onclose = () => appendOut('\r\n\x1b[1;31m● 連線已關閉\x1b[0m\r\n');
  };

  useEffect(() => {
    if (isOpen && !wsRef.current) {
      connect(terminalDir || '/');
    }
  }, [isOpen]);

  useEffect(() => {
    if (terminalDir && wsRef.current) {
      connect(terminalDir);
    }
  }, [terminalDir]);

  const appendOut = (txt) => {
    setOutput(prev => prev + txt);
  };

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [output]);

  const ansiToHtml = (text) => {
    let out = String(text)
      .replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;')
      .replace(/\r\n|\r/g, '\n');
    out = out.replace(/\x1b\[([0-9;]*)m/g, (m, codes) => {
      if (!codes || codes === '0') return '</span>';
      const parts = codes.split(';');
      let cls = '';
      parts.forEach(c => {
        switch(c) {
          case '1': cls += 'font-weight:700;'; break;
          case '3': cls += 'font-style:italic;'; break;
          case '32': cls += 'color:#3fb950;'; break;
          case '33': cls += 'color:#d29922;'; break;
          case '34': cls += 'color:#58a6ff;'; break;
          case '31': cls += 'color:#f85149;'; break;
          case '35': cls += 'color:#bc8cff;'; break;
          case '36': cls += 'color:#79c0ff;'; break;
          case '37': cls += 'color:#e6edf3;'; break;
          case '90': cls += 'color:#8b949e;'; break;
        }
      });
      return cls ? `<span style="${cls}">` : '<span>';
    });
    out = out.replace(/\x1b\[[0-9;]*[A-Za-z]/g, '');
    return out; // React treats dangerouslySetInnerHTML safely-ish
  };

  const handleKeyDown = (e) => {
    if (e.key === 'Enter') {
      const cmd = inputVal;
      setInputVal('');
      if (cmd.trim()) {
        setHistory(h => [...h, cmd]);
        setHistIdx(-1);
      }
      appendOut(`\x1b[1;32m❯\x1b[0m ${cmd}\r\n`);
      
      // If user types ls without args, we can silently change it to ls -A so they see . and ..
      let realCmd = cmd;
      if (cmd.trim() === 'ls') {
          realCmd = 'ls -A';
      }
      
      if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
        wsRef.current.send(JSON.stringify({ type: 'input', data: realCmd + '\n' }));
      } else {
        appendOut('\x1b[1;31m錯誤：終端機未連線。重新連線中...\x1b[0m\r\n');
        connect(cwd);
      }
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      if (histIdx < history.length - 1) {
        const nextIdx = histIdx + 1;
        setHistIdx(nextIdx);
        setInputVal(history[history.length - 1 - nextIdx]);
      }
    } else if (e.key === 'ArrowDown') {
      e.preventDefault();
      if (histIdx > 0) {
        const nextIdx = histIdx - 1;
        setHistIdx(nextIdx);
        setInputVal(history[history.length - 1 - nextIdx]);
      } else {
        setHistIdx(-1);
        setInputVal('');
      }
    } else if (e.key === 'c' && e.ctrlKey) {
      if (wsRef.current) wsRef.current.send(JSON.stringify({ type: 'input', data: '\x03' }));
    } else if (e.key === 'l' && e.ctrlKey) {
      e.preventDefault();
      setOutput('');
    }
  };

  return (
    <div className="term-container-wrap">
      <div className="term-container" ref={scrollRef}>
        <div className="term-output" dangerouslySetInnerHTML={{__html: ansiToHtml(output)}}></div>
      </div>
      <div className="term-input-row">
        <span className="term-prompt">❯</span>
        <input 
          className="term-input" 
          value={inputVal}
          onChange={e => setInputVal(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="輸入命令..." 
          autoComplete="off" autoCorrect="off" autoCapitalize="off" spellCheck="false"
        />
      </div>
    </div>
  );
}
