'use client'

import { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { api, Contact } from '@/lib/api'
import { Plus, Search, Filter } from 'lucide-react'
import { clsx } from 'clsx'
import Link from 'next/link'

const statusColors: Record<string, string> = {
  lead: 'badge-primary',
  customer: 'badge-success',
  partner: 'badge-warning',
  investor: 'bg-purple-100 text-purple-700',
  other: 'badge-gray',
}

export default function ContactsPage() {
  const [search, setSearch] = useState('')

  const { data: contacts, isLoading } = useQuery({
    queryKey: ['contacts', search],
    queryFn: () => api.contacts.list({ search: search || undefined }),
  })

  return (
    <div className="p-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-2xl font-bold text-gray-900">Contacts</h1>
          <p className="text-gray-600">Manage your contacts and relationships</p>
        </div>
        <button className="btn btn-primary">
          <Plus className="w-4 h-4 mr-2" />
          Add Contact
        </button>
      </div>

      {/* Search and Filters */}
      <div className="card mb-6">
        <div className="p-4 flex items-center space-x-4">
          <div className="flex-1 relative">
            <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
            <input
              type="text"
              placeholder="Search contacts..."
              className="input pl-10"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>
          <button className="btn btn-secondary">
            <Filter className="w-4 h-4 mr-2" />
            Filters
          </button>
        </div>
      </div>

      {/* Contacts Table */}
      <div className="card overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Name
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Email
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Status
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Tags
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Engagement
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-100">
            {isLoading ? (
              <tr>
                <td colSpan={5} className="px-6 py-8 text-center text-gray-500">
                  Loading contacts...
                </td>
              </tr>
            ) : contacts?.length === 0 ? (
              <tr>
                <td colSpan={5} className="px-6 py-8 text-center text-gray-500">
                  No contacts found
                </td>
              </tr>
            ) : (
              contacts?.map((contact) => (
                <tr key={contact.id} className="hover:bg-gray-50">
                  <td className="px-6 py-4">
                    <Link href={`/contacts/${contact.id}`} className="flex items-center">
                      <div className="w-8 h-8 bg-primary-100 rounded-full flex items-center justify-center mr-3">
                        <span className="text-sm font-medium text-primary-700">
                          {contact.first_name[0]}{contact.last_name[0]}
                        </span>
                      </div>
                      <div>
                        <p className="text-sm font-medium text-gray-900">
                          {contact.first_name} {contact.last_name}
                        </p>
                        {contact.company_id && (
                          <p className="text-xs text-gray-500">Company ID: {contact.company_id}</p>
                        )}
                      </div>
                    </Link>
                  </td>
                  <td className="px-6 py-4">
                    <p className="text-sm text-gray-600">{contact.email}</p>
                  </td>
                  <td className="px-6 py-4">
                    <span className={clsx('badge', statusColors[contact.status])}>
                      {contact.status}
                    </span>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex flex-wrap gap-1">
                      {contact.tags.slice(0, 3).map((tag) => (
                        <span key={tag} className="badge badge-gray">
                          {tag}
                        </span>
                      ))}
                      {contact.tags.length > 3 && (
                        <span className="badge badge-gray">+{contact.tags.length - 3}</span>
                      )}
                    </div>
                  </td>
                  <td className="px-6 py-4">
                    <div className="flex items-center">
                      <div className="w-16 bg-gray-200 rounded-full h-2 mr-2">
                        <div
                          className="bg-primary-600 h-2 rounded-full"
                          style={{ width: `${Math.min(contact.engagement_score, 100)}%` }}
                        />
                      </div>
                      <span className="text-sm text-gray-600">
                        {contact.engagement_score.toFixed(0)}
                      </span>
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  )
}
