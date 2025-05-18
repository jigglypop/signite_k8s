<script>
  let email = '';
  let password = '';
  let rememberMe = false;
  
  function handleSubmit() {
    console.log('로그인 시도:', { email, password, rememberMe });
    alert('로그인 기능이 구현되었습니다! (데모용)');
  }
</script>

<div class="login-form">
  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="email">이메일</label>
      <input 
        type="email" 
        id="email" 
        bind:value={email} 
        placeholder="이메일 주소를 입력하세요"
        required
      />
    </div>
    
    <div class="form-group">
      <label for="password">비밀번호</label>
      <input 
        type="password" 
        id="password" 
        bind:value={password} 
        placeholder="비밀번호를 입력하세요"
        required
      />
    </div>
    
    <div class="form-check">
      <input type="checkbox" id="remember-me" bind:checked={rememberMe} />
      <label for="remember-me">로그인 상태 유지</label>
    </div>
    
    <button type="submit" class="btn-login">로그인</button>
  </form>
  
  <div class="password-actions">
    <a href="#reset-password">비밀번호를 잊으셨나요?</a>
  </div>
</div>

<style>
  .login-form {
    width: 100%;
  }
  
  .form-group {
    margin-bottom: 16px;
  }
  
  label {
    display: block;
    margin-bottom: 6px;
    font-size: 14px;
    color: #333;
  }
  
  input[type="email"],
  input[type="password"] {
    width: 100%;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 16px;
  }
  
  .form-check {
    display: flex;
    align-items: center;
    margin-bottom: 16px;
  }
  
  .form-check input {
    margin-right: 8px;
  }
  
  .form-check label {
    margin-bottom: 0;
  }
  
  .btn-login {
    width: 100%;
    background-color: #3498db;
    color: white;
    border: none;
    border-radius: 4px;
    padding: 12px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s;
  }
  
  .btn-login:hover {
    background-color: #2980b9;
  }
  
  .password-actions {
    text-align: center;
    margin-top: 16px;
    font-size: 14px;
  }
  
  a {
    color: #3498db;
    text-decoration: none;
  }
  
  a:hover {
    text-decoration: underline;
  }
</style> 