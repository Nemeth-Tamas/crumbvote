export type AdminStatus = {
    setup_required: boolean
}

export type AdminSession = {
    authenticated: boolean
}

export type SetupResult = {
    configured: boolean
}

export type CrumbEvent = {
    id: number
    slug: string
    title: string
    description: string | null
    status: string
    results_public: boolean
    created_at: number
    updated_at: number
}

export type CreateEventInput = {
    title: string
    slug: string
    description: string | null
}

export class ApiError extends Error {
    constructor(
        public readonly status: number,
        public readonly code: string,
    ) {
        super(code)
        this.name = 'ApiError'
    }
}

async function request<T>(
    path: string,
    init: RequestInit = {},
): Promise<T> {
    const headers = new Headers(init.headers)

    if (init.body !== undefined && !headers.has('Content-Type')) {
        headers.set('Content-Type', 'application/json')
    }

    const response = await fetch(path, {
        ...init,
        headers,
        credentials: 'same-origin',
    })

    let body: unknown = null

    const contentType = response.headers.get('content-type') ?? ''

    if (contentType.includes('application/json')) {
        body = await response.json()
    }

    if (!response.ok) {
        let code = 'request_failed'

        if (
            typeof body === 'object' &&
            body !== null &&
            'error' in body
        ) {
            const error = (body as { error?: unknown }).error

            if (typeof error === 'string') {
                code = error
            }
        }

        throw new ApiError(response.status, code)
    }

    return body as T
}

export function getAdminStatus(): Promise<AdminStatus> {
    return request<AdminStatus>('/api/admin/status')
}

export function getAdminSession(): Promise<AdminSession> {
    return request<AdminSession>('/api/admin/session')
}

export function setupAdmin(
    setupCode: string,
    password: string,
): Promise<SetupResult> {
    return request<SetupResult>('/api/admin/setup', {
        method: 'POST',
        body: JSON.stringify({
            setup_code: setupCode,
            password,
        }),
    })
}

export function loginAdmin(
    password: string,
): Promise<AdminSession> {
    return request<AdminSession>('/api/admin/login', {
        method: 'POST',
        body: JSON.stringify({ password }),
    })
}

export function logoutAdmin(): Promise<AdminSession> {
    return request<AdminSession>('/api/admin/logout', {
        method: 'POST',
    })
}

export function listAdminEvents(): Promise<CrumbEvent[]> {
    return request<CrumbEvent[]>('/api/admin/events')
}

export function createAdminEvent(
    input: CreateEventInput,
): Promise<CrumbEvent> {
    return request<CrumbEvent>('/api/admin/events', {
        method: 'POST',
        body: JSON.stringify(input),
    })
}