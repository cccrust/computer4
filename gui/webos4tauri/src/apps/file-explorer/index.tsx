import { useState, useEffect, useCallback } from 'react'
import {
  Folder, File, ChevronRight, ChevronLeft, Home, Plus, FolderPlus, FilePlus,
  Trash2, Edit3, ArrowUp
} from 'lucide-react'
import { readDir, mkdir, remove, rename, readTextFile, writeTextFile, exists } from '@tauri-apps/plugin-fs'
import { documentDir, join } from '@tauri-apps/api/path'

interface FileItem {
  name: string
  isDirectory: boolean
  path: string
}

export default function FileExplorer() {
  const [currentPath, setCurrentPath] = useState<string>('')
  const [items, setItems] = useState<FileItem[]>([])
  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showNewDialog, setShowNewDialog] = useState<'file' | 'folder' | null>(null)
  const [newName, setNewName] = useState('')
  const [renameTarget, setRenameTarget] = useState<FileItem | null>(null)
  const [renameName, setRenameName] = useState('')

  const loadDirectory = useCallback(async (path: string) => {
    setLoading(true)
    setError(null)
    try {
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
      setItems(items)
      setCurrentPath(path)
    } catch (err) {
      setError(String(err))
    } finally {
      setLoading(false)
    }
  }, [])

  const initPath = useCallback(async () => {
    try {
      const dir = await documentDir()
      await loadDirectory(dir)
    } catch (err) {
      setError(String(err))
    }
  }, [loadDirectory])

  useEffect(() => { initPath() }, [initPath])

  const navigateTo = (item: FileItem) => {
    if (item.isDirectory) {
      loadDirectory(item.path)
    }
  }

  const goUp = async () => {
    if (!currentPath) return
    const parent = currentPath.split(/[/\\]/).slice(0, -1).join('/')
    if (parent) await loadDirectory(parent)
  }

  const goHome = () => { initPath() }

  const handleCreate = async () => {
    if (!newName.trim()) return
    try {
      const targetPath = await join(currentPath, newName.trim())
      if (showNewDialog === 'folder') {
        await mkdir(targetPath, { recursive: true })
      } else {
        await writeTextFile(targetPath, '')
      }
      setShowNewDialog(null)
      setNewName('')
      await loadDirectory(currentPath)
    } catch (err) {
      setError(String(err))
    }
  }

  const handleDelete = async (item: FileItem) => {
    if (!confirm(`Delete "${item.name}"?`)) return
    try {
      await remove(item.path)
      await loadDirectory(currentPath)
    } catch (err) {
      setError(String(err))
    }
  }

  const handleRename = async () => {
    if (!renameTarget || !renameName.trim()) return
    try {
      const oldPath = renameTarget.path
      const newPath = await join(currentPath, renameName.trim())
      await rename(oldPath, newPath)
      setRenameTarget(null)
      setRenameName('')
      await loadDirectory(currentPath)
    } catch (err) {
      setError(String(err))
    }
  }

  const getFileIcon = (item: FileItem) => {
    if (item.isDirectory) return <Folder size={28} className="text-yellow-500" />
    const ext = item.name.split('.').pop()?.toLowerCase() || ''
    const colorMap: Record<string, string> = {
      js: 'text-yellow-500', ts: 'text-blue-500', jsx: 'text-cyan-500',
      tsx: 'text-cyan-500', py: 'text-green-500', json: 'text-orange-500',
      html: 'text-red-500', css: 'text-purple-500', md: 'text-gray-400',
      txt: 'text-gray-400', png: 'text-pink-500', jpg: 'text-pink-500',
    }
    const color = colorMap[ext] || 'text-blue-400'
    return <File size={28} className={color} />
  }

  return (
    <div className="flex flex-col h-full bg-white text-gray-900">
      {/* Toolbar */}
      <div className="flex items-center gap-2 px-3 py-2 bg-gray-100 border-b border-gray-200">
        <button onClick={goHome} className="p-1.5 rounded hover:bg-gray-200 text-gray-600" title="Home">
          <Home size={18} />
        </button>
        <button onClick={goUp} className="p-1.5 rounded hover:bg-gray-200 text-gray-600" title="Go up">
          <ArrowUp size={18} />
        </button>
        <div className="flex-1 text-xs font-mono text-gray-500 truncate px-2">
          {currentPath}
        </div>
        <button onClick={() => setShowNewDialog('folder')} className="p-1.5 rounded hover:bg-gray-200 text-gray-600" title="New folder">
          <FolderPlus size={18} />
        </button>
        <button onClick={() => setShowNewDialog('file')} className="p-1.5 rounded hover:bg-gray-200 text-gray-600" title="New file">
          <FilePlus size={18} />
        </button>
      </div>

      {/* Error */}
      {error && (
        <div className="px-3 py-2 bg-red-100 text-red-700 text-xs font-mono">
          {error}
        </div>
      )}

      {/* File list */}
      <div className="flex-1 overflow-y-auto p-2">
        {loading ? (
          <div className="flex items-center justify-center h-full text-gray-400">
            <div className="w-6 h-6 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
          </div>
        ) : items.length === 0 ? (
          <div className="flex items-center justify-center h-full text-gray-400 text-sm">
            Empty folder
          </div>
        ) : (
          <div className="grid grid-cols-3 gap-1">
            {items.map((item) => (
              <div
                key={item.path}
                className="flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-gray-100 cursor-pointer group"
                onClick={() => navigateTo(item)}
                onDoubleClick={() => navigateTo(item)}
              >
                {getFileIcon(item)}
                <span className="flex-1 text-xs truncate">{item.name}</span>
                <div className="hidden group-hover:flex items-center gap-1">
                  <button
                    onClick={(e) => { e.stopPropagation(); setRenameTarget(item); setRenameName(item.name) }}
                    className="p-1 rounded hover:bg-gray-200 text-gray-500"
                  >
                    <Edit3 size={14} />
                  </button>
                  <button
                    onClick={(e) => { e.stopPropagation(); handleDelete(item) }}
                    className="p-1 rounded hover:bg-red-100 text-gray-500 hover:text-red-600"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* New item dialog */}
      {showNewDialog && (
        <div className="absolute inset-0 flex items-center justify-center bg-black/30">
          <div className="bg-white rounded-xl p-4 w-64 shadow-xl">
            <h3 className="text-sm font-semibold mb-3">New {showNewDialog}</h3>
            <input
              type="text"
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              placeholder={showNewDialog === 'folder' ? 'Folder name' : 'File name'}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm mb-3 outline-none focus:border-blue-500"
              autoFocus
              onKeyDown={(e) => e.key === 'Enter' && handleCreate()}
            />
            <div className="flex justify-end gap-2">
              <button onClick={() => { setShowNewDialog(null); setNewName('') }} className="px-3 py-1.5 text-sm text-gray-600 hover:bg-gray-100 rounded-lg">
                Cancel
              </button>
              <button onClick={handleCreate} className="px-3 py-1.5 text-sm bg-blue-500 text-white rounded-lg hover:bg-blue-600">
                Create
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Rename dialog */}
      {renameTarget && (
        <div className="absolute inset-0 flex items-center justify-center bg-black/30">
          <div className="bg-white rounded-xl p-4 w-64 shadow-xl">
            <h3 className="text-sm font-semibold mb-3">Rename</h3>
            <input
              type="text"
              value={renameName}
              onChange={(e) => setRenameName(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm mb-3 outline-none focus:border-blue-500"
              autoFocus
              onKeyDown={(e) => e.key === 'Enter' && handleRename()}
            />
            <div className="flex justify-end gap-2">
              <button onClick={() => { setRenameTarget(null); setRenameName('') }} className="px-3 py-1.5 text-sm text-gray-600 hover:bg-gray-100 rounded-lg">
                Cancel
              </button>
              <button onClick={handleRename} className="px-3 py-1.5 text-sm bg-blue-500 text-white rounded-lg hover:bg-blue-600">
                Rename
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}