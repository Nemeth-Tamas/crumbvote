<script lang="ts">
    import { onMount } from "svelte";
    import {
        ApiError,
        getAdminSession,
        getAdminStatus,
        loginAdmin,
        logoutAdmin,
        setupAdmin,
    } from "../lib/api";

    type View = "loading" | "setup" | "login" | "dashboard" | "error";

    let view: View = "loading";

    let setupCode = "";
    let password = "";
    let confirmPassword = "";

    let showPassword = false;
    let busy = false;
    let errorMessage = "";

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

            view = session.authenticated ? "dashboard" : "login";
        } catch (error) {
            errorMessage = describeError(error);
            view = "error";
        }
    }

    async function handleSetup(event: SubmitEvent) {
        event.preventDefault();
        errorMessage = "";

        if (password !== confirmPassword) {
            errorMessage = "The two passwords do not match.";
            return;
        }

        busy = true;

        try {
            await setupAdmin(setupCode.trim(), password);

            await loginAdmin(password);

            clearSensitiveFields();

            view = "dashboard";
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

            view = "login";
        } catch (error) {
            errorMessage = describeError(error);
        } finally {
            busy = false;
        }
    }

    function clearSensitiveFields() {
        setupCode = "";
        password = "";
        confirmPassword = "";
        showPassword = false;
    }

    function describeError(error: unknown): string {
        if (!(error instanceof ApiError)) {
            return "CrumbVote could not reach the server. Check that the backend is running.";
        }

        const messages: Record<string, string> = {
            invalid_setup_code:
                "That setup code does not match the code printed by the CrumbVote server.",

            password_too_short:
                "Choose a password with at least 12 characters.",

            password_too_long: "That password is too long.",

            already_configured: "CrumbVote has already been configured.",

            setup_required:
                "CrumbVote still needs to be configured before you can sign in.",

            invalid_credentials: "That password is not correct.",

            database_error: "CrumbVote could not access its database.",

            setup_state_unavailable:
                "The first-run setup state is unavailable. Restart CrumbVote and try again.",

            password_hashing_failed:
                "CrumbVote could not securely store that password.",

            password_verification_failed:
                "CrumbVote could not verify the password.",

            session_creation_failed:
                "CrumbVote could not create an administrator session.",
        };

        return (
            messages[error.code] ??
            `The request failed with error "${error.code}".`
        );
    }
</script>

<svelte:head>
    <title>Admin · CrumbVote</title>
    <meta name="description" content="CrumbVote administration console." />
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
                        Administration
                    </div>
                </div>
            </a>

            <a
                href="/"
                class="rounded-xl border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-400 transition hover:border-white/20 hover:bg-white/10 hover:text-white"
            >
                ← Public site
            </a>
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
                                Authenticated
                            </div>

                            <h1
                                class="text-3xl font-semibold tracking-tight sm:text-4xl"
                            >
                                Admin console
                            </h1>

                            <p class="mt-3 max-w-2xl leading-7 text-slate-400">
                                CrumbVote is configured and your administrator
                                session is active. Event management is our next
                                stop.
                            </p>
                        </div>

                        <button
                            type="button"
                            disabled={busy}
                            onclick={handleLogout}
                            class="w-fit rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm font-medium text-slate-300 transition hover:border-white/20 hover:bg-white/10 hover:text-white disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {busy ? "Signing out…" : "Sign out"}
                        </button>
                    </div>
                </div>

                <div class="mt-6 grid gap-4 md:grid-cols-3">
                    <article
                        class="rounded-3xl border border-white/10 bg-white/[0.035] p-6"
                    >
                        <div class="text-sm text-slate-500">Events</div>
                        <div class="mt-3 text-3xl font-semibold">0</div>
                        <div class="mt-2 text-sm text-slate-400">
                            Event management arrives in M2.
                        </div>
                    </article>

                    <article
                        class="rounded-3xl border border-white/10 bg-white/[0.035] p-6"
                    >
                        <div class="text-sm text-slate-500">Votes</div>
                        <div class="mt-3 text-3xl font-semibold">—</div>
                        <div class="mt-2 text-sm text-slate-400">
                            Waiting for the first event.
                        </div>
                    </article>

                    <article
                        class="rounded-3xl border border-white/10 bg-white/[0.035] p-6"
                    >
                        <div class="text-sm text-slate-500">Admin security</div>

                        <div
                            class="mt-3 flex items-center gap-2 text-lg font-semibold"
                        >
                            <span class="h-2 w-2 rounded-full bg-emerald-400"
                            ></span>
                            Active
                        </div>

                        <div class="mt-2 text-sm text-slate-400">
                            Session-backed administrator access is online.
                        </div>
                    </article>
                </div>

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
                            No events yet
                        </h2>

                        <p class="mt-2 leading-7 text-slate-500">
                            Soon this is where you'll create the cake
                            competition, add entries and open voting.
                        </p>

                        <button
                            type="button"
                            disabled
                            class="mt-6 cursor-not-allowed rounded-xl bg-white/10 px-5 py-3 text-sm font-medium text-slate-500"
                        >
                            Create event · coming in M2
                        </button>
                    </div>
                </div>
            </section>
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
                                    Checking CrumbVote…
                                </div>

                                <div class="mt-2 text-sm text-slate-500">
                                    Verifying setup and administrator session.
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
                                    Couldn't load the admin console
                                </h1>

                                <p class="mt-3 leading-7 text-slate-400">
                                    {errorMessage}
                                </p>

                                <button
                                    type="button"
                                    onclick={bootstrap}
                                    class="mt-6 rounded-xl bg-white px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-slate-200"
                                >
                                    Try again
                                </button>
                            </div>
                        {:else if view === "setup"}
                            <div>
                                <div
                                    class="mb-5 inline-flex rounded-full border border-violet-400/20 bg-violet-400/10 px-3 py-1.5 text-xs font-medium text-violet-300"
                                >
                                    First-time setup
                                </div>

                                <h1
                                    class="text-3xl font-semibold tracking-tight"
                                >
                                    Claim this CrumbVote
                                </h1>

                                <p class="mt-3 leading-7 text-slate-400">
                                    Enter the one-time setup code printed in the
                                    server console, then choose the
                                    administrator password.
                                </p>

                                <form
                                    class="mt-7 space-y-5"
                                    onsubmit={handleSetup}
                                >
                                    <label class="block">
                                        <span
                                            class="text-sm font-medium text-slate-300"
                                        >
                                            Setup code
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
                                            Administrator password
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
                                                {showPassword ? "Hide" : "Show"}
                                            </button>
                                        </div>

                                        <span
                                            class="mt-2 block text-xs text-slate-600"
                                        >
                                            Minimum 12 characters.
                                        </span>
                                    </label>

                                    <label class="block">
                                        <span
                                            class="text-sm font-medium text-slate-300"
                                        >
                                            Confirm password
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
                                            ? "Configuring…"
                                            : "Configure CrumbVote"}
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
                                    CrumbVote is configured
                                </div>

                                <h1
                                    class="text-3xl font-semibold tracking-tight"
                                >
                                    Welcome back
                                </h1>

                                <p class="mt-3 leading-7 text-slate-400">
                                    Enter the administrator password to
                                    continue.
                                </p>

                                <form
                                    class="mt-7 space-y-5"
                                    onsubmit={handleLogin}
                                >
                                    <label class="block">
                                        <span
                                            class="text-sm font-medium text-slate-300"
                                        >
                                            Administrator password
                                        </span>

                                        <div class="relative mt-2">
                                            <input
                                                bind:value={password}
                                                type={showPassword
                                                    ? "text"
                                                    : "password"}
                                                autocomplete="current-password"
                                                required
                                                autofocus
                                                class="w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 pr-20 text-white outline-none transition focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                                            />

                                            <button
                                                type="button"
                                                onclick={() =>
                                                    (showPassword =
                                                        !showPassword)}
                                                class="absolute right-3 top-1/2 -translate-y-1/2 rounded-lg px-2.5 py-1.5 text-xs font-medium text-slate-500 transition hover:bg-white/5 hover:text-slate-300"
                                            >
                                                {showPassword ? "Hide" : "Show"}
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
                                        {busy ? "Signing in…" : "Sign in"}
                                    </button>
                                </form>
                            </div>
                        {/if}
                    </div>

                    <p
                        class="mt-5 text-center text-xs leading-5 text-slate-600"
                    >
                        Administrator sessions are stored in an HttpOnly browser
                        cookie.
                    </p>
                </div>
            </section>
        {/if}
    </div>
</main>
