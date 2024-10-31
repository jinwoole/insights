// src/lib/stores/authStore.js
import { writable } from 'svelte/store';

export const isAuthenticated = writable(false);
export const userProfile = writable(null); // 사용자 프로필 정보 저장
