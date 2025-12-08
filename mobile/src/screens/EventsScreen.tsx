import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  FlatList,
  TouchableOpacity,
  RefreshControl,
} from 'react-native';
import {useQuery} from '@tanstack/react-query';
import {api, Event} from '../lib/api';
import {format, parseISO} from 'date-fns';
import Icon from 'react-native-vector-icons/Feather';

const eventTypeColors: Record<string, string> = {
  webinar: '#dbeafe',
  meetup: '#d1fae5',
  ama: '#fef3c7',
  demo: '#ede9fe',
  other: '#f3f4f6',
};

const eventTypeTextColors: Record<string, string> = {
  webinar: '#1d4ed8',
  meetup: '#059669',
  ama: '#d97706',
  demo: '#7c3aed',
  other: '#374151',
};

export default function EventsScreen() {
  const {data: events, isLoading, refetch} = useQuery({
    queryKey: ['events'],
    queryFn: () => api.events.list(),
  });

  const renderEvent = ({item}: {item: Event}) => (
    <TouchableOpacity style={styles.eventCard}>
      <View style={styles.dateContainer}>
        <Text style={styles.dateDay}>
          {format(parseISO(item.start_time), 'd')}
        </Text>
        <Text style={styles.dateMonth}>
          {format(parseISO(item.start_time), 'MMM')}
        </Text>
      </View>
      <View style={styles.eventInfo}>
        <View
          style={[
            styles.typeBadge,
            {backgroundColor: eventTypeColors[item.type] || '#f3f4f6'},
          ]}>
          <Text
            style={[
              styles.typeText,
              {color: eventTypeTextColors[item.type] || '#374151'},
            ]}>
            {item.type}
          </Text>
        </View>
        <Text style={styles.eventName}>{item.name}</Text>
        <Text style={styles.eventDescription} numberOfLines={2}>
          {item.description}
        </Text>
        <View style={styles.eventMeta}>
          <View style={styles.metaItem}>
            <Icon name="clock" size={12} color="#6b7280" />
            <Text style={styles.metaText}>
              {format(parseISO(item.start_time), 'h:mm a')}
            </Text>
          </View>
          <View style={styles.metaItem}>
            <Icon name="map-pin" size={12} color="#6b7280" />
            <Text style={styles.metaText} numberOfLines={1}>
              {item.location}
            </Text>
          </View>
        </View>
      </View>
    </TouchableOpacity>
  );

  return (
    <View style={styles.container}>
      <FlatList
        data={events}
        renderItem={renderEvent}
        keyExtractor={item => item.id}
        contentContainerStyle={styles.listContent}
        refreshControl={
          <RefreshControl refreshing={isLoading} onRefresh={refetch} />
        }
        ListEmptyComponent={
          <View style={styles.emptyContainer}>
            <Icon name="calendar" size={48} color="#d1d5db" />
            <Text style={styles.emptyText}>No events scheduled</Text>
            <Text style={styles.emptySubtext}>
              Create an event to start inviting contacts
            </Text>
          </View>
        }
      />

      {/* Add Button */}
      <TouchableOpacity style={styles.addButton}>
        <Icon name="plus" size={24} color="#fff" />
      </TouchableOpacity>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#f9fafb',
  },
  listContent: {
    padding: 16,
  },
  eventCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    marginBottom: 12,
    flexDirection: 'row',
    shadowColor: '#000',
    shadowOffset: {width: 0, height: 1},
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 2,
  },
  dateContainer: {
    width: 50,
    alignItems: 'center',
    justifyContent: 'center',
    marginRight: 16,
    backgroundColor: '#f3f4f6',
    borderRadius: 8,
    paddingVertical: 8,
  },
  dateDay: {
    fontSize: 20,
    fontWeight: '700',
    color: '#111827',
  },
  dateMonth: {
    fontSize: 12,
    fontWeight: '500',
    color: '#6b7280',
    textTransform: 'uppercase',
  },
  eventInfo: {
    flex: 1,
  },
  typeBadge: {
    alignSelf: 'flex-start',
    paddingHorizontal: 8,
    paddingVertical: 4,
    borderRadius: 4,
    marginBottom: 8,
  },
  typeText: {
    fontSize: 10,
    fontWeight: '600',
    textTransform: 'uppercase',
  },
  eventName: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
  },
  eventDescription: {
    fontSize: 13,
    color: '#6b7280',
    marginTop: 4,
    lineHeight: 18,
  },
  eventMeta: {
    flexDirection: 'row',
    marginTop: 12,
    gap: 16,
  },
  metaItem: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: 4,
  },
  metaText: {
    fontSize: 12,
    color: '#6b7280',
  },
  emptyContainer: {
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 60,
  },
  emptyText: {
    marginTop: 12,
    fontSize: 16,
    fontWeight: '600',
    color: '#374151',
  },
  emptySubtext: {
    marginTop: 4,
    fontSize: 14,
    color: '#6b7280',
    textAlign: 'center',
  },
  addButton: {
    position: 'absolute',
    bottom: 24,
    right: 24,
    width: 56,
    height: 56,
    borderRadius: 28,
    backgroundColor: '#2563eb',
    justifyContent: 'center',
    alignItems: 'center',
    shadowColor: '#2563eb',
    shadowOffset: {width: 0, height: 4},
    shadowOpacity: 0.3,
    shadowRadius: 8,
    elevation: 8,
  },
});
