<script lang="ts">
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';
  import api from '$lib/api';
  
  let email = '';
  let password = '';
  let confirmPassword = '';
  let loading = false;
  let error = '';
  let success = '';
  
  async function handleRegister() {
    // Clear previous messages
    error = '';
    success = '';
    
    // Validate form
    if (!email || !password || !confirmPassword) {
      error = 'All fields are required';
      return;
    }
    
    if (password !== confirmPassword) {
      error = 'Passwords do not match';
      return;
    }
    
    if (password.length < 8) {
      error = 'Password must be at least 8 characters long';
      return;
    }
    
    // Submit registration
    loading = true;
    
    try {
      await api.auth.register({ email, password });
      success = 'Account created successfully! Redirecting to login...';
      
      // Reset form
      email = '';
      password = '';
      confirmPassword = '';
      
      // Redirect to login after a brief delay
      setTimeout(() => {
        goto('/login');
      }, 1500);
      
    } catch (err) {
      error = err instanceof Error ? err.message : 'Registration failed. Please try again.';
    } finally {
      loading = false;
    }
  }
</script>

<div class="auth-container">
  <div class="auth-card">
    <h1>Create an Account</h1>
    
    {#if error}
      <div class="error">{error}</div>
    {/if}
    
    {#if success}
      <div class="success">{success}</div>
    {/if}
    
    <form on:submit|preventDefault={handleRegister}>
      <div class="form-group">
        <label for="email">Email</label>
        <input 
          type="email" 
          id="email" 
          bind:value={email} 
          placeholder="Your email address"
          required
          disabled={loading}
        />
      </div>
      
      <div class="form-group">
        <label for="password">Password</label>
        <input 
          type="password" 
          id="password" 
          bind:value={password} 
          placeholder="Create a secure password"
          required
          disabled={loading}
        />
      </div>
      
      <div class="form-group">
        <label for="confirmPassword">Confirm Password</label>
        <input 
          type="password" 
          id="confirmPassword" 
          bind:value={confirmPassword} 
          placeholder="Confirm your password"
          required
          disabled={loading}
        />
      </div>
      
      <button type="submit" class="auth-button" disabled={loading}>
        {loading ? 'Creating Account...' : 'Register'}
      </button>
    </form>
    
    <div class="auth-link">
      Already have an account? <a href="/login">Login</a>
    </div>
  </div>
</div>

<style>
  .auth-container {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: calc(100vh - 180px);
  }
  
  .auth-card {
    background-color: var(--card-bg);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    padding: 2rem;
    width: 100%;
    max-width: 450px;
  }
  
  h1 {
    color: var(--primary-color);
    margin-bottom: 1.5rem;
    text-align: center;
  }
  
  .form-group {
    margin-bottom: 1.2rem;
  }
  
  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
  }
  
  .auth-button {
    width: 100%;
    padding: 0.75rem;
    margin-top: 1rem;
    background-color: var(--primary-color);
    color: white;
    font-size: 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }
  
  .auth-button:hover:not(:disabled) {
    background-color: #34495e;
  }
  
  .auth-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }
  
  .auth-link {
    text-align: center;
    margin-top: 1.5rem;
  }
  
  .auth-link a {
    color: var(--primary-color);
    font-weight: 500;
  }
  
  .auth-link a:hover {
    text-decoration: underline;
  }
</style> 