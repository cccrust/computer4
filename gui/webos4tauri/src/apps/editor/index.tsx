import { useState, useCallback } from 'react'
import { FolderOpen, Save, File, AlertCircle } from 'lucide-react'
import { readTextFile, writeTextFile, readDir } from '@tauri-apps/plugin-fs'
import { documentDir, join } from '@tauri-apps/api/path'

interface FileItem {
  name: string
  isDirectory: boolean
  path: string
}

export default function Editor() {
  const [content, setContent] = useState('')
  const [filename, setFilename] = useState('untitled.txt')
  const [filepath, setFilepath] = useState('')
  const [isDirty, setIsDirty] = useState(false)
  const [showOpen, setShowOpen] = useState(false)
  const [currentPath, setCurrentPath] = useState('')
  const [files, setFiles] = useState<FileItem[]>([])
  const [error, setError] = useState<string | null>(null)

  const loadDirectory = useCallback(async (path: string) => {
    try {
      setError(null)
      const entries = await readDir(path)
      const items: FileItem[] = []
      for (const entry of entries) {
        items.push({
          name: entry.name,
          isDirectory: entry.isDirectory,
          path: await join(path, entry.name),
        })
      }
      items.sort((a, b) => {
        if (a.isDirectory !== b.isDirectory) return a.isDirectory ? -1 : 1
        return a.name.localeCompare(b.name)
      })
      setFiles(items)
      setCurrentPath(path)
    } catch (err) {
      const msg = String(err)
      setError('Cannot read directory: ' + msg)
      setFiles([])
    }
  }, [])

  const handleOpen = async () => {
    try {
      const dir = await documentDir()
      await loadDirectory(dir)
      setShowOpen(true)
    } catch (err) {
      setError('Cannot access documents: ' + String(err))
      setShowOpen(true)
    }
  }

  const handleSelectFile = async (item: FileItem) => {
    if (item.isDirectory) {
      await loadDirectory(item.path)
    } else {
      try {
        setError(null)
        const text = await readTextFile(item.path)
        setContent(text)
        setFilename(item.name)
        setFilepath(item.path)
        setIsDirty(false)
        setShowOpen(false)
      } catch (err) {
        setError('Cannot read file: ' + String(err))
      }
    }
  }

  const handleSave = async () => {
    try {
      setError(null)
      let targetPath = filepath
      if (!targetPath) {
        const dir = await documentDir()
        targetPath = await join(dir, filename)
      }
      await writeTextFile(targetPath, content)
      setFilepath(targetPath)
      setIsDirty(false)
    } catch (err) {
      setError('Cannot save file: ' + String(err))
    }
  }

  const handleContentChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    setContent(e.target.value)
    setIsDirty(true)
  }

  const goUp = async () => {
    if (!currentPath) return
    const parts = currentPath.split(/[/\\]/)
    if (parts.length <= 1) return
    parts.pop()
    const parent = parts.join('/')
    if (parent) {
      await loadDirectory(parent)
    }
  }

  const closeDialog = () => {
    setShowOpen(false)
    setError(null)
  }

  return (
    <div className="flex flex-col h-full bg-gray-900 text-gray-100">
      {/* Toolbar */}
      <div className="flex items-center gap-2 px-3 py-2 bg-gray-800 border-b border-gray-700">
        <button
          onClick={handleOpen}
          className="flex items-center gap-1 px-2 py-1 text-xs rounded hover:bg-gray-700 text-gray-300"
          title="Open file"
        >
          <FolderOpen size={14} />
          <span>Open</span>
        </button>
        <button
          onClick={handleSave}
          className={`flex items-center gap-1 px-2 py-1 text-xs rounded ${isDirty ? 'bg-blue-600 text-white' : 'hover:bg-gray-700 text-gray-300'}`}
          title="Save file"
        >
          <Save size={14} />
          <span>Save{isDirty ? '*' : ''}</span>
        </button>
        <div className="flex-1" />
        <span className="text-xs text-gray-400">{filename}{isDirty ? ' •' : ''}</span>
      </div>

      {/* Error display */}
      {error && (
        <div className="flex items-center gap-2 px-3 py-2 bg-red-900/50 text-red-300 text-xs">
          <AlertCircle size={14} />
          <span>{error}</span>
          <button onClick={() => setError(null)} className="ml-auto hover:text-white">✕</button>
        </div>
      )}

      {/* Editor */}
      <div className="flex-1 overflow-hidden">
        <textarea
          value={content}
          onChange={handleContentChange}
          className="w-full h-full p-4 bg-gray-900 text-gray-100 font-mono text-sm leading-relaxed outline-none resize-none border-none"
          spellCheck={false}
          placeholder="Open a file or start typing..."
        />
      </div>

      {/* Status bar */}
      <div className="flex items-center gap-4 px-3 py-1 bg-gray-800 border-t border-gray-700 text-[10px] text-gray-500">
        <span>Ln 1, Col 1</span>
        <span>{content.length} chars</span>
        <span>{filepath || filename}</span>
      </div>

      {/* File browser dialog */}
      {showOpen && (
        <div className="absolute inset-0 flex items-center justify-center bg-black/50 z-50">
          <div className="bg-gray-800 rounded-xl w-80 max-h-[80%] flex flex-col shadow-xl">
            <div className="flex items-center justify-between px-4 py-3 border-b border-gray-700">
              <span className="text-sm font-medium">Open File</span>
              <button onClick={closeDialog} className="text-gray-400 hover:text-white">✕</button>
            </div>
            <div className="flex items-center gap-2 px-4 py-2 bg-gray-750 border-b border-gray-700">
              <button onClick={goUp} className="text-xs text-blue-400 hover:text-blue-300 disabled:opacity-50" disabled={!currentPath}>↑ Up</button>
              <span className="text-[10px] text-gray-500 truncate flex-1">{currentPath}</span>
            </div>
            <div className="flex-1 overflow-y-auto p-2">
              {files.length === 0 ? (
                <div className="text-center text-gray-500 py-4 text-sm">Empty folder</div>
              ) : (
                files.map((file) => (
                  <button
                    key={file.path}
                    onClick={() => handleSelectFile(file)}
                    className="flex items-center gap-2 w-full px-3 py-2 rounded-lg hover:bg-gray-700 text-left"
                  >
                    {file.isDirectory ? (
                      <FolderOpen size={16} className="text-yellow-500" />
                    ) : (
                      <File size={16} className="text-blue-400" />
                    )}
                    <span className="text-sm truncate">{file.name}</span>
                  </button>
                ))
              )}
            </div>
          </div>
        </div>
      )}
    </div>
  )
}