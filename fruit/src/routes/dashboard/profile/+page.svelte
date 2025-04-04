<script lang="ts">
  import { onMount } from 'svelte';
  import { auth } from '$lib/stores/auth';
  import api from '$lib/api';
  
  // Profile data
  let email = '';
  let currentPassword = '';
  let newPassword = '';
  let confirmPassword = '';
  
  // 2FA data
  let totpSetupData: { secret: string, qrCodeUrl: string } | null = null;
  let totpCode = '';
  let showTotpSetup = false;
  
  // UI state
  let loading = false;
  let error = '';
  let success = '';
  
  onMount(() => {
    if ($auth.user) {
      email = $auth.user.email;
    }
  });
  
  async function handleProfileUpdate() {
    // Validate passwords if changing
    if (newPassword) {
      if (!currentPassword) {
        error = 'Current password is required to set a new password';
        return;
      }
      
      if (newPassword.length < 8) {
        error = 'New password must be at least 8 characters long';
        return;
      }
      
      if (newPassword !== confirmPassword) {
        error = 'New passwords do not match';
        return;
      }
    }
    
    loading = true;
    error = '';
    success = '';
    
    try {
      const userData: any = {};
      
      // Only include email if it has changed
      if (email !== $auth.user?.email) {
        userData.email = email;
      }
      
      // Only include password fields if changing password
      if (newPassword) {
        userData.current_password = currentPassword;
        userData.new_password = newPassword;
      }
      
      // Skip API call if no changes
      if (Object.keys(userData).length === 0) {
        success = 'No changes to save';
        loading = false;
        return;
      }
      
      // Update profile
      const updatedUser = await api.auth.updateProfile(userData);
      auth.setUser(updatedUser);
      
      success = 'Profile updated successfully';
      
      // Reset password fields
      currentPassword = '';
      newPassword = '';
      confirmPassword = '';
      
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to update profile';
      console.error(err);
    } finally {
      loading = false;
    }
  }
  
  async function setupTotp() {
    loading = true;
    error = '';
    success = '';
    
    try {
      totpSetupData = await api.auth.generateTotp();
      showTotpSetup = true;
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to setup 2FA';
      console.error(err);
    } finally {
      loading = false;
    }
  }
  
  async function verifyAndEnableTotp() {
    if (!totpCode) {
      error = 'Please enter the verification code from your authenticator app';
      return;
    }
    
    loading = true;
    error = '';
    success = '';
    
    try {
      await api.auth.enableTotp(totpCode);
      
      // Update user data with 2FA enabled
      const updatedUser = await api.auth.getProfile();
      auth.setUser(updatedUser);
      
      success = '2FA has been successfully enabled for your account';
      showTotpSetup = false;
      totpSetupData = null;
      totpCode = '';
      
    } catch (err) {
      error = err instanceof Error ? err.message : 'Failed to verify 2FA code';
      console.error(err);
    } finally {
      loading = false;
    }
  }
  
  function cancelTotpSetup() {
    showTotpSetup = false;
    totpSetupData = null;
    totpCode = '';
    error = '';
    success = '';
  }
</script>

<div class="profile-page">
  <h1>Account Settings</h1>
  
  {#if error}
    <div class="error">{error}</div>
  {/if}
  
  {#if success}
    <div class="success">{success}</div>
  {/if}
  
  <div class="card">
    <h2>Profile Information</h2>
    
    <form on:submit|preventDefault={handleProfileUpdate}>
      <div class="form-group">
        <label for="email">Email Address</label>
        <input 
          type="email" 
          id="email" 
          bind:value={email} 
          disabled={loading}
          required
        />
      </div>
      
      <h3>Change Password</h3>
      <p class="form-subtitle">Leave blank to keep your current password</p>
      
      <div class="form-group">
        <label for="currentPassword">Current Password</label>
        <input 
          type="password" 
          id="currentPassword" 
          bind:value={currentPassword} 
          disabled={loading}
          placeholder="Enter your current password"
        />
      </div>
      
      <div class="form-group">
        <label for="newPassword">New Password</label>
        <input 
          type="password" 
          id="newPassword" 
          bind:value={newPassword} 
          disabled={loading}
          placeholder="Enter a new password"
        />
      </div>
      
      <div class="form-group">
        <label for="confirmPassword">Confirm New Password</label>
        <input 
          type="password" 
          id="confirmPassword" 
          bind:value={confirmPassword} 
          disabled={loading}
          placeholder="Confirm your new password"
        />
      </div>
      
      <div class="form-actions">
        <button type="submit" class="primary-button" disabled={loading}>
          {loading ? 'Saving...' : 'Save Changes'}
        </button>
      </div>
    </form>
  </div>
  
  <div class="card">
    <h2>Two-Factor Authentication</h2>
    
    {#if $auth.user?.totp_enabled}
      <div class="totp-status enabled">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"></path>
          <polyline points="22 4 12 14.01 9 11.01"></polyline>
        </svg>
        <div>
          <strong>2FA is enabled</strong>
          <p>Your account is protected with two-factor authentication.</p>
        </div>
      </div>
    {:else if showTotpSetup && totpSetupData}
      <div class="totp-setup">
        <p>Scan the QR code with your authenticator app (like Google Authenticator or Authy)</p>
        
        <div class="totp-qr-container">
          <img src={totpSetupData.qrCodeUrl} alt="2FA QR Code" class="totp-qr"/>
        </div>
        
        <div class="totp-manual">
          <p>If you can't scan the QR code, enter this code manually in your app:</p>
          <code class="totp-secret">{totpSetupData.secret}</code>
        </div>
        
        <div class="form-group">
          <label for="totp-code">Enter Verification Code</label>
          <input 
            type="text" 
            id="totp-code" 
            bind:value={totpCode} 
            placeholder="6-digit code from your app"
            maxlength="6"
            pattern="[0-9]{6}"
            inputmode="numeric"
            autocomplete="one-time-code"
            disabled={loading}
          />
        </div>
        
        <div class="form-actions">
          <button 
            type="button" 
            class="primary-button" 
            on:click={verifyAndEnableTotp}
            disabled={loading || !totpCode}
          >
            {loading ? 'Verifying...' : 'Verify and Enable 2FA'}
          </button>
          <button 
            type="button" 
            class="secondary-button" 
            on:click={cancelTotpSetup}
            disabled={loading}
          >
            Cancel
          </button>
        </div>
      </div>
    {:else}
      <div class="totp-status disabled">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
          <line x1="12" y1="9" x2="12" y2="13"></line>
          <line x1="12" y1="17" x2="12.01" y2="17"></line>
        </svg>
        <div>
          <strong>2FA is not enabled</strong>
          <p>Enable two-factor authentication for added security</p>
        </div>
      </div>
      
      <div class="form-actions totp-enable">
        <button 
          type="button" 
          class="primary-button" 
          on:click={setupTotp}
          disabled={loading}
        >
          {loading ? 'Setting up...' : 'Set up 2FA'}
        </button>
      </div>
    {/if}
  </div>
  
  <div class="back-link">
    <a href="/dashboard">&larr; Back to Dashboard</a>
  </div>
</div>

<style>
  .profile-page {
    max-width: 800px;
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  .card {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 1.5rem;
  }
  
  h1 {
    margin-bottom: 1rem;
  }
  
  h2 {
    color: var(--primary-color);
    margin-bottom: 1.5rem;
  }
  
  h3 {
    margin-top: 2rem;
    margin-bottom: 0.5rem;
    font-size: 1.2rem;
  }
  
  .form-subtitle {
    color: #666;
    margin-bottom: 1rem;
    font-size: 0.9rem;
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
  
  /* 2FA Styles */
  .totp-status {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    border-radius: 6px;
    margin-bottom: 1rem;
  }
  
  .totp-status.enabled {
    background-color: rgba(46, 204, 113, 0.1);
    border: 1px solid var(--success-color);
    color: var(--success-color);
  }
  
  .totp-status.disabled {
    background-color: rgba(231, 76, 60, 0.1);
    border: 1px solid var(--error-color);
    color: var(--error-color);
  }
  
  .totp-status p {
    margin: 0.3rem 0 0 0;
    font-size: 0.9rem;
  }
  
  .totp-enable {
    margin-top: 1.5rem;
  }
  
  .totp-setup {
    margin-top: 1.5rem;
  }
  
  .totp-qr-container {
    display: flex;
    justify-content: center;
    margin: 1.5rem 0;
  }
  
  .totp-qr {
    max-width: 200px;
    height: auto;
  }
  
  .totp-manual {
    background-color: var(--bg-color);
    padding: 1rem;
    border-radius: 6px;
    margin-bottom: 1.5rem;
    text-align: center;
  }
  
  .totp-secret {
    display: block;
    font-family: monospace;
    font-size: 1.2rem;
    background-color: white;
    padding: 0.5rem;
    border-radius: 4px;
    margin-top: 0.5rem;
    word-break: break-all;
  }
  
  .back-link {
    margin-top: 1rem;
    margin-bottom: 2rem;
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