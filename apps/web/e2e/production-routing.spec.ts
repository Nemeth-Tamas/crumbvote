import {
    expect,
    test,
} from '@playwright/test'

const backendOrigin =
    'http://127.0.0.1:3100'

test(
    'Rust serves the production SPA without swallowing backend 404s',
    async ({ request }) => {
        const spaRoutes = [
            '/',
            '/admin',
            '/admin/events/123',
            '/e/demo-event/1',
            '/e/demo-event/results',
        ]

        for (const route of spaRoutes) {
            const response =
                await request.get(
                    `${backendOrigin}${route}`,
                )

            expect(
                response.status(),
            ).toBe(200)

            expect(
                response.headers()[
                'content-type'
                ],
            ).toContain(
                'text/html',
            )

            const html =
                await response.text()

            expect(html).toContain(
                '<html lang="hu">',
            )

            expect(html).toContain(
                '<title>CrumbVote</title>',
            )

            expect(html).toContain(
                '<div id="app"></div>',
            )
        }

        const missingApi =
            await request.get(
                `${backendOrigin}/api/not-a-route`,
            )

        expect(
            missingApi.status(),
        ).toBe(404)

        expect(
            await missingApi.text(),
        ).not.toContain(
            '<div id="app"></div>',
        )

        const missingAdminApi =
            await request.get(
                `${backendOrigin}/api/admin/not-a-route`,
            )

        expect(
            missingAdminApi.status(),
        ).toBe(404)

        expect(
            await missingAdminApi.text(),
        ).not.toContain(
            '<div id="app"></div>',
        )

        const missingMedia =
            await request.get(
                `${backendOrigin}/media/entries/not-a-real-image.jpg`,
            )

        expect(
            missingMedia.status(),
        ).toBe(404)

        expect(
            await missingMedia.text(),
        ).not.toContain(
            '<div id="app"></div>',
        )
    },
)