<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { categories } from '$lib/stores/categories';
  import { credentials } from '$lib/stores/credentials';
  import type { Credential } from '$lib/types';

  // Credential details
  let credential: Credential | null = null;
  let name = '';
  let categoryId: string | null = null;
  let website = '';
  let username = '';
  let password = '';
  let notes = '';
  
  // UI state
  let editMode = false;
  let showPassword = false;
  let loading = false;
  let error = '';
  let success = '';
  
  const credentialId = $page.params.id;

  onMount(async () => {
    loading = true;
    
    try {
      // Load categories for the dropdown
      await categories.fetchCategories();
      
      // Fetch the credential details
      const fetchedCredential = await credentials.getById(credentialId);
      
      if (fetchedCredential) {
        credential = fetchedCredential;
        
        // Populate form fields using the non-null credential
        name = fetchedCredential.name;
        categoryId = fetchedCredential.category_id;
        website = fetchedCredential.website || '';
        username = fetchedCredential.username || '';
        password = fetchedCredential.password || '';
        notes = fetchedCredential.notes || '';
      } else {
        error = 'Credential not found';
      }
      
    } catch (err) {
      error = 'Failed to load credential data.';
      console.error(err);
    } finally {
      loading = false;
    }
  });

  async function handleSave() {
    // Validate form
    if (!name || !password) {
      error = 'Name and password are required fields';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const credentialData = {
        name,
        category_id: categoryId,
        website: website || null,
        username: username || null,
        password,
        notes: notes || null
      };

      await credentials.update(credentialId, credentialData);
      
      success = 'Credential updated successfully!';
      editMode = false;
      
      // Refresh credential data
      const updatedCredential = await credentials.getById(credentialId);
      if (updatedCredential) {
        credential = updatedCredential;
      }
      
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to update credential';
      console.error(err);
    } finally {
      loading = false;
    }
  }

  async function handleDelete() {
    if (!confirm('Are you sure you want to delete this credential? This action cannot be undone.')) {
      return;
    }

    loading = true;
    error = '';

    try {
      await credentials.delete(credentialId);
      goto('/dashboard');
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to delete credential';
      console.error(err);
      loading = false;
    }
  }

  function toggleEditMode() {
    editMode = !editMode;
    error = '';
    success = '';
  }

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }

  function copyToClipboard(text: string, field: string) {
    navigator.clipboard.writeText(text)
      .then(() => {
        success = `${field} copied to clipboard!`;
        setTimeout(() => success = '', 2000);
      })
      .catch(() => {
        error = 'Failed to copy to clipboard';
      });
  }

  function returnToDashboard() {
    goto('/dashboard');
  }
</script>

<div class="credential-detail-page">
  <div class="header">
    <h1>{editMode ? 'Edit Credential' : 'Credential Details'}</h1>
    
    {#if !editMode}
      <div class="action-buttons">
        <button class="edit-button" on:click={toggleEditMode} disabled={loading}>
          Edit
        </button>
        <button class="delete-button" on:click={handleDelete} disabled={loading}>
          Delete
        </button>
      </div>
    {/if}
  </div>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}
  
  {#if success}
    <div class="success">{success}</div>
  {/if}
  
  {#if loading && !credential}
    <div class="loading">Loading credential data...</div>
  {:else if credential}
    <div class="card">
      {#if editMode}
        <!-- Edit Form -->
        <form on:submit|preventDefault={handleSave}>
          <div class="form-group">
            <label for="name">Name <span class="required">*</span></label>
            <input 
              type="text" 
              id="name" 
              bind:value={name} 
              required
              disabled={loading}
            />
          </div>
          
          <div class="form-group">
            <label for="category">Category</label>
            <select id="category" bind:value={categoryId} disabled={loading}>
              <option value={null}>No Category</option>
              {#each $categories.categories as category}
                <option value={category.id}>{category.name}</option>
                {#each category.children || [] as child}
                  <option value={child.id}>↳ {child.name}</option>
                {/each}
              {/each}
            </select>
          </div>
          
          <div class="form-group">
            <label for="website">Website URL</label>
            <input 
              type="url" 
              id="website" 
              bind:value={website} 
              disabled={loading}
            />
          </div>
          
          <div class="form-group">
            <label for="username">Username</label>
            <input 
              type="text" 
              id="username" 
              bind:value={username} 
              disabled={loading}
            />
          </div>
          
          <div class="form-group password-group">
            <label for="password">Password <span class="required">*</span></label>
            <div class="password-input-container">
              <input 
                type={showPassword ? 'text' : 'password'} 
                id="password" 
                bind:value={password} 
                required
                disabled={loading}
              />
              <button 
                type="button" 
                class="toggle-password" 
                on:click={togglePasswordVisibility}
                disabled={loading}
              >
                {showPassword ? 'Hide' : 'Show'}
              </button>
            </div>
          </div>
          
          <div class="form-group">
            <label for="notes">Notes</label>
            <textarea 
              id="notes" 
              bind:value={notes} 
              rows="4"
              disabled={loading}
            ></textarea>
          </div>
          
          <div class="form-actions">
            <button type="submit" class="primary-button" disabled={loading}>
              {loading ? 'Saving...' : 'Update Credential'}
            </button>
            <button type="button" class="secondary-button" on:click={toggleEditMode} disabled={loading}>
              Cancel
            </button>
          </div>
        </form>
      {:else}
        <!-- View Details -->
        <div class="credential-details">
          <div class="detail-group">
            <div class="detail-label">Name:</div>
            <div class="detail-value">{credential.name}</div>
          </div>
          
          <div class="detail-group">
            <div class="detail-label">Category:</div>
            <div class="detail-value">
              {#if credential.category_id}
                {$categories.categories.find(c => c.id === credential.category_id)?.name || 
                  $categories.categories.flatMap(c => c.children || []).find(c => c.id === credential.category_id)?.name || 
                  'Unknown'}
              {:else}
                None
              {/if}
            </div>
          </div>
          
          {#if website}
            <div class="detail-group">
              <div class="detail-label">Website:</div>
              <div class="detail-value website-value">
                <a href={website} target="_blank" rel="noopener noreferrer">{website}</a>
                <button class="copy-button" on:click={() => copyToClipboard(website, 'Website URL')}>
                  Copy
                </button>
              </div>
            </div>
          {/if}
          
          {#if username}
            <div class="detail-group">
              <div class="detail-label">Username:</div>
              <div class="detail-value with-copy">
                {username}
                <button class="copy-button" on:click={() => copyToClipboard(username, 'Username')}>
                  Copy
                </button>
              </div>
            </div>
          {/if}
          
          <div class="detail-group">
            <div class="detail-label">Password:</div>
            <div class="detail-value with-copy">
              <div class="password-display">
                {showPassword ? password : '••••••••••••'}
                <button class="toggle-button" on:click={togglePasswordVisibility}>
                  {showPassword ? 'Hide' : 'Show'}
                </button>
              </div>
              <button class="copy-button" on:click={() => copyToClipboard(password, 'Password')}>
                Copy
              </button>
            </div>
          </div>
          
          {#if notes}
            <div class="detail-group">
              <div class="detail-label">Notes:</div>
              <div class="detail-value notes-value">
                {notes}
              </div>
            </div>
          {/if}
          
          <div class="detail-group metadata">
            <div class="detail-label">Created:</div>
            <div class="detail-value">{new Date(credential.created_at).toLocaleString()}</div>
          </div>
          
          <div class="detail-group metadata">
            <div class="detail-label">Last Updated:</div>
            <div class="detail-value">{new Date(credential.updated_at).toLocaleString()}</div>
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="error-message">Credential not found or could not be loaded.</div>
  {/if}
  
  <div class="back-link">
    <a href="/dashboard" on:click|preventDefault={returnToDashboard}>&larr; Back to Dashboard</a>
  </div>
</div>

<style>
  .credential-detail-page {
    max-width: 800px;
    margin: 0 auto;
  }
  
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }
  
  .action-buttons {
    display: flex;
    gap: 0.5rem;
  }
  
  .edit-button, .delete-button {
    padding: 0.5rem 1rem;
    border-radius: 4px;
    font-size: 0.9rem;
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
  
  .loading {
    text-align: center;
    padding: 2rem;
    font-style: italic;
    color: #666;
  }
  
  .card {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
    margin-bottom: 2rem;
  }
  
  /* Form Styles */
  .form-group {
    margin-bottom: 1.2rem;
  }
  
  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  .required {
    color: var(--error-color);
  }
  
  .password-group {
    margin-bottom: 1.5rem;
  }
  
  .password-input-container {
    display: flex;
    gap: 0.5rem;
  }
  
  .password-input-container input {
    flex: 1;
    margin-bottom: 0;
  }
  
  .toggle-password {
    padding: 0.5rem;
    font-size: 0.85rem;
    background-color: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    cursor: pointer;
  }
  
  textarea {
    width: 100%;
    padding: 0.5rem;
    margin-bottom: 1rem;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 1rem;
    resize: vertical;
    min-height: 100px;
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
  
  .secondary-button {
    background-color: transparent;
    color: var(--primary-color);
    border: 1px solid var(--primary-color);
  }
  
  /* Detail View Styles */
  .credential-details {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  
  .detail-group {
    display: grid;
    grid-template-columns: 120px 1fr;
    gap: 1rem;
    align-items: start;
    padding-bottom: 0.5rem;
    border-bottom: 1px solid var(--border-color);
  }
  
  .detail-label {
    font-weight: 600;
    color: #555;
  }
  
  .with-copy {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  
  .password-display {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .toggle-button {
    padding: 0.2rem 0.5rem;
    font-size: 0.8rem;
    background-color: transparent;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    cursor: pointer;
  }
  
  .copy-button {
    padding: 0.2rem 0.5rem;
    font-size: 0.8rem;
    background-color: var(--bg-color);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    cursor: pointer;
  }
  
  .copy-button:hover {
    background-color: #eee;
  }
  
  .website-value a {
    color: var(--primary-color);
    word-break: break-all;
  }
  
  .notes-value {
    white-space: pre-line;
  }
  
  .metadata {
    font-size: 0.9rem;
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
  
  @media (max-width: 600px) {
    .detail-group {
      grid-template-columns: 1fr;
      gap: 0.3rem;
    }
    
    .with-copy {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.5rem;
    }
    
    .header {
      flex-direction: column;
      align-items: flex-start;
      gap: 1rem;
    }
  }
</style> 