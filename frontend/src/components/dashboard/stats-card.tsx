import { LucideIcon } from 'lucide-react'
import { clsx } from 'clsx'

interface StatsCardProps {
  name: string
  value: number | string
  icon: LucideIcon
  change: string
  changeType: 'positive' | 'negative' | 'neutral'
}

export function StatsCard({ name, value, icon: Icon, change, changeType }: StatsCardProps) {
  return (
    <div className="card p-6">
      <div className="flex items-center justify-between">
        <div className="p-2 bg-primary-50 rounded-lg">
          <Icon className="w-5 h-5 text-primary-600" />
        </div>
        <span className={clsx(
          'text-xs font-medium',
          changeType === 'positive' && 'text-green-600',
          changeType === 'negative' && 'text-red-600',
          changeType === 'neutral' && 'text-gray-500'
        )}>
          {change}
        </span>
      </div>
      <div className="mt-4">
        <p className="text-2xl font-bold text-gray-900">
          {typeof value === 'number' ? value.toLocaleString() : value}
        </p>
        <p className="text-sm text-gray-500">{name}</p>
      </div>
    </div>
  )
}
