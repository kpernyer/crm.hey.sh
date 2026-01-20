import {useEffect, useState, useCallback} from 'react';
import {useQuery, useMutation, useQueryClient} from '@tanstack/react-query';
import {
  PurchasesOffering,
  PurchasesPackage,
  CustomerInfo,
} from 'react-native-purchases';
import {
  getCustomerInfo,
  getOfferings,
  hasProAccess,
  purchasePackage,
  restorePurchases,
  ENTITLEMENT_ID,
} from './revenuecat';

/**
 * Hook to check if user has pro subscription
 */
export function useProAccess() {
  return useQuery({
    queryKey: ['subscription', 'proAccess'],
    queryFn: hasProAccess,
    staleTime: 5 * 60 * 1000, // 5 minutes
  });
}

/**
 * Hook to get customer info
 */
export function useCustomerInfo() {
  return useQuery({
    queryKey: ['subscription', 'customerInfo'],
    queryFn: getCustomerInfo,
    staleTime: 5 * 60 * 1000,
  });
}

/**
 * Hook to get available offerings
 */
export function useOfferings() {
  return useQuery({
    queryKey: ['subscription', 'offerings'],
    queryFn: getOfferings,
    staleTime: 30 * 60 * 1000, // 30 minutes - offerings don't change often
  });
}

/**
 * Hook to purchase a subscription
 */
export function usePurchase() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: (pkg: PurchasesPackage) => purchasePackage(pkg),
    onSuccess: () => {
      // Invalidate subscription queries to refetch updated status
      queryClient.invalidateQueries({queryKey: ['subscription']});
    },
  });
}

/**
 * Hook to restore purchases
 */
export function useRestorePurchases() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: restorePurchases,
    onSuccess: () => {
      queryClient.invalidateQueries({queryKey: ['subscription']});
    },
  });
}

/**
 * Combined hook for subscription management
 * Provides all subscription-related state and actions in one place
 */
export function useSubscription() {
  const proAccess = useProAccess();
  const customerInfo = useCustomerInfo();
  const offerings = useOfferings();
  const purchase = usePurchase();
  const restore = useRestorePurchases();

  const activeSubscription = customerInfo.data?.entitlements.active[ENTITLEMENT_ID];
  const expirationDate = activeSubscription?.expirationDate
    ? new Date(activeSubscription.expirationDate)
    : null;

  return {
    // Status
    isPro: proAccess.data ?? false,
    isLoading: proAccess.isLoading || offerings.isLoading,

    // Customer info
    customerInfo: customerInfo.data,
    expirationDate,

    // Offerings
    offerings: offerings.data,
    packages: offerings.data?.availablePackages ?? [],

    // Actions
    purchase: purchase.mutateAsync,
    restore: restore.mutateAsync,
    isPurchasing: purchase.isPending,
    isRestoring: restore.isPending,
    purchaseError: purchase.error,
    restoreError: restore.error,

    // Refetch
    refetch: () => {
      proAccess.refetch();
      customerInfo.refetch();
    },
  };
}
