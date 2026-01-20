import React from 'react';
import {
  View,
  Text,
  StyleSheet,
  TouchableOpacity,
  ScrollView,
  ActivityIndicator,
  Alert,
} from 'react-native';
import Icon from 'react-native-vector-icons/Feather';
import {PurchasesPackage} from 'react-native-purchases';
import {useSubscription} from '../lib/useSubscription';

const FEATURES = [
  {icon: 'users', title: 'Unlimited Contacts', description: 'No limits on your contact list'},
  {icon: 'mail', title: 'AI Email Drafts', description: 'Generate personalized emails with AI'},
  {icon: 'trending-up', title: 'Advanced Analytics', description: 'Deep insights into your network'},
  {icon: 'zap', title: 'Priority Support', description: 'Get help when you need it'},
];

function PackageCard({
  pkg,
  isSelected,
  onSelect,
}: {
  pkg: PurchasesPackage;
  isSelected: boolean;
  onSelect: () => void;
}) {
  const product = pkg.product;

  return (
    <TouchableOpacity
      style={[styles.packageCard, isSelected && styles.packageCardSelected]}
      onPress={onSelect}>
      <View style={styles.packageHeader}>
        <Text style={styles.packageTitle}>{product.title}</Text>
        {isSelected && <Icon name="check-circle" size={20} color="#2563eb" />}
      </View>
      <Text style={styles.packagePrice}>{product.priceString}</Text>
      <Text style={styles.packageDescription}>{product.description}</Text>
    </TouchableOpacity>
  );
}

export default function SubscriptionScreen() {
  const {
    isPro,
    isLoading,
    packages,
    purchase,
    restore,
    isPurchasing,
    isRestoring,
    expirationDate,
  } = useSubscription();

  const [selectedPackage, setSelectedPackage] = React.useState<PurchasesPackage | null>(null);

  React.useEffect(() => {
    if (packages.length > 0 && !selectedPackage) {
      setSelectedPackage(packages[0]);
    }
  }, [packages, selectedPackage]);

  const handlePurchase = async () => {
    if (!selectedPackage) return;

    try {
      await purchase(selectedPackage);
      Alert.alert('Success', 'Thank you for subscribing to CRM Pro!');
    } catch (error: any) {
      if (!error.userCancelled) {
        Alert.alert('Purchase Failed', error.message || 'Something went wrong');
      }
    }
  };

  const handleRestore = async () => {
    try {
      await restore();
      Alert.alert('Restored', 'Your purchases have been restored.');
    } catch (error: any) {
      Alert.alert('Restore Failed', error.message || 'Could not restore purchases');
    }
  };

  if (isLoading) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color="#2563eb" />
      </View>
    );
  }

  if (isPro) {
    return (
      <View style={styles.container}>
        <View style={styles.proContainer}>
          <Icon name="award" size={64} color="#2563eb" />
          <Text style={styles.proTitle}>You're a Pro!</Text>
          <Text style={styles.proDescription}>
            Thank you for supporting CRM.HEY.SH
          </Text>
          {expirationDate && (
            <Text style={styles.expirationText}>
              Renews on {expirationDate.toLocaleDateString()}
            </Text>
          )}
        </View>
      </View>
    );
  }

  return (
    <ScrollView style={styles.container} contentContainerStyle={styles.content}>
      <View style={styles.header}>
        <Icon name="star" size={48} color="#2563eb" />
        <Text style={styles.title}>Upgrade to Pro</Text>
        <Text style={styles.subtitle}>
          Unlock all features and supercharge your CRM
        </Text>
      </View>

      <View style={styles.features}>
        {FEATURES.map((feature, index) => (
          <View key={index} style={styles.featureRow}>
            <View style={styles.featureIcon}>
              <Icon name={feature.icon} size={20} color="#2563eb" />
            </View>
            <View style={styles.featureText}>
              <Text style={styles.featureTitle}>{feature.title}</Text>
              <Text style={styles.featureDescription}>{feature.description}</Text>
            </View>
          </View>
        ))}
      </View>

      <View style={styles.packages}>
        {packages.map(pkg => (
          <PackageCard
            key={pkg.identifier}
            pkg={pkg}
            isSelected={selectedPackage?.identifier === pkg.identifier}
            onSelect={() => setSelectedPackage(pkg)}
          />
        ))}
      </View>

      <TouchableOpacity
        style={[styles.purchaseButton, isPurchasing && styles.buttonDisabled]}
        onPress={handlePurchase}
        disabled={isPurchasing || !selectedPackage}>
        {isPurchasing ? (
          <ActivityIndicator color="#fff" />
        ) : (
          <Text style={styles.purchaseButtonText}>Subscribe Now</Text>
        )}
      </TouchableOpacity>

      <TouchableOpacity
        style={styles.restoreButton}
        onPress={handleRestore}
        disabled={isRestoring}>
        {isRestoring ? (
          <ActivityIndicator color="#2563eb" />
        ) : (
          <Text style={styles.restoreButtonText}>Restore Purchases</Text>
        )}
      </TouchableOpacity>

      <Text style={styles.terms}>
        Payment will be charged to your App Store account. Subscription
        automatically renews unless cancelled at least 24 hours before the end
        of the current period.
      </Text>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
  },
  content: {
    padding: 20,
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
  },
  header: {
    alignItems: 'center',
    marginBottom: 32,
  },
  title: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#1f2937',
    marginTop: 16,
  },
  subtitle: {
    fontSize: 16,
    color: '#6b7280',
    marginTop: 8,
    textAlign: 'center',
  },
  features: {
    marginBottom: 32,
  },
  featureRow: {
    flexDirection: 'row',
    alignItems: 'center',
    marginBottom: 16,
  },
  featureIcon: {
    width: 40,
    height: 40,
    borderRadius: 20,
    backgroundColor: '#eff6ff',
    justifyContent: 'center',
    alignItems: 'center',
    marginRight: 12,
  },
  featureText: {
    flex: 1,
  },
  featureTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1f2937',
  },
  featureDescription: {
    fontSize: 14,
    color: '#6b7280',
    marginTop: 2,
  },
  packages: {
    marginBottom: 24,
  },
  packageCard: {
    borderWidth: 2,
    borderColor: '#e5e7eb',
    borderRadius: 12,
    padding: 16,
    marginBottom: 12,
  },
  packageCardSelected: {
    borderColor: '#2563eb',
    backgroundColor: '#eff6ff',
  },
  packageHeader: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  packageTitle: {
    fontSize: 16,
    fontWeight: '600',
    color: '#1f2937',
  },
  packagePrice: {
    fontSize: 24,
    fontWeight: 'bold',
    color: '#2563eb',
    marginTop: 8,
  },
  packageDescription: {
    fontSize: 14,
    color: '#6b7280',
    marginTop: 4,
  },
  purchaseButton: {
    backgroundColor: '#2563eb',
    paddingVertical: 16,
    borderRadius: 12,
    alignItems: 'center',
    marginBottom: 12,
  },
  buttonDisabled: {
    opacity: 0.6,
  },
  purchaseButtonText: {
    color: '#fff',
    fontSize: 18,
    fontWeight: '600',
  },
  restoreButton: {
    paddingVertical: 12,
    alignItems: 'center',
    marginBottom: 24,
  },
  restoreButtonText: {
    color: '#2563eb',
    fontSize: 16,
  },
  terms: {
    fontSize: 12,
    color: '#9ca3af',
    textAlign: 'center',
    lineHeight: 18,
  },
  proContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    padding: 20,
  },
  proTitle: {
    fontSize: 28,
    fontWeight: 'bold',
    color: '#1f2937',
    marginTop: 16,
  },
  proDescription: {
    fontSize: 16,
    color: '#6b7280',
    marginTop: 8,
    textAlign: 'center',
  },
  expirationText: {
    fontSize: 14,
    color: '#2563eb',
    marginTop: 16,
  },
});
