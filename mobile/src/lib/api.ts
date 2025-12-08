import axios from 'axios';

const API_URL = __DEV__
  ? 'http://localhost:8080/api'
  : 'https://crm.hey.sh/api';

const client = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Types
export interface Contact {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  phone?: string;
  linkedin_url?: string;
  tags: string[];
  status: 'lead' | 'customer' | 'partner' | 'investor' | 'other';
  engagement_score: number;
  company_id?: string;
  created_at: string;
  updated_at: string;
}

export interface Event {
  id: string;
  campaign_id?: string;
  name: string;
  type: 'webinar' | 'meetup' | 'ama' | 'demo' | 'other';
  description: string;
  start_time: string;
  end_time: string;
  location: string;
  created_at: string;
}

export interface TimelineEntry {
  id: string;
  contact_id: string;
  company_id?: string;
  type: string;
  content: string;
  metadata: Record<string, unknown>;
  timestamp: string;
}

// API functions
export const api = {
  contacts: {
    list: async () => {
      const {data} = await client.get<Contact[]>('/contacts');
      return data;
    },
    get: async (id: string) => {
      const {data} = await client.get<Contact>(`/contacts/${id}`);
      return data;
    },
    timeline: async (id: string) => {
      const {data} = await client.get<TimelineEntry[]>(
        `/contacts/${id}/timeline`,
      );
      return data;
    },
  },

  events: {
    list: async () => {
      const {data} = await client.get<Event[]>('/events');
      return data;
    },
  },

  timeline: {
    create: async (entry: {
      contact_id: string;
      type: string;
      content: string;
      metadata?: Record<string, unknown>;
    }) => {
      const {data} = await client.post<TimelineEntry>('/timeline', entry);
      return data;
    },
  },
};
