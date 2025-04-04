<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';
  import { categories } from '$lib/stores/categories';
  import { credentials } from '$lib/stores/credentials';
  import type { Category, Credential } from '$lib/types';

  let selectedCategory: Category | null = null;
  let categoryCredentials: Credential[] = [];
  let loadingCredentials = false;
  let error = '';

  onMount(async () => {
    try {
      // Fetch categories on mount
      await categories.fetchCategories();
      
      // Fetch all credentials
      await credentials.fetchAll();
    } catch (err) {
      error = 'Failed to load data. Please try again.';
      console.error(err);
    }
  });

  async function selectCategory(category: Category) {
    selectedCategory = category;
    loadingCredentials = true;
    error = '';
    
    try {
      await credentials.fetchByCategory(category.id);
      categoryCredentials = $credentials.credentials;
    } catch (err) {
      error = 'Failed to load credentials for this category.';
      console.error(err);
    } finally {
      loadingCredentials = false;
    }
  }

  function viewAllCredentials() {
    selectedCategory = null;
    categoryCredentials = $credentials.credentials;
  }

  function navigateToCredentialForm() {
    goto('/dashboard/credentials/new');
  }

  function navigateToCategoryForm() {
    goto('/dashboard/categories/new');
  }

  function viewCredential(id: string) {
    goto(`/dashboard/credentials/${id}`);
  }
</script>

<div class="dashboard">
  <div class="dashboard-header">
    <h1>Welcome, {$auth.user?.email || 'User'}</h1>
    <p>Manage your secure credentials and categories.</p>
  </div>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}
  
  <div class="dashboard-actions">
    <button on:click={navigateToCredentialForm} class="action-button">
      Add New Credential
    </button>
    <button on:click={navigateToCategoryForm} class="action-button secondary">
      Add New Category
    </button>
  </div>
  
  <div class="dashboard-content">
    <div class="categories-panel">
      <h2>Categories</h2>
      
      <ul class="category-list">
        <li>
          <button 
            class="category-item" 
            class:active={selectedCategory === null}
            on:click={viewAllCredentials}
          >
            All Credentials
          </button>
        </li>
        
        {#if $categories.loading}
          <li>Loading categories...</li>
        {:else if $categories.categories.length === 0}
          <li>No categories found</li>
        {:else}
          {#each $categories.categories as category}
            <li>
              <button 
                class="category-item" 
                class:active={selectedCategory?.id === category.id}
                on:click={() => selectCategory(category)}
              >
                {category.name}
              </button>
              
              {#if category.children && category.children.length > 0}
                <ul class="subcategory-list">
                  {#each category.children as subcategory}
                    <li>
                      <button 
                        class="category-item subcategory" 
                        class:active={selectedCategory?.id === subcategory.id}
                        on:click={() => selectCategory(subcategory)}
                      >
                        {subcategory.name}
                      </button>
                    </li>
                  {/each}
                </ul>
              {/if}
            </li>
          {/each}
        {/if}
      </ul>
      
      <div class="category-actions">
        <a href="/dashboard/categories" class="view-all-link">
          Manage Categories
        </a>
      </div>
    </div>
    
    <div class="credentials-panel">
      <h2>
        {#if selectedCategory}
          {selectedCategory.name} Credentials
        {:else}
          All Credentials
        {/if}
      </h2>
      
      {#if loadingCredentials || $credentials.loading}
        <p>Loading credentials...</p>
      {:else if categoryCredentials.length === 0}
        <div class="empty-state">
          <p>No credentials found for this selection.</p>
          <button on:click={navigateToCredentialForm} class="empty-action">
            Add Your First Credential
          </button>
        </div>
      {:else}
        <div class="credentials-grid">
          {#each $credentials.credentials as credential}
            <div class="credential-card" on:click={() => viewCredential(credential.id)}>
              <h3>{credential.name}</h3>
              {#if credential.website}
                <p class="credential-website">{credential.website}</p>
              {/if}
              {#if credential.username}
                <p class="credential-username">Username: {credential.username}</p>
              {/if}
              <div class="credential-actions">
                <button class="view-button">View Details</button>
              </div>
            </div>
          {/each}
        </div>
      {/if}
      
      <div class="credential-actions">
        <a href="/dashboard/credentials" class="view-all-link">
          Manage All Credentials
        </a>
      </div>
    </div>
  </div>
</div>

<style>
  .dashboard {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  .dashboard-header {
    margin-bottom: 1rem;
  }
  
  .dashboard-actions {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  
  .action-button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    background-color: var(--primary-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .action-button:hover {
    background-color: #34495e;
  }
  
  .action-button.secondary {
    background-color: transparent;
    border: 1px solid var(--primary-color);
    color: var(--primary-color);
  }
  
  .action-button.secondary:hover {
    background-color: rgba(44, 62, 80, 0.1);
  }
  
  .dashboard-content {
    display: grid;
    grid-template-columns: 300px 1fr;
    gap: 2rem;
  }
  
  .categories-panel, .credentials-panel {
    background-color: var(--card-bg);
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
  }
  
  .category-list {
    list-style: none;
    margin: 1rem 0;
    padding: 0;
  }
  
  .category-item {
    display: block;
    width: 100%;
    text-align: left;
    padding: 0.6rem 1rem;
    margin-bottom: 0.3rem;
    background-color: transparent;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .category-item:hover {
    background-color: rgba(44, 62, 80, 0.1);
  }
  
  .category-item.active {
    background-color: var(--primary-color);
    color: white;
  }
  
  .subcategory-list {
    list-style: none;
    margin-left: 1.5rem;
    padding: 0;
  }
  
  .subcategory {
    font-size: 0.9rem;
  }
  
  .category-actions, .credential-actions {
    margin-top: 1rem;
    text-align: center;
  }
  
  .view-all-link {
    color: var(--primary-color);
    font-size: 0.9rem;
    text-decoration: underline;
  }
  
  .credentials-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: 1rem;
    margin-top: 1.5rem;
  }
  
  .credential-card {
    background-color: var(--bg-color);
    border-radius: 6px;
    padding: 1.2rem;
    border: 1px solid var(--border-color);
    cursor: pointer;
    transition: transform 0.2s, box-shadow 0.2s;
  }
  
  .credential-card:hover {
    transform: translateY(-3px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
  
  .credential-website {
    color: #666;
    font-size: 0.9rem;
    margin-bottom: 0.5rem;
  }
  
  .credential-username {
    font-size: 0.9rem;
    margin-bottom: 1rem;
  }
  
  .view-button {
    padding: 0.4rem 0.8rem;
    font-size: 0.85rem;
    background-color: var(--primary-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .empty-state {
    padding: 2rem;
    text-align: center;
    border: 1px dashed var(--border-color);
    border-radius: 8px;
    margin-top: 1.5rem;
  }
  
  .empty-action {
    margin-top: 1rem;
    padding: 0.6rem 1.2rem;
    background-color: var(--primary-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
  
  @media (max-width: 768px) {
    .dashboard-content {
      grid-template-columns: 1fr;
    }
    
    .dashboard-actions {
      flex-direction: column;
    }
  }
</style> 