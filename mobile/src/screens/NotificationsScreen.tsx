import React from 'react';
import {View, Text, StyleSheet, ScrollView} from 'react-native';
import Icon from 'react-native-vector-icons/Feather';

export default function NotificationsScreen() {
  // Placeholder notifications
  const notifications = [
    {
      id: '1',
      type: 'email_open',
      title: 'Email Opened',
      message: 'John Smith opened your campaign email',
      time: '5 minutes ago',
      read: false,
    },
    {
      id: '2',
      type: 'event_rsvp',
      title: 'New RSVP',
      message: 'Sarah Chen registered for Product Demo',
      time: '1 hour ago',
      read: false,
    },
    {
      id: '3',
      type: 'landing_page',
      title: 'Form Submission',
      message: 'New lead from Launch Landing Page',
      time: '2 hours ago',
      read: true,
    },
    {
      id: '4',
      type: 'campaign',
      title: 'Campaign Completed',
      message: 'Q4 Investor Outreach campaign finished',
      time: '3 hours ago',
      read: true,
    },
  ];

  const getIcon = (type: string) => {
    switch (type) {
      case 'email_open':
        return 'mail';
      case 'event_rsvp':
        return 'calendar';
      case 'landing_page':
        return 'file-text';
      case 'campaign':
        return 'rocket';
      default:
        return 'bell';
    }
  };

  const getIconColor = (type: string) => {
    switch (type) {
      case 'email_open':
        return '#2563eb';
      case 'event_rsvp':
        return '#059669';
      case 'landing_page':
        return '#d97706';
      case 'campaign':
        return '#7c3aed';
      default:
        return '#6b7280';
    }
  };

  const getIconBg = (type: string) => {
    switch (type) {
      case 'email_open':
        return '#dbeafe';
      case 'event_rsvp':
        return '#d1fae5';
      case 'landing_page':
        return '#fef3c7';
      case 'campaign':
        return '#ede9fe';
      default:
        return '#f3f4f6';
    }
  };

  return (
    <ScrollView style={styles.container}>
      <View style={styles.header}>
        <Text style={styles.title}>Notifications</Text>
        <Text style={styles.subtitle}>Stay updated with your CRM activity</Text>
      </View>

      {notifications.length === 0 ? (
        <View style={styles.emptyContainer}>
          <Icon name="bell-off" size={48} color="#d1d5db" />
          <Text style={styles.emptyText}>No notifications</Text>
          <Text style={styles.emptySubtext}>
            You'll be notified about important CRM activity here
          </Text>
        </View>
      ) : (
        <View style={styles.notificationsContainer}>
          {notifications.map(notification => (
            <View
              key={notification.id}
              style={[
                styles.notificationCard,
                !notification.read && styles.unreadCard,
              ]}>
              <View
                style={[
                  styles.iconContainer,
                  {backgroundColor: getIconBg(notification.type)},
                ]}>
                <Icon
                  name={getIcon(notification.type)}
                  size={20}
                  color={getIconColor(notification.type)}
                />
              </View>
              <View style={styles.notificationContent}>
                <View style={styles.notificationHeader}>
                  <Text style={styles.notificationTitle}>
                    {notification.title}
                  </Text>
                  {!notification.read && <View style={styles.unreadDot} />}
                </View>
                <Text style={styles.notificationMessage}>
                  {notification.message}
                </Text>
                <Text style={styles.notificationTime}>{notification.time}</Text>
              </View>
            </View>
          ))}
        </View>
      )}

      {/* Settings hint */}
      <View style={styles.settingsHint}>
        <Icon name="settings" size={16} color="#6b7280" />
        <Text style={styles.settingsText}>
          Configure notification preferences in Settings
        </Text>
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
    paddingTop: 10,
  },
  title: {
    fontSize: 24,
    fontWeight: '700',
    color: '#111827',
  },
  subtitle: {
    fontSize: 14,
    color: '#6b7280',
    marginTop: 4,
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
    paddingHorizontal: 40,
  },
  notificationsContainer: {
    padding: 16,
  },
  notificationCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
    marginBottom: 8,
    flexDirection: 'row',
    shadowColor: '#000',
    shadowOffset: {width: 0, height: 1},
    shadowOpacity: 0.05,
    shadowRadius: 2,
    elevation: 2,
  },
  unreadCard: {
    backgroundColor: '#f0f9ff',
    borderWidth: 1,
    borderColor: '#bfdbfe',
  },
  iconContainer: {
    width: 44,
    height: 44,
    borderRadius: 22,
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: 12,
  },
  notificationContent: {
    flex: 1,
  },
  notificationHeader: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  notificationTitle: {
    fontSize: 15,
    fontWeight: '600',
    color: '#111827',
    flex: 1,
  },
  unreadDot: {
    width: 8,
    height: 8,
    borderRadius: 4,
    backgroundColor: '#2563eb',
  },
  notificationMessage: {
    fontSize: 13,
    color: '#6b7280',
    marginTop: 4,
    lineHeight: 18,
  },
  notificationTime: {
    fontSize: 12,
    color: '#9ca3af',
    marginTop: 6,
  },
  settingsHint: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    padding: 20,
    gap: 8,
  },
  settingsText: {
    fontSize: 13,
    color: '#6b7280',
  },
});
