import { create } from 'zustand'
import type { AppInfo, AppState } from '../types/app'
import { listApps, isTauri } from '../api/tauri'

const DEFAULT_APPS: AppInfo[] = [
  {
    manifest: {
      appId: 'com.webos.fileexplorer',
      name: 'File Explorer',
      version: '0.1.0',
      icon: 'folder-open',
      entry: 'index.html',
      permissions: ['fs:read', 'fs:write'],
      orientation: 'portrait',
      description: 'File manager & storage browser',
      author: 'webOS',
      category: 'utilities',
    },
    path: '/src/apps/file-explorer',
    isBuiltin: true,
  },
  {
    manifest: {
      appId: 'com.webos.terminal',
      name: 'Terminal',
      version: '0.1.0',
      icon: 'terminal',
      entry: 'index.html',
      permissions: ['shell:exec'],
      orientation: 'portrait',
      description: 'Command line shell',
      author: 'webOS',
      category: 'utilities',
    },
    path: '/src/apps/terminal',
    isBuiltin: true,
  },
  {
    manifest: {
      appId: 'com.webos.editor',
      name: 'Editor',
      version: '0.1.0',
      icon: 'file-code',
      entry: 'index.html',
      permissions: ['fs:read', 'fs:write'],
      orientation: 'portrait',
      description: 'Code & text editor',
      author: 'webOS',
      category: 'utilities',
    },
    path: '/src/apps/editor',
    isBuiltin: true,
  },
  {
    manifest: {
      appId: 'com.webos.browser',
      name: 'Browser',
      version: '0.1.0',
      icon: 'globe',
      entry: 'index.html',
      permissions: ['network'],
      orientation: 'both',
      description: 'Web browser',
      author: 'webOS',
      category: 'utilities',
    },
    path: '/src/apps/browser',
    isBuiltin: true,
  },
]

interface AppStore extends AppState {
  loadApps: () => Promise<void>
  openApp: (app: AppInfo) => void
  closeApp: () => void
  goHome: () => void
}

export const useAppStore = create<AppStore>((set) => ({
  apps: DEFAULT_APPS,
  currentApp: null,
  viewState: 'home',
  isLoading: false,
  error: null,

  loadApps: async () => {
    if (!isTauri()) {
      set({ apps: DEFAULT_APPS, isLoading: false })
      return
    }
    set({ isLoading: true, error: null })
    try {
      const apps = await listApps()
      set({ apps, isLoading: false })
    } catch (err) {
      set({ error: String(err), isLoading: false, apps: DEFAULT_APPS })
    }
  },

  openApp: (app: AppInfo) => {
    set({ currentApp: app, viewState: 'app' })
  },

  closeApp: () => {
    set({ currentApp: null, viewState: 'home' })
  },

  goHome: () => {
    set({ currentApp: null, viewState: 'home' })
  },
}))
