'use client'

import { useQuery } from '@tanstack/react-query'
import { api } from '@/lib/api'
import { StatsCard } from '@/components/dashboard/stats-card'
import { RecentActivity } from '@/components/dashboard/recent-activity'
import { UpcomingEvents } from '@/components/dashboard/upcoming-events'
import { CampaignOverview } from '@/components/dashboard/campaign-overview'
import { Users, Building2, Rocket, Calendar } from 'lucide-react'

export default function DashboardPage() {
  const { data: contactsAnalytics } = useQuery({
    queryKey: ['analytics', 'contacts'],
    queryFn: () => api.analytics.contacts(),
  })

  const { data: campaigns } = useQuery({
    queryKey: ['campaigns'],
    queryFn: () => api.campaigns.list(),
  })

  const { data: events } = useQuery({
    queryKey: ['events'],
    queryFn: () => api.events.list(),
  })

  const stats = [
    {
      name: 'Total Contacts',
      value: contactsAnalytics?.total_contacts ?? 0,
      icon: Users,
      change: '+12%',
      changeType: 'positive' as const,
    },
    {
      name: 'Companies',
      value: 245,
      icon: Building2,
      change: '+3%',
      changeType: 'positive' as const,
    },
    {
      name: 'Active Campaigns',
      value: campaigns?.filter(c => c.status === 'running').length ?? 0,
      icon: Rocket,
      change: '2 new',
      changeType: 'neutral' as const,
    },
    {
      name: 'Upcoming Events',
      value: events?.length ?? 0,
      icon: Calendar,
      change: 'Next: 3 days',
      changeType: 'neutral' as const,
    },
  ]

  return (
    <div className="p-8">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">Dashboard</h1>
        <p className="text-gray-600">Welcome back! Here's what's happening with your CRM.</p>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        {stats.map((stat) => (
          <StatsCard key={stat.name} {...stat} />
        ))}
      </div>

      {/* Main Content Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <div className="lg:col-span-2 space-y-6">
          <CampaignOverview campaigns={campaigns ?? []} />
          <RecentActivity />
        </div>
        <div>
          <UpcomingEvents events={events ?? []} />
        </div>
      </div>
    </div>
  )
}
