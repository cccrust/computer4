import { useEffect, useState } from 'react'
import { Wifi, BatteryFull } from 'lucide-react'

export function StatusBar() {
  const [time, setTime] = useState(new Date())

  useEffect(() => {
    const timer = setInterval(() => setTime(new Date()), 10000)
    return () => clearInterval(timer)
  }, [])

  const formatTime = (d: Date) => {
    const h = d.getHours().toString().padStart(2, '0')
    const m = d.getMinutes().toString().padStart(2, '0')
    return `${h}:${m}`
  }

  return (
    <div className="flex items-center justify-between px-6 h-[var(--status-bar-height)] bg-os-bg/80 backdrop-blur-xl z-50">
      <span className="text-xs font-semibold text-os-text">
        {formatTime(time)}
      </span>
      <div className="flex items-center gap-2">
        <Wifi size={14} className="text-os-text" />
        <BatteryFull size={14} className="text-os-text" />
      </div>
    </div>
  )
}
