import { useState, useRef } from 'react'
import { Globe, ArrowLeft, ArrowRight, RotateCw, Home, X } from 'lucide-react'

export default function Browser() {
  const [url, setUrl] = useState('https://example.com')
  const [inputUrl, setInputUrl] = useState('https://example.com')
  const [history, setHistory] = useState<string[]>(['https://example.com'])
  const [historyIndex, setHistoryIndex] = useState(0)
  const [canGoBack, setCanGoBack] = useState(false)
  const [canGoForward, setCanGoForward] = useState(false)
  const [title, setTitle] = useState('Example Domain')
  const iframeRef = useRef<HTMLIFrameElement>(null)

  const navigate = (newUrl: string) => {
    let finalUrl = newUrl.trim()
    if (!finalUrl.startsWith('http://') && !finalUrl.startsWith('https://')) {
      if (finalUrl.includes('.') && !finalUrl.includes(' ')) {
        finalUrl = 'https://' + finalUrl
      } else {
        finalUrl = `https://www.google.com/search?q=${encodeURIComponent(finalUrl)}&igu=1`
      }
    }

    const newHistory = [...history.slice(0, historyIndex + 1), finalUrl]
    setHistory(newHistory)
    setHistoryIndex(newHistory.length - 1)
    setUrl(finalUrl)
    setInputUrl(finalUrl)
    updateNavState(newHistory.length - 1, newHistory.length)
  }

  const updateNavState = (idx: number, total: number) => {
    setCanGoBack(idx > 0)
    setCanGoForward(idx < total - 1)
  }

  const goBack = () => {
    if (historyIndex > 0) {
      const newIdx = historyIndex - 1
      const prevUrl = history[newIdx]
      setHistoryIndex(newIdx)
      setUrl(prevUrl)
      setInputUrl(prevUrl)
      updateNavState(newIdx, history.length)
    }
  }

  const goForward = () => {
    if (historyIndex < history.length - 1) {
      const newIdx = historyIndex + 1
      const nextUrl = history[newIdx]
      setHistoryIndex(newIdx)
      setUrl(nextUrl)
      setInputUrl(nextUrl)
      updateNavState(newIdx, history.length)
    }
  }

  const reload = () => {
    if (iframeRef.current) {
      iframeRef.current.src = url
    }
  }

  const goHome = () => {
    navigate('https://example.com')
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    navigate(inputUrl)
  }

  return (
    <div className="flex flex-col h-full bg-white">
      {/* Toolbar */}
      <div className="flex items-center gap-1 px-2 py-1.5 bg-gray-100 border-b border-gray-200">
        <button
          onClick={goBack}
          disabled={!canGoBack}
          className={`p-1.5 rounded ${canGoBack ? 'hover:bg-gray-200 text-gray-700' : 'text-gray-300 cursor-not-allowed'}`}
        >
          <ArrowLeft size={16} />
        </button>
        <button
          onClick={goForward}
          disabled={!canGoForward}
          className={`p-1.5 rounded ${canGoForward ? 'hover:bg-gray-200 text-gray-700' : 'text-gray-300 cursor-not-allowed'}`}
        >
          <ArrowRight size={16} />
        </button>
        <button onClick={reload} className="p-1.5 rounded hover:bg-gray-200 text-gray-700">
          <RotateCw size={16} />
        </button>
        <button onClick={goHome} className="p-1.5 rounded hover:bg-gray-200 text-gray-700">
          <Home size={16} />
        </button>
        <form onSubmit={handleSubmit} className="flex-1 mx-1">
          <div className="flex items-center gap-1 bg-white rounded-full px-3 py-1.5 border border-gray-300 focus-within:border-blue-500 focus-within:ring-1 focus-within:ring-blue-500">
            <Globe size={14} className="text-gray-400 flex-shrink-0" />
            <input
              type="text"
              value={inputUrl}
              onChange={(e) => setInputUrl(e.target.value)}
              className="flex-1 text-xs bg-transparent outline-none text-gray-700"
              placeholder="Search or enter URL"
            />
          </div>
        </form>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-hidden">
        <iframe
          ref={iframeRef}
          src={url}
          className="w-full h-full border-none"
          title={title}
          sandbox="allow-scripts allow-same-origin allow-forms allow-popups"
        />
      </div>
    </div>
  )
}