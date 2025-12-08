import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  ScrollView,
  TouchableOpacity,
  Linking,
} from 'react-native';
import {useRoute} from '@react-navigation/native';
import {useQuery} from '@tanstack/react-query';
import {api} from '../lib/api';
import {format, parseISO} from 'date-fns';
import Icon from 'react-native-vector-icons/Feather';

export default function ContactDetailScreen() {
  const route = useRoute();
  const {id} = route.params as {id: string};

  const {data: contact, isLoading: contactLoading} = useQuery({
    queryKey: ['contact', id],
    queryFn: () => api.contacts.get(id),
  });

  const {data: timeline, isLoading: timelineLoading} = useQuery({
    queryKey: ['contact-timeline', id],
    queryFn: () => api.contacts.timeline(id),
  });

  if (contactLoading || !contact) {
    return (
      <View style={styles.loadingContainer}>
        <Text>Loading...</Text>
      </View>
    );
  }

  return (
    <ScrollView style={styles.container}>
      {/* Header */}
      <View style={styles.header}>
        <View style={styles.avatar}>
          <Text style={styles.avatarText}>
            {contact.first_name[0]}
            {contact.last_name[0]}
          </Text>
        </View>
        <Text style={styles.name}>
          {contact.first_name} {contact.last_name}
        </Text>
        <View style={styles.statusBadge}>
          <Text style={styles.statusText}>{contact.status}</Text>
        </View>
        <View style={styles.engagementContainer}>
          <Text style={styles.engagementLabel}>Engagement Score</Text>
          <View style={styles.engagementBar}>
            <View
              style={[
                styles.engagementFill,
                {width: `${Math.min(contact.engagement_score, 100)}%`},
              ]}
            />
          </View>
          <Text style={styles.engagementValue}>
            {contact.engagement_score.toFixed(0)}%
          </Text>
        </View>
      </View>

      {/* Quick Actions */}
      <View style={styles.actionsContainer}>
        <TouchableOpacity
          style={styles.actionButton}
          onPress={() => Linking.openURL(`mailto:${contact.email}`)}>
          <Icon name="mail" size={20} color="#2563eb" />
          <Text style={styles.actionText}>Email</Text>
        </TouchableOpacity>
        {contact.phone && (
          <TouchableOpacity
            style={styles.actionButton}
            onPress={() => Linking.openURL(`tel:${contact.phone}`)}>
            <Icon name="phone" size={20} color="#2563eb" />
            <Text style={styles.actionText}>Call</Text>
          </TouchableOpacity>
        )}
        <TouchableOpacity style={styles.actionButton}>
          <Icon name="edit-3" size={20} color="#2563eb" />
          <Text style={styles.actionText}>Note</Text>
        </TouchableOpacity>
      </View>

      {/* Contact Info */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>Contact Information</Text>
        <View style={styles.infoCard}>
          <View style={styles.infoRow}>
            <Icon name="mail" size={16} color="#6b7280" />
            <Text style={styles.infoText}>{contact.email}</Text>
          </View>
          {contact.phone && (
            <View style={styles.infoRow}>
              <Icon name="phone" size={16} color="#6b7280" />
              <Text style={styles.infoText}>{contact.phone}</Text>
            </View>
          )}
          {contact.linkedin_url && (
            <TouchableOpacity
              style={styles.infoRow}
              onPress={() => Linking.openURL(contact.linkedin_url!)}>
              <Icon name="linkedin" size={16} color="#6b7280" />
              <Text style={[styles.infoText, styles.linkText]}>
                LinkedIn Profile
              </Text>
            </TouchableOpacity>
          )}
        </View>
      </View>

      {/* Tags */}
      {contact.tags.length > 0 && (
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>Tags</Text>
          <View style={styles.tagsContainer}>
            {contact.tags.map(tag => (
              <View key={tag} style={styles.tag}>
                <Text style={styles.tagText}>{tag}</Text>
              </View>
            ))}
          </View>
        </View>
      )}

      {/* Timeline */}
      <View style={styles.section}>
        <Text style={styles.sectionTitle}>Timeline</Text>
        {timelineLoading ? (
          <Text style={styles.loadingText}>Loading timeline...</Text>
        ) : timeline?.length === 0 ? (
          <Text style={styles.emptyText}>No activity yet</Text>
        ) : (
          <View style={styles.timelineContainer}>
            {timeline?.map((entry, index) => (
              <View key={entry.id} style={styles.timelineItem}>
                <View style={styles.timelineDot} />
                {index < (timeline?.length ?? 0) - 1 && (
                  <View style={styles.timelineLine} />
                )}
                <View style={styles.timelineContent}>
                  <Text style={styles.timelineText}>{entry.content}</Text>
                  <Text style={styles.timelineDate}>
                    {format(parseISO(entry.timestamp), 'MMM d, yyyy h:mm a')}
                  </Text>
                </View>
              </View>
            ))}
          </View>
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
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  header: {
    backgroundColor: '#fff',
    padding: 24,
    alignItems: 'center',
    borderBottomWidth: 1,
    borderBottomColor: '#e5e7eb',
  },
  avatar: {
    width: 80,
    height: 80,
    borderRadius: 40,
    backgroundColor: '#dbeafe',
    justifyContent: 'center',
    alignItems: 'center',
  },
  avatarText: {
    fontSize: 28,
    fontWeight: '700',
    color: '#1d4ed8',
  },
  name: {
    fontSize: 24,
    fontWeight: '700',
    color: '#111827',
    marginTop: 12,
  },
  statusBadge: {
    marginTop: 8,
    paddingHorizontal: 12,
    paddingVertical: 4,
    borderRadius: 16,
    backgroundColor: '#dbeafe',
  },
  statusText: {
    fontSize: 12,
    fontWeight: '600',
    color: '#1d4ed8',
    textTransform: 'capitalize',
  },
  engagementContainer: {
    marginTop: 16,
    alignItems: 'center',
    width: '100%',
  },
  engagementLabel: {
    fontSize: 12,
    color: '#6b7280',
  },
  engagementBar: {
    width: '60%',
    height: 8,
    backgroundColor: '#e5e7eb',
    borderRadius: 4,
    marginTop: 6,
  },
  engagementFill: {
    height: '100%',
    backgroundColor: '#2563eb',
    borderRadius: 4,
  },
  engagementValue: {
    marginTop: 4,
    fontSize: 14,
    fontWeight: '600',
    color: '#111827',
  },
  actionsContainer: {
    flexDirection: 'row',
    justifyContent: 'center',
    gap: 24,
    padding: 16,
    backgroundColor: '#fff',
    borderBottomWidth: 1,
    borderBottomColor: '#e5e7eb',
  },
  actionButton: {
    alignItems: 'center',
    gap: 4,
  },
  actionText: {
    fontSize: 12,
    color: '#2563eb',
  },
  section: {
    padding: 16,
  },
  sectionTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
    marginBottom: 12,
  },
  infoCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
  },
  infoRow: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 12,
    paddingVertical: 8,
  },
  infoText: {
    fontSize: 14,
    color: '#374151',
  },
  linkText: {
    color: '#2563eb',
  },
  tagsContainer: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    gap: 8,
  },
  tag: {
    backgroundColor: '#dbeafe',
    paddingHorizontal: 12,
    paddingVertical: 6,
    borderRadius: 16,
  },
  tagText: {
    fontSize: 12,
    fontWeight: '500',
    color: '#1d4ed8',
  },
  timelineContainer: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
  },
  timelineItem: {
    flexDirection: 'row',
    paddingBottom: 16,
  },
  timelineDot: {
    width: 12,
    height: 12,
    borderRadius: 6,
    backgroundColor: '#2563eb',
    marginTop: 4,
  },
  timelineLine: {
    position: 'absolute',
    left: 5,
    top: 16,
    bottom: 0,
    width: 2,
    backgroundColor: '#e5e7eb',
  },
  timelineContent: {
    flex: 1,
    marginLeft: 12,
  },
  timelineText: {
    fontSize: 14,
    color: '#374151',
  },
  timelineDate: {
    fontSize: 12,
    color: '#9ca3af',
    marginTop: 4,
  },
  loadingText: {
    color: '#6b7280',
    textAlign: 'center',
    padding: 20,
  },
  emptyText: {
    color: '#6b7280',
    textAlign: 'center',
    padding: 20,
  },
});
