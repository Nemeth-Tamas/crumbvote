import {
    expect,
    test,
} from '@playwright/test'

const SETUP_CODE =
    'E2E0-E2E0-E2E0-E2E0'

const PASSWORD =
    'this is my e2e password'

const PNG_BASE64 =
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII='

test(
    'admin setup and event management happy path',
    async ({ page }) => {
        await page.goto('/admin')

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'Claim this CrumbVote',
                },
            ),
        ).toBeVisible()

        await page
            .getByLabel(
                'Setup code',
                { exact: true },
            )
            .fill(SETUP_CODE)

        await page
            .getByLabel(
                'Administrator password',
            )
            .fill(PASSWORD)

        await page
            .getByLabel(
                'Confirm password',
                { exact: true },
            )
            .fill(PASSWORD)

        await page
            .getByRole(
                'button',
                {
                    name: 'Configure CrumbVote',
                },
            )
            .click()

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'Admin console',
                },
            ),
        ).toBeVisible()

        await page
            .getByRole(
                'button',
                {
                    name: /Create event/,
                },
            )
            .first()
            .click()

        const createEventDialog =
            page.getByRole(
                'dialog',
                {
                    name: 'Create event',
                },
            )

        await createEventDialog
            .getByLabel(
                'Event title',
                { exact: true },
            )
            .fill('E2E Cake Show')

        await createEventDialog
            .getByLabel(
                'Description',
            )
            .fill(
                'A cake show created by our tiny robot.',
            )

        await createEventDialog
            .getByRole(
                'button',
                {
                    name: 'Create event',
                    exact: true,
                },
            )
            .click()

        const eventCard =
            page
                .getByRole('article')
                .filter({
                    hasText: 'E2E Cake Show',
                })

        await expect(eventCard).toBeVisible()

        await eventCard
            .getByRole(
                'link',
                {
                    name: /Manage event/,
                },
            )
            .click()

        await expect(page).toHaveURL(
            /\/admin\/events\/\d+$/,
        )

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'E2E Cake Show',
                },
            ),
        ).toBeVisible()

        await page
            .getByRole(
                'button',
                {
                    name: /Add entry/,
                },
            )
            .first()
            .click()

        const addEntryDialog =
            page.getByRole(
                'dialog',
                {
                    name: 'Add entry',
                },
            )

        await addEntryDialog
            .getByLabel(
                'Entry name',
                { exact: true },
            )
            .fill('E2E Raspberry Cake')

        await addEntryDialog
            .getByLabel(
                'Description',
            )
            .fill(
                'Raspberry science, automated.',
            )

        await addEntryDialog
            .getByRole(
                'button',
                {
                    name: 'Add entry',
                    exact: true,
                },
            )
            .click()

        const entryCard =
            page
                .getByRole('article')
                .filter({
                    hasText:
                        'E2E Raspberry Cake',
                })
                .filter({
                    has: page.getByRole(
                        'button',
                        {
                            name: 'Edit',
                        },
                    ),
                })

        await expect(entryCard).toBeVisible()

        await entryCard
            .getByRole(
                'button',
                {
                    name: 'Edit',
                    exact: true,
                },
            )
            .click()

        const editDialog =
            page.getByRole(
                'dialog',
                {
                    name: 'Edit entry',
                },
            )

        await editDialog
            .getByLabel(
                'Entry name',
                { exact: true },
            )
            .fill(
                'E2E Raspberry Cake Deluxe',
            )

        await editDialog
            .locator('input[type="file"]')
            .setInputFiles({
                name: 'cake.png',
                mimeType: 'image/png',
                buffer: Buffer.from(
                    PNG_BASE64,
                    'base64',
                ),
            })

        await expect(
            editDialog.getByAltText(
                'Selected preview',
            ),
        ).toBeVisible()

        await editDialog
            .getByRole(
                'button',
                {
                    name: 'Save entry',
                    exact: true,
                },
            )
            .click()

        const editedEntryCard =
            page
                .getByRole('article')
                .filter({
                    hasText:
                        'E2E Raspberry Cake Deluxe',
                })
                .filter({
                    has: page.getByRole(
                        'button',
                        {
                            name: 'Edit',
                        },
                    ),
                })

        await expect(
            editedEntryCard,
        ).toBeVisible()

        await expect(
            editedEntryCard.getByAltText(
                'E2E Raspberry Cake Deluxe',
            ),
        ).toBeVisible()

        await page
            .getByRole(
                'button',
                {
                    name: /Add entry/,
                },
            )
            .first()
            .click()

        const secondEntryDialog =
            page.getByRole(
                'dialog',
                {
                    name: 'Add entry',
                },
            )

        await secondEntryDialog
            .getByLabel(
                'Entry name',
            )
            .fill('E2E Chocolate Cake')

        await secondEntryDialog
            .getByLabel(
                'Description',
            )
            .fill(
                'Chocolate opposition candidate.',
            )

        await secondEntryDialog
            .getByRole(
                'button',
                {
                    name: 'Add entry',
                },
            )
            .click()

        await expect(
            page
                .getByRole('article')
                .filter({
                    hasText:
                        'E2E Chocolate Cake',
                })
                .filter({
                    has: page.getByRole(
                        'button',
                        {
                            name: 'Edit',
                        },
                    ),
                }),
        ).toBeVisible()

        await page
            .getByRole(
                'button',
                {
                    name: 'Open voting',
                },
            )
            .click()

        await expect(
            page.getByRole(
                'button',
                {
                    name: 'Close voting',
                },
            ),
        ).toBeVisible()

        const publicResultsSave =
            page.waitForResponse(
                (response) =>
                    response
                        .url()
                        .endsWith(
                            '/api/admin/events/1',
                        ) &&
                    response
                        .request()
                        .method() === 'PATCH',
            )

        await page
            .getByLabel(
                'Public results',
            )
            .check()

        await page
            .getByRole(
                'button',
                {
                    name: 'Save changes',
                },
            )
            .click()

        expect(
            (await publicResultsSave).ok(),
        ).toBeTruthy()

        await expect(
            page
                .getByRole(
                    'button',
                    {
                        name: /Add entry/,
                    },
                )
                .first(),
        ).toBeDisabled()

        await expect(
            editedEntryCard.getByRole(
                'button',
                {
                    name: 'Edit',
                    exact: true,
                },
            ),
        ).toBeDisabled()

        await page.reload()

        await expect(
            page.getByRole(
                'button',
                {
                    name: 'Close voting',
                },
            ),
        ).toBeVisible()

        await expect(
            page.getByAltText(
                'E2E Raspberry Cake Deluxe',
            ),
        ).toBeVisible()

        await page
            .getByRole(
                'link',
                {
                    name: /Back to events/,
                },
            )
            .click()

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'Admin console',
                },
            ),
        ).toBeVisible()

        await page
            .getByRole(
                'button',
                {
                    name: 'Sign out',
                },
            )
            .click()

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'Welcome back',
                },
            ),
        ).toBeVisible()

        await page.evaluate(() => {
            window.localStorage.setItem(
                'crumbvote_locale',
                'en',
            )
        })

        await page.goto(
            '/e/e2e-cake-show/1',
        )

        await expect(
            page.getByTestId(
                'language-selector',
            ),
        ).toHaveValue('en')

        await expect(
            page.getByRole(
                'heading',
                {
                    name:
                        'E2E Raspberry Cake Deluxe',
                },
            ),
        ).toBeVisible()

        await expect(
            page.getByAltText(
                'E2E Raspberry Cake Deluxe',
            ),
        ).toBeVisible()

        await page
            .getByRole(
                'button',
                {
                    name: /Vote for #1/,
                },
            )
            .click()

        await expect(
            page.getByRole(
                'button',
                {
                    name: /Your vote/,
                },
            ),
        ).toBeDisabled()

        await page.reload()

        await expect(
            page.getByRole(
                'button',
                {
                    name: /Your vote/,
                },
            ),
        ).toBeDisabled()

        await page.goto(
            '/e/e2e-cake-show/2',
        )

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'E2E Chocolate Cake',
                },
            ),
        ).toBeVisible()

        await page
            .getByRole(
                'button',
                {
                    name: /Change vote to #2/,
                },
            )
            .click()

        await expect(
            page.getByRole(
                'button',
                {
                    name: /Your vote/,
                },
            ),
        ).toBeDisabled()

        await page.evaluate(() => {
            window.localStorage.removeItem(
                'crumbvote_voter_token',
            )
        })

        await page.reload()

        await expect(
            page.getByRole(
                'button',
                {
                    name: /Your vote/,
                },
            ),
        ).toBeDisabled()

        const firstScan =
            page.waitForResponse(
                (response) =>
                    response
                        .url()
                        .endsWith(
                            '/api/public/events/e2e-cake-show/entries/1/scan',
                        ) &&
                    response
                        .request()
                        .method() === 'POST',
            )

        await page.goto(
            '/e/e2e-cake-show/1',
        )

        expect(
            (await firstScan).ok(),
        ).toBeTruthy()

        await expect(
            page.getByRole(
                'button',
                {
                    name: /Change vote to #1/,
                },
            ),
        ).toBeVisible()

        await page
            .getByRole(
                'link',
                {
                    name: 'View public results',
                },
            )
            .click()

        await expect(page).toHaveURL(
            '/e/e2e-cake-show/results',
        )

        const publicResults =
            page.getByTestId(
                /public-result-entry-/,
            )

        await expect(
            publicResults.first(),
        ).toHaveAttribute(
            'data-testid',
            'public-result-entry-2',
        )

        await expect(
            page.getByTestId(
                'public-result-entry-2',
            ),
        ).toContainText(
            /1\s*Votes/,
        )

        await expect(
            page.getByTestId(
                'public-result-entry-2',
            ),
        ).toContainText(
            /100%\s*Vote share/,
        )

        await expect(
            page.getByTestId(
                'public-result-entry-1',
            ),
        ).toContainText(
            /0\s*Votes/,
        )

        await page.goto('/admin')

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'Welcome back',
                },
            ),
        ).toBeVisible()

        await page
            .getByLabel(
                'Administrator password',
            )
            .fill(PASSWORD)

        await page
            .getByRole(
                'button',
                {
                    name: 'Sign in',
                },
            )
            .click()

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'Admin console',
                },
            ),
        ).toBeVisible()

        const analyticsEventCard =
            page
                .getByRole('article')
                .filter({
                    hasText: 'E2E Cake Show',
                })

        await analyticsEventCard
            .getByRole(
                'link',
                {
                    name: /Manage event/,
                },
            )
            .click()

        const analyticsSummary =
            page.getByTestId(
                'analytics-summary',
            )

        const scansCard =
            analyticsSummary
                .getByRole('article')
                .filter({
                    hasText: 'Scans / opens',
                })

        await expect(
            scansCard,
        ).toContainText('5')

        const visitorsCard =
            analyticsSummary
                .getByRole('article')
                .filter({
                    hasText: 'Unique visitors',
                })

        await expect(
            visitorsCard,
        ).toContainText('1')

        const votesCard =
            analyticsSummary
                .getByRole('article')
                .filter({
                    hasText: 'Current votes',
                })

        await expect(
            votesCard,
        ).toContainText('1')

        const conversionCard =
            analyticsSummary
                .getByRole('article')
                .filter({
                    hasText: 'Conversion',
                })

        await expect(
            conversionCard,
        ).toContainText('100%')

        const changesCard =
            analyticsSummary
                .getByRole('article')
                .filter({
                    hasText: 'Vote changes',
                })

        await expect(
            changesCard,
        ).toContainText('1')

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'Analyse',
                },
            ),
        ).toBeVisible()

        const rankedEntries =
            page.getByTestId(
                /analytics-entry-/,
            )

        await expect(
            rankedEntries.first(),
        ).toHaveAttribute(
            'data-testid',
            'analytics-entry-2',
        )

        const chocolateAnalytics =
            page.getByTestId(
                'analytics-entry-2',
            )

        await expect(
            chocolateAnalytics,
        ).toContainText(
            /1\s*Current votes/,
        )

        await expect(
            chocolateAnalytics,
        ).toContainText(
            /100%\s*Vote share/,
        )

        await expect(
            chocolateAnalytics,
        ).toContainText(
            /2\s*Scans/,
        )

        await expect(
            chocolateAnalytics,
        ).toContainText(
            /100%\s*Conversion/,
        )

        const raspberryAnalytics =
            page.getByTestId(
                'analytics-entry-1',
            )

        await expect(
            raspberryAnalytics,
        ).toContainText(
            /0\s*Current votes/,
        )

        await expect(
            raspberryAnalytics,
        ).toContainText(
            /0%\s*Vote share/,
        )

        await expect(
            raspberryAnalytics,
        ).toContainText(
            /3\s*Scans/,
        )

        await expect(
            raspberryAnalytics,
        ).toContainText(
            /0%\s*Conversion/,
        )

        await expect(
            page.getByText(
                'No unusual activity signals.',
            ),
        ).toBeVisible()

        await expect(
            page.getByRole(
                'heading',
                {
                    name: 'Recent activity',
                },
            ),
        ).toBeVisible()

        await expect(
            page.getByText(
                'Vote changed',
            ),
        ).toBeVisible()

        const downloadPromise =
            page.waitForEvent(
                'download',
            )

        await page
            .getByRole(
                'button',
                {
                    name: 'Export CSV',
                },
            )
            .click()

        const download =
            await downloadPromise

        expect(
            download.suggestedFilename(),
        ).toBe(
            'crumbvote-e2e-cake-show-analytics.csv',
        )

        const stream =
            await download.createReadStream()

        let csv = ''

        for await (const chunk of stream) {
            csv += chunk.toString()
        }

        expect(csv).toContain(
            '"1","2","E2E Chocolate Cake","1","100%","2","1","100%"',
        )

        expect(csv).toContain(
            '"2","1","E2E Raspberry Cake Deluxe","0","0%","3","1","0%"',
        )
    },
)