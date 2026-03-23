import { apiFetch } from './client';

export function register(data: {
    name: string,
    email: string,
    password: string
}){
    return apiFetch("/auth/register", {
        method: "POST",
        body: JSON.stringify(data)
    })
}