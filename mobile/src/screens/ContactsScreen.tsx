import React, {useState} from 'react';
import {
  View,
  Text,
  StyleSheet,
  FlatList,
  TouchableOpacity,
  TextInput,
  RefreshControl,
} from 'react-native';
import {useNavigation} from '@react-navigation/native';
import {useQuery} from '@tanstack/react-query';
import {api, Contact} from '../lib/api';
import Icon from 'react-native-vector-icons/Feather';

export default function ContactsScreen() {
  const navigation = useNavigation();
  const [search, setSearch] = useState('');

  const {data: contacts, isLoading, refetch} = useQuery({
    queryKey: ['contacts'],
    queryFn: () => api.contacts.list(),
  });

  const filteredContacts = contacts?.filter(contact => {
    const fullName = `${contact.first_name} ${contact.last_name}`.toLowerCase();
    const email = contact.email.toLowerCase();
    const searchLower = search.toLowerCase();
    return fullName.includes(searchLower) || email.includes(searchLower);
  });

  const renderContact = ({item}: {item: Contact}) => (
    <TouchableOpacity
      style={styles.contactCard}
      onPress={() =>
        navigation.navigate('ContactDetail' as never, {id: item.id} as never)
      }>
      <View style={styles.avatar}>
        <Text style={styles.avatarText}>
          {item.first_name[0]}
          {item.last_name[0]}
        </Text>
      </View>
      <View style={styles.contactInfo}>
        <Text style={styles.contactName}>
          {item.first_name} {item.last_name}
        </Text>
        <Text style={styles.contactEmail}>{item.email}</Text>
        <View style={styles.tagsContainer}>
          {item.tags.slice(0, 2).map(tag => (
            <View key={tag} style={styles.tag}>
              <Text style={styles.tagText}>{tag}</Text>
            </View>
          ))}
        </View>
      </View>
      <View style={styles.rightSection}>
        <View
          style={[
            styles.statusBadge,
            item.status === 'customer' && styles.statusCustomer,
            item.status === 'lead' && styles.statusLead,
            item.status === 'partner' && styles.statusPartner,
            item.status === 'investor' && styles.statusInvestor,
          ]}>
          <Text style={styles.statusText}>{item.status}</Text>
        </View>
        <Icon name="chevron-right" size={20} color="#9ca3af" />
      </View>
    </TouchableOpacity>
  );

  return (
    <View style={styles.container}>
      {/* Search */}
      <View style={styles.searchContainer}>
        <Icon name="search" size={20} color="#9ca3af" style={styles.searchIcon} />
        <TextInput
          style={styles.searchInput}
          placeholder="Search contacts..."
          placeholderTextColor="#9ca3af"
          value={search}
          onChangeText={setSearch}
        />
        {search.length > 0 && (
          <TouchableOpacity onPress={() => setSearch('')}>
            <Icon name="x" size={20} color="#9ca3af" />
          </TouchableOpacity>
        )}
      </View>

      {/* Contacts List */}
      <FlatList
        data={filteredContacts}
        renderItem={renderContact}
        keyExtractor={item => item.id}
        contentContainerStyle={styles.listContent}
        refreshControl={
          <RefreshControl refreshing={isLoading} onRefresh={refetch} />
        }
        ListEmptyComponent={
          <View style={styles.emptyContainer}>
            <Icon name="users" size={48} color="#d1d5db" />
            <Text style={styles.emptyText}>
              {search ? 'No contacts found' : 'No contacts yet'}
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
  searchContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    backgroundColor: '#fff',
    margin: 16,
    paddingHorizontal: 12,
    borderRadius: 8,
    borderWidth: 1,
    borderColor: '#e5e7eb',
  },
  searchIcon: {
    marginRight: 8,
  },
  searchInput: {
    flex: 1,
    paddingVertical: 12,
    fontSize: 16,
    color: '#111827',
  },
  listContent: {
    padding: 16,
    paddingTop: 0,
  },
  contactCard: {
    backgroundColor: '#fff',
    borderRadius: 12,
    padding: 16,
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
    width: 48,
    height: 48,
    borderRadius: 24,
    backgroundColor: '#dbeafe',
    justifyContent: 'center',
    alignItems: 'center',
  },
  avatarText: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1d4ed8',
  },
  contactInfo: {
    flex: 1,
    marginLeft: 12,
  },
  contactName: {
    fontSize: 16,
    fontWeight: '600',
    color: '#111827',
  },
  contactEmail: {
    fontSize: 13,
    color: '#6b7280',
    marginTop: 2,
  },
  tagsContainer: {
    flexDirection: 'row',
    marginTop: 6,
    gap: 4,
  },
  tag: {
    backgroundColor: '#f3f4f6',
    paddingHorizontal: 6,
    paddingVertical: 2,
    borderRadius: 4,
  },
  tagText: {
    fontSize: 10,
    color: '#6b7280',
  },
  rightSection: {
    alignItems: 'flex-end',
    gap: 8,
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
  statusPartner: {
    backgroundColor: '#fef3c7',
  },
  statusInvestor: {
    backgroundColor: '#ede9fe',
  },
  statusText: {
    fontSize: 10,
    fontWeight: '600',
    color: '#374151',
    textTransform: 'capitalize',
  },
  emptyContainer: {
    alignItems: 'center',
    justifyContent: 'center',
    paddingVertical: 60,
  },
  emptyText: {
    marginTop: 12,
    fontSize: 16,
    color: '#6b7280',
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
