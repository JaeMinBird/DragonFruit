<script lang="ts">
  import { goto } from '$app/navigation';

  function navigateToLogin() {
    goto('/login');
  }

  function navigateToRegister() {
    goto('/register');
  }
</script>

<main class="container">
  <div class="content">
    <img src="/DGF Logo.png" alt="DragonFruit Logo" class="logo" />
    <h1 class="doto-font">dragonfruit</h1>
    <div class="buttons">
      <button on:click={navigateToLogin} class="btn">login</button>
      <button on:click={navigateToRegister} class="btn">register</button>
    </div>
  </div>
</main>

<style>
  :global(body) {
    background-color: white;
  }
  
  .container {
    min-height: 50vh;
    display: flex;
    justify-content: center;
    align-items: flex-start;
    padding-top:10vh;
  }
  
  .content {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  
  .logo {
    max-width: 150px;
    height: auto;
    margin-bottom: 3vh;
  }
  
  h1 {
    font-size: 2rem;
    color: #e74c3c; /* dragonfruit red */
    font-weight: 600;
    margin: 0;
    margin-bottom: 4vh;
  }
  
  .doto-font {
    font-family: 'Doto', sans-serif;
  }
  
  .buttons {
    display: flex;
    flex-direction: column;
    gap: 2vh;
  }
  
  .btn {
    min-width: 200px;
    padding: 0.6rem 0;
    background-color: transparent;
    border: 1px solid #ddd;
    color: #333;
    font-size: 0.9rem;
    border-radius: 4px;
  }
  
  .btn:hover {
    background-color: #f5f5f5;
  }
</style>
