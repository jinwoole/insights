<script>
    import "../app.css";

    import Sidebar from "$lib/components/Sidebar.svelte";
</script>
  
  <div class="flex h-screen">
    <Sidebar />
    <main class="flex-1 p-4">
      <slot />
    </main>
  </div>
  