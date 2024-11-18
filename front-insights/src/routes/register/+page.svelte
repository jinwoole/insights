<script>
    let username = '';
    let email = '';
    let verificationCode = '';
    let isVerifying = false;
    let errorMessage = '';

    async function register() {
        try {
            const response = await fetch('http://localhost:8080/api/auth/register', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ username, email })
            });

            if (response.ok) {
                isVerifying = true;
                console.log('Verification code sent to email');
            } else {
                const error = await response.json();
                errorMessage = error.message || 'Registration failed';
                console.error('Registration failed:', error);
            }
        } catch (err) {
            errorMessage = 'An unexpected error occurred';
            console.error('An unexpected error occurred:', err);
        }
    }

    async function verifyRegister() {
        try {
            const response = await fetch('http://localhost:8080/api/auth/verify-register', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ email, verificationCode })
            });
            
            if (response.ok) {
                console.log('Registration verified successfully');
                // Optionally redirect to another page
            } else {
                const error = await response.json();
                errorMessage = error.message || 'Verification failed';
                console.error('Verification failed:', error);
            }
        } catch (err) {
            errorMessage = 'An unexpected error occurred during verification';
            console.error('An unexpected error occurred during verification:', err);
        }
    }
</script>

<div class="flex justify-center p-4">
    <div class="w-full max-w-sm">
        <h2 class="text-lg font-semibold mb-2 text-center">Register</h2>
        <!-- Registration Form -->
        {#if !isVerifying}
            <form on:submit|preventDefault={register} class="space-y-4 border rounded-md p-4 shadow-md">
                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        Username:
                        <input type="text" bind:value={username} required class="mt-1 block w-full" />
                    </label>
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        Email:
                        <input type="email" bind:value={email} required disabled={isVerifying} class="mt-1 block w-full" />
                    </label>
                </div>
                {#if errorMessage}
                    <div class="text-red-500 text-sm">{errorMessage}</div>
                {/if}
                <div>
                    <button type="submit" class="w-full py-2 px-4 bg-indigo-600 text-white rounded-md">
                        Register
                    </button>
                </div>
            </form>
        {/if}

        <!-- Verification Form -->
        {#if isVerifying}
            <form on:submit|preventDefault={verifyRegister} class="space-y-4 border rounded-md p-4 shadow-md">
                <div>
                    <label class="block text-sm font-medium text-gray-700">
                        Verification Code:
                        <input type="text" bind:value={verificationCode} required class="mt-1 block w-full" />
                    </label>
                </div>
                {#if errorMessage}
                    <div class="text-red-500 text-sm">{errorMessage}</div>
                {/if}
                <div>
                    <button type="submit" class="w-full py-2 px-4 bg-indigo-600 text-white rounded-md">
                        Verify
                    </button>
                </div>
            </form>
        {/if}
    </div>
</div>
