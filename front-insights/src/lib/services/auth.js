import { auth } from '../stores/auth';

const API_BASE = 'http://localhost:8080/api';

export async function register(username, email) {
    try {
        const response = await fetch(`${API_BASE}/auth/register`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, email })
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.message || 'Registration failed');
        }

        return true;
    } catch (error) {
        throw error;
    }
}

export async function verifyRegister(username, email, code) {
    try {
        const response = await fetch(`${API_BASE}/auth/verify/register`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, email, code: Number(code) }),
            credentials: 'include'
        });

        if (!response.ok) {
            throw new Error(await response.text());
        }

        const userInfo = await getUserInfo();
        auth.setUser(userInfo);
        return true;
    } catch (error) {
        throw error;
    }
}

export async function login(email) {
    try {
        const response = await fetch(`${API_BASE}/auth/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email })
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.message || 'Login failed');
        }

        return true;
    } catch (error) {
        throw error;
    }
}

export async function verifyLogin(email, code) {
    try {
        const response = await fetch(`${API_BASE}/auth/verify/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ email, code: Number(code) }),
            credentials: 'include'
        });

        if (!response.ok) {
            throw new Error(await response.text());
        }

        const userInfo = await getUserInfo();
        auth.setUser(userInfo);
        return true;
    } catch (error) {
        throw error;
    }
}

export async function getUserInfo() {
    try {
        const response = await fetch(`${API_BASE}/user/get`, {
            method: 'GET',
            credentials: 'include'
        });

        if (!response.ok) {
            throw new Error('Failed to fetch user info');
        }

        const userInfo = await response.json();
        return userInfo;
    } catch (error) {
        throw error;
    }
}

export async function updateUsername(username) {
    try {
        const response = await fetch(`${API_BASE}/user/username`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username }),
            credentials: 'include'
        });

        if (!response.ok) {
            const error = await response.json();
            throw new Error(error.message || 'Failed to update username');
        }

        const userInfo = await getUserInfo();
        auth.setUser(userInfo);
        return true;
    } catch (error) {
        throw error;
    }
}
