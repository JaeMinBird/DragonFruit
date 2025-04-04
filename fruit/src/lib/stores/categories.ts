import { writable } from 'svelte/store';
import api from '$lib/api';
import type { Category } from '$lib/types';

interface CategoryState {
  categories: Category[];
  loading: boolean;
  error: string | null;
}

const initialState: CategoryState = {
  categories: [],
  loading: false,
  error: null
};

function createCategoryStore() {
  const { subscribe, set, update } = writable<CategoryState>(initialState);

  return {
    subscribe,

    fetchCategories: async () => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const categories = await api.categories.getAll();
        update(state => ({ ...state, categories, loading: false }));
        return categories;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to fetch categories';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    addCategory: async (name: string, parentId?: string | null) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const newCategory = await api.categories.create({ name, parent_id: parentId });
        
        // Refetch to get the updated nested structure
        await categories.fetchCategories();
        
        return newCategory;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to add category';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    updateCategory: async (id: string, name: string, parentId?: string | null) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        const updatedCategory = await api.categories.update(id, { name, parent_id: parentId });
        
        // Refetch to get the updated nested structure
        await categories.fetchCategories();
        
        return updatedCategory;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to update category';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    deleteCategory: async (id: string) => {
      update(state => ({ ...state, loading: true, error: null }));
      
      try {
        await api.categories.delete(id);
        
        // Refetch to get the updated nested structure
        await categories.fetchCategories();
        
        return true;
      } catch (err) {
        const message = err instanceof Error ? err.message : 'Failed to delete category';
        update(state => ({ ...state, loading: false, error: message }));
        throw err;
      }
    },

    reset: () => {
      set(initialState);
    }
  };
}

export const categories = createCategoryStore(); 