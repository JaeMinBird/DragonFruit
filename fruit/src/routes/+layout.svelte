<script lang="ts">
  import { onMount } from 'svelte';
  import { page } from '$app/stores';
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';
  import api from '$lib/api';

  // Protected routes that require auth
  const protectedRoutes = ['/dashboard', '/categories', '/credentials'];
  
  onMount(async () => {
    // Check if user is authenticated
    if ($auth.token) {
      try {
        // Verify token by fetching user profile
        const user = await api.auth.getProfile();
        auth.setUser(user);
      } catch (error) {
        // Token is invalid, clear auth
        auth.logout();
        redirectIfNeeded();
      }
    } else {
      redirectIfNeeded();
    }
  });

  // Subscribe to route changes
  $: {
    redirectIfNeeded();
  }

  function redirectIfNeeded() {
    const currentPath = $page.url.pathname;
    
    // Redirect to login if accessing protected route while not authenticated
    if (protectedRoutes.some(route => currentPath.startsWith(route)) && !$auth.token) {
      goto('/login');
    }
    
    // Redirect to dashboard if accessing auth routes while already authenticated
    if ((currentPath === '/login' || currentPath === '/register') && $auth.token) {
      goto('/dashboard');
    }
  }

  function logout() {
    auth.logout();
    goto('/login');
  }
</script>

<div class="app">
  <header>
    <nav>
      <div class="brand">
        <a href="/">🐉 DragonFruit</a>
      </div>
      
      {#if $auth.token}
        <ul class="nav-links">
          <li><a href="/dashboard" class:active={$page.url.pathname === '/dashboard'}>Dashboard</a></li>
          <li><a href="/dashboard/categories" class:active={$page.url.pathname.includes('/categories')}>Categories</a></li>
          <li><a href="/dashboard/credentials" class:active={$page.url.pathname.includes('/credentials')}>Credentials</a></li>
        </ul>
        
        <div class="auth-links">
          <span>{$auth.user?.email}</span>
          <button on:click={logout}>Logout</button>
        </div>
      {:else}
        <div class="auth-links">
          <a href="/login" class:active={$page.url.pathname === '/login'}>Login</a>
          <a href="/register" class:active={$page.url.pathname === '/register'}>Register</a>
        </div>
      {/if}
    </nav>
  </header>
  
  <main>
    <slot />
  </main>
  
  <footer>
    <p>&copy; {new Date().getFullYear()} DragonFruit - Secure Password Manager</p>
  </footer>
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
  
  header {
    background-color: #2c3e50;
    color: white;
    padding: 0 1rem;
  }
  
  nav {
    display: flex;
    justify-content: space-between;
    align-items: center;
    max-width: 1200px;
    margin: 0 auto;
    height: 60px;
  }
  
  .brand a {
    font-size: 1.5rem;
    font-weight: bold;
    color: white;
    text-decoration: none;
  }
  
  .nav-links {
    display: flex;
    list-style: none;
    gap: 1.5rem;
    margin: 0;
    padding: 0;
  }
  
  .nav-links a, .auth-links a {
    color: #ecf0f1;
    text-decoration: none;
    padding: 0.5rem;
  }
  
  .auth-links {
    display: flex;
    align-items: center;
    gap: 1rem;
  }
  
  .auth-links button {
    background-color: transparent;
    border: 1px solid #ecf0f1;
    color: #ecf0f1;
    padding: 0.3rem 0.8rem;
    border-radius: 4px;
    cursor: pointer;
  }
  
  .auth-links button:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }
  
  .active {
    font-weight: bold;
    border-bottom: 2px solid #ecf0f1;
  }
  
  main {
    flex: 1;
    max-width: 1200px;
    width: 100%;
    margin: 0 auto;
    padding: 2rem 1rem;
  }
  
  footer {
    background-color: #2c3e50;
    color: white;
    text-align: center;
    padding: 1rem;
    margin-top: auto;
  }
</style> 