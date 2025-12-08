'use client'

import { useParams } from 'next/navigation'
import { useQuery } from '@tanstack/react-query'
import { api } from '@/lib/api'
import { ArrowLeft, Mail, Phone, Linkedin, Building2, Edit, Trash2 } from 'lucide-react'
import Link from 'next/link'
import { clsx } from 'clsx'
import { format, parseISO } from 'date-fns'

const statusColors: Record<string, string> = {
  lead: 'badge-primary',
  customer: 'badge-success',
  partner: 'badge-warning',
  investor: 'bg-purple-100 text-purple-700',
  other: 'badge-gray',
}

const timelineTypeColors: Record<string, string> = {
  email_sent: 'bg-blue-100 text-blue-600',
  email_open: 'bg-green-100 text-green-600',
  email_click: 'bg-green-100 text-green-600',
  note: 'bg-yellow-100 text-yellow-600',
  call: 'bg-purple-100 text-purple-600',
  event_invite: 'bg-pink-100 text-pink-600',
  event_attend: 'bg-pink-100 text-pink-600',
  landing_page_visit: 'bg-orange-100 text-orange-600',
  task: 'bg-gray-100 text-gray-600',
  social_touch: 'bg-indigo-100 text-indigo-600',
}

export default function ContactDetailPage() {
  const params = useParams()
  const id = params.id as string

  const { data: contact, isLoading: contactLoading } = useQuery({
    queryKey: ['contact', id],
    queryFn: () => api.contacts.get(id),
  })

  const { data: timeline, isLoading: timelineLoading } = useQuery({
    queryKey: ['contact-timeline', id],
    queryFn: () => api.contacts.timeline(id),
  })

  if (contactLoading) {
    return (
      <div className="p-8">
        <div className="animate-pulse">
          <div className="h-8 bg-gray-200 rounded w-1/4 mb-4"></div>
          <div className="h-4 bg-gray-200 rounded w-1/2"></div>
        </div>
      </div>
    )
  }

  if (!contact) {
    return (
      <div className="p-8">
        <p className="text-gray-600">Contact not found</p>
      </div>
    )
  }

  return (
    <div className="p-8">
      {/* Header */}
      <div className="mb-8">
        <Link href="/contacts" className="inline-flex items-center text-sm text-gray-600 hover:text-gray-900 mb-4">
          <ArrowLeft className="w-4 h-4 mr-1" />
          Back to Contacts
        </Link>

        <div className="flex items-start justify-between">
          <div className="flex items-center">
            <div className="w-16 h-16 bg-primary-100 rounded-full flex items-center justify-center mr-4">
              <span className="text-2xl font-bold text-primary-700">
                {contact.first_name[0]}{contact.last_name[0]}
              </span>
            </div>
            <div>
              <h1 className="text-2xl font-bold text-gray-900">
                {contact.first_name} {contact.last_name}
              </h1>
              <div className="flex items-center space-x-3 mt-1">
                <span className={clsx('badge', statusColors[contact.status])}>
                  {contact.status}
                </span>
                <span className="text-sm text-gray-500">
                  Engagement: {contact.engagement_score.toFixed(0)}%
                </span>
              </div>
            </div>
          </div>
          <div className="flex space-x-2">
            <button className="btn btn-secondary">
              <Edit className="w-4 h-4 mr-2" />
              Edit
            </button>
            <button className="btn btn-ghost text-red-600 hover:bg-red-50">
              <Trash2 className="w-4 h-4" />
            </button>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Contact Info */}
        <div className="lg:col-span-1 space-y-6">
          <div className="card p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">Contact Info</h2>
            <div className="space-y-4">
              <div className="flex items-center">
                <Mail className="w-4 h-4 text-gray-400 mr-3" />
                <a href={`mailto:${contact.email}`} className="text-sm text-primary-600 hover:underline">
                  {contact.email}
                </a>
              </div>
              {contact.phone && (
                <div className="flex items-center">
                  <Phone className="w-4 h-4 text-gray-400 mr-3" />
                  <a href={`tel:${contact.phone}`} className="text-sm text-gray-900">
                    {contact.phone}
                  </a>
                </div>
              )}
              {contact.linkedin_url && (
                <div className="flex items-center">
                  <Linkedin className="w-4 h-4 text-gray-400 mr-3" />
                  <a href={contact.linkedin_url} target="_blank" rel="noopener noreferrer" className="text-sm text-primary-600 hover:underline">
                    LinkedIn Profile
                  </a>
                </div>
              )}
              {contact.company_id && (
                <div className="flex items-center">
                  <Building2 className="w-4 h-4 text-gray-400 mr-3" />
                  <Link href={`/companies/${contact.company_id}`} className="text-sm text-primary-600 hover:underline">
                    View Company
                  </Link>
                </div>
              )}
            </div>
          </div>

          <div className="card p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">Tags</h2>
            <div className="flex flex-wrap gap-2">
              {contact.tags.map((tag) => (
                <span key={tag} className="badge badge-primary">
                  {tag}
                </span>
              ))}
              {contact.tags.length === 0 && (
                <p className="text-sm text-gray-500">No tags</p>
              )}
            </div>
          </div>

          <div className="card p-6">
            <h2 className="text-lg font-semibold text-gray-900 mb-4">Quick Actions</h2>
            <div className="space-y-2">
              <button className="btn btn-secondary w-full justify-start">
                <Mail className="w-4 h-4 mr-2" />
                Send Email
              </button>
              <button className="btn btn-secondary w-full justify-start">
                <Phone className="w-4 h-4 mr-2" />
                Log Call
              </button>
              <button className="btn btn-secondary w-full justify-start">
                Add Note
              </button>
            </div>
          </div>
        </div>

        {/* Timeline */}
        <div className="lg:col-span-2">
          <div className="card">
            <div className="px-6 py-4 border-b border-gray-200">
              <h2 className="text-lg font-semibold text-gray-900">Timeline</h2>
            </div>
            <div className="p-6">
              {timelineLoading ? (
                <div className="animate-pulse space-y-4">
                  <div className="h-16 bg-gray-200 rounded"></div>
                  <div className="h-16 bg-gray-200 rounded"></div>
                  <div className="h-16 bg-gray-200 rounded"></div>
                </div>
              ) : timeline?.length === 0 ? (
                <p className="text-center text-gray-500 py-8">No activity yet</p>
              ) : (
                <div className="space-y-4">
                  {timeline?.map((entry) => (
                    <div key={entry.id} className="flex items-start space-x-4">
                      <div className={clsx(
                        'p-2 rounded-lg',
                        timelineTypeColors[entry.type] || 'bg-gray-100 text-gray-600'
                      )}>
                        <Mail className="w-4 h-4" />
                      </div>
                      <div className="flex-1">
                        <p className="text-sm text-gray-900">{entry.content}</p>
                        <p className="text-xs text-gray-500 mt-1">
                          {format(parseISO(entry.timestamp), 'MMM d, yyyy h:mm a')}
                        </p>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
