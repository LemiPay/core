import { apiFetch } from './client';

export function register(data: {
    name: string,
    email: string,
    password: string
}){
    //todo hashear la contraseña y validar mail

    const x = {
        name: data.name || null,
        email: data.email || null,
        password: data.password || null,
    }

    return apiFetch("/auth/register", {
        method: "POST",
        body: JSON.stringify(x)
    })
}