<script lang="ts">
    import { onMount } from "svelte";
    import {
        ApiError,
        getAdminEvent,
        updateAdminEvent,
        type CrumbEvent,
        type EventStatus,
    } from "../lib/api";

    export let eventId: number;
    export let onSessionExpired: () => void;

    let event: CrumbEvent | null = null;

    let title = "";
    let description = "";
    let resultsPublic = false;

    let loading = true;
    let busy = false;

    let errorMessage = "";
    let successMessage = "";

    onMount(() => {
        void loadEvent();
    });

    async function loadEvent() {
        loading = true;
        errorMessage = "";

        try {
            const loaded = await getAdminEvent(eventId);

            applyEvent(loaded);
        } catch (error) {
            handleError(error);
        } finally {
            loading = false;
        }
    }

    async function handleSave(eventSubmit: SubmitEvent) {
        eventSubmit.preventDefault();

        if (event === null) {
            return;
        }

        await persistEvent(event.status);
    }

    async function handleStatusChange(status: EventStatus) {
        await persistEvent(status);
    }

    async function persistEvent(status: EventStatus) {
        if (event === null) {
            return;
        }

        busy = true;
        errorMessage = "";
        successMessage = "";

        try {
            const updated = await updateAdminEvent(event.id, {
                title: title.trim(),
                description: description.trim() || null,
                status,
                results_public: resultsPublic,
            });

            applyEvent(updated);

            successMessage = "Changes saved.";
        } catch (error) {
            handleError(error);
        } finally {
            busy = false;
        }
    }

    function applyEvent(updated: CrumbEvent) {
        event = updated;
        title = updated.title;
        description = updated.description ?? "";
        resultsPublic = updated.results_public;
    }

    function handleError(error: unknown) {
        if (
            error instanceof ApiError &&
            error.code === "authentication_required"
        ) {
            onSessionExpired();
            return;
        }

        errorMessage = describeError(error);
    }

    function describeError(error: unknown): string {
        if (!(error instanceof ApiError)) {
            return "CrumbVote could not reach the server.";
        }

        const messages: Record<string, string> = {
            event_not_found: "This event no longer exists.",

            title_required: "Give the event a title.",

            title_too_long: "The event title is too long.",

            description_too_long: "The event description is too long.",

            invalid_event_status:
                "CrumbVote rejected the requested event status.",

            invalid_status_transition:
                "That event status transition is not allowed.",

            database_error: "CrumbVote could not access its database.",
        };

        return (
            messages[error.code] ??
            `The request failed with error "${error.code}".`
        );
    }

    function statusLabel(status: EventStatus): string {
        switch (status) {
            case "draft":
                return "Draft";

            case "open":
                return "Open";

            case "closed":
                return "Closed";
        }
    }
</script>

{#if loading}
    <section class="flex flex-1 items-center justify-center py-24">
        <div class="text-center">
            <div
                class="mx-auto h-9 w-9 animate-spin rounded-full border-2 border-white/10 border-t-violet-400"
            ></div>

            <div class="mt-5 font-medium">Loading event…</div>

            <div class="mt-2 text-sm text-slate-500">
                Fetching event settings.
            </div>
        </div>
    </section>
{:else if event === null}
    <section class="flex flex-1 items-center justify-center py-24">
        <div
            class="w-full max-w-lg rounded-[2rem] border border-red-400/15 bg-red-400/[0.06] p-8 text-center"
        >
            <div
                class="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl bg-red-400/10 text-xl text-red-300"
            >
                !
            </div>

            <h1 class="mt-5 text-2xl font-semibold">Couldn't load event</h1>

            <p class="mt-3 leading-7 text-slate-400">
                {errorMessage}
            </p>

            <a
                href="/admin"
                class="mt-6 inline-flex rounded-xl bg-white px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-slate-200"
            >
                Back to events
            </a>
        </div>
    </section>
{:else}
    <section class="flex flex-1 flex-col py-10 lg:py-14">
        <div class="mb-5 flex flex-wrap items-center justify-between gap-4">
            <a
                href="/admin"
                class="inline-flex items-center gap-2 text-sm text-slate-500 transition hover:text-white"
            >
                ← Back to events
            </a>

            <div class="text-xs text-slate-600">
                Event #{event.id}
            </div>
        </div>

        <div
            class="rounded-[2rem] border border-white/10 bg-white/[0.035] p-7 shadow-2xl shadow-black/20 backdrop-blur-xl sm:p-9"
        >
            <div
                class="flex flex-col gap-6 lg:flex-row lg:items-start lg:justify-between"
            >
                <div class="min-w-0">
                    <div class="flex flex-wrap items-center gap-3">
                        {#if event.status === "open"}
                            <span
                                class="inline-flex items-center gap-2 rounded-full border border-emerald-400/15 bg-emerald-400/10 px-3 py-1.5 text-xs font-medium text-emerald-300"
                            >
                                <span
                                    class="h-1.5 w-1.5 rounded-full bg-emerald-400"
                                ></span>

                                Open
                            </span>
                        {:else if event.status === "closed"}
                            <span
                                class="inline-flex items-center gap-2 rounded-full border border-slate-400/15 bg-slate-400/10 px-3 py-1.5 text-xs font-medium text-slate-300"
                            >
                                <span
                                    class="h-1.5 w-1.5 rounded-full bg-slate-400"
                                ></span>

                                Closed
                            </span>
                        {:else}
                            <span
                                class="inline-flex items-center gap-2 rounded-full border border-violet-400/15 bg-violet-400/10 px-3 py-1.5 text-xs font-medium text-violet-300"
                            >
                                <span
                                    class="h-1.5 w-1.5 rounded-full bg-violet-400"
                                ></span>

                                Draft
                            </span>
                        {/if}

                        <span class="font-mono text-xs text-slate-600">
                            {event.slug}
                        </span>
                    </div>

                    <h1
                        class="mt-4 break-words text-3xl font-semibold tracking-tight sm:text-4xl"
                    >
                        {event.title}
                    </h1>

                    <p class="mt-3 max-w-3xl leading-7 text-slate-400">
                        {event.description ?? "No description yet."}
                    </p>
                </div>

                <div class="flex shrink-0 flex-wrap gap-3">
                    {#if event.status === "draft"}
                        <button
                            type="button"
                            disabled={busy}
                            onclick={() => void handleStatusChange("open")}
                            class="rounded-xl bg-emerald-400 px-5 py-3 text-sm font-semibold text-emerald-950 transition hover:bg-emerald-300 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            Open voting
                        </button>
                    {:else if event.status === "open"}
                        <button
                            type="button"
                            disabled={busy}
                            onclick={() => void handleStatusChange("closed")}
                            class="rounded-xl bg-amber-300 px-5 py-3 text-sm font-semibold text-amber-950 transition hover:bg-amber-200 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            Close voting
                        </button>
                    {:else}
                        <button
                            type="button"
                            disabled={busy}
                            onclick={() => void handleStatusChange("open")}
                            class="rounded-xl border border-emerald-400/20 bg-emerald-400/10 px-5 py-3 text-sm font-semibold text-emerald-300 transition hover:bg-emerald-400/15 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            Reopen voting
                        </button>
                    {/if}
                </div>
            </div>
        </div>

        <div class="mt-6 grid gap-6 xl:grid-cols-[minmax(0,1fr)_22rem]">
            <form
                onsubmit={handleSave}
                class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6 sm:p-8"
            >
                <div>
                    <h2 class="text-xl font-semibold">Event settings</h2>

                    <p class="mt-1 text-sm text-slate-500">
                        Basic information and result visibility.
                    </p>
                </div>

                <div class="mt-7 space-y-6">
                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            Event title
                        </span>

                        <input
                            bind:value={title}
                            type="text"
                            maxlength="120"
                            required
                            class="mt-2 w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        />
                    </label>

                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            Description
                        </span>

                        <textarea
                            bind:value={description}
                            maxlength="2000"
                            rows="6"
                            class="mt-2 w-full resize-none rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        ></textarea>

                        <div class="mt-2 text-right text-xs text-slate-600">
                            {description.length} / 2000
                        </div>
                    </label>

                    <div>
                        <span class="text-sm font-medium text-slate-300">
                            Event URL
                        </span>

                        <div
                            class="mt-2 rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 font-mono text-sm text-slate-400"
                        >
                            /e/{event.slug}
                        </div>

                        <p class="mt-2 text-xs leading-5 text-slate-600">
                            The event slug is permanent after creation so
                            printed QR codes cannot be accidentally broken.
                        </p>
                    </div>

                    <label
                        class="flex cursor-pointer items-start justify-between gap-5 rounded-2xl border border-white/10 bg-slate-950/40 p-5"
                    >
                        <div>
                            <div class="font-medium text-slate-200">
                                Public results
                            </div>

                            <div
                                class="mt-1 max-w-md text-sm leading-6 text-slate-500"
                            >
                                Allow visitors to see voting results when the
                                public results page is available.
                            </div>
                        </div>

                        <input
                            bind:checked={resultsPublic}
                            type="checkbox"
                            class="mt-1 h-5 w-5 shrink-0 accent-violet-500"
                        />
                    </label>

                    {#if errorMessage}
                        <div
                            class="rounded-xl border border-red-400/15 bg-red-400/10 px-4 py-3 text-sm leading-6 text-red-200"
                        >
                            {errorMessage}
                        </div>
                    {/if}

                    {#if successMessage}
                        <div
                            class="rounded-xl border border-emerald-400/15 bg-emerald-400/10 px-4 py-3 text-sm leading-6 text-emerald-200"
                        >
                            {successMessage}
                        </div>
                    {/if}

                    <div class="flex justify-end">
                        <button
                            type="submit"
                            disabled={busy}
                            class="rounded-xl bg-white px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-slate-200 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {busy ? "Saving…" : "Save changes"}
                        </button>
                    </div>
                </div>
            </form>

            <div class="space-y-6">
                <article
                    class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6"
                >
                    <div class="text-sm text-slate-500">Voting status</div>

                    <div class="mt-3 text-2xl font-semibold">
                        {statusLabel(event.status)}
                    </div>

                    <p class="mt-2 text-sm leading-6 text-slate-500">
                        {#if event.status === "draft"}
                            Voting has not been opened yet.
                        {:else if event.status === "open"}
                            Visitors can vote while this event is open.
                        {:else}
                            Voting is currently closed.
                        {/if}
                    </p>
                </article>

                <article
                    class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6"
                >
                    <div class="text-sm text-slate-500">Entries</div>

                    <div class="mt-3 text-3xl font-semibold">0</div>

                    <p class="mt-2 text-sm leading-6 text-slate-500">
                        Contestants and their individual voting links are coming
                        next.
                    </p>

                    <button
                        type="button"
                        disabled
                        class="mt-5 w-full cursor-not-allowed rounded-xl border border-white/5 bg-white/[0.03] px-4 py-2.5 text-sm text-slate-600"
                    >
                        Add entry · M3
                    </button>
                </article>

                <article
                    class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6"
                >
                    <div class="text-sm text-slate-500">Last updated</div>

                    <div class="mt-3 text-sm font-medium text-slate-300">
                        {new Date(event.updated_at * 1000).toLocaleString()}
                    </div>
                </article>
            </div>
        </div>
    </section>
{/if}
