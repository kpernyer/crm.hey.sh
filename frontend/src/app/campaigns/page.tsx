'use client'

import { useQuery } from '@tanstack/react-query'
import { api } from '@/lib/api'
import { Plus, Rocket, Mail, Share2, FileText, Calendar } from 'lucide-react'
import { clsx } from 'clsx'
import Link from 'next/link'
import { format, parseISO } from 'date-fns'

const statusColors: Record<string, string> = {
  draft: 'badge-gray',
  scheduled: 'badge-warning',
  running: 'badge-success',
  completed: 'badge-primary',
}

const channelIcons: Record<string, typeof Mail> = {
  email: Mail,
  social: Share2,
  landing_page: FileText,
  event: Calendar,
}

export default function CampaignsPage() {
  const { data: campaigns, isLoading } = useQuery({
    queryKey: ['campaigns'],
    queryFn: () => api.campaigns.list(),
  })

  return (
    <div className="p-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Campaigns</h1>
          <p className="text-gray-600">Create and manage your marketing campaigns</p>
        </div>
        <Link href="/campaigns/new" className="btn btn-primary">
          <Plus className="w-4 h-4 mr-2" />
          New Campaign
        </Link>
      </div>

      {isLoading ? (
        <div className="card p-8">
          <div className="animate-pulse space-y-4">
            <div className="h-16 bg-gray-200 rounded"></div>
            <div className="h-16 bg-gray-200 rounded"></div>
            <div className="h-16 bg-gray-200 rounded"></div>
          </div>
        </div>
      ) : campaigns?.length === 0 ? (
        <div className="card p-12 text-center">
          <Rocket className="w-16 h-16 text-gray-300 mx-auto mb-4" />
          <h2 className="text-xl font-semibold text-gray-900 mb-2">No campaigns yet</h2>
          <p className="text-gray-600 mb-6">
            Create your first campaign to start reaching your contacts
          </p>
          <Link href="/campaigns/new" className="btn btn-primary">
            Create Campaign
          </Link>
        </div>
      ) : (
        <div className="grid gap-6">
          {campaigns?.map((campaign) => (
            <Link key={campaign.id} href={`/campaigns/${campaign.id}`}>
              <div className="card p-6 hover:shadow-md transition-shadow">
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center space-x-3">
                      <h2 className="text-lg font-semibold text-gray-900">{campaign.name}</h2>
                      <span className={clsx('badge', statusColors[campaign.status])}>
                        {campaign.status}
                      </span>
                    </div>
                    <p className="text-sm text-gray-500 mt-1 capitalize">
                      Objective: {campaign.objective.replace('_', ' ')}
                    </p>
                    {campaign.prompt && (
                      <p className="text-sm text-gray-600 mt-2 line-clamp-2">
                        "{campaign.prompt}"
                      </p>
                    )}
                    <div className="flex items-center space-x-4 mt-4">
                      <div className="flex items-center space-x-1">
                        {campaign.channels.map((channel) => {
                          const Icon = channelIcons[channel] || Mail
                          return (
                            <div
                              key={channel}
                              className="p-1.5 bg-gray-100 rounded"
                              title={channel}
                            >
                              <Icon className="w-4 h-4 text-gray-600" />
                            </div>
                          )
                        })}
                      </div>
                      <span className="text-xs text-gray-500">
                        Created {format(parseISO(campaign.created_at), 'MMM d, yyyy')}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}
