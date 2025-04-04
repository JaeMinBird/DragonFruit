<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';
  import api from '$lib/api';
  
  onMount(async () => {
    // Check authentication status
    if (!$auth.token) {
      // Redirect to login if not authenticated
      goto('/login');
      return;
    }
    
    try {
      // Verify token by fetching user profile
      const user = await api.auth.getProfile();
      auth.setUser(user);
    } catch (error) {
      // If authentication fails, redirect to login
      auth.logout();
      goto('/login');
    }
  });
</script>

<slot /> 