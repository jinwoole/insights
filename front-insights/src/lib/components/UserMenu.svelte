<script>
    import { auth } from '$lib/stores/auth';
    import { browser } from '$app/environment';

    export let username = '';
    let isMenuOpen = false;
    let isChangingUsername = false;
    let newUsername = '';

    function handleLogout() {
        // Clear auth state
        auth.clearUser();
        
        // Clear session cookie by setting it to expire
        document.cookie = 'session=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/';
        
        // Redirect to home
        if (browser) {
            window.location.href = '/';
        }
    }

    async function changeUsername() {
        try {
            const response = await fetch('http://localhost:8080/api/user/username', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ username: newUsername }),
                credentials: 'include'
            });

            if (response.ok) {
                auth.updateUsername(newUsername);
                isChangingUsername = false;
                newUsername = '';
            }
        } catch (error) {
            console.error('Change username failed:', error);
        }
    }
</script>

<div class="relative">
    <button
        on:click={() => isMenuOpen = !isMenuOpen}
        class="flex items-center space-x-2 p-2 hover:bg-gray-100 rounded-md"
    >
        <span>{username || 'User'}</span>
        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
    </button>

    {#if isMenuOpen}
        <div class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg py-1">
            {#if isChangingUsername}
                <form on:submit|preventDefault={changeUsername} class="px-4 py-2">
                    <input
                        type="text"
                        bind:value={newUsername}
                        placeholder="New username"
                        class="w-full px-2 py-1 border rounded"
                    />
                    <div class="mt-2 flex space-x-2">
                        <button
                            type="submit"
                            class="px-2 py-1 bg-blue-500 text-white rounded hover:bg-blue-600"
                        >
                            Save
                        </button>
                        <button
                            type="button"
                            on:click={() => isChangingUsername = false}
                            class="px-2 py-1 bg-gray-200 rounded hover:bg-gray-300"
                        >
                            Cancel
                        </button>
                    </div>
                </form>
            {:else}
                <button
                    on:click={() => isChangingUsername = true}
                    class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                >
                    Change Username
                </button>
            {/if}
            
            <button
                on:click={handleLogout}
                class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
            >
                Logout
            </button>
        </div>
    {/if}
</div>
