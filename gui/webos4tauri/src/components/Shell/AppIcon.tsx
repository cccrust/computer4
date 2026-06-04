import type { AppInfo } from '../../types/app'
import * as Icons from 'lucide-react'
import type { LucideIcon } from 'lucide-react'

interface AppIconProps {
  app: AppInfo
  onOpen: (app: AppInfo) => void
  size?: 'normal' | 'small'
}

export function AppIcon({ app, onOpen, size = 'normal' }: AppIconProps) {
  const iconName = app.manifest.icon
    .split('-')
    .map((s) => s.charAt(0).toUpperCase() + s.slice(1))
    .join('') as keyof typeof Icons

  const IconComponent = (Icons[iconName] || Icons['AppWindow']) as LucideIcon
  const isSmall = size === 'small'

  return (
    <button
      onClick={() => onOpen(app)}
      className="flex flex-col items-center gap-1.5 p-2 rounded-2xl transition-colors active:bg-os-card/50"
    >
      <div
        className={`flex items-center justify-center rounded-icon bg-gradient-to-br from-os-accent to-purple-600 shadow-lg ${
          isSmall ? 'w-12 h-12' : 'w-app-icon h-app-icon'
        }`}
      >
        <IconComponent
          size={isSmall ? 22 : 28}
          className="text-white"
        />
      </div>
      <span
        className={`text-os-text text-center leading-tight ${
          isSmall ? 'text-[10px] max-w-[64px]' : 'text-xs max-w-[80px]'
        }`}
      >
        {app.manifest.name}
      </span>
    </button>
  )
}
