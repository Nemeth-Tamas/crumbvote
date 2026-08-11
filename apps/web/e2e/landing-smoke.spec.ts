import {
    expect,
    test,
} from '@playwright/test'

test(
    'landing page opens CrumbVote public links safely',
    async ({ page }) => {
        await page.goto('/')

        const linkInput =
            page.getByTestId(
                'landing-link-input',
            )

        const openButton =
            page.getByTestId(
                'landing-open-link',
            )

        await linkInput.fill(
            'definitely-not-a-voting-link',
        )

        await openButton.click()

        await expect(
            page.getByTestId(
                'landing-link-error',
            ),
        ).toBeVisible()

        await expect(page).toHaveURL('/')

        await linkInput.fill(
            'https://example.invalid/e/demo-event/42?source=test',
        )

        await openButton.click()

        await expect(page).toHaveURL(
            '/e/demo-event/42',
        )

        await page.goto('/')

        await page
            .getByTestId(
                'landing-link-input',
            )
            .fill(
                'e/demo-event/results',
            )

        await page
            .getByTestId(
                'landing-open-link',
            )
            .click()

        await expect(page).toHaveURL(
            '/e/demo-event/results',
        )
    },
)