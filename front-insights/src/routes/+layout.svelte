<script>
    import "../app.css";
    import { auth } from '$lib/stores/auth';
    import { page } from '$app/stores';
    import { browser } from '$app/environment';
    import { onMount } from 'svelte';

    // List of routes that don't require authentication
    const publicRoutes = ['/', '/login', '/register', '/auth/login', '/auth/register', '/auth/verify/login', '/auth/verify/register'];

    // Check if current route requires authentication
    $: requiresAuth = browser && !publicRoutes.includes($page.url.pathname);

    // Check authentication on route change
    $: if (browser && requiresAuth) {
        checkAuth();
    }

    async function checkAuth() {
        try {
            const response = await fetch('http://localhost:8080/api/user/get', {
                credentials: 'include'
            });
            
            if (response.ok) {
                const userData = await response.json();
                auth.setUser(userData);
            } else {
                auth.clearUser();
                window.location.href = '/';
            }
        } catch (error) {
            console.error('Auth check failed:', error);
            auth.clearUser();
            window.location.href = '/';
        }
    }

    onMount(() => {
        if (browser && requiresAuth) {
            checkAuth();
        }
    });
</script>

<slot />