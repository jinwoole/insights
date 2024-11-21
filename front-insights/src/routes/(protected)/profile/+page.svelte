<script>
    import { auth } from '$lib/stores/auth';
    import { browser } from '$app/environment';

    let isChangingUsername = false;
    let newUsername = '';

    async function handleLogout() {
        try {
            // Call server logout API to clear HTTP-only cookies
            const response = await fetch('http://localhost:8080/api/auth/logout', {
                method: 'POST',
                credentials: 'include'
            });

            if (response.ok) {
                // Clear frontend state
                auth.clearUser();
                
                // Force reload and redirect
                if (browser) {
                    window.location.href = '/';
                    window.location.reload();
                }
            }
        } catch (error) {
            console.error('Logout failed:', error);
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

<div class="p-8">
    <div class="max-w-7xl mx-auto">
        <div class="mb-8">
            <h1 class="text-3xl font-bold text-gray-900">Profile</h1>
            <p class="mt-2 text-gray-600">Manage your account settings</p>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
            <div class="space-y-6">
                <div>
                    <h2 class="text-xl font-medium text-gray-900">Account Information</h2>
                    <div class="mt-4 space-y-4">
                        <div>
                            <label class="block text-sm font-medium text-gray-700">Username</label>
                            {#if isChangingUsername}
                                <form on:submit|preventDefault={changeUsername} class="mt-1 flex space-x-2">
                                    <input
                                        type="text"
                                        bind:value={newUsername}
                                        placeholder="New username"
                                        class="flex-1 px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
                                    />
                                    <button
                                        type="submit"
                                        class="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                    >
                                        Save
                                    </button>
                                    <button
                                        type="button"
                                        on:click={() => isChangingUsername = false}
                                        class="px-4 py-2 bg-gray-100 text-gray-700 rounded-md hover:bg-gray-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-500"
                                    >
                                        Cancel
                                    </button>
                                </form>
                            {:else}
                                <div class="mt-1 flex justify-between items-center">
                                    <span class="text-gray-900">{$auth.userInfo?.username || 'Not set'}</span>
                                    <button
                                        on:click={() => isChangingUsername = true}
                                        class="text-blue-600 hover:text-blue-800"
                                    >
                                        Change
                                    </button>
                                </div>
                            {/if}
                        </div>

                        <div>
                            <label class="block text-sm font-medium text-gray-700">Email</label>
                            <div class="mt-1">
                                <span class="text-gray-900">{$auth.userInfo?.email || 'Not set'}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="pt-6 border-t border-gray-200">
                    <h2 class="text-xl font-medium text-gray-900">Account Actions</h2>
                    <div class="mt-4">
                        <button
                            on:click={handleLogout}
                            class="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                        >
                            Logout
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>
