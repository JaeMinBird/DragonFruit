import { auth } from '$lib/stores/auth';
import { get } from 'svelte/store';
import { browser } from '$app/environment';

const API_URL = import.meta.env.VITE_API_URL;

interface ApiOptions {
  method?: string;
  body?: any;
  token?: string | null;
  contentType?: string;
}

export class ApiError extends Error {
  status: number;
  
  constructor(message: string, status: number) {
    super(message);
    this.status = status;
    this.name = 'ApiError';
  }
}

async function api(endpoint: string, options: ApiOptions = {}) {
  const {
    method = 'GET',
    body,
    token = get(auth).token,
    contentType = 'application/json'
  } = options;

  const headers: Record<string, string> = {
    'Content-Type': contentType
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const config: RequestInit = { method, headers };

  if (body) {
    config.body = contentType === 'application/json' ? JSON.stringify(body) : body;
  }

  const response = await fetch(`${API_URL}${endpoint}`, config);
  
  if (response.status === 401 && browser) {
    // Handle unauthorized by logging out
    auth.logout();
  }

  const data = response.status !== 204 ? await response.json() : null;

  if (!response.ok) {
    const message = data?.message || 'An error occurred';
    throw new ApiError(message, response.status);
  }

  return data;
}

export default {
  // Auth endpoints
  auth: {
    register: (userData: { email: string, password: string }) => 
      api('/auth/register', { method: 'POST', body: userData }),
    
    login: (credentials: { email: string, password: string, totp_code?: string }) => 
      api('/auth/login', { method: 'POST', body: credentials }),
    
    getProfile: () => api('/auth/profile'),
    
    updateProfile: (userData: { email?: string, password?: string }) => 
      api('/auth/profile', { method: 'PUT', body: userData }),
      
    generateTotp: () => api('/auth/totp/generate', { method: 'POST' }),
    
    enableTotp: (code: string) => 
      api('/auth/totp/enable', { method: 'POST', body: { code } })
  },
  
  // Categories endpoints
  categories: {
    getAll: () => api('/categories'),
    
    create: (categoryData: { name: string, parent_id?: string | null }) => 
      api('/categories', { method: 'POST', body: categoryData }),
    
    update: (id: string, categoryData: { name: string, parent_id?: string | null }) => 
      api(`/categories/${id}`, { method: 'PUT', body: categoryData }),
    
    delete: (id: string) => 
      api(`/categories/${id}`, { method: 'DELETE' })
  },
  
  // Credentials endpoints
  credentials: {
    getAll: () => api('/credentials'),
    
    getById: (id: string) => api(`/credentials/${id}`),
    
    getByCategory: (categoryId: string) => api(`/categories/${categoryId}/credentials`),
    
    create: (credentialData: {
      name: string,
      category_id?: string | null,
      website?: string | null,
      username?: string | null,
      password: string,
      notes?: string | null
    }) => api('/credentials', { method: 'POST', body: credentialData }),
    
    update: (id: string, credentialData: {
      name?: string,
      category_id?: string | null,
      website?: string | null,
      username?: string | null,
      password?: string,
      notes?: string | null
    }) => api(`/credentials/${id}`, { method: 'PUT', body: credentialData }),
    
    delete: (id: string) => api(`/credentials/${id}`, { method: 'DELETE' })
  }
}; 