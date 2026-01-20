import Purchases, {
  PurchasesOffering,
  CustomerInfo,
  PurchasesPackage,
  LOG_LEVEL,
} from 'react-native-purchases';
import {Platform} from 'react-native';

// RevenueCat API keys - replace with your actual keys from RevenueCat dashboard
const REVENUECAT_API_KEY_IOS = 'your_ios_api_key';
const REVENUECAT_API_KEY_ANDROID = 'your_android_api_key';

// Entitlement identifier configured in RevenueCat dashboard
export const ENTITLEMENT_ID = 'pro';

/**
 * Initialize RevenueCat SDK
 * Call this once when the app starts
 */
export async function initializeRevenueCat(userId?: string): Promise<void> {
  const apiKey =
    Platform.OS === 'ios' ? REVENUECAT_API_KEY_IOS : REVENUECAT_API_KEY_ANDROID;

  Purchases.setLogLevel(LOG_LEVEL.DEBUG);

  await Purchases.configure({
    apiKey,
    appUserID: userId,
  });
}

/**
 * Identify user with RevenueCat (call after user logs in)
 */
export async function identifyUser(userId: string): Promise<CustomerInfo> {
  return await Purchases.logIn(userId);
}

/**
 * Log out user from RevenueCat (call after user logs out)
 */
export async function logOutUser(): Promise<CustomerInfo> {
  return await Purchases.logOut();
}

/**
 * Get current customer info including active subscriptions
 */
export async function getCustomerInfo(): Promise<CustomerInfo> {
  return await Purchases.getCustomerInfo();
}

/**
 * Check if user has active pro subscription
 */
export async function hasProAccess(): Promise<boolean> {
  const customerInfo = await getCustomerInfo();
  return customerInfo.entitlements.active[ENTITLEMENT_ID] !== undefined;
}

/**
 * Get available subscription offerings
 */
export async function getOfferings(): Promise<PurchasesOffering | null> {
  const offerings = await Purchases.getOfferings();
  return offerings.current;
}

/**
 * Purchase a subscription package
 */
export async function purchasePackage(
  pkg: PurchasesPackage,
): Promise<{customerInfo: CustomerInfo; productIdentifier: string}> {
  const {customerInfo, productIdentifier} = await Purchases.purchasePackage(pkg);
  return {customerInfo, productIdentifier};
}

/**
 * Restore previous purchases
 */
export async function restorePurchases(): Promise<CustomerInfo> {
  return await Purchases.restorePurchases();
}

/**
 * Get subscription management URL (for cancellation, etc.)
 */
export async function getManagementURL(): Promise<string | null> {
  const customerInfo = await getCustomerInfo();
  return customerInfo.managementURL;
}
