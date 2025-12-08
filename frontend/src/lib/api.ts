import axios from 'axios'

const client = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL || '/api',
  headers: {
    'Content-Type': 'application/json',
  },
})

// Types
export interface Contact {
  id: string
  first_name: string
  last_name: string
  email: string
  phone?: string
  linkedin_url?: string
  tags: string[]
  status: 'lead' | 'customer' | 'partner' | 'investor' | 'other'
  engagement_score: number
  company_id?: string
  created_at: string
  updated_at: string
}

export interface Company {
  id: string
  name: string
  domain?: string
  industry?: string
  size?: string
  tags: string[]
  created_at: string
  updated_at: string
}

export interface Campaign {
  id: string
  name: string
  objective: 'awareness' | 'lead_gen' | 'event' | 'investor' | 'early_adopters'
  status: 'draft' | 'scheduled' | 'running' | 'completed'
  channels: ('email' | 'social' | 'landing_page' | 'event')[]
  prompt?: string
  segment_definition: Record<string, unknown>
  created_at: string
  updated_at: string
}

export interface Event {
  id: string
  campaign_id?: string
  name: string
  type: 'webinar' | 'meetup' | 'ama' | 'demo' | 'other'
  description: string
  start_time: string
  end_time: string
  location: string
  created_at: string
}

export interface TimelineEntry {
  id: string
  contact_id: string
  company_id?: string
  type: string
  content: string
  metadata: Record<string, unknown>
  timestamp: string
}

export interface ContactsAnalytics {
  total_contacts: number
  leads: number
  customers: number
  partners: number
  investors: number
  other: number
  avg_engagement_score: number
  new_this_month: number
  top_engaged: { id: string; name: string; engagement_score: number }[]
}

export interface CampaignAnalytics {
  campaign_id: string
  total_contacts: number
  emails_sent: number
  emails_opened: number
  emails_clicked: number
  landing_page_visits: number
  conversions: number
  open_rate: number
  click_rate: number
  conversion_rate: number
}

export interface FunnelAnalytics {
  stages: { name: string; count: number; percentage: number }[]
  overall_conversion_rate: number
}

// API functions
export const api = {
  contacts: {
    list: async (params?: { search?: string; status?: string; limit?: number; offset?: number }) => {
      const { data } = await client.get<Contact[]>('/contacts', { params })
      return data
    },
    get: async (id: string) => {
      const { data } = await client.get<Contact>(`/contacts/${id}`)
      return data
    },
    create: async (contact: Omit<Contact, 'id' | 'created_at' | 'updated_at' | 'engagement_score'>) => {
      const { data } = await client.post<Contact>('/contacts', contact)
      return data
    },
    update: async (id: string, contact: Partial<Contact>) => {
      const { data } = await client.patch<Contact>(`/contacts/${id}`, contact)
      return data
    },
    delete: async (id: string) => {
      await client.delete(`/contacts/${id}`)
    },
    timeline: async (id: string) => {
      const { data } = await client.get<TimelineEntry[]>(`/contacts/${id}/timeline`)
      return data
    },
  },

  companies: {
    list: async (params?: { search?: string; industry?: string; limit?: number; offset?: number }) => {
      const { data } = await client.get<Company[]>('/companies', { params })
      return data
    },
    get: async (id: string) => {
      const { data } = await client.get<Company>(`/companies/${id}`)
      return data
    },
    create: async (company: Omit<Company, 'id' | 'created_at' | 'updated_at'>) => {
      const { data } = await client.post<Company>('/companies', company)
      return data
    },
    update: async (id: string, company: Partial<Company>) => {
      const { data } = await client.patch<Company>(`/companies/${id}`, company)
      return data
    },
    delete: async (id: string) => {
      await client.delete(`/companies/${id}`)
    },
  },

  campaigns: {
    list: async () => {
      const { data } = await client.get<Campaign[]>('/campaigns')
      return data
    },
    get: async (id: string) => {
      const { data } = await client.get<Campaign>(`/campaigns/${id}`)
      return data
    },
    create: async (campaign: Omit<Campaign, 'id' | 'created_at' | 'updated_at' | 'status'>) => {
      const { data } = await client.post<Campaign>('/campaigns', campaign)
      return data
    },
    update: async (id: string, campaign: Partial<Campaign>) => {
      const { data } = await client.patch<Campaign>(`/campaigns/${id}`, campaign)
      return data
    },
    generateAssets: async (id: string, prompt: string, assetTypes: string[]) => {
      const { data } = await client.post(`/campaigns/${id}/assets`, { prompt, asset_types: assetTypes })
      return data
    },
    execute: async (id: string) => {
      const { data } = await client.post(`/campaigns/${id}/execute`)
      return data
    },
  },

  events: {
    list: async () => {
      const { data } = await client.get<Event[]>('/events')
      return data
    },
    get: async (id: string) => {
      const { data } = await client.get<Event>(`/events/${id}`)
      return data
    },
    create: async (event: Omit<Event, 'id' | 'created_at'>) => {
      const { data } = await client.post<Event>('/events', event)
      return data
    },
    invite: async (id: string, contactIds: string[]) => {
      const { data } = await client.post(`/events/${id}/invite`, { contact_ids: contactIds })
      return data
    },
    rsvp: async (id: string, contactId: string, status: string) => {
      const { data } = await client.post(`/events/${id}/rsvp`, { contact_id: contactId, status })
      return data
    },
  },

  landingPages: {
    generate: async (prompt: string, campaignId?: string) => {
      const { data } = await client.post('/landing-pages/generate', { prompt, campaign_id: campaignId })
      return data
    },
    get: async (id: string) => {
      const { data } = await client.get(`/lp/${id}`)
      return data
    },
  },

  timeline: {
    create: async (entry: { contact_id: string; type: string; content: string; metadata?: Record<string, unknown> }) => {
      const { data } = await client.post<TimelineEntry>('/timeline', entry)
      return data
    },
  },

  analytics: {
    contacts: async () => {
      const { data } = await client.get<ContactsAnalytics>('/analytics/contacts')
      return data
    },
    campaign: async (id: string) => {
      const { data } = await client.get<CampaignAnalytics>(`/analytics/campaign/${id}`)
      return data
    },
    funnel: async () => {
      const { data } = await client.get<FunnelAnalytics>('/analytics/funnel')
      return data
    },
  },
}
