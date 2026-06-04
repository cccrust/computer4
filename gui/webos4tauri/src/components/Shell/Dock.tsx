import { useAppStore } from '../../store/appStore'
import { AppIcon } from './AppIcon'

export function Dock() {
  const { apps, openApp } = useAppStore()

  const dockApps = apps.slice(0, 4)

  return (
    <div className="flex justify-center items-center gap-3 px-6 py-3 mx-4 mb-2 rounded-2xl bg-os-surface/80 backdrop-blur-xl border border-os-border/50">
      {dockApps.map((app) => (
        <AppIcon
          key={app.manifest.appId}
          app={app}
          onOpen={openApp}
          size="small"
        />
      ))}
    </div>
  )
}
