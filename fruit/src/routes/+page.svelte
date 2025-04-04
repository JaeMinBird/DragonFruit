<script lang="ts">
  import { goto } from '$app/navigation';
  import { auth } from '$lib/stores/auth';

  function navigateToLogin() {
    goto('/login');
  }

  function navigateToRegister() {
    goto('/register');
  }

  function navigateToDashboard() {
    goto('/dashboard');
  }
</script>

<section class="hero">
  <div class="hero-content">
    <h1>DragonFruit Secure Password Manager</h1>
    <p>Keep your credentials safe and organized with our secure password management solution.</p>
    
    {#if $auth.token}
      <button on:click={navigateToDashboard} class="cta-button">Go to Dashboard</button>
    {:else}
      <div class="cta-buttons">
        <button on:click={navigateToLogin} class="cta-button">Login</button>
        <button on:click={navigateToRegister} class="cta-button secondary">Register</button>
      </div>
    {/if}
  </div>
</section>

<section class="features">
  <div class="feature">
    <h3>Secure Storage</h3>
    <p>All your passwords are encrypted at rest using industry-standard encryption.</p>
  </div>
  <div class="feature">
    <h3>Organized Categories</h3>
    <p>Keep your credentials organized with customizable categories and nesting.</p>
  </div>
  <div class="feature">
    <h3>Easy Access</h3>
    <p>Quickly find and access your credentials whenever you need them.</p>
  </div>
</section>

<style>
  .hero {
    background-color: var(--primary-color);
    color: var(--light-text);
    padding: 4rem 2rem;
    text-align: center;
    margin-bottom: 2rem;
    border-radius: 8px;
  }

  .hero-content {
    max-width: 800px;
    margin: 0 auto;
  }

  h1 {
    font-size: 2.5rem;
    margin-bottom: 1.5rem;
  }

  .hero p {
    font-size: 1.2rem;
    margin-bottom: 2rem;
  }

  .cta-buttons {
    display: flex;
    gap: 1rem;
    justify-content: center;
  }

  .cta-button {
    padding: 0.75rem 1.5rem;
    font-size: 1.1rem;
    background-color: var(--accent-color);
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .cta-button:hover {
    background-color: #c0392b;
  }

  .cta-button.secondary {
    background-color: transparent;
    border: 2px solid var(--light-text);
    color: var(--light-text);
  }

  .cta-button.secondary:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

  .features {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 2rem;
    margin: 3rem 0;
  }

  .feature {
    background-color: var(--card-bg);
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    text-align: center;
  }

  .feature h3 {
    margin-bottom: 1rem;
    color: var(--primary-color);
  }

  @media (max-width: 768px) {
    .cta-buttons {
      flex-direction: column;
      width: 100%;
      max-width: 300px;
      margin: 0 auto;
    }
  }
</style>
