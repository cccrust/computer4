import { ArrowLeft, X } from 'lucide-react'
import { useAppStore } from '../../store/appStore'
import FileExplorer from '../../apps/file-explorer'
import Terminal from '../../apps/terminal'
import Editor from '../../apps/editor'
import Browser from '../../apps/browser'

const appComponents: Record<string, React.ComponentType> = {
  'com.webos.fileexplorer': FileExplorer,
  'com.webos.terminal': Terminal,
  'com.webos.editor': Editor,
  'com.webos.browser': Browser,
}

export function AppContainer() {
  const { currentApp, closeApp } = useAppStore()

  if (!currentApp) return null

  const AppComponent = appComponents[currentApp.manifest.appId]

  return (
    <div className="absolute inset-0 z-50 flex flex-col bg-white">
      {/* Header - 明顯的返回按鈕 */}
      <div className="flex items-center gap-3 px-4 py-3 bg-gray-100 border-b border-gray-200">
        <button
          onClick={closeApp}
          className="flex items-center gap-2 px-3 py-2 rounded-full bg-blue-500 text-white font-medium text-sm hover:bg-blue-600 active:bg-blue-700 transition-colors shadow-sm"
        >
          <ArrowLeft size={18} />
          <span>Back</span>
        </button>
        <span className="text-sm font-semibold text-gray-800 flex-1 text-center mr-12">
          {currentApp.manifest.name}
        </span>
      </div>
      {/* App Content */}
      <div className="flex-1 overflow-hidden">
        {AppComponent ? (
          <AppComponent />
        ) : (
          <div className="flex items-center justify-center h-full text-gray-500">
            <p>App not available</p>
          </div>
        )}
      </div>
    </div>
  )
}