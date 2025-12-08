'use client'

import { useQuery } from '@tanstack/react-query'
import { api } from '@/lib/api'
import { BarChart3, TrendingUp, Users, Target } from 'lucide-react'

export default function AnalyticsPage() {
  const { data: contactsAnalytics } = useQuery({
    queryKey: ['analytics', 'contacts'],
    queryFn: () => api.analytics.contacts(),
  })

  const { data: funnelAnalytics } = useQuery({
    queryKey: ['analytics', 'funnel'],
    queryFn: () => api.analytics.funnel(),
  })

  return (
    <div className="p-8">
      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">Analytics</h1>
        <p className="text-gray-600">Track your CRM performance and engagement metrics</p>
      </div>

      {/* Overview Stats */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <div className="card p-6">
          <div className="flex items-center justify-between">
            <div className="p-2 bg-primary-50 rounded-lg">
              <Users className="w-5 h-5 text-primary-600" />
            </div>
          </div>
          <p className="text-2xl font-bold text-gray-900 mt-4">
            {contactsAnalytics?.total_contacts.toLocaleString() ?? 0}
          </p>
          <p className="text-sm text-gray-500">Total Contacts</p>
        </div>

        <div className="card p-6">
          <div className="flex items-center justify-between">
            <div className="p-2 bg-green-50 rounded-lg">
              <TrendingUp className="w-5 h-5 text-green-600" />
            </div>
          </div>
          <p className="text-2xl font-bold text-gray-900 mt-4">
            {contactsAnalytics?.new_this_month ?? 0}
          </p>
          <p className="text-sm text-gray-500">New This Month</p>
        </div>

        <div className="card p-6">
          <div className="flex items-center justify-between">
            <div className="p-2 bg-yellow-50 rounded-lg">
              <BarChart3 className="w-5 h-5 text-yellow-600" />
            </div>
          </div>
          <p className="text-2xl font-bold text-gray-900 mt-4">
            {contactsAnalytics?.avg_engagement_score.toFixed(1) ?? 0}%
          </p>
          <p className="text-sm text-gray-500">Avg Engagement</p>
        </div>

        <div className="card p-6">
          <div className="flex items-center justify-between">
            <div className="p-2 bg-purple-50 rounded-lg">
              <Target className="w-5 h-5 text-purple-600" />
            </div>
          </div>
          <p className="text-2xl font-bold text-gray-900 mt-4">
            {funnelAnalytics?.overall_conversion_rate.toFixed(2) ?? 0}%
          </p>
          <p className="text-sm text-gray-500">Conversion Rate</p>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Contact Breakdown */}
        <div className="card">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-semibold text-gray-900">Contacts by Status</h2>
          </div>
          <div className="p-6">
            <div className="space-y-4">
              {[
                { label: 'Leads', value: contactsAnalytics?.leads ?? 0, color: 'bg-primary-600' },
                { label: 'Customers', value: contactsAnalytics?.customers ?? 0, color: 'bg-green-600' },
                { label: 'Partners', value: contactsAnalytics?.partners ?? 0, color: 'bg-yellow-600' },
                { label: 'Investors', value: contactsAnalytics?.investors ?? 0, color: 'bg-purple-600' },
                { label: 'Other', value: contactsAnalytics?.other ?? 0, color: 'bg-gray-600' },
              ].map((item) => {
                const total = contactsAnalytics?.total_contacts ?? 1
                const percentage = ((item.value / total) * 100).toFixed(1)
                return (
                  <div key={item.label}>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-gray-600">{item.label}</span>
                      <span className="text-gray-900 font-medium">
                        {item.value.toLocaleString()} ({percentage}%)
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2">
                      <div
                        className={`${item.color} h-2 rounded-full`}
                        style={{ width: `${percentage}%` }}
                      />
                    </div>
                  </div>
                )
              })}
            </div>
          </div>
        </div>

        {/* Funnel */}
        <div className="card">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-semibold text-gray-900">Conversion Funnel</h2>
          </div>
          <div className="p-6">
            <div className="space-y-4">
              {funnelAnalytics?.stages.map((stage, index) => {
                const maxCount = funnelAnalytics.stages[0]?.count ?? 1
                const width = ((stage.count / maxCount) * 100).toFixed(1)
                return (
                  <div key={stage.name}>
                    <div className="flex justify-between text-sm mb-1">
                      <span className="text-gray-600">{stage.name}</span>
                      <span className="text-gray-900 font-medium">
                        {stage.count.toLocaleString()} ({stage.percentage}%)
                      </span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-8 flex items-center">
                      <div
                        className="bg-primary-600 h-8 rounded-full flex items-center justify-center"
                        style={{ width: `${width}%`, minWidth: '40px' }}
                      >
                        <span className="text-xs text-white font-medium px-2">
                          {stage.count.toLocaleString()}
                        </span>
                      </div>
                    </div>
                  </div>
                )
              })}
            </div>
          </div>
        </div>

        {/* Top Engaged Contacts */}
        <div className="card">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-semibold text-gray-900">Top Engaged Contacts</h2>
          </div>
          <div className="divide-y divide-gray-100">
            {contactsAnalytics?.top_engaged.map((contact, index) => (
              <div key={contact.id} className="px-6 py-4 flex items-center justify-between">
                <div className="flex items-center">
                  <span className="w-6 h-6 bg-primary-100 rounded-full flex items-center justify-center text-xs font-medium text-primary-700 mr-3">
                    {index + 1}
                  </span>
                  <span className="text-sm font-medium text-gray-900">{contact.name}</span>
                </div>
                <span className="text-sm text-gray-600">
                  {contact.engagement_score.toFixed(1)}%
                </span>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  )
}
