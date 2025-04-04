<script lang="ts">
  import { onMount } from 'svelte';
  import { categories } from '$lib/stores/categories';
  import type { Category } from '$lib/types';

  let name = '';
  let selectedParentId: string | null = null;
  let editingCategory: Category | null = null;
  let loading = false;
  let error = '';
  let success = '';

  onMount(async () => {
    try {
      await categories.fetchCategories();
    } catch (err) {
      error = 'Failed to load categories. Please refresh the page.';
      console.error(err);
    }
  });

  async function handleSubmit() {
    if (!name.trim()) {
      error = 'Category name is required';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      if (editingCategory) {
        // Update existing category
        await categories.updateCategory(editingCategory.id, name, selectedParentId);
        success = `Category "${name}" updated successfully`;
      } else {
        // Create new category
        await categories.addCategory(name, selectedParentId);
        success = `Category "${name}" created successfully`;
      }

      // Reset form
      resetForm();
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to save category';
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function handleDelete(category: Category) {
    if (!confirm(`Are you sure you want to delete "${category.name}"?`)) {
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      await categories.deleteCategory(category.id);
      success = `Category "${category.name}" deleted successfully`;
      
      // If we were editing this category, reset the form
      if (editingCategory?.id === category.id) {
        resetForm();
      }
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to delete category';
      console.error(err);
    } finally {
      loading = false;
    }
  }

  function editCategory(category: Category) {
    editingCategory = category;
    name = category.name;
    selectedParentId = category.parent_id;
  }

  function resetForm() {
    name = '';
    selectedParentId = null;
    editingCategory = null;
  }

  // Recursive function to check if a category is a descendant of another
  function isDescendant(categoryId: string, potentialParentId: string | null): boolean {
    if (!potentialParentId) return false;
    
    const parent = $categories.categories.find(c => c.id === potentialParentId);
    if (!parent) return false;
    
    if (parent.parent_id === categoryId) return true;
    
    if (parent.parent_id) {
      return isDescendant(categoryId, parent.parent_id);
    }
    
    return false;
  }

  // Filter out the current category and its descendants from parent options
  $: parentOptions = editingCategory 
    ? $categories.categories.filter(c => {
        if (!editingCategory?.id) return true;
        return c.id !== editingCategory.id && !isDescendant(editingCategory.id, c.id);
      })
    : $categories.categories;
</script>

<div class="categories-page">
  <h1>{editingCategory ? 'Edit Category' : 'Manage Categories'}</h1>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}
  
  {#if success}
    <div class="success">{success}</div>
  {/if}
  
  <div class="categories-container">
    <div class="category-form card">
      <h2>{editingCategory ? `Edit "${editingCategory.name}"` : 'Create New Category'}</h2>
      
      <form on:submit|preventDefault={handleSubmit}>
        <div class="form-group">
          <label for="name">Category Name</label>
          <input 
            type="text" 
            id="name" 
            bind:value={name} 
            placeholder="e.g. Work, Personal, Games"
            required
            disabled={loading}
          />
        </div>
        
        <div class="form-group">
          <label for="parent">Parent Category (Optional)</label>
          <select id="parent" bind:value={selectedParentId} disabled={loading}>
            <option value={null}>None (Top Level)</option>
            {#each parentOptions as category}
              <option value={category.id}>{category.name}</option>
            {/each}
          </select>
        </div>
        
        <div class="form-actions">
          <button type="submit" class="primary-button" disabled={loading}>
            {loading ? 'Saving...' : editingCategory ? 'Update Category' : 'Create Category'}
          </button>
          
          {#if editingCategory}
            <button type="button" class="secondary-button" on:click={resetForm} disabled={loading}>
              Cancel
            </button>
          {/if}
        </div>
      </form>
    </div>
    
    <div class="category-list card">
      <h2>Your Categories</h2>
      
      {#if $categories.loading}
        <p>Loading categories...</p>
      {:else if $categories.categories.length === 0}
        <p class="empty-message">No categories found. Create your first category!</p>
      {:else}
        <div class="category-table-container">
          <table class="category-table">
            <thead>
              <tr>
                <th>Name</th>
                <th>Parent</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              {#each $categories.categories as category}
                <tr>
                  <td>{category.name}</td>
                  <td>
                    {#if category.parent_id}
                      {$categories.categories.find(c => c.id === category.parent_id)?.name || 'Unknown'}
                    {:else}
                      None
                    {/if}
                  </td>
                  <td class="actions">
                    <button 
                      class="edit-button"
                      on:click={() => editCategory(category)}
                      disabled={loading}
                    >
                      Edit
                    </button>
                    <button 
                      class="delete-button"
                      on:click={() => handleDelete(category)}
                      disabled={loading}
                    >
                      Delete
                    </button>
                  </td>
                </tr>
                
                {#each category.children || [] as child}
                  <tr class="child-row">
                    <td>↳ {child.name}</td>
                    <td>{category.name}</td>
                    <td class="actions">
                      <button 
                        class="edit-button"
                        on:click={() => editCategory(child)}
                        disabled={loading}
                      >
                        Edit
                      </button>
                      <button 
                        class="delete-button"
                        on:click={() => handleDelete(child)}
                        disabled={loading}
                      >
                        Delete
                      </button>
                    </td>
                  </tr>
                {/each}
              {/each}
            </tbody>
          </table>
        </div>
      {/if}
    </div>
  </div>
  
  <div class="back-link">
    <a href="/dashboard">&larr; Back to Dashboard</a>
  </div>
</div>

<style>
  .categories-page {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  h1 {
    margin-bottom: 1rem;
  }
  
  .categories-container {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
  }
  
  .card {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
  }
  
  .form-group {
    margin-bottom: 1.2rem;
  }
  
  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  .form-actions {
    display: flex;
    gap: 1rem;
    margin-top: 1.5rem;
  }
  
  .primary-button, .secondary-button {
    padding: 0.6rem 1.2rem;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .primary-button {
    background-color: var(--primary-color);
    color: white;
    border: none;
  }
  
  .primary-button:hover:not(:disabled) {
    background-color: #34495e;
  }
  
  .secondary-button {
    background-color: transparent;
    color: var(--primary-color);
    border: 1px solid var(--primary-color);
  }
  
  .secondary-button:hover:not(:disabled) {
    background-color: rgba(44, 62, 80, 0.1);
  }
  
  button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
  
  .category-table-container {
    overflow-x: auto;
  }
  
  .category-table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 1rem;
  }
  
  .category-table th,
  .category-table td {
    padding: 0.8rem;
    text-align: left;
    border-bottom: 1px solid var(--border-color);
  }
  
  .category-table th {
    background-color: var(--bg-color);
    font-weight: 600;
  }
  
  .actions {
    display: flex;
    gap: 0.5rem;
  }
  
  .edit-button, .delete-button {
    padding: 0.4rem 0.6rem;
    font-size: 0.85rem;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .edit-button {
    background-color: var(--primary-color);
    color: white;
    border: none;
  }
  
  .delete-button {
    background-color: var(--error-color);
    color: white;
    border: none;
  }
  
  .child-row {
    background-color: rgba(0, 0, 0, 0.02);
  }
  
  .empty-message {
    padding: 1rem;
    text-align: center;
    color: #666;
  }
  
  .back-link {
    margin-top: 2rem;
  }
  
  .back-link a {
    color: var(--primary-color);
    text-decoration: none;
    font-weight: 500;
  }
  
  .back-link a:hover {
    text-decoration: underline;
  }
  
  @media (max-width: 768px) {
    .categories-container {
      grid-template-columns: 1fr;
    }
  }
</style> 