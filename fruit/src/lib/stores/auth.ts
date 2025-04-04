import { writable } from 'svelte/store';
import { browser } from '$app/environment';

interface User {
  id: string;
  email: string;
  totp_enabled: boolean;
}

interface AuthState {
  user: User | null;
  token: string | null;
  loading: boolean;
  error: string | null;
}

const defaultState: AuthState = {
  user: null,
  token: null,
  loading: false,
  error: null
};

function createAuthStore() {
  // Initialize from localStorage if in browser
  const initialState = browser 
    ? JSON.parse(localStorage.getItem('auth') || JSON.stringify(defaultState)) 
    : defaultState;
    
  const { subscribe, set, update } = writable<AuthState>(initialState);

  return {
    subscribe,
    
    setToken: (token: string) => {
      update(state => {
        const newState = { ...state, token, error: null };
        if (browser) localStorage.setItem('auth', JSON.stringify(newState));
        return newState;
      });
    },
    
    setUser: (user: User) => {
      update(state => {
        const newState = { ...state, user, error: null };
        if (browser) localStorage.setItem('auth', JSON.stringify(newState));
        return newState;
      });
    },
    
    logout: () => {
      set(defaultState);
      if (browser) localStorage.removeItem('auth');
    },
    
    setError: (error: string) => {
      update(state => ({ ...state, error }));
    },
    
    setLoading: (loading: boolean) => {
      update(state => ({ ...state, loading }));
    }
  };
}

export const auth = createAuthStore(); 