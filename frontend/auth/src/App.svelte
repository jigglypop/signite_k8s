<script>
  import Login from './components/Login.svelte';
  import Register from './components/Register.svelte';
  
  let isLoginView = true;
  
  function toggleView() {
    isLoginView = !isLoginView;
  }
</script>

<main>
  <div class="auth-container">
    <div class="auth-header">
      <h1>{isLoginView ? '로그인' : '회원가입'}</h1>
    </div>
    
    {#if isLoginView}
      <Login />
      <p class="toggle-text">
        계정이 없으신가요? <button on:click={toggleView}>회원가입</button>
      </p>
    {:else}
      <Register />
      <p class="toggle-text">
        이미 계정이 있으신가요? <button on:click={toggleView}>로그인</button>
      </p>
    {/if}
  </div>
</main>

<style>
  .auth-container {
    max-width: 400px;
    margin: 40px auto;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    background-color: white;
  }
  
  .auth-header {
    text-align: center;
    margin-bottom: 20px;
  }
  
  h1 {
    color: #333;
    font-size: 24px;
    margin: 0;
  }
  
  .toggle-text {
    text-align: center;
    margin-top: 20px;
    font-size: 14px;
    color: #555;
  }
  
  button {
    background: none;
    border: none;
    color: #3498db;
    cursor: pointer;
    padding: 0;
    font-size: 14px;
    text-decoration: underline;
  }
  
  button:hover {
    color: #2980b9;
  }
  
  main {
    padding: 20px;
    background-color: #f5f5f5;
    min-height: 100vh;
  }
</style> 