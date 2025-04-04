<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { credentials } from '$lib/stores/credentials';
  import { categories } from '$lib/stores/categories';
  import type { Credential } from '$lib/types';

  // Filter and sort variables
  let searchQuery = '';
  let selectedCategoryId: string | null = null;
  let sortBy: 'name' | 'created_at' | 'updated_at' = 'name';
  let sortDirection: 'asc' | 'desc' = 'asc';
  
  let error = '';

  onMount(async () => {
    try {
      // Load data for filtering and display
      await Promise.all([
        credentials.fetchAll(),
        categories.fetchCategories()
      ]);
    } catch (err) {
      error = 'Failed to load data. Please refresh the page.';
      console.error(err);
    }
  });

  function viewCredential(id: string) {
    goto(`/dashboard/credentials/${id}`);
  }

  function createNewCredential() {
    goto('/dashboard/credentials/new');
  }

  $: filteredCredentials = filterCredentials($credentials.credentials, searchQuery, selectedCategoryId);
  
  $: sortedCredentials = sortCredentials(filteredCredentials, sortBy, sortDirection);

  function filterCredentials(creds: Credential[], query: string, categoryId: string | null): Credential[] {
    return creds.filter(cred => {
      // First filter by search query
      const matchesQuery = !query || 
        cred.name.toLowerCase().includes(query.toLowerCase()) ||
        (cred.website && cred.website.toLowerCase().includes(query.toLowerCase())) ||
        (cred.username && cred.username.toLowerCase().includes(query.toLowerCase())) ||
        (cred.notes && cred.notes.toLowerCase().includes(query.toLowerCase()));
      
      // Then filter by category
      const matchesCategory = !categoryId || cred.category_id === categoryId;
      
      return matchesQuery && matchesCategory;
    });
  }

  function sortCredentials(creds: Credential[], by: string, direction: 'asc' | 'desc'): Credential[] {
    return [...creds].sort((a, b) => {
      let comparison = 0;
      
      switch (by) {
        case 'name':
          comparison = a.name.localeCompare(b.name);
          break;
        case 'created_at':
          comparison = new Date(a.created_at).getTime() - new Date(b.created_at).getTime();
          break;
        case 'updated_at':
          comparison = new Date(a.updated_at).getTime() - new Date(b.updated_at).getTime();
          break;
      }
      
      return direction === 'asc' ? comparison : -comparison;
    });
  }

  function toggleSortDirection() {
    sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
  }

  function setSortBy(column: 'name' | 'created_at' | 'updated_at') {
    if (sortBy === column) {
      toggleSortDirection();
    } else {
      sortBy = column;
      sortDirection = 'asc';
    }
  }

  function getCategoryName(categoryId: string | null): string {
    if (!categoryId) return 'None';
    
    // Try to find in top-level categories
    const category = $categories.categories.find(c => c.id === categoryId);
    if (category) return category.name;
    
    // Look for it in subcategories
    for (const parent of $categories.categories) {
      if (parent.children) {
        const child = parent.children.find(c => c.id === categoryId);
        if (child) return `${parent.name} > ${child.name}`;
      }
    }
    
    return 'Unknown';
  }
</script>

<div class="credentials-page">
  <div class="header">
    <h1>All Credentials</h1>
    <button class="create-button" on:click={createNewCredential}>
      Add New Credential
    </button>
  </div>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}
  
  <div class="filters card">
    <div class="filter-row">
      <div class="search-field">
        <label for="search">Search</label>
        <input 
          type="text" 
          id="search" 
          placeholder="Search by name, website, username..." 
          bind:value={searchQuery}
        />
      </div>
      
      <div class="category-filter">
        <label for="category">Filter by Category</label>
        <select id="category" bind:value={selectedCategoryId}>
          <option value={null}>All Categories</option>
          {#each $categories.categories as category}
            <option value={category.id}>{category.name}</option>
            {#each category.children || [] as child}
              <option value={child.id}>↳ {child.name}</option>
            {/each}
          {/each}
        </select>
      </div>
    </div>
  </div>
  
  <div class="credentials-container card">
    {#if $credentials.loading}
      <div class="loading">Loading credentials...</div>
    {:else if sortedCredentials.length === 0}
      <div class="empty-message">
        <p>No credentials found matching your criteria.</p>
        <button class="create-button small" on:click={createNewCredential}>
          Add Your First Credential
        </button>
      </div>
    {:else}
      <div class="results-info">
        <span>Found {sortedCredentials.length} credential{sortedCredentials.length !== 1 ? 's' : ''}</span>
      </div>
      
      <div class="table-container">
        <table class="credentials-table">
          <thead>
            <tr>
              <th class="sort-header" on:click={() => setSortBy('name')}>
                Name
                {#if sortBy === 'name'}
                  <span class="sort-indicator">{sortDirection === 'asc' ? '↑' : '↓'}</span>
                {/if}
              </th>
              <th>Category</th>
              <th>Website</th>
              <th>Username</th>
              <th class="sort-header" on:click={() => setSortBy('updated_at')}>
                Last Updated
                {#if sortBy === 'updated_at'}
                  <span class="sort-indicator">{sortDirection === 'asc' ? '↑' : '↓'}</span>
                {/if}
              </th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each sortedCredentials as credential}
              <tr on:click={() => viewCredential(credential.id)}>
                <td>{credential.name}</td>
                <td>{getCategoryName(credential.category_id)}</td>
                <td>
                  {#if credential.website}
                    <a 
                      href={credential.website} 
                      target="_blank" 
                      rel="noopener noreferrer"
                      on:click|stopPropagation
                    >
                      {credential.website.replace(/^https?:\/\//, '').replace(/\/$/, '')}
                    </a>
                  {/if}
                </td>
                <td>{credential.username || '-'}</td>
                <td>{new Date(credential.updated_at).toLocaleDateString()}</td>
                <td>
                  <button 
                    class="view-button"
                    on:click|stopPropagation={() => viewCredential(credential.id)}
                  >
                    View
                  </button>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  </div>
  
  <div class="back-link">
    <a href="/dashboard">&larr; Back to Dashboard</a>
  </div>
</div>

<style>
  .credentials-page {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  
  .card {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
  }
  
  .create-button {
    padding: 0.6rem 1.2rem;
    background-color: var(--primary-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
  }
  
  .create-button.small {
    font-size: 0.9rem;
    padding: 0.5rem 1rem;
    margin-top: 0.5rem;
  }
  
  .filter-row {
    display: grid;
    grid-template-columns: 2fr 1fr;
    gap: 1.5rem;
  }
  
  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  .loading {
    text-align: center;
    padding: 2rem;
    font-style: italic;
    color: #666;
  }
  
  .empty-message {
    text-align: center;
    padding: 2rem;
    color: #666;
  }
  
  .results-info {
    margin-bottom: 1rem;
    color: #666;
  }
  
  .table-container {
    overflow-x: auto;
  }
  
  .credentials-table {
    width: 100%;
    border-collapse: collapse;
  }
  
  .credentials-table th, 
  .credentials-table td {
    padding: 0.8rem;
    text-align: left;
    border-bottom: 1px solid var(--border-color);
  }
  
  .credentials-table th {
    background-color: var(--bg-color);
    font-weight: 600;
  }
  
  .credentials-table tbody tr {
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .credentials-table tbody tr:hover {
    background-color: rgba(0, 0, 0, 0.03);
  }
  
  .sort-header {
    cursor: pointer;
    user-select: none;
  }
  
  .sort-indicator {
    margin-left: 0.5rem;
  }
  
  .view-button {
    padding: 0.3rem 0.6rem;
    font-size: 0.85rem;
    background-color: var(--primary-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .back-link {
    margin-top: 1rem;
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
    .filter-row {
      grid-template-columns: 1fr;
      gap: 1rem;
    }
  }
</style> 