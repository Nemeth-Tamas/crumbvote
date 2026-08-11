import {
    expect,
    test,
} from '@playwright/test'

test(
    'public locale defaults to Hungarian and persists',
    async ({ page }) => {
        await page.goto('/')

        await expect(
            page.locator('html'),
        ).toHaveAttribute(
            'lang',
            'hu',
        )

        await expect(
            page.getByRole(
                'heading',
                {
                    name:
                        'Szavazás minden felesleges kör nélkül.',
                },
            ),
        ).toBeVisible()

        const languageSelector =
            page.getByTestId(
                'language-selector',
            )

        await expect(
            languageSelector,
        ).toHaveValue('hu')

        await languageSelector
            .selectOption('en')

        await expect(
            page.locator('html'),
        ).toHaveAttribute(
            'lang',
            'en',
        )

        await expect(
            page.getByRole(
                'heading',
                {
                    name:
                        'Voting without the nonsense.',
                },
            ),
        ).toBeVisible()

        await page.reload()

        await expect(
            page.getByTestId(
                'language-selector',
            ),
        ).toHaveValue('en')

        await expect(
            page.locator('html'),
        ).toHaveAttribute(
            'lang',
            'en',
        )

        await page
            .getByTestId(
                'language-selector',
            )
            .selectOption('de')

        await expect(
            page.locator('html'),
        ).toHaveAttribute(
            'lang',
            'de',
        )

        await expect(
            page.getByRole(
                'heading',
                {
                    name:
                        'Abstimmen ohne unnötigen Schnickschnack.',
                },
            ),
        ).toBeVisible()
    },
)