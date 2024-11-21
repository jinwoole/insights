<script>
    import { auth } from '$lib/stores/auth';
    import { updateUsername } from '$lib/services/auth';

    let newUsername = '';
    let isEditing = false;
    let errorMessage = '';

    async function handleUpdateUsername() {
        try {
            errorMessage = '';
            await updateUsername(newUsername);
            isEditing = false;
            newUsername = '';
        } catch (error) {
            errorMessage = error.message;
        }
    }
</script>

<div class="w-full max-w-md mx-auto">
    <h2 class="text-2xl font-bold mb-6">Profile</h2>
    
    {#if $auth.error}
        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4" role="alert">
            <span class="block sm:inline">{$auth.error}</span>
        </div>
    {/if}

    {#if $auth.userInfo}
        <div class="bg-white shadow overflow-hidden sm:rounded-lg">
            <div class="px-4 py-5 sm:px-6">
                <h3 class="text-lg leading-6 font-medium text-gray-900">User Information</h3>
            </div>
            <div class="border-t border-gray-200">
                <dl>
                    <div class="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                        <dt class="text-sm font-medium text-gray-500">Username</dt>
                        <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                            {#if isEditing}
                                <form on:submit|preventDefault={handleUpdateUsername} class="flex gap-2">
                                    <input
                                        type="text"
                                        bind:value={newUsername}
                                        class="flex-1 rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                                        placeholder={$auth.userInfo.username}
                                        required
                                    />
                                    <button
                                        type="submit"
                                        class="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                        disabled={$auth.isLoading}
                                    >
                                        {$auth.isLoading ? 'Saving...' : 'Save'}
                                    </button>
                                    <button
                                        type="button"
                                        class="inline-flex justify-center py-2 px-4 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                        on:click={() => {
                                            isEditing = false;
                                            newUsername = '';
                                        }}
                                    >
                                        Cancel
                                    </button>
                                </form>
                            {:else}
                                <div class="flex justify-between items-center">
                                    <span>{$auth.userInfo.username}</span>
                                    <button
                                        type="button"
                                        class="inline-flex justify-center py-2 px-4 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                                        on:click={() => isEditing = true}
                                    >
                                        Edit
                                    </button>
                                </div>
                            {/if}
                        </dd>
                    </div>
                    <div class="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                        <dt class="text-sm font-medium text-gray-500">Email</dt>
                        <dd class="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">{$auth.userInfo.email}</dd>
                    </div>
                </dl>
            </div>
        </div>
    {:else}
        <div class="text-center text-gray-600">
            Please log in to view your profile.
        </div>
    {/if}
</div>
