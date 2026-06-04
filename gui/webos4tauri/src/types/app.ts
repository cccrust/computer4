export interface AppManifest {
  appId: string
  name: string
  version: string
  icon: string
  entry: string
  permissions: string[]
  orientation: string
  description: string
  author: string
  category: string
}

export interface AppInfo {
  manifest: AppManifest
  path: string
  isBuiltin: boolean
}

export interface AppMessage {
  type: 'request' | 'response' | 'event'
  id?: string
  method?: string
  params?: Record<string, unknown>
  result?: unknown
  error?: string | null
  event?: string
  data?: unknown
}

export type ViewState = 'home' | 'app'

export interface AppState {
  apps: AppInfo[]
  currentApp: AppInfo | null
  viewState: ViewState
  isLoading: boolean
  error: string | null
}
