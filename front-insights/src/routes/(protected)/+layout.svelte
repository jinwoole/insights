<script>
    import { browser } from '$app/environment';
    import { goto } from '$app/navigation';
    import { auth } from '$lib/stores/auth';
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import "../../app.css";

    onMount(async () => {
        if (browser) {
            try {
                const response = await fetch('http://localhost:8080/api/user/get', {
                    credentials: 'include'
                });
                
                if (!response.ok) {
                    goto('/');
                    return;
                }

                const userInfo = await response.json();
                auth.setUser(userInfo);
            } catch (error) {
                goto('/');
            }
        }
    });

    // Subscribe to auth store to check login status
    $: if (browser && !$auth.userInfo) {
        goto('/');
    }

    // Get current path for active state
    $: currentPath = $page.url.pathname;
</script>

<div class="flex">
    <aside class="flex flex-col items-center w-16 h-screen py-8 overflow-y-auto bg-white border-r rtl:border-l rtl:border-r-0 dark:bg-gray-900 dark:border-gray-700">
        <nav class="flex flex-col flex-1 space-y-6">
            <a 
                href="/dashboard" 
                class="p-1.5 transition-colors duration-200 rounded-lg {currentPath === '/dashboard' ? 'bg-indigo-100 text-indigo-600' : 'text-gray-500 hover:bg-gray-100'}"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 12l8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" />
                </svg>
            </a>

            <a 
                href="/insights" 
                class="p-1.5 transition-colors duration-200 rounded-lg {currentPath === '/insights' ? 'bg-indigo-100 text-indigo-600' : 'text-gray-500 hover:bg-gray-100'}"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
                </svg>
            </a>

            <a 
                href="/sources" 
                class="p-1.5 transition-colors duration-200 rounded-lg {currentPath === '/sources' ? 'bg-indigo-100 text-indigo-600' : 'text-gray-500 hover:bg-gray-100'}"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
                </svg>
            </a>

            <a 
                href="/alerts" 
                class="p-1.5 transition-colors duration-200 rounded-lg {currentPath === '/alerts' ? 'bg-indigo-100 text-indigo-600' : 'text-gray-500 hover:bg-gray-100'}"
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0" />
                </svg>
            </a>
        </nav>

        <div class="flex flex-col space-y-6">
            <a 
                href="/profile" 
                class="p-1.5 transition-colors duration-200 rounded-lg {currentPath === '/profile' ? 'bg-indigo-100' : 'hover:bg-gray-100'}"
            >
                {#if $auth.userInfo?.username}
                    <div class="w-8 h-8 rounded-full {currentPath === '/profile' ? 'bg-indigo-600' : 'bg-gray-600'} flex items-center justify-center text-white font-semibold">
                        {$auth.userInfo.username[0].toUpperCase()}
                    </div>
                {:else}
                    <div class="w-8 h-8 rounded-full bg-gray-200 flex items-center justify-center">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-5 h-5 text-gray-500">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z" />
                        </svg>
                    </div>
                {/if}
            </a>
        </div>
    </aside>

    <div class="flex-1">
        <slot />
    </div>
</div>
