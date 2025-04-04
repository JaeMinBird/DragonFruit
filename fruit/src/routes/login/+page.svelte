<script lang="ts">
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';
  import api from '$lib/api';

  let email = '';
  let password = '';
  let totpCode = '';
  let showTotpField = false;
  let loading = false;
  let error = '';

  async function handleLogin() {
    // Clear previous error
    error = '';
    
    // Validate form
    if (!email || !password) {
      error = 'Email and password are required';
      return;
    }
    
    // If TOTP field is shown, validate the code
    if (showTotpField && !totpCode) {
      error = 'Verification code is required';
      return;
    }
    
    // Submit login
    loading = true;
    
    try {
      const credentials = {
        email,
        password,
        ...(showTotpField && { totp_code: totpCode })
      };
      
      const response = await api.auth.login(credentials);
      
      // Check if we need to show TOTP field
      if (response.require_totp && !showTotpField) {
        showTotpField = true;
        error = 'Please enter your 2FA verification code';
        loading = false;
        return;
      }
      
      // Login successful
      const { token, user } = response;
      
      // Set auth state
      auth.setToken(token);
      auth.setUser(user);
      
      // Redirect to dashboard
      goto('/dashboard');
      
    } catch (err) {
      error = err instanceof Error ? err.message : 'Login failed. Please check your credentials.';
    } finally {
      loading = false;
    }
  }
</script>

<div class="auth-container">
  <div class="auth-card">
    <h1>Login to DragonFruit</h1>
    
    {#if error}
      <div class="error">{error}</div>
    {/if}
    
    <form on:submit|preventDefault={handleLogin}>
      <div class="form-group">
        <label for="email">Email</label>
        <input 
          type="email" 
          id="email" 
          bind:value={email} 
          placeholder="Your email address"
          required
          disabled={loading || showTotpField}
        />
      </div>
      
      <div class="form-group">
        <label for="password">Password</label>
        <input 
          type="password" 
          id="password" 
          bind:value={password} 
          placeholder="Your password"
          required
          disabled={loading || showTotpField}
        />
      </div>
      
      {#if showTotpField}
        <div class="form-group">
          <label for="totpCode">2FA Verification Code</label>
          <input 
            type="text" 
            id="totpCode" 
            bind:value={totpCode} 
            placeholder="6-digit code"
            required
            disabled={loading}
            pattern="[0-9]{6}"
            inputmode="numeric"
            autocomplete="one-time-code"
          />
        </div>
      {/if}
      
      <button type="submit" class="auth-button" disabled={loading}>
        {#if loading}
          Logging in...
        {:else if showTotpField}
          Verify
        {:else}
          Login
        {/if}
      </button>
    </form>
    
    <div class="auth-link">
      Don't have an account? <a href="/register">Register</a>
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