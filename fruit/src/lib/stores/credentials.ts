import { writable } from 'svelte/store';
import api from '$lib/api';
import type { Credential } from '$lib/types';

interface CredentialState {
  credentials: Credential[];
  loading: boolean;
  error: string | null;
}

const initialState: CredentialState = {
  credentials: [],
  loading: false,
  error: null
};

function createCredentialStore() {
  const { subscribe, set, update } = writable<CredentialState>(initialState);

  return {
    subscribe,

    fetchAll: async () => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const credentials = await api.credentials.getAll();
        update(state => ({ ...state, credentials, loading: false }));
        return credentials;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to fetch credentials';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    fetchByCategory: async (categoryId: string) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const credentials = await api.credentials.getByCategory(categoryId);
        update(state => ({ ...state, credentials, loading: false }));
        return credentials;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to fetch credentials';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    getById: async (id: string) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const credential = await api.credentials.getById(id);
        update(state => ({ ...state, loading: false }));
        return credential;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to fetch credential';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    create: async (credentialData: {
      name: string,
      category_id?: string | null,
      website?: string | null,
      username?: string | null,
      password: string,
      notes?: string | null
    }) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const newCredential = await api.credentials.create(credentialData);
        
        // Refetch to get updated data
        await credentials.fetchAll();
        
        return newCredential;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to create credential';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    update: async (id: string, credentialData: {
      name?: string,
      category_id?: string | null,
      website?: string | null,
      username?: string | null,
      password?: string,
      notes?: string | null
    }) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const updatedCredential = await api.credentials.update(id, credentialData);
        
        // Refetch to get updated data
        await credentials.fetchAll();
        
        return updatedCredential;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to update credential';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    delete: async (id: string) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        await api.credentials.delete(id);
        
        // Refetch to get updated data
        await credentials.fetchAll();
        
        return true;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to delete credential';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    reset: () => {
      set(initialState);
    }
  };
}

export const credentials = createCredentialStore(); 