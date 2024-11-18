<script>
    let email = '';
    let verificationCode = '';
    let isVerifying = false;
    let errorMessage = '';

    async function login() {
        try {
            const response = await fetch('http://localhost:8080/api/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ email })
            });

            if (response.ok) {
                isVerifying = true;
                console.log('Verification code sent to email');
            } else {
                const error = await response.json();
                errorMessage = error.message || 'Login failed';
                console.error('Login failed:', error);
            }
        } catch (err) {
            errorMessage = 'An unexpected error occurred';
            console.error('An unexpected error occurred:', err);
        }
    }

    async function verifyLogin() {
        try {
            const response = await fetch('http://localhost:8080/api/auth/verify/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ email: email, code: Number(verificationCode) }),
                credentials: 'include'
            });

            if (response.ok) {
                console.log('Login verified successfully');
            } else {
                const errorText = await response.text();
                console.error('Verification failed:', errorText);
            }
        } catch (err) {
            console.error('An unexpected error occurred:', err);
        }
    }
</script>

<div class="flex justify-center p-4">
    <div class="w-full max-w-sm">
        <h2 class="text-lg font-semibold mb-2 text-center">Login</h2>
        <!-- Login Form -->
        {#if !isVerifying}
            <form on:submit|preventDefault={login} class="space-y-4 border rounded-md p-4 shadow-md">
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
                        Login
                    </button>
                </div>
            </form>
        {/if}

        <!-- Verification Form -->
        {#if isVerifying}
            <form on:submit|preventDefault={verifyLogin} class="space-y-4 border rounded-md p-4 shadow-md">
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
                        Verify Login
                    </button>
                </div>
            </form>
        {/if}
    </div>
</div>
