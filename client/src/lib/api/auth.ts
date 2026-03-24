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

export function login(data:{
    email: string,
    password:string
}): Promise<{ token: string }>{
    return apiFetch("/auth/login", {
        method:"POST",
        body: JSON.stringify(data)
    })
}