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

    'landing.linkEyebrow':
        'Van már szavazási linked?',

    'landing.linkTitle':
        'Nyisd meg itt',

    'landing.linkDescription':
        'A QR-kód általában közvetlenül a nevezéshez visz. Ha inkább linket kaptál, másold be ide.',

    'landing.linkLabel':
        'Szavazási vagy eredménylink',

    'landing.linkPlaceholder':
        'https://…/e/torta-2026/12',

    'landing.openLink':
        'Megnyitás',

    'landing.invalidLink':
        'Ez nem érvényes CrumbVote szavazási vagy eredménylink.',

    'landing.organizerPrompt':
        'Te szervezed az eseményt?',

    'landing.organizerDescription':
        'Hozz létre eseményeket, nevezéseket, kezeld a szavazást és nézd meg a statisztikákat.',

    'landing.openAdmin':
        'Adminfelület megnyitása →',

    'landing.howTitle':
        'Így működik',

    'landing.howDescription':
        'Nincs alkalmazás és nincs regisztráció. Egy telefon és a nevezések QR-kódjai elegendők.',

    'landing.stepScanTitle':
        '1. Olvasd be',

    'landing.stepScanDescription':
        'Olvasd be egy nevezés QR-kódját, vagy nyisd meg a kapott linket.',

    'landing.stepReviewTitle':
        '2. Nézd meg',

    'landing.stepReviewDescription':
        'Megjelenik a nevezés képe, száma és a szervező által megadott leírás.',

    'landing.stepVoteTitle':
        '3. Szavazz',

    'landing.stepVoteDescription':
        'Add le a szavazatod. Amíg nyitva van a szavazás, egy másik nevezésnél módosíthatod.',

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

    'admin.metaDescription':
        'CrumbVote adminisztrációs felület.',

    'admin.administration':
        'Adminisztráció',

    'admin.publicSite':
        '← Nyilvános oldal',

    'admin.loading':
        'CrumbVote ellenőrzése…',

    'admin.loadingDescription':
        'A beállítás és az adminisztrátori munkamenet ellenőrzése.',

    'admin.loadErrorTitle':
        'Az adminfelület nem tölthető be',

    'admin.tryAgain':
        'Újra',

    'admin.setupBadge':
        'Első beállítás',

    'admin.setupTitle':
        'CrumbVote beállítása',

    'admin.setupDescription':
        'Add meg a szerver konzoljában megjelenő egyszer használatos beállítási kódot, majd válassz adminisztrátori jelszót.',

    'admin.setupCode':
        'Beállítási kód',

    'admin.password':
        'Adminisztrátori jelszó',

    'admin.showPassword':
        'Mutat',

    'admin.hidePassword':
        'Elrejt',

    'admin.passwordMinimum':
        'Legalább 12 karakter.',

    'admin.confirmPassword':
        'Jelszó megerősítése',

    'admin.configuring':
        'Beállítás…',

    'admin.configure':
        'CrumbVote beállítása',

    'admin.configuredBadge':
        'A CrumbVote be van állítva',

    'admin.welcomeBack':
        'Üdv újra',

    'admin.loginDescription':
        'A folytatáshoz add meg az adminisztrátori jelszót.',

    'admin.signingIn':
        'Bejelentkezés…',

    'admin.signIn':
        'Bejelentkezés',

    'admin.sessionCookie':
        'Az adminisztrátori munkamenet HttpOnly böngészősütiben tárolódik.',

    'admin.errorPasswordMismatch':
        'A két jelszó nem egyezik.',

    'admin.errorNetwork':
        'A CrumbVote nem éri el a szervert. Ellenőrizd, hogy a backend fut-e.',

    'admin.errorInvalidSetupCode':
        'A beállítási kód nem egyezik a CrumbVote szervere által kiírt kóddal.',

    'admin.errorPasswordTooShort':
        'Válassz legalább 12 karakteres jelszót.',

    'admin.errorPasswordTooLong':
        'A jelszó túl hosszú.',

    'admin.errorAlreadyConfigured':
        'A CrumbVote már be van állítva.',

    'admin.errorSetupRequired':
        'A CrumbVote-ot be kell állítani a bejelentkezés előtt.',

    'admin.errorInvalidCredentials':
        'A jelszó nem megfelelő.',

    'admin.errorDatabase':
        'A CrumbVote nem éri el az adatbázist.',

    'admin.errorSetupStateUnavailable':
        'Az első beállítás állapota nem érhető el. Indítsd újra a CrumbVote-ot, majd próbáld újra.',

    'admin.errorPasswordHashingFailed':
        'A CrumbVote nem tudta biztonságosan eltárolni a jelszót.',

    'admin.errorPasswordVerificationFailed':
        'A CrumbVote nem tudta ellenőrizni a jelszót.',

    'admin.errorAuthenticationRequired':
        'Az adminisztrátori munkamenet lejárt. Jelentkezz be újra.',

    'admin.errorTitleRequired':
        'Adj címet az eseménynek.',

    'admin.errorTitleTooLong':
        'Az esemény címe túl hosszú.',

    'admin.errorSlugTooShort':
        'Az esemény URL-azonosítója legalább 3 karakter legyen.',

    'admin.errorSlugTooLong':
        'Az esemény URL-azonosítója túl hosszú.',

    'admin.errorInvalidSlug':
        'Az esemény URL-je csak kisbetűket, számokat és egyszeres kötőjeleket tartalmazhat.',

    'admin.errorDescriptionTooLong':
        'Az esemény leírása túl hosszú.',

    'admin.errorEventSlugTaken':
        'Ez az esemény-URL már használatban van. Válassz másikat.',

    'admin.authenticated':
        'Bejelentkezve',

    'admin.console':
        'Adminfelület',

    'admin.dashboardDescription':
        'Hozz létre és kezelj szavazási eseményeket, majd add hozzá a nevezéseket, amelyekre a látogatók szavazhatnak.',

    'admin.createEvent':
        'Esemény létrehozása',

    'admin.signingOut':
        'Kijelentkezés…',

    'admin.signOut':
        'Kijelentkezés',

    'admin.events':
        'Események',

    'admin.eventsConfigured':
        'A CrumbVote-ban beállított szavazási események.',

    'admin.votes':
        'Szavazatok',

    'admin.waitingForFirstEvent':
        'Az első eseményre vár.',

    'admin.security':
        'Adminbiztonság',

    'admin.active':
        'Aktív',

    'admin.securityDescription':
        'A munkamenet-alapú adminisztrátori hozzáférés működik.',

    'admin.noEventsTitle':
        'Még nincsenek események',

    'admin.noEventsDescription':
        'Hozd létre az első szavazási eseményedet. A nevezések, szavazási linkek és eredmények ezen belül lesznek.',

    'admin.createFirstEvent':
        'Első esemény létrehozása',

    'admin.eventsDescription':
        'CrumbVote szavazási eseményeid.',

    'admin.newEvent':
        'Új esemény',

    'admin.statusOpen':
        'Nyitva',

    'admin.statusClosed':
        'Lezárva',

    'admin.statusDraft':
        'Piszkozat',

    'admin.noDescription':
        'Még nincs leírás.',

    'admin.created':
        'Létrehozva',

    'admin.results':
        'Eredmények:',

    'admin.public':
        'nyilvános',

    'admin.private':
        'privát',

    'admin.manageEvent':
        'Esemény kezelése →',

    'admin.newVotingEvent':
        'Új szavazási esemény',

    'admin.createEventDescription':
        'Kezdd magával az eseménnyel. A nevezéseket és a szavazási beállításokat ezután adjuk hozzá.',

    'admin.closeCreateEventDialog':
        'Esemény létrehozása ablak bezárása',

    'admin.close':
        'Bezárás',

    'admin.eventTitle':
        'Esemény címe',

    'admin.eventTitlePlaceholder':
        'Tortaszépség 2026',

    'admin.eventUrl':
        'Esemény URL-je',

    'admin.eventUrlHelp':
        'Automatikusan létrejön a címből, de módosíthatod.',

    'admin.description':
        'Leírás',

    'admin.descriptionPlaceholder':
        'Írd le a látogatóknak, miről szól az esemény…',

    'admin.cancel':
        'Mégse',

    'admin.creating':
        'Létrehozás…',
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

    'landing.linkEyebrow':
        'Already have a voting link?',

    'landing.linkTitle':
        'Open it here',

    'landing.linkDescription':
        'A QR code normally takes you straight to an entry. If you received a link instead, paste it here.',

    'landing.linkLabel':
        'Voting or results link',

    'landing.linkPlaceholder':
        'https://…/e/cake-show-2026/12',

    'landing.openLink':
        'Open',

    'landing.invalidLink':
        'That is not a valid CrumbVote voting or results link.',

    'landing.organizerPrompt':
        'Organizing the event?',

    'landing.organizerDescription':
        'Create events and entries, control voting and inspect your analytics.',

    'landing.openAdmin':
        'Open admin console →',

    'landing.howTitle':
        'How it works',

    'landing.howDescription':
        'No app and no registration. A phone and the entries’ QR codes are enough.',

    'landing.stepScanTitle':
        '1. Scan',

    'landing.stepScanDescription':
        'Scan an entry QR code or open the voting link you received.',

    'landing.stepReviewTitle':
        '2. Review',

    'landing.stepReviewDescription':
        'See the entry image, number and description provided by the organizer.',

    'landing.stepVoteTitle':
        '3. Vote',

    'landing.stepVoteDescription':
        'Cast your vote. While voting is open, visiting another entry lets you change it.',

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

    'admin.metaDescription':
        'CrumbVote administration console.',

    'admin.administration':
        'Administration',

    'admin.publicSite':
        '← Public site',

    'admin.loading':
        'Checking CrumbVote…',

    'admin.loadingDescription':
        'Verifying setup and administrator session.',

    'admin.loadErrorTitle':
        "Couldn't load the admin console",

    'admin.tryAgain':
        'Try again',

    'admin.setupBadge':
        'First-time setup',

    'admin.setupTitle':
        'Claim this CrumbVote',

    'admin.setupDescription':
        'Enter the one-time setup code printed in the server console, then choose the administrator password.',

    'admin.setupCode':
        'Setup code',

    'admin.password':
        'Administrator password',

    'admin.showPassword':
        'Show',

    'admin.hidePassword':
        'Hide',

    'admin.passwordMinimum':
        'Minimum 12 characters.',

    'admin.confirmPassword':
        'Confirm password',

    'admin.configuring':
        'Configuring…',

    'admin.configure':
        'Configure CrumbVote',

    'admin.configuredBadge':
        'CrumbVote is configured',

    'admin.welcomeBack':
        'Welcome back',

    'admin.loginDescription':
        'Enter the administrator password to continue.',

    'admin.signingIn':
        'Signing in…',

    'admin.signIn':
        'Sign in',

    'admin.sessionCookie':
        'Administrator sessions are stored in an HttpOnly browser cookie.',

    'admin.errorPasswordMismatch':
        'The two passwords do not match.',

    'admin.errorNetwork':
        'CrumbVote could not reach the server. Check that the backend is running.',

    'admin.errorInvalidSetupCode':
        'That setup code does not match the code printed by the CrumbVote server.',

    'admin.errorPasswordTooShort':
        'Choose a password with at least 12 characters.',

    'admin.errorPasswordTooLong':
        'That password is too long.',

    'admin.errorAlreadyConfigured':
        'CrumbVote has already been configured.',

    'admin.errorSetupRequired':
        'CrumbVote still needs to be configured before you can sign in.',

    'admin.errorInvalidCredentials':
        'That password is not correct.',

    'admin.errorDatabase':
        'CrumbVote could not access its database.',

    'admin.errorSetupStateUnavailable':
        'The first-run setup state is unavailable. Restart CrumbVote and try again.',

    'admin.errorPasswordHashingFailed':
        'CrumbVote could not securely store that password.',

    'admin.errorPasswordVerificationFailed':
        'CrumbVote could not verify the password.',

    'admin.errorAuthenticationRequired':
        'Your administrator session has expired. Sign in again.',

    'admin.errorTitleRequired':
        'Give the event a title.',

    'admin.errorTitleTooLong':
        'The event title is too long.',

    'admin.errorSlugTooShort':
        'The event URL slug must contain at least 3 characters.',

    'admin.errorSlugTooLong':
        'The event URL slug is too long.',

    'admin.errorInvalidSlug':
        'The event URL may only contain lowercase letters, numbers and single hyphens.',

    'admin.errorDescriptionTooLong':
        'The event description is too long.',

    'admin.errorEventSlugTaken':
        'That event URL is already being used. Choose another slug.',

    'admin.authenticated':
        'Authenticated',

    'admin.console':
        'Admin console',

    'admin.dashboardDescription':
        'Create and manage voting events, then add the entries your visitors will vote for.',

    'admin.createEvent':
        'Create event',

    'admin.signingOut':
        'Signing out…',

    'admin.signOut':
        'Sign out',

    'admin.events':
        'Events',

    'admin.eventsConfigured':
        'Voting events configured in CrumbVote.',

    'admin.votes':
        'Votes',

    'admin.waitingForFirstEvent':
        'Waiting for the first event.',

    'admin.security':
        'Admin security',

    'admin.active':
        'Active',

    'admin.securityDescription':
        'Session-backed administrator access is online.',

    'admin.noEventsTitle':
        'No events yet',

    'admin.noEventsDescription':
        'Create your first voting event. Entries, voting links and results will live inside it.',

    'admin.createFirstEvent':
        'Create first event',

    'admin.eventsDescription':
        'Your CrumbVote voting events.',

    'admin.newEvent':
        'New event',

    'admin.statusOpen':
        'Open',

    'admin.statusClosed':
        'Closed',

    'admin.statusDraft':
        'Draft',

    'admin.noDescription':
        'No description yet.',

    'admin.created':
        'Created',

    'admin.results':
        'Results:',

    'admin.public':
        'public',

    'admin.private':
        'private',

    'admin.manageEvent':
        'Manage event →',

    'admin.newVotingEvent':
        'New voting event',

    'admin.createEventDescription':
        "Start with the event itself. We'll add contestants and voting settings next.",

    'admin.closeCreateEventDialog':
        'Close create event dialog',

    'admin.close':
        'Close',

    'admin.eventTitle':
        'Event title',

    'admin.eventTitlePlaceholder':
        'Cake Beauty 2026',

    'admin.eventUrl':
        'Event URL',

    'admin.eventUrlHelp':
        'Generated automatically from the title, but you can edit it.',

    'admin.description':
        'Description',

    'admin.descriptionPlaceholder':
        'Tell visitors what this event is about…',

    'admin.cancel':
        'Cancel',

    'admin.creating':
        'Creating…',
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

    'landing.linkEyebrow':
        'Schon einen Abstimmungslink erhalten?',

    'landing.linkTitle':
        'Hier öffnen',

    'landing.linkDescription':
        'Ein QR-Code führt normalerweise direkt zu einem Beitrag. Wenn du stattdessen einen Link erhalten hast, füge ihn hier ein.',

    'landing.linkLabel':
        'Abstimmungs- oder Ergebnislink',

    'landing.linkPlaceholder':
        'https://…/e/kuchen-2026/12',

    'landing.openLink':
        'Öffnen',

    'landing.invalidLink':
        'Das ist kein gültiger CrumbVote-Abstimmungs- oder Ergebnislink.',

    'landing.organizerPrompt':
        'Du organisierst die Veranstaltung?',

    'landing.organizerDescription':
        'Erstelle Veranstaltungen und Beiträge, steuere die Abstimmung und sieh dir die Statistiken an.',

    'landing.openAdmin':
        'Adminbereich öffnen →',

    'landing.howTitle':
        'So funktioniert es',

    'landing.howDescription':
        'Keine App und keine Registrierung. Ein Smartphone und die QR-Codes der Beiträge reichen aus.',

    'landing.stepScanTitle':
        '1. Scannen',

    'landing.stepScanDescription':
        'Scanne den QR-Code eines Beitrags oder öffne den erhaltenen Abstimmungslink.',

    'landing.stepReviewTitle':
        '2. Ansehen',

    'landing.stepReviewDescription':
        'Sieh dir Bild, Nummer und die Beschreibung des Veranstalters an.',

    'landing.stepVoteTitle':
        '3. Abstimmen',

    'landing.stepVoteDescription':
        'Gib deine Stimme ab. Solange die Abstimmung geöffnet ist, kannst du sie bei einem anderen Beitrag ändern.',

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

    'admin.metaDescription':
        'CrumbVote-Administrationsoberfläche.',

    'admin.administration':
        'Administration',

    'admin.publicSite':
        '← Öffentliche Seite',

    'admin.loading':
        'CrumbVote wird geprüft…',

    'admin.loadingDescription':
        'Einrichtung und Administratorsitzung werden überprüft.',

    'admin.loadErrorTitle':
        'Adminbereich konnte nicht geladen werden',

    'admin.tryAgain':
        'Erneut versuchen',

    'admin.setupBadge':
        'Ersteinrichtung',

    'admin.setupTitle':
        'CrumbVote einrichten',

    'admin.setupDescription':
        'Gib den einmaligen Einrichtungscode aus der Serverkonsole ein und wähle anschließend das Administratorpasswort.',

    'admin.setupCode':
        'Einrichtungscode',

    'admin.password':
        'Administratorpasswort',

    'admin.showPassword':
        'Anzeigen',

    'admin.hidePassword':
        'Ausblenden',

    'admin.passwordMinimum':
        'Mindestens 12 Zeichen.',

    'admin.confirmPassword':
        'Passwort bestätigen',

    'admin.configuring':
        'Wird eingerichtet…',

    'admin.configure':
        'CrumbVote einrichten',

    'admin.configuredBadge':
        'CrumbVote ist eingerichtet',

    'admin.welcomeBack':
        'Willkommen zurück',

    'admin.loginDescription':
        'Gib das Administratorpasswort ein, um fortzufahren.',

    'admin.signingIn':
        'Anmeldung…',

    'admin.signIn':
        'Anmelden',

    'admin.sessionCookie':
        'Administratorsitzungen werden in einem HttpOnly-Browser-Cookie gespeichert.',

    'admin.errorPasswordMismatch':
        'Die beiden Passwörter stimmen nicht überein.',

    'admin.errorNetwork':
        'CrumbVote konnte den Server nicht erreichen. Prüfe, ob das Backend läuft.',

    'admin.errorInvalidSetupCode':
        'Der Einrichtungscode stimmt nicht mit dem vom CrumbVote-Server ausgegebenen Code überein.',

    'admin.errorPasswordTooShort':
        'Wähle ein Passwort mit mindestens 12 Zeichen.',

    'admin.errorPasswordTooLong':
        'Das Passwort ist zu lang.',

    'admin.errorAlreadyConfigured':
        'CrumbVote wurde bereits eingerichtet.',

    'admin.errorSetupRequired':
        'CrumbVote muss eingerichtet werden, bevor du dich anmelden kannst.',

    'admin.errorInvalidCredentials':
        'Das Passwort ist nicht korrekt.',

    'admin.errorDatabase':
        'CrumbVote konnte nicht auf die Datenbank zugreifen.',

    'admin.errorSetupStateUnavailable':
        'Der Status der Ersteinrichtung ist nicht verfügbar. Starte CrumbVote neu und versuche es erneut.',

    'admin.errorPasswordHashingFailed':
        'CrumbVote konnte das Passwort nicht sicher speichern.',

    'admin.errorPasswordVerificationFailed':
        'CrumbVote konnte das Passwort nicht überprüfen.',

    'admin.errorAuthenticationRequired':
        'Die Administratorsitzung ist abgelaufen. Melde dich erneut an.',

    'admin.errorTitleRequired':
        'Gib der Veranstaltung einen Titel.',

    'admin.errorTitleTooLong':
        'Der Veranstaltungstitel ist zu lang.',

    'admin.errorSlugTooShort':
        'Die URL-Kennung der Veranstaltung muss mindestens 3 Zeichen enthalten.',

    'admin.errorSlugTooLong':
        'Die URL-Kennung der Veranstaltung ist zu lang.',

    'admin.errorInvalidSlug':
        'Die Veranstaltungs-URL darf nur Kleinbuchstaben, Zahlen und einzelne Bindestriche enthalten.',

    'admin.errorDescriptionTooLong':
        'Die Veranstaltungsbeschreibung ist zu lang.',

    'admin.errorEventSlugTaken':
        'Diese Veranstaltungs-URL wird bereits verwendet. Wähle eine andere.',

    'admin.authenticated':
        'Angemeldet',

    'admin.console':
        'Adminbereich',

    'admin.dashboardDescription':
        'Erstelle und verwalte Abstimmungen und füge anschließend die Beiträge hinzu, für die Besucher abstimmen können.',

    'admin.createEvent':
        'Veranstaltung erstellen',

    'admin.signingOut':
        'Abmeldung…',

    'admin.signOut':
        'Abmelden',

    'admin.events':
        'Veranstaltungen',

    'admin.eventsConfigured':
        'In CrumbVote eingerichtete Abstimmungen.',

    'admin.votes':
        'Stimmen',

    'admin.waitingForFirstEvent':
        'Wartet auf die erste Veranstaltung.',

    'admin.security':
        'Admin-Sicherheit',

    'admin.active':
        'Aktiv',

    'admin.securityDescription':
        'Der sitzungsbasierte Administratorzugriff ist aktiv.',

    'admin.noEventsTitle':
        'Noch keine Veranstaltungen',

    'admin.noEventsDescription':
        'Erstelle deine erste Abstimmung. Beiträge, Abstimmungslinks und Ergebnisse befinden sich anschließend darin.',

    'admin.createFirstEvent':
        'Erste Veranstaltung erstellen',

    'admin.eventsDescription':
        'Deine CrumbVote-Abstimmungen.',

    'admin.newEvent':
        'Neue Veranstaltung',

    'admin.statusOpen':
        'Geöffnet',

    'admin.statusClosed':
        'Geschlossen',

    'admin.statusDraft':
        'Entwurf',

    'admin.noDescription':
        'Noch keine Beschreibung.',

    'admin.created':
        'Erstellt',

    'admin.results':
        'Ergebnisse:',

    'admin.public':
        'öffentlich',

    'admin.private':
        'privat',

    'admin.manageEvent':
        'Veranstaltung verwalten →',

    'admin.newVotingEvent':
        'Neue Abstimmung',

    'admin.createEventDescription':
        'Beginne mit der Veranstaltung selbst. Beiträge und Abstimmungseinstellungen fügen wir anschließend hinzu.',

    'admin.closeCreateEventDialog':
        'Dialog zum Erstellen einer Veranstaltung schließen',

    'admin.close':
        'Schließen',

    'admin.eventTitle':
        'Veranstaltungstitel',

    'admin.eventTitlePlaceholder':
        'Kuchen-Schönheit 2026',

    'admin.eventUrl':
        'Veranstaltungs-URL',

    'admin.eventUrlHelp':
        'Wird automatisch aus dem Titel erstellt, kann aber geändert werden.',

    'admin.description':
        'Beschreibung',

    'admin.descriptionPlaceholder':
        'Beschreibe für Besucher, worum es bei dieser Veranstaltung geht…',

    'admin.cancel':
        'Abbrechen',

    'admin.creating':
        'Wird erstellt…',
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