import { writable } from 'svelte/store'

export const locales = [
    'hu',
    'en',
    'de',
] as const

export type Locale =
    (typeof locales)[number]

export const DEFAULT_LOCALE: Locale =
    'hu'

const STORAGE_KEY =
    'crumbvote_locale'

const hu = {
    'language': 'Nyelv',

    'landing.metaDescription':
        'Egyszerű, szép eseményszavazás.',

    'landing.admin': 'Admin',

    'landing.badge':
        'Egyszerű eseményszavazás',

    'landing.heroPrefix':
        'Szavazás minden',

    'landing.heroAccent':
        'felesleges kör nélkül.',

    'landing.description':
        'Egyszerű eseményszavazás, szép résztvevői oldalak és hasznos statisztikák anélkül, hogy a látogatóknak alkalmazást kellene telepíteniük.',

    'landing.featureVoteTitle':
        'Szavazás',

    'landing.featureVoteDescription':
        'Gyors, mobilbarát szavazás az egyes nevezések saját linkjeiről.',

    'landing.featureManageTitle':
        'Kezelés',

    'landing.featureManageDescription':
        'Események létrehozása és nevezések kezelése az adminfelületről.',

    'landing.featureAnalyseTitle':
        'Elemzés',

    'landing.featureAnalyseDescription':
        'Részvétel, eredmények és felülvizsgálatra érdemes aktivitás követése.',

    'landing.developmentBuild':
        'Fejlesztői verzió',
} as const

export type TranslationKey =
    keyof typeof hu

const en: Record<
    TranslationKey,
    string
> = {
    'language': 'Language',

    'landing.metaDescription':
        'Simple, beautiful event voting.',

    'landing.admin': 'Admin',

    'landing.badge':
        'Simple event voting',

    'landing.heroPrefix':
        'Voting without',

    'landing.heroAccent':
        'the nonsense.',

    'landing.description':
        'Simple event voting, beautiful participant pages and useful analytics without forcing visitors to install an app.',

    'landing.featureVoteTitle':
        'Vote',

    'landing.featureVoteDescription':
        'Fast, mobile-first voting from individual entry links.',

    'landing.featureManageTitle':
        'Manage',

    'landing.featureManageDescription':
        'Create events and manage entries from the admin dashboard.',

    'landing.featureAnalyseTitle':
        'Analyse',

    'landing.featureAnalyseDescription':
        'Watch turnout, results and activity worth reviewing.',

    'landing.developmentBuild':
        'Development build',
}

const de: Record<
    TranslationKey,
    string
> = {
    'language': 'Sprache',

    'landing.metaDescription':
        'Einfache und schöne Abstimmungen für Veranstaltungen.',

    'landing.admin': 'Admin',

    'landing.badge':
        'Einfaches Event-Voting',

    'landing.heroPrefix':
        'Abstimmen ohne',

    'landing.heroAccent':
        'unnötigen Schnickschnack.',

    'landing.description':
        'Einfache Abstimmungen, schöne Teilnehmerseiten und nützliche Statistiken, ohne dass Besucher eine App installieren müssen.',

    'landing.featureVoteTitle':
        'Abstimmen',

    'landing.featureVoteDescription':
        'Schnelle, mobilfreundliche Abstimmung über individuelle Teilnehmerlinks.',

    'landing.featureManageTitle':
        'Verwalten',

    'landing.featureManageDescription':
        'Veranstaltungen erstellen und Teilnehmer über den Adminbereich verwalten.',

    'landing.featureAnalyseTitle':
        'Analysieren',

    'landing.featureAnalyseDescription':
        'Teilnahme, Ergebnisse und auffällige Aktivitäten im Blick behalten.',

    'landing.developmentBuild':
        'Entwicklungsversion',
}

const translations: Record<
    Locale,
    Record<TranslationKey, string>
> = {
    hu,
    en,
    de,
}

function isLocale(
    value: string | null,
): value is Locale {
    return locales.includes(
        value as Locale,
    )
}

function initialLocale(): Locale {
    if (typeof window === 'undefined') {
        return DEFAULT_LOCALE
    }

    try {
        const stored =
            window.localStorage.getItem(
                STORAGE_KEY,
            )

        if (isLocale(stored)) {
            return stored
        }
    } catch {
        // Storage is optional. Hungarian remains
        // the deterministic default.
    }

    return DEFAULT_LOCALE
}

export const locale =
    writable<Locale>(
        initialLocale(),
    )

locale.subscribe((value) => {
    if (typeof window === 'undefined') {
        return
    }

    document.documentElement.lang =
        value

    try {
        window.localStorage.setItem(
            STORAGE_KEY,
            value,
        )
    } catch {
        // A blocked localStorage must not break
        // the voting UI.
    }
})

export function setLocale(
    value: Locale,
) {
    locale.set(value)
}

export function translate(
    selectedLocale: Locale,
    key: TranslationKey,
    replacements: Record<
        string,
        string | number
    > = {},
): string {
    let translated =
        translations[selectedLocale][key]

    for (
        const [name, value]
        of Object.entries(
            replacements,
        )
    ) {
        translated = translated
            .split(`{${name}}`)
            .join(String(value))
    }

    return translated
}