'use client'

import { format, parseISO } from 'date-fns'
import { Calendar, MapPin, Users } from 'lucide-react'
import { Event } from '@/lib/api'

interface UpcomingEventsProps {
  events: Event[]
}

export function UpcomingEvents({ events }: UpcomingEventsProps) {
  // Take only the first 5 upcoming events
  const upcomingEvents = events.slice(0, 5)

  return (
    <div className="card">
      <div className="px-6 py-4 border-b border-gray-200">
        <h2 className="text-lg font-semibold text-gray-900">Upcoming Events</h2>
      </div>
      {upcomingEvents.length === 0 ? (
        <div className="px-6 py-8 text-center">
          <Calendar className="w-12 h-12 text-gray-300 mx-auto mb-3" />
          <p className="text-sm text-gray-500">No upcoming events</p>
          <button className="mt-3 text-sm text-primary-600 hover:text-primary-700 font-medium">
            Create an event
          </button>
        </div>
      ) : (
        <div className="divide-y divide-gray-100">
          {upcomingEvents.map((event) => (
            <div key={event.id} className="px-6 py-4">
              <div className="flex items-start justify-between">
                <div>
                  <h3 className="text-sm font-medium text-gray-900">{event.name}</h3>
                  <div className="mt-1 flex items-center text-xs text-gray-500">
                    <Calendar className="w-3 h-3 mr-1" />
                    {format(parseISO(event.start_time), 'MMM d, yyyy h:mm a')}
                  </div>
                  <div className="mt-1 flex items-center text-xs text-gray-500">
                    <MapPin className="w-3 h-3 mr-1" />
                    {event.location}
                  </div>
                </div>
                <span className="badge badge-primary">{event.type}</span>
              </div>
            </div>
          ))}
        </div>
      )}
      <div className="px-6 py-4 border-t border-gray-200">
        <button className="text-sm text-primary-600 hover:text-primary-700 font-medium">
          View all events
        </button>
      </div>
    </div>
  )
}
