<script lang="ts">
    import { onMount } from "svelte";
    import {
        ApiError,
        castPublicVote,
        ensurePublicVoter,
        getPublicEntry,
        getPublicVote,
        trackPublicScan,
        type PublicEntryPayload,
    } from "../lib/api";

    const VOTER_STORAGE_KEY = "crumbvote_voter_token";

    export let eventSlug: string;
    export let entryId: number;

    let payload: PublicEntryPayload | null = null;

    let voterToken = "";
    let currentVoteEntryId: number | null = null;

    let loading = true;
    let voteBusy = false;

    let errorMessage = "";
    let voteErrorMessage = "";
    let voteSuccessMessage = "";

    onMount(() => {
        void loadEntry();
    });

    async function loadEntry() {
        loading = true;
        errorMessage = "";

        try {
            const loaded = await getPublicEntry(eventSlug, entryId);

            const token = await ensureVoterIdentity();

            const vote = await getPublicVote(eventSlug, token);

            try {
                await trackPublicScan(eventSlug, loaded.entry.id, token);
            } catch (error) {
                console.warn("Failed to track public entry scan:", error);
            }

            payload = loaded;
            voterToken = token;
            currentVoteEntryId = vote.entry_id;
        } catch (error) {
            payload = null;
            errorMessage = describeError(error);
        } finally {
            loading = false;
        }
    }

    async function ensureVoterIdentity(): Promise<string> {
        const stored = window.localStorage.getItem(VOTER_STORAGE_KEY);

        try {
            const voter = await ensurePublicVoter(stored);

            rememberVoterToken(voter.token);

            return voter.token;
        } catch (error) {
            if (
                stored !== null &&
                error instanceof ApiError &&
                error.code === "invalid_voter_token"
            ) {
                window.localStorage.removeItem(VOTER_STORAGE_KEY);

                const voter = await ensurePublicVoter(null);

                rememberVoterToken(voter.token);

                return voter.token;
            }

            throw error;
        }
    }

    function rememberVoterToken(token: string) {
        window.localStorage.setItem(VOTER_STORAGE_KEY, token);
    }

    async function handleVote() {
        if (
            payload === null ||
            voterToken.length === 0 ||
            payload.event.status !== "open"
        ) {
            return;
        }

        const previousVote = currentVoteEntryId;

        voteBusy = true;
        voteErrorMessage = "";
        voteSuccessMessage = "";

        try {
            const vote = await castPublicVote(
                eventSlug,
                payload.entry.id,
                voterToken,
            );

            currentVoteEntryId = vote.entry_id;

            voteSuccessMessage =
                previousVote === null ? "Vote recorded." : "Vote changed.";
        } catch (error) {
            voteErrorMessage = describeError(error);
        } finally {
            voteBusy = false;
        }
    }

    function describeError(error: unknown): string {
        if (!(error instanceof ApiError)) {
            return "CrumbVote could not reach the server.";
        }

        switch (error.code) {
            case "public_entry_not_found":
                return "This voting entry could not be found.";

            case "database_error":
                return "CrumbVote could not load this entry.";

            case "voter_token_required":
            case "invalid_voter_token":
                return "CrumbVote could not identify this browser.";

            case "voter_creation_failed":
                return "CrumbVote could not create a voter identity.";

            case "voting_not_open":
                return "Voting is not currently open.";

            default:
                return `The request failed with error "${error.code}".`;
        }
    }
</script>

<svelte:head>
    <title>
        {payload?.entry.name ?? "Vote"} · CrumbVote
    </title>
</svelte:head>

<main class="relative min-h-screen overflow-hidden bg-slate-950 text-white">
    <div
        class="pointer-events-none absolute left-1/2 top-[-18rem] h-[38rem] w-[38rem] -translate-x-1/2 rounded-full bg-violet-600/20 blur-[130px]"
    ></div>

    <div
        class="pointer-events-none absolute bottom-[-14rem] right-[-10rem] h-[30rem] w-[30rem] rounded-full bg-fuchsia-600/10 blur-[120px]"
    ></div>

    <div
        class="relative mx-auto flex min-h-screen w-full max-w-2xl flex-col px-4 py-5 sm:px-6 sm:py-8"
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

                <span>CrumbVote</span>
            </a>

            <span class="text-xs text-slate-600"> Public voting </span>
        </header>

        {#if loading}
            <section class="flex flex-1 items-center justify-center py-20">
                <div class="text-center">
                    <div
                        class="mx-auto h-9 w-9 animate-spin rounded-full border-2 border-white/10 border-t-violet-400"
                    ></div>

                    <div class="mt-5 font-medium">Loading entry…</div>
                </div>
            </section>
        {:else if payload === null}
            <section class="flex flex-1 items-center justify-center py-20">
                <div
                    class="w-full rounded-[2rem] border border-red-400/15 bg-red-400/[0.06] p-8 text-center"
                >
                    <div
                        class="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl bg-red-400/10 text-xl text-red-300"
                    >
                        !
                    </div>

                    <h1 class="mt-5 text-2xl font-semibold">Entry not found</h1>

                    <p class="mt-3 leading-7 text-slate-400">
                        {errorMessage}
                    </p>

                    <a
                        href="/"
                        class="mt-6 inline-flex rounded-xl bg-white px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-slate-200"
                    >
                        Back to CrumbVote
                    </a>
                </div>
            </section>
        {:else}
            <section class="flex flex-1 flex-col py-8 sm:py-12">
                <div class="mb-5 flex flex-wrap items-center gap-3">
                    {#if payload.event.status === "open"}
                        <span
                            class="inline-flex items-center gap-2 rounded-full border border-emerald-400/15 bg-emerald-400/10 px-3 py-1.5 text-xs font-medium text-emerald-300"
                        >
                            <span
                                class="h-1.5 w-1.5 rounded-full bg-emerald-400"
                            ></span>

                            Voting is open
                        </span>
                    {:else if payload.event.status === "closed"}
                        <span
                            class="inline-flex items-center gap-2 rounded-full border border-slate-400/15 bg-slate-400/10 px-3 py-1.5 text-xs font-medium text-slate-300"
                        >
                            <span class="h-1.5 w-1.5 rounded-full bg-slate-400"
                            ></span>

                            Voting has ended
                        </span>
                    {:else}
                        <span
                            class="inline-flex items-center gap-2 rounded-full border border-violet-400/15 bg-violet-400/10 px-3 py-1.5 text-xs font-medium text-violet-300"
                        >
                            <span class="h-1.5 w-1.5 rounded-full bg-violet-400"
                            ></span>

                            Voting has not started
                        </span>
                    {/if}

                    <span class="text-xs text-slate-600">
                        {payload.event.title}
                    </span>
                </div>

                <article
                    class="overflow-hidden rounded-[2rem] border border-white/10 bg-white/[0.035] shadow-2xl shadow-black/30 backdrop-blur-xl"
                >
                    <div
                        class="relative aspect-[4/3] overflow-hidden bg-black/20 sm:aspect-[16/10]"
                    >
                        {#if payload.entry.image_url !== null}
                            <img
                                src={payload.entry.image_url}
                                alt={payload.entry.name}
                                class="h-full w-full object-cover"
                            />
                        {:else}
                            <div
                                class="flex h-full items-center justify-center"
                            >
                                <div class="text-center">
                                    <div
                                        class="mx-auto flex h-16 w-16 items-center justify-center rounded-2xl bg-white/5 text-3xl text-slate-600"
                                    >
                                        ◇
                                    </div>

                                    <div class="mt-3 text-sm text-slate-600">
                                        No image
                                    </div>
                                </div>
                            </div>
                        {/if}

                        <div
                            class="absolute left-4 top-4 rounded-2xl border border-white/10 bg-slate-950/85 px-4 py-2.5 font-mono text-lg font-semibold backdrop-blur"
                        >
                            #{payload.entry.number}
                        </div>
                    </div>

                    <div class="p-6 sm:p-8">
                        <div class="text-sm font-medium text-violet-300">
                            Entry #{payload.entry.number}
                        </div>

                        <h1
                            class="mt-2 text-3xl font-semibold tracking-tight sm:text-4xl"
                        >
                            {payload.entry.name}
                        </h1>

                        <p class="mt-4 text-base leading-7 text-slate-400">
                            {payload.entry.description ??
                                "No description was provided for this entry."}
                        </p>

                        <div class="mt-7 border-t border-white/10 pt-6">
                            {#if payload.event.status === "open"}
                                <div
                                    class="rounded-2xl border border-emerald-400/15 bg-emerald-400/[0.07] p-5"
                                >
                                    {#if currentVoteEntryId === payload.entry.id}
                                        <div
                                            class="font-semibold text-emerald-200"
                                        >
                                            This is your current vote
                                        </div>

                                        <p
                                            class="mt-2 text-sm leading-6 text-slate-400"
                                        >
                                            You can visit another entry and move
                                            your vote there while voting remains
                                            open.
                                        </p>

                                        <button
                                            type="button"
                                            disabled
                                            class="mt-5 w-full cursor-default rounded-xl bg-emerald-300 px-5 py-3.5 text-sm font-semibold text-emerald-950"
                                        >
                                            Your vote ✓
                                        </button>
                                    {:else}
                                        <div
                                            class="font-semibold text-emerald-200"
                                        >
                                            {currentVoteEntryId === null
                                                ? "Ready to vote?"
                                                : "Change your vote?"}
                                        </div>

                                        <p
                                            class="mt-2 text-sm leading-6 text-slate-400"
                                        >
                                            {#if currentVoteEntryId === null}
                                                Choose this entry as your vote
                                                for the event.
                                            {:else}
                                                You already voted for another
                                                entry. Choosing this one moves
                                                your vote here.
                                            {/if}
                                        </p>

                                        <button
                                            type="button"
                                            disabled={voteBusy}
                                            onclick={() => void handleVote()}
                                            class="mt-5 w-full rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-5 py-3.5 text-sm font-semibold text-white shadow-lg shadow-violet-950/30 transition hover:brightness-110 disabled:cursor-not-allowed disabled:opacity-50"
                                        >
                                            {#if voteBusy}
                                                Saving vote…
                                            {:else if currentVoteEntryId === null}
                                                Vote for #{payload.entry.number}
                                            {:else}
                                                Change vote to #{payload.entry
                                                    .number}
                                            {/if}
                                        </button>
                                    {/if}

                                    {#if voteErrorMessage}
                                        <div
                                            class="mt-4 rounded-xl border border-red-400/15 bg-red-400/10 px-4 py-3 text-sm text-red-200"
                                        >
                                            {voteErrorMessage}
                                        </div>
                                    {/if}

                                    {#if voteSuccessMessage}
                                        <div
                                            class="mt-4 rounded-xl border border-emerald-400/15 bg-emerald-400/10 px-4 py-3 text-sm text-emerald-200"
                                        >
                                            {voteSuccessMessage}
                                        </div>
                                    {/if}
                                </div>
                            {:else if payload.event.status === "closed"}
                                <div
                                    class="rounded-2xl border border-white/10 bg-white/[0.03] p-5"
                                >
                                    <div class="font-semibold">
                                        Voting is closed
                                    </div>

                                    <p
                                        class="mt-2 text-sm leading-6 text-slate-500"
                                    >
                                        This event is no longer accepting votes.
                                    </p>
                                </div>
                            {:else}
                                <div
                                    class="rounded-2xl border border-violet-400/15 bg-violet-400/[0.07] p-5"
                                >
                                    <div class="font-semibold text-violet-200">
                                        Voting hasn't started yet
                                    </div>

                                    <p
                                        class="mt-2 text-sm leading-6 text-slate-500"
                                    >
                                        Come back when the organizer opens the
                                        event.
                                    </p>
                                </div>
                            {/if}
                        </div>
                    </div>
                </article>

                {#if payload.event.description !== null}
                    <div
                        class="mt-5 rounded-2xl border border-white/10 bg-white/[0.025] p-5"
                    >
                        <div
                            class="text-xs font-medium uppercase tracking-wider text-slate-600"
                        >
                            About the event
                        </div>

                        <p class="mt-2 text-sm leading-6 text-slate-400">
                            {payload.event.description}
                        </p>
                    </div>
                {/if}
            </section>
        {/if}
    </div>
</main>
