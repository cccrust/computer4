import { useAppStore } from '../../store/appStore'
import { AppIcon } from './AppIcon'

export function HomeScreen() {
  const { apps, openApp } = useAppStore()

  return (
    <div className="flex-1 flex flex-col px-4 pt-4 overflow-y-auto">
      <div className="grid grid-cols-4 gap-y-5 gap-x-2 justify-items-center">
        {apps.map((app) => (
          <AppIcon key={app.manifest.appId} app={app} onOpen={openApp} />
        ))}
      </div>
    </div>
  )
}
