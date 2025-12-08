'use client'

import { useState } from 'react'
import { useRouter } from 'next/navigation'
import { useMutation } from '@tanstack/react-query'
import { api } from '@/lib/api'
import { ArrowLeft, Mail, Share2, FileText, Calendar, Sparkles } from 'lucide-react'
import Link from 'next/link'
import { clsx } from 'clsx'

const objectives = [
  { id: 'awareness', name: 'Brand Awareness', description: 'Increase visibility and reach' },
  { id: 'lead_gen', name: 'Lead Generation', description: 'Capture new leads and contacts' },
  { id: 'event', name: 'Event Promotion', description: 'Drive event registrations' },
  { id: 'investor', name: 'Investor Outreach', description: 'Connect with potential investors' },
  { id: 'early_adopters', name: 'Early Adopters', description: 'Find and engage early users' },
]

const channels = [
  { id: 'email', name: 'Email', icon: Mail, description: 'Send targeted email campaigns' },
  { id: 'social', name: 'Social Media', icon: Share2, description: 'Post to social platforms' },
  { id: 'landing_page', name: 'Landing Page', icon: FileText, description: 'Generate a landing page' },
  { id: 'event', name: 'Event', icon: Calendar, description: 'Create and promote events' },
]

export default function NewCampaignPage() {
  const router = useRouter()
  const [name, setName] = useState('')
  const [objective, setObjective] = useState('')
  const [selectedChannels, setSelectedChannels] = useState<string[]>([])
  const [prompt, setPrompt] = useState('')

  const createMutation = useMutation({
    mutationFn: () =>
      api.campaigns.create({
        name,
        objective: objective as any,
        channels: selectedChannels as any[],
        prompt,
        segment_definition: {},
      }),
    onSuccess: (data) => {
      router.push(`/campaigns/${data.id}`)
    },
  })

  const toggleChannel = (channelId: string) => {
    setSelectedChannels((prev) =>
      prev.includes(channelId)
        ? prev.filter((c) => c !== channelId)
        : [...prev, channelId]
    )
  }

  const canSubmit = name && objective && selectedChannels.length > 0

  return (
    <div className="p-8 max-w-3xl mx-auto">
      <Link href="/campaigns" className="inline-flex items-center text-sm text-gray-600 hover:text-gray-900 mb-6">
        <ArrowLeft className="w-4 h-4 mr-1" />
        Back to Campaigns
      </Link>

      <div className="mb-8">
        <h1 className="text-2xl font-bold text-gray-900">Create Campaign</h1>
        <p className="text-gray-600">Set up a new marketing campaign</p>
      </div>

      <div className="space-y-8">
        {/* Campaign Name */}
        <div className="card p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Campaign Name</h2>
          <input
            type="text"
            placeholder="e.g., Product Launch Q1 2025"
            className="input"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>

        {/* Objective */}
        <div className="card p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Objective</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {objectives.map((obj) => (
              <button
                key={obj.id}
                onClick={() => setObjective(obj.id)}
                className={clsx(
                  'p-4 rounded-lg border-2 text-left transition-colors',
                  objective === obj.id
                    ? 'border-primary-600 bg-primary-50'
                    : 'border-gray-200 hover:border-gray-300'
                )}
              >
                <p className="font-medium text-gray-900">{obj.name}</p>
                <p className="text-sm text-gray-500 mt-1">{obj.description}</p>
              </button>
            ))}
          </div>
        </div>

        {/* Channels */}
        <div className="card p-6">
          <h2 className="text-lg font-semibold text-gray-900 mb-4">Channels</h2>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {channels.map((channel) => {
              const isSelected = selectedChannels.includes(channel.id)
              return (
                <button
                  key={channel.id}
                  onClick={() => toggleChannel(channel.id)}
                  className={clsx(
                    'p-4 rounded-lg border-2 text-left transition-colors flex items-start',
                    isSelected
                      ? 'border-primary-600 bg-primary-50'
                      : 'border-gray-200 hover:border-gray-300'
                  )}
                >
                  <channel.icon className={clsx(
                    'w-5 h-5 mr-3 mt-0.5',
                    isSelected ? 'text-primary-600' : 'text-gray-400'
                  )} />
                  <div>
                    <p className="font-medium text-gray-900">{channel.name}</p>
                    <p className="text-sm text-gray-500 mt-1">{channel.description}</p>
                  </div>
                </button>
              )
            })}
          </div>
        </div>

        {/* AI Prompt */}
        <div className="card p-6">
          <div className="flex items-center space-x-2 mb-4">
            <Sparkles className="w-5 h-5 text-primary-600" />
            <h2 className="text-lg font-semibold text-gray-900">AI Content Prompt</h2>
          </div>
          <p className="text-sm text-gray-600 mb-4">
            Describe what you want to communicate. Our AI will generate content for your selected channels.
          </p>
          <textarea
            placeholder="e.g., We're launching a new feature that helps founders manage their investor relationships more effectively. Highlight the time-saving benefits and ease of use..."
            className="input min-h-[120px]"
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
          />
        </div>

        {/* Actions */}
        <div className="flex justify-end space-x-3">
          <Link href="/campaigns" className="btn btn-secondary">
            Cancel
          </Link>
          <button
            className="btn btn-primary"
            disabled={!canSubmit || createMutation.isPending}
            onClick={() => createMutation.mutate()}
          >
            {createMutation.isPending ? 'Creating...' : 'Create Campaign'}
          </button>
        </div>
      </div>
    </div>
  )
}
