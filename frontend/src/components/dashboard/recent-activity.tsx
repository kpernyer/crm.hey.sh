'use client'

import { Mail, Phone, FileText, Calendar, MessageSquare } from 'lucide-react'
import { clsx } from 'clsx'

const activities = [
  {
    id: 1,
    type: 'email_sent',
    content: 'Email sent to John Smith',
    timestamp: '5 minutes ago',
    icon: Mail,
  },
  {
    id: 2,
    type: 'call',
    content: 'Call with Sarah Chen - 15 minutes',
    timestamp: '1 hour ago',
    icon: Phone,
  },
  {
    id: 3,
    type: 'note',
    content: 'Added note for TechCorp Inc.',
    timestamp: '2 hours ago',
    icon: FileText,
  },
  {
    id: 4,
    type: 'event_invite',
    content: 'Invited 25 contacts to Product Demo',
    timestamp: '3 hours ago',
    icon: Calendar,
  },
  {
    id: 5,
    type: 'email_open',
    content: 'Mike Johnson opened campaign email',
    timestamp: '4 hours ago',
    icon: MessageSquare,
  },
]

const iconColors: Record<string, string> = {
  email_sent: 'bg-blue-100 text-blue-600',
  call: 'bg-green-100 text-green-600',
  note: 'bg-yellow-100 text-yellow-600',
  event_invite: 'bg-purple-100 text-purple-600',
  email_open: 'bg-pink-100 text-pink-600',
}

export function RecentActivity() {
  return (
    <div className="card">
      <div className="px-6 py-4 border-b border-gray-200">
        <h2 className="text-lg font-semibold text-gray-900">Recent Activity</h2>
      </div>
      <div className="divide-y divide-gray-100">
        {activities.map((activity) => (
          <div key={activity.id} className="px-6 py-4 flex items-center space-x-4">
            <div className={clsx(
              'p-2 rounded-lg',
              iconColors[activity.type]
            )}>
              <activity.icon className="w-4 h-4" />
            </div>
            <div className="flex-1 min-w-0">
              <p className="text-sm text-gray-900">{activity.content}</p>
              <p className="text-xs text-gray-500">{activity.timestamp}</p>
            </div>
          </div>
        ))}
      </div>
      <div className="px-6 py-4 border-t border-gray-200">
        <button className="text-sm text-primary-600 hover:text-primary-700 font-medium">
          View all activity
        </button>
      </div>
    </div>
  )
}
