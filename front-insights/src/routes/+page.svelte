<script>
    import { browser } from '$app/environment';
    import { goto } from '$app/navigation';
    import { auth } from '$lib/stores/auth';
    import Login from '$lib/components/Login.svelte';
    import Register from '$lib/components/Register.svelte';
    import { onMount } from 'svelte';

    let isRegistering = false;

    onMount(async () => {
        if (browser) {
            try {
                const response = await fetch('http://localhost:8080/api/user/get', {
                    credentials: 'include'
                });
                
                if (response.ok) {
                    const userInfo = await response.json();
                    auth.setUser(userInfo);
                    goto('/dashboard');
                }
            } catch (error) {
                // Ignore error, user is not logged in
            }
        }
    });

    // If user is already logged in, redirect to dashboard
    $: if (browser && $auth.userInfo) {
        goto('/dashboard');
    }
</script>

<div class="min-h-screen bg-gray-50">
    <!-- Header -->
    <header class="bg-white shadow-sm">
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div class="flex justify-between h-16 items-center">
                <div class="flex-shrink-0">
                    <h1 class="text-2xl font-bold text-gray-900">Insights</h1>
                </div>
                <div>
                    <button
                        class="text-gray-600 hover:text-gray-900"
                        on:click={() => isRegistering = !isRegistering}
                    >
                        {isRegistering ? 'Already have an account?' : 'Need an account?'}
                    </button>
                </div>
            </div>
        </div>
    </header>

    <!-- Main content -->
    <main class="max-w-7xl mx-auto py-12 px-4 sm:px-6 lg:px-8">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
            <!-- Left side: App description -->
            <div class="flex flex-col justify-center">
                <h2 class="text-4xl font-extrabold text-gray-900 mb-4">
                    Welcome to Insights
                </h2>
                <p class="text-xl text-gray-500">
                    Your personal analytics dashboard for tracking and visualizing data insights.
                </p>
                <div class="mt-8 space-y-4">
                    <div class="flex items-center">
                        <svg class="h-6 w-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                        </svg>
                        <span class="ml-3 text-gray-700">Real-time data visualization</span>
                    </div>
                    <div class="flex items-center">
                        <svg class="h-6 w-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                        </svg>
                        <span class="ml-3 text-gray-700">Customizable dashboards</span>
                    </div>
                    <div class="flex items-center">
                        <svg class="h-6 w-6 text-green-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                        </svg>
                        <span class="ml-3 text-gray-700">Advanced analytics tools</span>
                    </div>
                </div>
            </div>

            <!-- Right side: Auth forms -->
            <div class="bg-white shadow-xl rounded-lg p-8">
                {#if isRegistering}
                    <Register />
                {:else}
                    <Login />
                {/if}
            </div>
        </div>
    </main>
</div>
