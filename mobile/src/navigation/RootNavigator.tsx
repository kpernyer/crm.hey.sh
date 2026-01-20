import React from 'react';
import {createBottomTabNavigator} from '@react-navigation/bottom-tabs';
import {createNativeStackNavigator} from '@react-navigation/native-stack';
import Icon from 'react-native-vector-icons/Feather';

import HomeScreen from '../screens/HomeScreen';
import ContactsScreen from '../screens/ContactsScreen';
import ContactDetailScreen from '../screens/ContactDetailScreen';
import EventsScreen from '../screens/EventsScreen';
import NotificationsScreen from '../screens/NotificationsScreen';
import SubscriptionScreen from '../screens/SubscriptionScreen';

const Tab = createBottomTabNavigator();
const Stack = createNativeStackNavigator();

function ContactsStack() {
  return (
    <Stack.Navigator>
      <Stack.Screen
        name="ContactsList"
        component={ContactsScreen}
        options={{title: 'Contacts'}}
      />
      <Stack.Screen
        name="ContactDetail"
        component={ContactDetailScreen}
        options={{title: 'Contact'}}
      />
    </Stack.Navigator>
  );
}

export function RootNavigator() {
  return (
    <Tab.Navigator
      screenOptions={({route}) => ({
        tabBarIcon: ({color, size}) => {
          let iconName: string;

          switch (route.name) {
            case 'Home':
              iconName = 'home';
              break;
            case 'Contacts':
              iconName = 'users';
              break;
            case 'Events':
              iconName = 'calendar';
              break;
            case 'Notifications':
              iconName = 'bell';
              break;
            case 'Pro':
              iconName = 'star';
              break;
            default:
              iconName = 'circle';
          }

          return <Icon name={iconName} size={size} color={color} />;
        },
        tabBarActiveTintColor: '#2563eb',
        tabBarInactiveTintColor: 'gray',
        headerShown: route.name !== 'Contacts',
      })}>
      <Tab.Screen name="Home" component={HomeScreen} />
      <Tab.Screen
        name="Contacts"
        component={ContactsStack}
        options={{headerShown: false}}
      />
      <Tab.Screen name="Events" component={EventsScreen} />
      <Tab.Screen name="Notifications" component={NotificationsScreen} />
      <Tab.Screen name="Pro" component={SubscriptionScreen} />
    </Tab.Navigator>
  );
}
