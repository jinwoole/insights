import { writable } from 'svelte/store';
import { browser } from '$app/environment';

// Initialize the store with data from localStorage if available
const initialState = browser ? JSON.parse(localStorage.getItem('auth')) || { isAuthenticated: false, userInfo: null } : { isAuthenticated: false, userInfo: null };

const createAuthStore = () => {
    const { subscribe, set, update } = writable(initialState);

    return {
        subscribe,
        setUser: (userInfo) => {
            const authState = { isAuthenticated: true, userInfo };
            if (browser) {
                localStorage.setItem('auth', JSON.stringify(authState));
            }
            set(authState);
        },
        clearUser: () => {
            const authState = { isAuthenticated: false, userInfo: null };
            if (browser) {
                localStorage.removeItem('auth');
            }
            set(authState);
        },
        updateUsername: (username) => {
            update(state => {
                const newState = {
                    ...state,
                    userInfo: { ...state.userInfo, username }
                };
                if (browser) {
                    localStorage.setItem('auth', JSON.stringify(newState));
                }
                return newState;
            });
        }
    };
};

export const auth = createAuthStore();
