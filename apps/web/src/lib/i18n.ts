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

    'common.backToCrumbVote':
        'Vissza a CrumbVote-hoz',

    'common.noImage':
        'Nincs kép',

    'common.requestFailed':
        'A kérés sikertelen: "{code}".',

    'entry.pageTitle':
        'Szavazás',

    'entry.loading':
        'Nevezés betöltése…',

    'entry.notFoundTitle':
        'A nevezés nem található',

    'entry.errorNotFound':
        'Ez a szavazási nevezés nem található.',

    'entry.errorDatabase':
        'A CrumbVote nem tudta betölteni ezt a nevezést.',

    'entry.errorBrowser':
        'A CrumbVote nem tudta azonosítani ezt a böngészőt.',

    'entry.errorVoterCreation':
        'A CrumbVote nem tudott szavazói azonosítót létrehozni.',

    'entry.errorVotingNotOpen':
        'A szavazás jelenleg nincs nyitva.',

    'entry.statusOpen':
        'A szavazás nyitva',

    'entry.statusClosed':
        'A szavazás véget ért',

    'entry.statusDraft':
        'A szavazás még nem kezdődött el',

    'entry.number':
        'Nevezés #{number}',

    'entry.noDescription':
        'Ehhez a nevezéshez nem adtak meg leírást.',

    'entry.currentVoteTitle':
        'Ez a jelenlegi szavazatod',

    'entry.currentVoteDescription':
        'Amíg a szavazás nyitva van, egy másik nevezésnél átteheted oda a szavazatodat.',

    'entry.yourVote':
        'A szavazatod ✓',

    'entry.readyTitle':
        'Készen állsz a szavazásra?',

    'entry.changeTitle':
        'Megváltoztatod a szavazatod?',

    'entry.readyDescription':
        'Válaszd ezt a nevezést az eseményre leadott szavazatodként.',

    'entry.changeDescription':
        'Már szavaztál egy másik nevezésre. Ha ezt választod, a szavazatod ide kerül.',

    'entry.savingVote':
        'Szavazat mentése…',

    'entry.voteFor':
        'Szavazok erre: #{number}',

    'entry.changeVoteTo':
        'Szavazat módosítása erre: #{number}',

    'entry.voteRecorded':
        'Szavazat rögzítve.',

    'entry.voteChanged':
        'Szavazat módosítva.',

    'entry.closedTitle':
        'A szavazás lezárult',

    'entry.closedDescription':
        'Ez az esemény már nem fogad szavazatokat.',

    'entry.notStartedTitle':
        'A szavazás még nem kezdődött el',

    'entry.notStartedDescription':
        'Gyere vissza, amikor a szervező megnyitja a szavazást.',

    'entry.aboutEvent':
        'Az eseményről',

    'entry.viewResults':
        'Nyilvános eredmények megtekintése',

    'results.pageTitle':
        'Eredmények',

    'results.loading':
        'Eredmények betöltése…',

    'results.unavailableTitle':
        'Az eredmények nem érhetők el',

    'results.errorUnavailable':
        'A szervező még nem tette nyilvánossá az eredményeket.',

    'results.errorNotFound':
        'Ez az esemény nem található.',

    'results.errorDatabase':
        'A CrumbVote nem tudta betölteni az eredményeket.',

    'results.final':
        'Végeredmény',

    'results.live':
        'Élő eredmények',

    'results.currentVoteOne':
        '{count} aktuális szavazat',

    'results.currentVoteMany':
        '{count} aktuális szavazat',

    'results.openNotice':
        'A szavazás még nyitva van. Az eredmények változhatnak.',

    'results.noEntries':
        'Még nincsenek nevezések.',

    'results.entryNumber':
        'Nevezés #{number}',

    'results.votes':
        'Szavazatok',

    'results.voteShare':
        'Szavazatarány',
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

    'common.backToCrumbVote':
        'Back to CrumbVote',

    'common.noImage':
        'No image',

    'common.requestFailed':
        'The request failed with error "{code}".',

    'entry.pageTitle':
        'Vote',

    'entry.loading':
        'Loading entry…',

    'entry.notFoundTitle':
        'Entry not found',

    'entry.errorNotFound':
        'This voting entry could not be found.',

    'entry.errorDatabase':
        'CrumbVote could not load this entry.',

    'entry.errorBrowser':
        'CrumbVote could not identify this browser.',

    'entry.errorVoterCreation':
        'CrumbVote could not create a voter identity.',

    'entry.errorVotingNotOpen':
        'Voting is not currently open.',

    'entry.statusOpen':
        'Voting is open',

    'entry.statusClosed':
        'Voting has ended',

    'entry.statusDraft':
        'Voting has not started',

    'entry.number':
        'Entry #{number}',

    'entry.noDescription':
        'No description was provided for this entry.',

    'entry.currentVoteTitle':
        'This is your current vote',

    'entry.currentVoteDescription':
        'You can visit another entry and move your vote there while voting remains open.',

    'entry.yourVote':
        'Your vote ✓',

    'entry.readyTitle':
        'Ready to vote?',

    'entry.changeTitle':
        'Change your vote?',

    'entry.readyDescription':
        'Choose this entry as your vote for the event.',

    'entry.changeDescription':
        'You already voted for another entry. Choosing this one moves your vote here.',

    'entry.savingVote':
        'Saving vote…',

    'entry.voteFor':
        'Vote for #{number}',

    'entry.changeVoteTo':
        'Change vote to #{number}',

    'entry.voteRecorded':
        'Vote recorded.',

    'entry.voteChanged':
        'Vote changed.',

    'entry.closedTitle':
        'Voting is closed',

    'entry.closedDescription':
        'This event is no longer accepting votes.',

    'entry.notStartedTitle':
        "Voting hasn't started yet",

    'entry.notStartedDescription':
        'Come back when the organizer opens the event.',

    'entry.aboutEvent':
        'About the event',

    'entry.viewResults':
        'View public results',

    'results.pageTitle':
        'Results',

    'results.loading':
        'Loading results…',

    'results.unavailableTitle':
        'Results unavailable',

    'results.errorUnavailable':
        'The organizer has not made results public.',

    'results.errorNotFound':
        'This event could not be found.',

    'results.errorDatabase':
        'CrumbVote could not load these results.',

    'results.final':
        'Final results',

    'results.live':
        'Live results',

    'results.currentVoteOne':
        '{count} current vote',

    'results.currentVoteMany':
        '{count} current votes',

    'results.openNotice':
        'Voting is still open. These results may change.',

    'results.noEntries':
        'No entries yet.',

    'results.entryNumber':
        'Entry #{number}',

    'results.votes':
        'Votes',

    'results.voteShare':
        'Vote share',
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

    'common.backToCrumbVote':
        'Zurück zu CrumbVote',

    'common.noImage':
        'Kein Bild',

    'common.requestFailed':
        'Die Anfrage ist mit dem Fehler "{code}" fehlgeschlagen.',

    'entry.pageTitle':
        'Abstimmen',

    'entry.loading':
        'Beitrag wird geladen…',

    'entry.notFoundTitle':
        'Beitrag nicht gefunden',

    'entry.errorNotFound':
        'Dieser Abstimmungsbeitrag wurde nicht gefunden.',

    'entry.errorDatabase':
        'CrumbVote konnte diesen Beitrag nicht laden.',

    'entry.errorBrowser':
        'CrumbVote konnte diesen Browser nicht identifizieren.',

    'entry.errorVoterCreation':
        'CrumbVote konnte keine Wählerkennung erstellen.',

    'entry.errorVotingNotOpen':
        'Die Abstimmung ist derzeit nicht geöffnet.',

    'entry.statusOpen':
        'Abstimmung ist geöffnet',

    'entry.statusClosed':
        'Abstimmung ist beendet',

    'entry.statusDraft':
        'Abstimmung hat noch nicht begonnen',

    'entry.number':
        'Beitrag #{number}',

    'entry.noDescription':
        'Für diesen Beitrag wurde keine Beschreibung angegeben.',

    'entry.currentVoteTitle':
        'Das ist deine aktuelle Stimme',

    'entry.currentVoteDescription':
        'Solange die Abstimmung geöffnet ist, kannst du einen anderen Beitrag besuchen und deine Stimme dorthin verschieben.',

    'entry.yourVote':
        'Deine Stimme ✓',

    'entry.readyTitle':
        'Bereit zum Abstimmen?',

    'entry.changeTitle':
        'Stimme ändern?',

    'entry.readyDescription':
        'Wähle diesen Beitrag als deine Stimme für die Veranstaltung.',

    'entry.changeDescription':
        'Du hast bereits für einen anderen Beitrag gestimmt. Mit dieser Auswahl wird deine Stimme hierher verschoben.',

    'entry.savingVote':
        'Stimme wird gespeichert…',

    'entry.voteFor':
        'Für #{number} abstimmen',

    'entry.changeVoteTo':
        'Stimme zu #{number} ändern',

    'entry.voteRecorded':
        'Stimme gespeichert.',

    'entry.voteChanged':
        'Stimme geändert.',

    'entry.closedTitle':
        'Abstimmung ist geschlossen',

    'entry.closedDescription':
        'Diese Veranstaltung nimmt keine Stimmen mehr an.',

    'entry.notStartedTitle':
        'Die Abstimmung hat noch nicht begonnen',

    'entry.notStartedDescription':
        'Komm zurück, sobald der Veranstalter die Abstimmung öffnet.',

    'entry.aboutEvent':
        'Über die Veranstaltung',

    'entry.viewResults':
        'Öffentliche Ergebnisse anzeigen',

    'results.pageTitle':
        'Ergebnisse',

    'results.loading':
        'Ergebnisse werden geladen…',

    'results.unavailableTitle':
        'Ergebnisse nicht verfügbar',

    'results.errorUnavailable':
        'Der Veranstalter hat die Ergebnisse noch nicht veröffentlicht.',

    'results.errorNotFound':
        'Diese Veranstaltung wurde nicht gefunden.',

    'results.errorDatabase':
        'CrumbVote konnte diese Ergebnisse nicht laden.',

    'results.final':
        'Endergebnis',

    'results.live':
        'Live-Ergebnisse',

    'results.currentVoteOne':
        '{count} aktuelle Stimme',

    'results.currentVoteMany':
        '{count} aktuelle Stimmen',

    'results.openNotice':
        'Die Abstimmung ist noch geöffnet. Diese Ergebnisse können sich ändern.',

    'results.noEntries':
        'Noch keine Beiträge vorhanden.',

    'results.entryNumber':
        'Beitrag #{number}',

    'results.votes':
        'Stimmen',

    'results.voteShare':
        'Stimmenanteil',
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