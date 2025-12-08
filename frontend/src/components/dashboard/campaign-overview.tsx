'use client'

import { Campaign } from '@/lib/api'
import { clsx } from 'clsx'
import { Rocket, Mail, Share2, FileText, Calendar } from 'lucide-react'

interface CampaignOverviewProps {
  campaigns: Campaign[]
}

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

export function CampaignOverview({ campaigns }: CampaignOverviewProps) {
  // Show recent/active campaigns
  const displayCampaigns = campaigns.slice(0, 5)

  return (
    <div className="card">
      <div className="px-6 py-4 border-b border-gray-200 flex items-center justify-between">
        <h2 className="text-lg font-semibold text-gray-900">Campaigns</h2>
        <button className="btn btn-primary text-sm">
          New Campaign
        </button>
      </div>
      {displayCampaigns.length === 0 ? (
        <div className="px-6 py-8 text-center">
          <Rocket className="w-12 h-12 text-gray-300 mx-auto mb-3" />
          <p className="text-sm text-gray-500">No campaigns yet</p>
          <button className="mt-3 text-sm text-primary-600 hover:text-primary-700 font-medium">
            Create your first campaign
          </button>
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-50">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Campaign
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Status
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Channels
                </th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Objective
                </th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-100">
              {displayCampaigns.map((campaign) => (
                <tr key={campaign.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4">
                    <p className="text-sm font-medium text-gray-900">{campaign.name}</p>
                  </td>
                  <td className="px-6 py-4">
                    <span className={clsx('badge', statusColors[campaign.status])}>
                      {campaign.status}
                    </span>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center space-x-1">
                      {campaign.channels.map((channel) => {
                        const Icon = channelIcons[channel] || Mail
                        return (
                          <div
                            key={channel}
                            className="p-1 bg-gray-100 rounded"
                            title={channel}
                          >
                            <Icon className="w-3 h-3 text-gray-600" />
                          </div>
                        )
                      })}
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <span className="text-sm text-gray-600 capitalize">
                      {campaign.objective.replace('_', ' ')}
                    </span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  )
}
