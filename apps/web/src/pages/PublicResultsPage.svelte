<script lang="ts">
    import { onMount } from "svelte";
    import {
        ApiError,
        getPublicResults,
        type PublicResultsPayload,
    } from "../lib/api";

    export let eventSlug: string;

    let payload: PublicResultsPayload | null = null;
    let loading = true;
    let errorMessage = "";

    onMount(() => {
        void loadResults();
    });

    async function loadResults() {
        loading = true;
        errorMessage = "";

        try {
            payload = await getPublicResults(eventSlug);
        } catch (error) {
            payload = null;
            errorMessage = describeError(error);
        } finally {
            loading = false;
        }
    }

    function voteShare(votes: number): string {
        if (payload === null || payload.total_votes === 0) {
            return "0%";
        }

        const percentage = (votes / payload.total_votes) * 100;

        return `${Math.round(percentage)}%`;
    }

    function describeError(error: unknown): string {
        if (!(error instanceof ApiError)) {
            return "CrumbVote could not reach the server.";
        }

        switch (error.code) {
            case "public_results_unavailable":
                return "The organizer has not made results public.";

            case "public_entry_not_found":
                return "This event could not be found.";

            case "database_error":
                return "CrumbVote could not load these results.";

            default:
                return `The request failed with error "${error.code}".`;
        }
    }
</script>

<svelte:head>
    <title>
        {payload?.event.title ?? "Results"} · CrumbVote
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
        class="relative mx-auto flex min-h-screen w-full max-w-3xl flex-col px-4 py-5 sm:px-6 sm:py-8"
    >
        <header class="flex items-center justify-between">
            <a
                href="/"
                class="flex items-center gap-3 font-semibold tracking-tight"
            >
                <span
                    class="flex h-10 w-10 items-center justify-center rounded-2xl bg-gradient-to-br from-violet-500 to-fuchsia-500 text-lg font-bold"
                >
                    C
                </span>

                <span>CrumbVote</span>
            </a>

            <span class="text-xs text-slate-600"> Public results </span>
        </header>

        {#if loading}
            <section class="flex flex-1 items-center justify-center py-20">
                <div class="text-center">
                    <div
                        class="mx-auto h-9 w-9 animate-spin rounded-full border-2 border-white/10 border-t-violet-400"
                    ></div>

                    <div class="mt-5 font-medium">Loading results…</div>
                </div>
            </section>
        {:else if payload === null}
            <section class="flex flex-1 items-center justify-center py-20">
                <div
                    class="w-full rounded-[2rem] border border-white/10 bg-white/[0.035] p-8 text-center"
                >
                    <div
                        class="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl bg-white/5 text-xl text-slate-400"
                    >
                        ?
                    </div>

                    <h1 class="mt-5 text-2xl font-semibold">
                        Results unavailable
                    </h1>

                    <p class="mt-3 leading-7 text-slate-400">
                        {errorMessage}
                    </p>

                    <a
                        href="/"
                        class="mt-6 inline-flex rounded-xl bg-white px-5 py-3 text-sm font-semibold text-slate-950"
                    >
                        Back to CrumbVote
                    </a>
                </div>
            </section>
        {:else}
            <section class="flex flex-1 flex-col py-8 sm:py-12">
                <div
                    class="rounded-[2rem] border border-white/10 bg-white/[0.035] p-6 sm:p-8"
                >
                    <div
                        class="inline-flex rounded-full border border-violet-400/15 bg-violet-400/10 px-3 py-1.5 text-xs font-medium text-violet-300"
                    >
                        {payload.event.status === "closed"
                            ? "Final results"
                            : "Live results"}
                    </div>

                    <h1
                        class="mt-4 text-3xl font-semibold tracking-tight sm:text-4xl"
                    >
                        {payload.event.title}
                    </h1>

                    <p class="mt-3 text-sm leading-6 text-slate-500">
                        {payload.total_votes}
                        {payload.total_votes === 1
                            ? "current vote"
                            : "current votes"}
                    </p>

                    {#if payload.event.status === "open"}
                        <p
                            class="mt-4 rounded-xl border border-emerald-400/10 bg-emerald-400/[0.06] px-4 py-3 text-sm leading-6 text-emerald-200"
                        >
                            Voting is still open. These results may change.
                        </p>
                    {/if}
                </div>

                {#if payload.entries.length === 0}
                    <div
                        class="mt-5 rounded-[2rem] border border-dashed border-white/10 px-6 py-14 text-center text-slate-600"
                    >
                        No entries yet.
                    </div>
                {:else}
                    <div class="mt-5 space-y-4">
                        {#each payload.entries as entry, index (entry.id)}
                            <article
                                data-testid={"public-result-entry-" + entry.id}
                                class="overflow-hidden rounded-[2rem] border border-white/10 bg-white/[0.025]"
                            >
                                <div
                                    class="flex flex-col sm:flex-row sm:items-stretch"
                                >
                                    <div
                                        class="relative aspect-[16/10] bg-black/20 sm:aspect-auto sm:w-48"
                                    >
                                        {#if entry.image_url !== null}
                                            <img
                                                src={entry.image_url}
                                                alt={entry.name}
                                                class="h-full w-full object-cover"
                                            />
                                        {:else}
                                            <div
                                                class="flex h-full min-h-36 items-center justify-center text-2xl text-slate-700"
                                            >
                                                ◇
                                            </div>
                                        {/if}

                                        <div
                                            class="absolute left-3 top-3 flex h-10 w-10 items-center justify-center rounded-xl bg-slate-950/85 font-semibold backdrop-blur"
                                        >
                                            {index + 1}
                                        </div>
                                    </div>

                                    <div class="flex-1 p-5 sm:p-6">
                                        <div
                                            class="text-xs font-medium text-violet-300"
                                        >
                                            Entry #{entry.number}
                                        </div>

                                        <h2 class="mt-1 text-xl font-semibold">
                                            {entry.name}
                                        </h2>

                                        <div
                                            class="mt-5 grid grid-cols-2 gap-3"
                                        >
                                            <div
                                                class="rounded-xl bg-black/20 p-3"
                                            >
                                                <div
                                                    class="text-2xl font-semibold"
                                                >
                                                    {entry.current_votes}
                                                </div>

                                                <div
                                                    class="mt-1 text-xs text-slate-600"
                                                >
                                                    Votes
                                                </div>
                                            </div>

                                            <div
                                                class="rounded-xl bg-black/20 p-3"
                                            >
                                                <div
                                                    class="text-2xl font-semibold"
                                                >
                                                    {voteShare(
                                                        entry.current_votes,
                                                    )}
                                                </div>

                                                <div
                                                    class="mt-1 text-xs text-slate-600"
                                                >
                                                    Vote share
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </article>
                        {/each}
                    </div>
                {/if}
            </section>
        {/if}
    </div>
</main>
