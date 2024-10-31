<script>
  import { isAuthenticated, userProfile, login, logout } from '$lib/stores/authStore';
  import { onDestroy } from 'svelte';
  
  let authenticated = false;
  let user = null;
  
  const unsubscribeAuth = isAuthenticated.subscribe(value => {
    authenticated = value;
  });
  
  const unsubscribeUser = userProfile.subscribe(value => {
    user = value;
  });
  
  onDestroy(() => {
    unsubscribeAuth();
    unsubscribeUser();
  });
  
  // 로그인 함수 예시
  async function handleLogin() {
    try {
      // 실제 애플리케이션에서는 API 호출을 통해 토큰과 사용자 정보를 받아옵니다.
      const response = await fetch('https://api.example.com/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ username: 'user', password: 'pass' }),
      });
      
      if (!response.ok) {
        throw new Error('로그인 실패');
      }
      
      const data = await response.json();
      
      // 로그인 성공 시 토큰과 사용자 정보 설정
      login(data.token, {
        name: data.user.name,
        avatar: data.user.avatar,
      });
    } catch (error) {
      console.error('로그인 에러:', error);
      // 에러 메시지 표시 등 추가 처리
    }
  }
  
  function handleLogout() {
    logout();
  }
</script>

{#if authenticated && user}
  <div class="border-t border-gray-300 pt-4 flex items-center justify-between">
    <div class="flex -space-x-2 overflow-hidden">
      <img class="inline-block h-10 w-10 rounded-full ring-2 ring-white" src={user.avatar} alt="User profile">
    </div>
    <button on:click={handleLogout} class="py-2 px-4 rounded hover:bg-gray-200">Logout</button>
  </div>
{:else}
  <div class="border-t border-gray-300 pt-4 flex items-center justify-between">
    <div class="flex -space-x-2 overflow-hidden">
      <img class="inline-block h-10 w-10 rounded-full ring-2 ring-white" src="https://via.placeholder.com/40" alt="Guest">
    </div>
    <button on:click={handleLogin} class="py-2 px-4 rounded hover:bg-gray-200">Login</button>
  </div>
{/if}
