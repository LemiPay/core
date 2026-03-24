import { writable } from 'svelte/store';

// 1. El estado inicial
export const isAuthenticated = writable(false);

// 2. Sincronización con la realidad
export const checkAuth = () => {
    const token = localStorage.getItem("token");
    isAuthenticated.set(!!token); // Convierte el token (string o null) a booleano (true o false)
};

// 3. Limpieza de seguridad
export const logout = () => {
    localStorage.removeItem("token");
    isAuthenticated.set(false);
    window.location.href = "/login";
};