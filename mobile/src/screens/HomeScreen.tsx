import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  RefreshControl,
} from 'react-native';
import {useQuery} from '@tanstack/react-query';
import {api} from '../lib/api';
import {format, parseISO} from 'date-fns';

export default function HomeScreen() {
  const {
    data: contacts,
    isLoading: contactsLoading,
    refetch: refetchContacts,
  } = useQuery({
    queryKey: ['contacts'],
    queryFn: () => api.contacts.list(),
  });

  const {
    data: events,
    isLoading: eventsLoading,
    refetch: refetchEvents,
  } = useQuery({
    queryKey: ['events'],
    queryFn: () => api.events.list(),
  });

  const isLoading = contactsLoading || eventsLoading;

  const onRefresh = () => {
    refetchContacts();
    refetchEvents();
  };

  return (
    <ScrollView
      style={styles.container}
      refreshControl={
        <RefreshControl refreshing={isLoading} onRefresh={onRefresh} />
      }>
      {/* Header */}
      <View style={styles.header}>
        <Text style={styles.greeting}>Welcome back!</Text>
        <Text style={styles.subtitle}>Here's your CRM overview</Text>
      </View>

      {/* Stats */}
      <View style={styles.statsContainer}>
        <View style={styles.statCard}>
          <Text style={styles.statValue}>{contacts?.length ?? 0}</Text>
          <Text style={styles.statLabel}>Contacts</Text>
        </View>
        <View style={styles.statCard}>
          <Text style={styles.statValue}>{events?.length ?? 0}</Text>
          <Text style={styles.statLabel}>Events</Text>
        </View>
      </View>

      {/* Upcoming Events */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>Upcoming Events</Text>
        {events?.slice(0, 3).map(event => (
          <TouchableOpacity key={event.id} style={styles.eventCard}>
            <View style={styles.eventBadge}>
              <Text style={styles.eventBadgeText}>{event.type}</Text>
            </View>
            <Text style={styles.eventName}>{event.name}</Text>
            <Text style={styles.eventDate}>
              {format(parseISO(event.start_time), 'MMM d, h:mm a')}
            </Text>
          </TouchableOpacity>
        ))}
        {(!events || events.length === 0) && (
          <Text style={styles.emptyText}>No upcoming events</Text>
        )}
      </View>

      {/* Recent Contacts */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>Recent Contacts</Text>
        {contacts?.slice(0, 5).map(contact => (
          <TouchableOpacity key={contact.id} style={styles.contactCard}>
            <View style={styles.avatar}>
              <Text style={styles.avatarText}>
                {contact.first_name[0]}
                {contact.last_name[0]}
              </Text>
            </View>
            <View style={styles.contactInfo}>
              <Text style={styles.contactName}>
                {contact.first_name} {contact.last_name}
              </Text>
              <Text style={styles.contactEmail}>{contact.email}</Text>
            </View>
            <View
              style={[
                styles.statusBadge,
                contact.status === 'customer' && styles.statusCustomer,
                contact.status === 'lead' && styles.statusLead,
              ]}>
              <Text style={styles.statusText}>{contact.status}</Text>
            </View>
          </TouchableOpacity>
        ))}
        {(!contacts || contacts.length === 0) && (
          <Text style={styles.emptyText}>No contacts yet</Text>
        )}
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f9fafb',
  },
  header: {
    padding: 20,
    paddingTop: 40,
    backgroundColor: '#2563eb',
  },
  greeting: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#fff',
  },
  subtitle: {
    fontSize: 14,
    color: '#bfdbfe',
    marginTop: 4,
  },
  statsContainer: {
    flexDirection: 'row',
    padding: 16,
    gap: 12,
  },
  statCard: {
    flex: 1,
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: {width: 0, height: 1},
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 2,
  },
  statValue: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#111827',
  },
  statLabel: {
    fontSize: 12,
    color: '#6b7280',
    marginTop: 4,
  },
  section: {
    padding: 16,
  },
  sectionTitle: {
    fontSize: 18,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 12,
  },
  eventCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    marginBottom: 8,
    shadowColor: '#000',
    shadowOffset: {width: 0, height: 1},
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 2,
  },
  eventBadge: {
    backgroundColor: '#dbeafe',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 4,
    alignSelf: 'flex-start',
    marginBottom: 8,
  },
  eventBadgeText: {
    fontSize: 10,
    fontWeight: '600',
    color: '#1d4ed8',
    textTransform: 'uppercase',
  },
  eventName: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
  },
  eventDate: {
    fontSize: 12,
    color: '#6b7280',
    marginTop: 4,
  },
  contactCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 12,
    marginBottom: 8,
    flexDirection: 'row',
    alignItems: 'center',
    shadowColor: '#000',
    shadowOffset: {width: 0, height: 1},
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 2,
  },
  avatar: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: '#dbeafe',
    justifyContent: 'center',
    alignItems: 'center',
  },
  avatarText: {
    fontSize: 14,
    fontWeight: '600',
    color: '#1d4ed8',
  },
  contactInfo: {
    flex: 1,
    marginLeft: 12,
  },
  contactName: {
    fontSize: 14,
    fontWeight: '600',
    color: '#111827',
  },
  contactEmail: {
    fontSize: 12,
    color: '#6b7280',
  },
  statusBadge: {
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 4,
    backgroundColor: '#e5e7eb',
  },
  statusCustomer: {
    backgroundColor: '#d1fae5',
  },
  statusLead: {
    backgroundColor: '#dbeafe',
  },
  statusText: {
    fontSize: 10,
    fontWeight: '600',
    color: '#374151',
    textTransform: 'capitalize',
  },
  emptyText: {
    textAlign: 'center',
    color: '#6b7280',
    paddingVertical: 20,
  },
});
