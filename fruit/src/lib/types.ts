export interface User {
  id: string;
  email: string;
  totp_enabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface Category {
  id: string;
  name: string;
  parent_id: string | null;
  created_at: string;
  updated_at: string;
  children: Category[];
}

export interface Credential {
  id: string;
  name: string;
  category_id: string | null;
  website: string | null;
  username: string | null;
  password?: string; // Only included in single credential response
  notes: string | null;
  created_at: string;
  updated_at: string;
} 