export type RegisterData = NewUser;

export type LoginData = {
    email: string
    password:string
}

export type User = {
    id: string
    email: string
    password: string
    name: string
}

export type NewUser = {
    email: string
    password: string
    name: string
}

export type SuccessResponse<T> = {
    status: 200
    body: T
    message: string
}

export type FailedResponse =  {
    status: number
    message: string
    body: unknown
}

export type ApiResponse<T> = Promise<SuccessResponse<T> | FailedResponse>