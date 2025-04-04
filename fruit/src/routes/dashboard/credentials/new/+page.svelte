<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { categories } from '$lib/stores/categories';
  import { credentials } from '$lib/stores/credentials';

  let name = '';
  let categoryId: string | null = null;
  let website = '';
  let username = '';
  let password = '';
  let notes = '';
  let showPassword = false;
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

      await credentials.create(credentialData);
      success = 'Credential saved successfully!';
      
      // Reset form after a delay
      setTimeout(() => {
        goto('/dashboard');
      }, 1500);
      
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to save credential';
      console.error(err);
    } finally {
      loading = false;
    }
  }

  function cancelForm() {
    goto('/dashboard');
  }

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }

  function generatePassword() {
    const length = 16;
    const charset = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_-+=';
    let result = '';
    
    // Ensure at least one of each character type
    result += getRandomChar('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
    result += getRandomChar('abcdefghijklmnopqrstuvwxyz');
    result += getRandomChar('0123456789');
    result += getRandomChar('!@#$%^&*()_-+=');
    
    // Fill the rest with random characters
    for (let i = result.length; i < length; i++) {
      result += charset.charAt(Math.floor(Math.random() * charset.length));
    }
    
    // Shuffle the result
    password = shuffleString(result);
  }
  
  function getRandomChar(charset: string): string {
    return charset.charAt(Math.floor(Math.random() * charset.length));
  }
  
  function shuffleString(str: string): string {
    const array = str.split('');
    for (let i = array.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [array[i], array[j]] = [array[j], array[i]];
    }
    return array.join('');
  }
</script>

<div class="credential-form-page">
  <h1>Add New Credential</h1>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}
  
  {#if success}
    <div class="success">{success}</div>
  {/if}
  
  <div class="form-container card">
    <form on:submit|preventDefault={handleSubmit}>
      <div class="form-group">
        <label for="name">Name <span class="required">*</span></label>
        <input 
          type="text" 
          id="name" 
          bind:value={name} 
          placeholder="e.g. Gmail, Twitter, Bank Account"
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
          placeholder="https://example.com"
          disabled={loading}
        />
      </div>
      
      <div class="form-group">
        <label for="username">Username</label>
        <input 
          type="text" 
          id="username" 
          bind:value={username} 
          placeholder="Your username or email"
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
            placeholder="Your secure password"
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
          <button 
            type="button" 
            class="generate-password" 
            on:click={generatePassword}
            disabled={loading}
          >
            Generate
          </button>
        </div>
      </div>
      
      <div class="form-group">
        <label for="notes">Notes</label>
        <textarea 
          id="notes" 
          bind:value={notes} 
          placeholder="Add any additional notes or information here"
          rows="4"
          disabled={loading}
        ></textarea>
      </div>
      
      <div class="form-actions">
        <button type="submit" class="primary-button" disabled={loading}>
          {loading ? 'Saving...' : 'Save Credential'}
        </button>
        <button type="button" class="secondary-button" on:click={cancelForm} disabled={loading}>
          Cancel
        </button>
      </div>
    </form>
  </div>
  
  <div class="back-link">
    <a href="/dashboard">&larr; Back to Dashboard</a>
  </div>
</div>

<style>
  .credential-form-page {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    max-width: 600px;
    margin: 0 auto;
  }
  
  h1 {
    margin-bottom: 1rem;
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
  
  .toggle-password, .generate-password {
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
</style> 