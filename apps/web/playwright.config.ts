import {
    defineConfig,
    devices,
} from '@playwright/test'

const setupCode =
    'E2E0-E2E0-E2E0-E2E0'

const inheritedEnv = Object.fromEntries(
    Object.entries(process.env).filter(
        (entry): entry is [string, string] =>
            entry[1] !== undefined,
    ),
)

export default defineConfig({
    testDir: './e2e',

    fullyParallel: false,
    workers: 1,

    timeout: 60_000,

    expect: {
        timeout: 10_000,
    },

    reporter: [
        ['list'],
        ['html', { open: 'never' }],
    ],

    use: {
        baseURL: 'http://127.0.0.1:5174',
        trace: 'retain-on-failure',
        screenshot: 'only-on-failure',
    },

    projects: [
        {
            name: 'chromium',
            use: {
                ...devices['Desktop Chrome'],
            },
        },
    ],

    webServer: [
        {
            name: 'CrumbVote backend',
            command: 'npm run e2e:backend',
            url: 'http://127.0.0.1:3100/health',
            timeout: 120_000,
            reuseExistingServer: false,

            env: {
                ...inheritedEnv,
                PLAYWRIGHT_TEST: '1',
                CRUMBVOTE_LISTEN_ADDRESS:
                    '127.0.0.1:3100',
                CRUMBVOTE_DATABASE_URL:
                    'sqlite://data/crumbvote-e2e.sqlite?mode=rwc',
                CRUMBVOTE_E2E_SETUP_CODE:
                    setupCode,
            },
        },

        {
            name: 'CrumbVote frontend',
            command:
                'npm run dev -- --host 127.0.0.1 --port 5174',
            url: 'http://127.0.0.1:5174',
            timeout: 60_000,
            reuseExistingServer: false,

            env: {
                ...inheritedEnv,
                CRUMBVOTE_DEV_API:
                    'http://127.0.0.1:3100',
            },
        },
    ],
})