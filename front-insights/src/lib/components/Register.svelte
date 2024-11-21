<script>
    import { register, verifyRegister } from '$lib/services/auth';
    import { auth } from '$lib/stores/auth';

    let username = '';
    let email = '';
    let verificationCode = '';
    let isVerifying = false;
    let errorMessage = '';

    async function handleRegister() {
        try {
            errorMessage = '';
            await register(username, email);
            isVerifying = true;
        } catch (error) {
            errorMessage = error.message;
        }
    }

    async function handleVerify() {
        try {
            errorMessage = '';
            await verifyRegister(username, email, verificationCode);
        } catch (error) {
            errorMessage = error.message;
        }
    }
</script>

<div class="w-full max-w-md mx-auto">
    <h2 class="text-2xl font-bold mb-6">Register</h2>
    
    {#if $auth.error}
        <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4" role="alert">
            <span class="block sm:inline">{$auth.error}</span>
        </div>
    {/if}

    {#if !isVerifying}
        <form on:submit|preventDefault={handleRegister} class="space-y-4">
            <div>
                <label class="block text-sm font-medium text-gray-700" for="username">
                    Username
                </label>
                <input
                    type="text"
                    id="username"
                    bind:value={username}
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    required
                />
            </div>

            <div>
                <label class="block text-sm font-medium text-gray-700" for="email">
                    Email
                </label>
                <input
                    type="email"
                    id="email"
                    bind:value={email}
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    required
                />
            </div>

            <button
                type="submit"
                class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                disabled={$auth.isLoading}
            >
                {$auth.isLoading ? 'Loading...' : 'Register'}
            </button>
        </form>
    {:else}
        <form on:submit|preventDefault={handleVerify} class="space-y-4">
            <div>
                <label class="block text-sm font-medium text-gray-700" for="code">
                    Verification Code
                </label>
                <input
                    type="text"
                    id="code"
                    bind:value={verificationCode}
                    class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm"
                    required
                />
            </div>

            <button
                type="submit"
                class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                disabled={$auth.isLoading}
            >
                {$auth.isLoading ? 'Verifying...' : 'Verify Code'}
            </button>

            <button
                type="button"
                class="w-full flex justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                on:click={() => isVerifying = false}
            >
                Back to Register
            </button>
        </form>
    {/if}
</div>
