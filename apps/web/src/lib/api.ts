export type AdminStatus = {
    setup_required: boolean
}

export type AdminSession = {
    authenticated: boolean
}

export type SetupResult = {
    configured: boolean
}

export type EventStatus =
    | 'draft'
    | 'open'
    | 'closed'

export type CrumbEvent = {
    id: number
    slug: string
    title: string
    description: string | null
    status: EventStatus
    results_public: boolean
    created_at: number
    updated_at: number
}

export type CreateEventInput = {
    title: string
    slug: string
    description: string | null
}

export type UpdateEventInput = {
    title: string
    description: string | null
    status: EventStatus
    results_public: boolean
}

export type CrumbEntry = {
    id: number
    event_id: number
    number: number
    name: string
    description: string | null
    image_url: string | null
    created_at: number
    updated_at: number
}

export type CreateEntryInput = {
    name: string
    description: string | null
}

export type UpdateEntryInput = {
    name: string
    description: string | null
}

export type PublicEvent = {
    slug: string
    title: string
    description: string | null
    status: EventStatus
    results_public: boolean
}

export type PublicEntry = {
    id: number
    number: number
    name: string
    description: string | null
    image_url: string | null
}

export type PublicEntryPayload = {
    event: PublicEvent
    entry: PublicEntry
}

export type PublicVoterIdentity = {
    token: string
}

export type PublicVote = {
    entry_id: number | null
}

export type PublicScan = {
    tracked: boolean
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

    if (
        init.body !== undefined &&
        !(init.body instanceof FormData) &&
        !headers.has('Content-Type')
    ) {
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

export function getAdminEvent(
    eventId: number,
): Promise<CrumbEvent> {
    return request<CrumbEvent>(
        `/api/admin/events/${eventId}`,
    )
}

export function updateAdminEvent(
    eventId: number,
    input: UpdateEventInput,
): Promise<CrumbEvent> {
    return request<CrumbEvent>(
        `/api/admin/events/${eventId}`,
        {
            method: 'PATCH',
            body: JSON.stringify(input),
        },
    )
}

export function listAdminEntries(
    eventId: number,
): Promise<CrumbEntry[]> {
    return request<CrumbEntry[]>(
        `/api/admin/events/${eventId}/entries`,
    )
}

export function createAdminEntry(
    eventId: number,
    input: CreateEntryInput,
): Promise<CrumbEntry> {
    return request<CrumbEntry>(
        `/api/admin/events/${eventId}/entries`,
        {
            method: 'POST',
            body: JSON.stringify(input),
        },
    )
}

export function updateAdminEntry(
    eventId: number,
    entryId: number,
    input: UpdateEntryInput,
): Promise<CrumbEntry> {
    return request<CrumbEntry>(
        `/api/admin/events/${eventId}/entries/${entryId}`,
        {
            method: 'PATCH',
            body: JSON.stringify(input),
        },
    )
}

export function uploadAdminEntryImage(
    eventId: number,
    entryId: number,
    image: File,
): Promise<CrumbEntry> {
    const formData = new FormData()

    formData.append('image', image)

    return request<CrumbEntry>(
        `/api/admin/events/${eventId}/entries/${entryId}/image`,
        {
            method: 'POST',
            body: formData,
        },
    )
}

export function getPublicEntry(
    eventSlug: string,
    entryId: number,
): Promise<PublicEntryPayload> {
    return request<PublicEntryPayload>(
        `/api/public/events/${eventSlug}/entries/${entryId}`,
    )
}

export function ensurePublicVoter(
    token: string | null,
): Promise<PublicVoterIdentity> {
    return request<PublicVoterIdentity>(
        '/api/public/voter',
        {
            method: 'POST',
            body: JSON.stringify({ token }),
        },
    )
}

export function getPublicVote(
    eventSlug: string,
    voterToken: string,
): Promise<PublicVote> {
    return request<PublicVote>(
        `/api/public/events/${eventSlug}/vote`,
        {
            headers: {
                'X-CrumbVote-Voter':
                    voterToken,
            },
        },
    )
}

export function castPublicVote(
    eventSlug: string,
    entryId: number,
    voterToken: string,
): Promise<PublicVote> {
    return request<PublicVote>(
        `/api/public/events/${eventSlug}/vote`,
        {
            method: 'POST',
            headers: {
                'X-CrumbVote-Voter':
                    voterToken,
            },
            body: JSON.stringify({
                entry_id: entryId,
            }),
        },
    )
}

export function trackPublicScan(
    eventSlug: string,
    entryId: number,
    voterToken: string,
): Promise<PublicScan> {
    return request<PublicScan>(
        `/api/public/events/${eventSlug}/entries/${entryId}/scan`,
        {
            method: 'POST',
            headers: {
                'X-CrumbVote-Voter':
                    voterToken,
            },
        },
    )
}