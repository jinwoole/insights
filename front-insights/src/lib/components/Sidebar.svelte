<!-- src/lib/components/Sidebar/Sidebar.svelte -->
<script>
    import NavMenu from './NavMenu.svelte';
    import UserSection from './UserSection.svelte';
  </script>
  
  <aside class="w-64 border-r border-gray-300 flex flex-col justify-between p-4">
    <!-- Logo 섹션 -->
    <div>
      <div class="mb-6">
        <img src="/path-to-logo.png" alt="Logo" class="h-8">
      </div>
      <!-- 네비게이션 메뉴 -->
      <NavMenu />
    </div>
    
    <!-- 사용자 섹션 -->
    <UserSection />
  </aside>
  