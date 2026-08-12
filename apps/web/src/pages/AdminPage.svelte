<script lang="ts">
    import { onMount } from "svelte";
    import LanguageSelector from "../components/LanguageSelector.svelte";
    import EventWorkspace from "./EventWorkspace.svelte";
    import {
        ApiError,
        createAdminEvent,
        getAdminSession,
        getAdminStatus,
        listAdminEvents,
        loginAdmin,
        logoutAdmin,
        setupAdmin,
        type CrumbEvent,
    } from "../lib/api";
    import { locale, translate, type TranslationKey } from "../lib/i18n";

    export let initialEventId: number | null = null;

    type View = "loading" | "setup" | "login" | "dashboard" | "event" | "error";

    let view: View = "loading";

    let setupCode = "";
    let password = "";
    let confirmPassword = "";

    let showPassword = false;
    let busy = false;
    let errorMessage = "";

    let events: CrumbEvent[] = [];

    let createEventOpen = false;
    let eventTitle = "";
    let eventSlug = "";
    let eventDescription = "";
    let eventSlugManuallyEdited = false;
    let eventBusy = false;
    let eventErrorMessage = "";

    onMount(() => {
        void bootstrap();
    });

    async function bootstrap() {
        view = "loading";
        errorMessage = "";

        try {
            const status = await getAdminStatus();

            if (status.setup_required) {
                view = "setup";
                return;
            }

            const session = await getAdminSession();

            if (session.authenticated) {
                await showAuthenticatedDestination();
            } else {
                view = "login";
            }
        } catch (error) {
            errorMessage = describeError(error);
            view = "error";
        }
    }

    async function handleSetup(event: SubmitEvent) {
        event.preventDefault();
        errorMessage = "";

        if (password !== confirmPassword) {
            errorMessage = translate($locale, "admin.errorPasswordMismatch");
            return;
        }

        busy = true;

        try {
            await setupAdmin(setupCode.trim(), password);

            await loginAdmin(password);

            clearSensitiveFields();

            await showAuthenticatedDestination();
        } catch (error) {
            errorMessage = describeError(error);
        } finally {
            busy = false;
        }
    }

    async function handleLogin(event: SubmitEvent) {
        event.preventDefault();

        errorMessage = "";
        busy = true;

        try {
            await loginAdmin(password);

            clearSensitiveFields();

            await loadEvents();

            view = "dashboard";
        } catch (error) {
            errorMessage = describeError(error);
        } finally {
            busy = false;
        }
    }

    async function handleLogout() {
        errorMessage = "";
        busy = true;

        try {
            await logoutAdmin();

            clearSensitiveFields();
            closeCreateEvent();
            events = [];

            view = "login";
        } catch (error) {
            errorMessage = describeError(error);
        } finally {
            busy = false;
        }
    }

    async function showAuthenticatedDestination() {
        if (initialEventId !== null) {
            view = "event";
            return;
        }

        await loadEvents();
        view = "dashboard";
    }

    async function loadEvents() {
        events = await listAdminEvents();
    }

    function openCreateEvent() {
        eventTitle = "";
        eventSlug = "";
        eventDescription = "";
        eventSlugManuallyEdited = false;
        eventErrorMessage = "";
        createEventOpen = true;
    }

    function closeCreateEvent() {
        createEventOpen = false;
        eventTitle = "";
        eventSlug = "";
        eventDescription = "";
        eventSlugManuallyEdited = false;
        eventErrorMessage = "";
    }

    function handleEventTitleInput(event: Event) {
        const input = event.currentTarget as HTMLInputElement;

        eventTitle = input.value;

        if (!eventSlugManuallyEdited) {
            eventSlug = slugify(eventTitle);
        }
    }

    function handleEventSlugInput(event: Event) {
        const input = event.currentTarget as HTMLInputElement;

        eventSlug = input.value
            .toLowerCase()
            .replace(/[^a-z0-9-]/g, "")
            .replace(/-{2,}/g, "-")
            .slice(0, 80);

        eventSlugManuallyEdited = eventSlug.length > 0;
    }

    function slugify(value: string): string {
        return value
            .normalize("NFD")
            .replace(/[\u0300-\u036f]/g, "")
            .toLowerCase()
            .replace(/[^a-z0-9]+/g, "-")
            .replace(/-{2,}/g, "-")
            .replace(/^-+|-+$/g, "")
            .slice(0, 80);
    }

    async function handleCreateEvent(event: SubmitEvent) {
        event.preventDefault();

        eventErrorMessage = "";
        eventBusy = true;

        try {
            const created = await createAdminEvent({
                title: eventTitle.trim(),
                slug: eventSlug.trim(),
                description: eventDescription.trim() || null,
            });

            events = [created, ...events];

            closeCreateEvent();
        } catch (error) {
            if (
                error instanceof ApiError &&
                error.code === "authentication_required"
            ) {
                closeCreateEvent();
                clearSensitiveFields();
                events = [];
                view = "login";
                return;
            }

            eventErrorMessage = describeError(error);
        } finally {
            eventBusy = false;
        }
    }

    function handleWorkspaceSessionExpired() {
        clearSensitiveFields();
        events = [];
        view = "login";
    }

    function clearSensitiveFields() {
        setupCode = "";
        password = "";
        confirmPassword = "";
        showPassword = false;
    }

    function describeError(error: unknown): string {
        if (!(error instanceof ApiError)) {
            return translate($locale, "admin.errorNetwork");
        }

        const messages: Record<string, TranslationKey> = {
            invalid_setup_code: "admin.errorInvalidSetupCode",

            password_too_short: "admin.errorPasswordTooShort",

            password_too_long: "admin.errorPasswordTooLong",

            already_configured: "admin.errorAlreadyConfigured",

            setup_required: "admin.errorSetupRequired",

            invalid_credentials: "admin.errorInvalidCredentials",

            database_error: "admin.errorDatabase",

            setup_state_unavailable: "admin.errorSetupStateUnavailable",

            password_hashing_failed: "admin.errorPasswordHashingFailed",

            password_verification_failed:
                "admin.errorPasswordVerificationFailed",

            authentication_required: "admin.errorAuthenticationRequired",

            title_required: "admin.errorTitleRequired",

            title_too_long: "admin.errorTitleTooLong",

            slug_too_short: "admin.errorSlugTooShort",

            slug_too_long: "admin.errorSlugTooLong",

            invalid_slug: "admin.errorInvalidSlug",

            description_too_long: "admin.errorDescriptionTooLong",

            event_slug_taken: "admin.errorEventSlugTaken",
        };

        const key = messages[error.code];

        if (key !== undefined) {
            return translate($locale, key);
        }

        return translate($locale, "common.requestFailed", {
            code: error.code,
        });
    }
</script>

<svelte:head>
    <title>Admin · CrumbVote</title>

    <meta
        name="description"
        content={translate($locale, "admin.metaDescription")}
    />
</svelte:head>

<main class="relative min-h-screen overflow-hidden bg-slate-950 text-white">
    <div
        class="pointer-events-none absolute left-1/2 top-[-22rem] h-[44rem] w-[44rem] -translate-x-1/2 rounded-full bg-violet-600/20 blur-[140px]"
    ></div>

    <div
        class="pointer-events-none absolute bottom-[-18rem] right-[-12rem] h-[34rem] w-[34rem] rounded-full bg-fuchsia-600/10 blur-[130px]"
    ></div>

    <div
        class="relative mx-auto flex min-h-screen max-w-7xl flex-col px-6 py-6 sm:px-10"
    >
        <header class="flex items-center justify-between">
            <a
                href="/"
                class="flex items-center gap-3 font-semibold tracking-tight"
            >
                <span
                    class="flex h-10 w-10 items-center justify-center rounded-2xl bg-gradient-to-br from-violet-500 to-fuchsia-500 text-lg font-bold shadow-lg shadow-violet-950/40"
                >
                    C
                </span>

                <div>
                    <div class="leading-none">CrumbVote</div>
                    <div class="mt-1 text-xs font-normal text-slate-500">
                        {translate($locale, "admin.administration")}
                    </div>
                </div>
            </a>

            <div class="flex items-center gap-2">
                <LanguageSelector />

                <a
                    href="/"
                    class="rounded-xl border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-400 transition hover:border-white/20 hover:bg-white/10 hover:text-white"
                >
                    {translate($locale, "admin.publicSite")}
                </a>
            </div>
        </header>

        {#if view === "dashboard"}
            <section class="flex flex-1 flex-col py-12 lg:py-16">
                <div
                    class="rounded-[2rem] border border-white/10 bg-white/[0.035] p-7 shadow-2xl shadow-black/20 backdrop-blur-xl sm:p-9"
                >
                    <div
                        class="flex flex-col gap-6 lg:flex-row lg:items-center lg:justify-between"
                    >
                        <div>
                            <div
                                class="mb-4 inline-flex items-center gap-2 rounded-full border border-emerald-400/15 bg-emerald-400/10 px-3 py-1.5 text-xs font-medium text-emerald-300"
                            >
                                <span
                                    class="h-1.5 w-1.5 rounded-full bg-emerald-400"
                                ></span>
                                {translate($locale, "admin.authenticated")}
                            </div>

                            <h1
                                class="text-3xl font-semibold tracking-tight sm:text-4xl"
                            >
                                {translate($locale, "admin.console")}
                            </h1>

                            <p class="mt-3 max-w-2xl leading-7 text-slate-400">
                                {translate(
                                    $locale,
                                    "admin.dashboardDescription",
                                )}
                            </p>
                        </div>

                        <div class="flex flex-wrap items-center gap-3">
                            <button
                                type="button"
                                onclick={openCreateEvent}
                                class="rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-4 py-2.5 text-sm font-semibold text-white shadow-lg shadow-violet-950/30 transition hover:brightness-110"
                            >
                                + {translate($locale, "admin.createEvent")}
                            </button>

                            <button
                                type="button"
                                disabled={busy}
                                onclick={handleLogout}
                                class="rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm font-medium text-slate-300 transition hover:border-white/20 hover:bg-white/10 hover:text-white disabled:cursor-not-allowed disabled:opacity-50"
                            >
                                {busy
                                    ? translate($locale, "admin.signingOut")
                                    : translate($locale, "admin.signOut")}
                            </button>
                        </div>
                    </div>
                </div>

                <div class="mt-6 grid gap-4 md:grid-cols-3">
                    <article
                        class="rounded-3xl border border-white/10 bg-white/[0.035] p-6"
                    >
                        <div class="text-sm text-slate-500">
                            {translate($locale, "admin.events")}
                        </div>
                        <div class="mt-3 text-3xl font-semibold">
                            {events.length}
                        </div>

                        <div class="mt-2 text-sm text-slate-400">
                            {translate($locale, "admin.eventsConfigured")}
                        </div>
                    </article>

                    <article
                        class="rounded-3xl border border-white/10 bg-white/[0.035] p-6"
                    >
                        <div class="text-sm text-slate-500">
                            {translate($locale, "admin.votes")}
                        </div>
                        <div class="mt-3 text-3xl font-semibold">—</div>
                        <div class="mt-2 text-sm text-slate-400">
                            {translate($locale, "admin.waitingForFirstEvent")}
                        </div>
                    </article>

                    <article
                        class="rounded-3xl border border-white/10 bg-white/[0.035] p-6"
                    >
                        <div class="text-sm text-slate-500">
                            {translate($locale, "admin.security")}
                        </div>

                        <div
                            class="mt-3 flex items-center gap-2 text-lg font-semibold"
                        >
                            <span class="h-2 w-2 rounded-full bg-emerald-400"
                            ></span>
                            {translate($locale, "admin.active")}
                        </div>

                        <div class="mt-2 text-sm text-slate-400">
                            {translate($locale, "admin.securityDescription")}
                        </div>
                    </article>
                </div>

                {#if events.length === 0}
                    <div
                        class="mt-6 flex flex-1 items-center justify-center rounded-[2rem] border border-dashed border-white/10 bg-white/[0.02] px-6 py-16 text-center"
                    >
                        <div class="max-w-md">
                            <div
                                class="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl border border-white/10 bg-white/5 text-xl"
                            >
                                +
                            </div>

                            <h2 class="mt-5 text-xl font-semibold">
                                {translate($locale, "admin.noEventsTitle")}
                            </h2>

                            <p class="mt-2 leading-7 text-slate-500">
                                {translate(
                                    $locale,
                                    "admin.noEventsDescription",
                                )}
                            </p>

                            <button
                                type="button"
                                onclick={openCreateEvent}
                                class="mt-6 rounded-xl bg-white px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-slate-200"
                            >
                                {translate($locale, "admin.createFirstEvent")}
                            </button>
                        </div>
                    </div>
                {:else}
                    <div
                        class="mt-6 rounded-[2rem] border border-white/10 bg-white/[0.025] p-6 sm:p-8"
                    >
                        <div
                            class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between"
                        >
                            <div>
                                <h2 class="text-xl font-semibold">
                                    {translate($locale, "admin.events")}
                                </h2>

                                <p class="mt-1 text-sm text-slate-500">
                                    {translate(
                                        $locale,
                                        "admin.eventsDescription",
                                    )}
                                </p>
                            </div>

                            <button
                                type="button"
                                onclick={openCreateEvent}
                                class="w-fit rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm font-medium text-slate-300 transition hover:border-white/20 hover:bg-white/10 hover:text-white"
                            >
                                + {translate($locale, "admin.newEvent")}
                            </button>
                        </div>

                        <div class="mt-6 grid gap-4 lg:grid-cols-2">
                            {#each events as event (event.id)}
                                <article
                                    class="group rounded-3xl border border-white/10 bg-slate-950/40 p-6 transition hover:border-white/20 hover:bg-slate-950/60"
                                >
                                    <div
                                        class="flex items-start justify-between gap-4"
                                    >
                                        <div class="min-w-0">
                                            <h3
                                                class="truncate text-lg font-semibold text-white"
                                            >
                                                {event.title}
                                            </h3>

                                            <div
                                                class="mt-1 truncate font-mono text-xs text-slate-600"
                                            >
                                                {event.slug}
                                            </div>
                                        </div>

                                        {#if event.status === "open"}
                                            <span
                                                class="shrink-0 rounded-full border border-emerald-400/15 bg-emerald-400/10 px-3 py-1 text-xs font-medium text-emerald-300"
                                            >
                                                {translate(
                                                    $locale,
                                                    "admin.statusOpen",
                                                )}
                                            </span>
                                        {:else if event.status === "closed"}
                                            <span
                                                class="shrink-0 rounded-full border border-slate-400/15 bg-slate-400/10 px-3 py-1 text-xs font-medium text-slate-300"
                                            >
                                                {translate(
                                                    $locale,
                                                    "admin.statusClosed",
                                                )}
                                            </span>
                                        {:else}
                                            <span
                                                class="shrink-0 rounded-full border border-violet-400/15 bg-violet-400/10 px-3 py-1 text-xs font-medium text-violet-300"
                                            >
                                                {translate(
                                                    $locale,
                                                    "admin.statusDraft",
                                                )}
                                            </span>
                                        {/if}
                                    </div>

                                    <p
                                        class="mt-4 min-h-12 text-sm leading-6 text-slate-400"
                                    >
                                        {event.description ??
                                            translate(
                                                $locale,
                                                "admin.noDescription",
                                            )}
                                    </p>

                                    <div
                                        class="mt-5 flex flex-wrap items-center gap-x-4 gap-y-2 border-t border-white/5 pt-4 text-xs text-slate-600"
                                    >
                                        <span>
                                            {translate(
                                                $locale,
                                                "admin.created",
                                            )}
                                            {new Date(
                                                event.created_at * 1000,
                                            ).toLocaleString(
                                                $locale === "hu"
                                                    ? "hu-HU"
                                                    : $locale === "de"
                                                      ? "de-DE"
                                                      : "en-US",
                                            )}
                                        </span>

                                        <span>
                                            {translate(
                                                $locale,
                                                "admin.results",
                                            )}
                                            {event.results_public
                                                ? translate(
                                                      $locale,
                                                      "admin.public",
                                                  )
                                                : translate(
                                                      $locale,
                                                      "admin.private",
                                                  )}
                                        </span>
                                    </div>

                                    <a
                                        href={`/admin/events/${event.id}`}
                                        class="mt-5 block w-full rounded-xl border border-white/10 bg-white/[0.04] px-4 py-2.5 text-center text-sm font-medium text-slate-300 transition hover:border-violet-400/30 hover:bg-violet-400/10 hover:text-white"
                                    >
                                        {translate(
                                            $locale,
                                            "admin.manageEvent",
                                        )}
                                    </a>
                                </article>
                            {/each}
                        </div>
                    </div>
                {/if}
            </section>
        {:else if view === "event" && initialEventId !== null}
            <EventWorkspace
                eventId={initialEventId}
                onSessionExpired={handleWorkspaceSessionExpired}
            />
        {:else}
            <section class="flex flex-1 items-center justify-center py-12">
                <div class="w-full max-w-md">
                    <div
                        class="rounded-[2rem] border border-white/10 bg-white/[0.045] p-7 shadow-2xl shadow-black/30 backdrop-blur-xl sm:p-8"
                    >
                        {#if view === "loading"}
                            <div class="py-14 text-center">
                                <div
                                    class="mx-auto h-9 w-9 animate-spin rounded-full border-2 border-white/10 border-t-violet-400"
                                ></div>

                                <div class="mt-5 font-medium">
                                    {translate($locale, "admin.loading")}
                                </div>

                                <div class="mt-2 text-sm text-slate-500">
                                    {translate(
                                        $locale,
                                        "admin.loadingDescription",
                                    )}
                                </div>
                            </div>
                        {:else if view === "error"}
                            <div class="py-6 text-center">
                                <div
                                    class="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl bg-red-400/10 text-xl text-red-300"
                                >
                                    !
                                </div>

                                <h1 class="mt-5 text-2xl font-semibold">
                                    {translate($locale, "admin.loadErrorTitle")}
                                </h1>

                                <p class="mt-3 leading-7 text-slate-400">
                                    {errorMessage}
                                </p>

                                <button
                                    type="button"
                                    onclick={bootstrap}
                                    class="mt-6 rounded-xl bg-white px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-slate-200"
                                >
                                    {translate($locale, "admin.tryAgain")}
                                </button>
                            </div>
                        {:else if view === "setup"}
                            <div>
                                <div
                                    class="mb-5 inline-flex rounded-full border border-violet-400/20 bg-violet-400/10 px-3 py-1.5 text-xs font-medium text-violet-300"
                                >
                                    {translate($locale, "admin.setupBadge")}
                                </div>

                                <h1
                                    class="text-3xl font-semibold tracking-tight"
                                >
                                    {translate($locale, "admin.setupTitle")}
                                </h1>

                                <p class="mt-3 leading-7 text-slate-400">
                                    {translate(
                                        $locale,
                                        "admin.setupDescription",
                                    )}
                                </p>

                                <form
                                    class="mt-7 space-y-5"
                                    onsubmit={handleSetup}
                                >
                                    <label class="block">
                                        <span
                                            class="text-sm font-medium text-slate-300"
                                        >
                                            {translate(
                                                $locale,
                                                "admin.setupCode",
                                            )}
                                        </span>

                                        <input
                                            bind:value={setupCode}
                                            type="text"
                                            autocomplete="one-time-code"
                                            spellcheck="false"
                                            placeholder="1234-ABCD-5678-EF90"
                                            required
                                            class="mt-2 w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 font-mono tracking-wide text-white outline-none transition placeholder:text-slate-700 focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                                        />
                                    </label>

                                    <label class="block">
                                        <span
                                            class="text-sm font-medium text-slate-300"
                                        >
                                            {translate(
                                                $locale,
                                                "admin.password",
                                            )}
                                        </span>

                                        <div class="relative mt-2">
                                            <input
                                                bind:value={password}
                                                type={showPassword
                                                    ? "text"
                                                    : "password"}
                                                autocomplete="new-password"
                                                minlength="12"
                                                required
                                                class="w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 pr-20 text-white outline-none transition focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                                            />

                                            <button
                                                type="button"
                                                onclick={() =>
                                                    (showPassword =
                                                        !showPassword)}
                                                class="absolute right-3 top-1/2 -translate-y-1/2 rounded-lg px-2.5 py-1.5 text-xs font-medium text-slate-500 transition hover:bg-white/5 hover:text-slate-300"
                                            >
                                                {showPassword
                                                    ? translate(
                                                          $locale,
                                                          "admin.hidePassword",
                                                      )
                                                    : translate(
                                                          $locale,
                                                          "admin.showPassword",
                                                      )}
                                            </button>
                                        </div>

                                        <span
                                            class="mt-2 block text-xs text-slate-600"
                                        >
                                            {translate(
                                                $locale,
                                                "admin.passwordMinimum",
                                            )}
                                        </span>
                                    </label>

                                    <label class="block">
                                        <span
                                            class="text-sm font-medium text-slate-300"
                                        >
                                            {translate(
                                                $locale,
                                                "admin.confirmPassword",
                                            )}
                                        </span>

                                        <input
                                            bind:value={confirmPassword}
                                            type={showPassword
                                                ? "text"
                                                : "password"}
                                            autocomplete="new-password"
                                            minlength="12"
                                            required
                                            class="mt-2 w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                                        />
                                    </label>

                                    {#if errorMessage}
                                        <div
                                            class="rounded-xl border border-red-400/15 bg-red-400/10 px-4 py-3 text-sm leading-6 text-red-200"
                                        >
                                            {errorMessage}
                                        </div>
                                    {/if}

                                    <button
                                        type="submit"
                                        disabled={busy}
                                        class="w-full rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-5 py-3.5 font-semibold text-white shadow-lg shadow-violet-950/30 transition hover:brightness-110 disabled:cursor-not-allowed disabled:opacity-50"
                                    >
                                        {busy
                                            ? translate(
                                                  $locale,
                                                  "admin.configuring",
                                              )
                                            : translate(
                                                  $locale,
                                                  "admin.configure",
                                              )}
                                    </button>
                                </form>
                            </div>
                        {:else}
                            <div>
                                <div
                                    class="mb-5 inline-flex items-center gap-2 rounded-full border border-emerald-400/15 bg-emerald-400/10 px-3 py-1.5 text-xs font-medium text-emerald-300"
                                >
                                    <span
                                        class="h-1.5 w-1.5 rounded-full bg-emerald-400"
                                    ></span>
                                    {translate(
                                        $locale,
                                        "admin.configuredBadge",
                                    )}
                                </div>

                                <h1
                                    class="text-3xl font-semibold tracking-tight"
                                >
                                    {translate($locale, "admin.welcomeBack")}
                                </h1>

                                <p class="mt-3 leading-7 text-slate-400">
                                    {translate(
                                        $locale,
                                        "admin.loginDescription",
                                    )}
                                </p>

                                <form
                                    class="mt-7 space-y-5"
                                    onsubmit={handleLogin}
                                >
                                    <label class="block">
                                        <span
                                            class="text-sm font-medium text-slate-300"
                                        >
                                            {translate(
                                                $locale,
                                                "admin.password",
                                            )}
                                        </span>

                                        <div class="relative mt-2">
                                            <input
                                                bind:value={password}
                                                type={showPassword
                                                    ? "text"
                                                    : "password"}
                                                autocomplete="current-password"
                                                required
                                                class="w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 pr-20 text-white outline-none transition focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                                            />

                                            <button
                                                type="button"
                                                onclick={() =>
                                                    (showPassword =
                                                        !showPassword)}
                                                class="absolute right-3 top-1/2 -translate-y-1/2 rounded-lg px-2.5 py-1.5 text-xs font-medium text-slate-500 transition hover:bg-white/5 hover:text-slate-300"
                                            >
                                                {showPassword
                                                    ? translate(
                                                          $locale,
                                                          "admin.hidePassword",
                                                      )
                                                    : translate(
                                                          $locale,
                                                          "admin.showPassword",
                                                      )}
                                            </button>
                                        </div>
                                    </label>

                                    {#if errorMessage}
                                        <div
                                            class="rounded-xl border border-red-400/15 bg-red-400/10 px-4 py-3 text-sm leading-6 text-red-200"
                                        >
                                            {errorMessage}
                                        </div>
                                    {/if}

                                    <button
                                        type="submit"
                                        disabled={busy}
                                        class="w-full rounded-xl bg-white px-5 py-3.5 font-semibold text-slate-950 transition hover:bg-slate-200 disabled:cursor-not-allowed disabled:opacity-50"
                                    >
                                        {busy
                                            ? translate(
                                                  $locale,
                                                  "admin.signingIn",
                                              )
                                            : translate(
                                                  $locale,
                                                  "admin.signIn",
                                              )}
                                    </button>
                                </form>
                            </div>
                        {/if}
                    </div>

                    <p
                        class="mt-5 text-center text-xs leading-5 text-slate-600"
                    >
                        {translate($locale, "admin.sessionCookie")}
                    </p>
                </div>
            </section>
        {/if}
    </div>

    {#if createEventOpen}
        <div
            class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6"
        >
            <button
                type="button"
                aria-label={translate($locale, "admin.closeCreateEventDialog")}
                disabled={eventBusy}
                onclick={closeCreateEvent}
                class="absolute inset-0 bg-slate-950/80 backdrop-blur-sm"
            ></button>

            <div
                role="dialog"
                aria-modal="true"
                aria-labelledby="create-event-title"
                class="relative z-10 w-full max-w-xl rounded-[2rem] border border-white/10 bg-slate-900 p-6 shadow-2xl shadow-black/50 sm:p-8"
            >
                <div class="flex items-start justify-between gap-4">
                    <div>
                        <div
                            class="mb-3 inline-flex rounded-full border border-violet-400/20 bg-violet-400/10 px-3 py-1 text-xs font-medium text-violet-300"
                        >
                            {translate($locale, "admin.newVotingEvent")}
                        </div>

                        <h2
                            id="create-event-title"
                            class="text-2xl font-semibold tracking-tight"
                        >
                            {translate($locale, "admin.createEvent")}
                        </h2>

                        <p class="mt-2 leading-6 text-slate-400">
                            {translate($locale, "admin.createEventDescription")}
                        </p>
                    </div>

                    <button
                        type="button"
                        aria-label={translate($locale, "admin.close")}
                        disabled={eventBusy}
                        onclick={closeCreateEvent}
                        class="rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-slate-400 transition hover:bg-white/10 hover:text-white disabled:opacity-50"
                    >
                        ✕
                    </button>
                </div>

                <form class="mt-7 space-y-5" onsubmit={handleCreateEvent}>
                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            {translate($locale, "admin.eventTitle")}
                        </span>

                        <input
                            value={eventTitle}
                            oninput={handleEventTitleInput}
                            type="text"
                            maxlength="120"
                            placeholder={translate(
                                $locale,
                                "admin.eventTitlePlaceholder",
                            )}
                            required
                            class="mt-2 w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition placeholder:text-slate-700 focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        />
                    </label>

                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            {translate($locale, "admin.eventUrl")}
                        </span>

                        <div
                            class="mt-2 flex overflow-hidden rounded-xl border border-white/10 bg-slate-950/60 transition focus-within:border-violet-400/50 focus-within:ring-4 focus-within:ring-violet-400/10"
                        >
                            <span
                                class="flex items-center border-r border-white/10 px-3 text-sm text-slate-600"
                            >
                                /e/
                            </span>

                            <input
                                value={eventSlug}
                                oninput={handleEventSlugInput}
                                type="text"
                                maxlength="80"
                                spellcheck="false"
                                placeholder="cake-beauty-2026"
                                required
                                class="min-w-0 flex-1 bg-transparent px-3 py-3.5 font-mono text-sm text-white outline-none placeholder:text-slate-700"
                            />
                        </div>

                        <span class="mt-2 block text-xs text-slate-600">
                            {translate($locale, "admin.eventUrlHelp")}
                        </span>
                    </label>

                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            {translate($locale, "admin.description")}
                        </span>

                        <textarea
                            bind:value={eventDescription}
                            maxlength="2000"
                            rows="4"
                            placeholder={translate(
                                $locale,
                                "admin.descriptionPlaceholder",
                            )}
                            class="mt-2 w-full resize-none rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition placeholder:text-slate-700 focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        ></textarea>

                        <div class="mt-2 text-right text-xs text-slate-600">
                            {eventDescription.length} / 2000
                        </div>
                    </label>

                    {#if eventErrorMessage}
                        <div
                            class="rounded-xl border border-red-400/15 bg-red-400/10 px-4 py-3 text-sm leading-6 text-red-200"
                        >
                            {eventErrorMessage}
                        </div>
                    {/if}

                    <div
                        class="flex flex-col-reverse gap-3 pt-2 sm:flex-row sm:justify-end"
                    >
                        <button
                            type="button"
                            disabled={eventBusy}
                            onclick={closeCreateEvent}
                            class="rounded-xl border border-white/10 bg-white/5 px-5 py-3 text-sm font-medium text-slate-300 transition hover:bg-white/10 hover:text-white disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {translate($locale, "admin.cancel")}
                        </button>

                        <button
                            type="submit"
                            disabled={eventBusy}
                            class="rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-5 py-3 text-sm font-semibold text-white shadow-lg shadow-violet-950/30 transition hover:brightness-110 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {eventBusy
                                ? translate($locale, "admin.creating")
                                : translate($locale, "admin.createEvent")}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    {/if}
</main>
