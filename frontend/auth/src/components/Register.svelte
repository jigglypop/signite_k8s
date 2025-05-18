<script>
  let name = '';
  let email = '';
  let password = '';
  let confirmPassword = '';
  let termsAccepted = false;
  
  let passwordError = '';
  
  function validatePassword() {
    if (password !== confirmPassword) {
      passwordError = '비밀번호가 일치하지 않습니다';
      return false;
    }
    
    if (password.length < 8) {
      passwordError = '비밀번호는 최소 8자 이상이어야 합니다';
      return false;
    }
    
    passwordError = '';
    return true;
  }
  
  function handleSubmit() {
    if (!validatePassword()) {
      return;
    }
    
    if (!termsAccepted) {
      alert('이용약관에 동의해주세요');
      return;
    }
    
    console.log('회원가입 시도:', { name, email, password });
    alert('회원가입 기능이 구현되었습니다! (데모용)');
  }
</script>

<div class="register-form">
  <form on:submit|preventDefault={handleSubmit}>
    <div class="form-group">
      <label for="name">이름</label>
      <input 
        type="text" 
        id="name" 
        bind:value={name} 
        placeholder="이름을 입력하세요"
        required
      />
    </div>
    
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
        on:blur={validatePassword}
        placeholder="비밀번호를 입력하세요 (8자 이상)"
        required
      />
    </div>
    
    <div class="form-group">
      <label for="confirm-password">비밀번호 확인</label>
      <input 
        type="password" 
        id="confirm-password" 
        bind:value={confirmPassword} 
        on:blur={validatePassword}
        placeholder="비밀번호를 다시 입력하세요"
        required
      />
      {#if passwordError}
        <div class="password-error">{passwordError}</div>
      {/if}
    </div>
    
    <div class="form-check">
      <input type="checkbox" id="terms" bind:checked={termsAccepted} />
      <label for="terms">이용약관 및 개인정보 처리방침에 동의합니다</label>
    </div>
    
    <button type="submit" class="btn-register">회원가입</button>
  </form>
</div>

<style>
  .register-form {
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
  
  input[type="text"],
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
    align-items: flex-start;
    margin-bottom: 16px;
  }
  
  .form-check input {
    margin-right: 8px;
    margin-top: 3px;
  }
  
  .password-error {
    color: #e74c3c;
    font-size: 12px;
    margin-top: 4px;
  }
  
  .btn-register {
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
  
  .btn-register:hover {
    background-color: #2980b9;
  }
</style> 