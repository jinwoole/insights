<script>
    let username = '';
    let password = '';

    async function register() {
        const response = await fetch('http://localhost:8080/api/auth/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password })
        });

        if (response.ok) {
            // Handle success
            console.log('Registration successful');
        } else {
            // Handle error
            const error = await response.json();
            console.error('Registration failed:', error);
        }
    }

    async function login() {
        const response = await fetch('http://localhost:8080/api/auth/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password }),
            credentials: 'include' // Include cookies in the request
        });

        if (response.ok) {
            // The token is now stored in an HTTP-only cookie
            console.log('Login successful');
        } else {
            const error = await response.json();
            console.error('Login failed:', error);
        }
    }

    let userInfo = null;

    async function getUserInfo() {
        const response = await fetch('http://localhost:8080/api/user', {
            method: 'GET',
            credentials: 'include'
        });

        if (response.ok) {
            userInfo = await response.json();
            console.log('User info:', userInfo);
        } else {
            console.error('Failed to fetch user info');
        }
    }

    async function updateUser() {
        const response = await fetch('http://localhost:8080/api/user', {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ username, password }),
            credentials: 'include'
        });

        if (response.ok) {
            console.log('User updated successfully');
        } else {
            const error = await response.json();
            console.error('Update failed:', error);
        }
    }


</script>

<form on:submit|preventDefault={register} class="space-y-4 p-4 border rounded-md shadow-md">
    <div>
        <label class="block text-sm font-medium text-gray-700">
            Username:
            <input type="text" bind:value={username} required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </label>
    </div>
    <div>
        <label class="block text-sm font-medium text-gray-700">
            Password:
            <input type="password" bind:value={password} required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </label>
    </div>
    <div>
        <button type="submit" class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
            Register
        </button>
    </div>
</form>

<form on:submit|preventDefault={login} class="space-y-4 p-4 border rounded-md shadow-md mt-6">
    <div>
        <label class="block text-sm font-medium text-gray-700">
            Username:
            <input type="text" bind:value={username} required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </label>
    </div>
    <div>
        <label class="block text-sm font-medium text-gray-700">
            Password:
            <input type="password" bind:value={password} required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </label>
    </div>
    <div>
        <button type="submit" class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
            Login
        </button>
    </div>
</form>

<form on:submit|preventDefault={getUserInfo} class="space-y-4 p-4 border rounded-md shadow-md mt-6">
    <div>
        <button type="submit" class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
            Get User Info
        </button>
    </div>
</form>

<form on:submit|preventDefault={updateUser} class="space-y-4 p-4 border rounded-md shadow-md mt-6">
    <div>
        <label class="block text-sm font-medium text-gray-700">
            New Username:
            <input type="text" bind:value={username} required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </label>
    </div>
    <div>
        <label class="block text-sm font-medium text-gray-700">
            New Password:
            <input type="password" bind:value={password} required class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm" />
        </label>
    </div>
    <div>
        <button type="submit" class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
            Update User
        </button>
    </div>
</form>