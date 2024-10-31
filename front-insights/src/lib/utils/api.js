// src/lib/utils/api.js

import { get } from 'svelte/store';
import { authToken } from '$lib/stores/authStore';

const BASE_URL = 'https://api.example.com'; // 실제 API의 기본 URL로 변경하세요

/**
 * 공통 헤더 설정을 동적으로 생성
 */
function getHeaders() {
  const token = get(authToken);
  return {
    'Content-Type': 'application/json',
    ...(token && { 'Authorization': `Bearer ${token}` }),
  };
}

/**
 * GET 요청
 * @param {string} endpoint - API 엔드포인트
 * @returns {Promise<any>} - 응답 데이터
 */
export async function apiGet(endpoint) {
  try {
    const response = await fetch(`${BASE_URL}${endpoint}`, {
      method: 'GET',
      headers: getHeaders(),
    });
    if (!response.ok) {
      throw new Error(`GET 요청 실패: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('apiGet 에러:', error);
    throw error;
  }
}

/**
 * POST 요청
 * @param {string} endpoint - API 엔드포인트
 * @param {Object} data - 전송할 데이터
 * @returns {Promise<any>} - 응답 데이터
 */
export async function apiPost(endpoint, data) {
  try {
    const response = await fetch(`${BASE_URL}${endpoint}`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    if (!response.ok) {
      throw new Error(`POST 요청 실패: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('apiPost 에러:', error);
    throw error;
  }
}

/**
 * PUT 요청
 * @param {string} endpoint - API 엔드포인트
 * @param {Object} data - 전송할 데이터
 * @returns {Promise<any>} - 응답 데이터
 */
export async function apiPut(endpoint, data) {
  try {
    const response = await fetch(`${BASE_URL}${endpoint}`, {
      method: 'PUT',
      headers: getHeaders(),
      body: JSON.stringify(data),
    });
    if (!response.ok) {
      throw new Error(`PUT 요청 실패: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('apiPut 에러:', error);
    throw error;
  }
}

/**
 * DELETE 요청
 * @param {string} endpoint - API 엔드포인트
 * @returns {Promise<any>} - 응답 데이터
 */
export async function apiDelete(endpoint) {
  try {
    const response = await fetch(`${BASE_URL}${endpoint}`, {
      method: 'DELETE',
      headers: getHeaders(),
    });
    if (!response.ok) {
      throw new Error(`DELETE 요청 실패: ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error('apiDelete 에러:', error);
    throw error;
  }
}
