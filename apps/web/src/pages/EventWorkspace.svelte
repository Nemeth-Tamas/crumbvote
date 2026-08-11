<script lang="ts">
    import { onMount } from "svelte";
    import {
        ApiError,
        createAdminEntry,
        getAdminEvent,
        listAdminEntries,
        updateAdminEvent,
        type CrumbEntry,
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

    let entries: CrumbEntry[] = [];

    let createEntryOpen = false;
    let entryName = "";
    let entryDescription = "";
    let entryBusy = false;
    let entryErrorMessage = "";

    let copiedEntryId: number | null = null;

    onMount(() => {
        void loadWorkspace();
    });

    async function loadWorkspace() {
        loading = true;
        errorMessage = "";

        try {
            const [loadedEvent, loadedEntries] = await Promise.all([
                getAdminEvent(eventId),
                listAdminEntries(eventId),
            ]);

            applyEvent(loadedEvent);
            entries = loadedEntries;
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

    function openCreateEntry() {
        entryName = "";
        entryDescription = "";
        entryErrorMessage = "";
        createEntryOpen = true;
    }

    function closeCreateEntry() {
        createEntryOpen = false;
        entryName = "";
        entryDescription = "";
        entryErrorMessage = "";
    }

    async function handleCreateEntry(submitEvent: SubmitEvent) {
        submitEvent.preventDefault();

        if (event === null) {
            return;
        }

        entryBusy = true;
        entryErrorMessage = "";

        try {
            const created = await createAdminEntry(event.id, {
                name: entryName.trim(),
                description: entryDescription.trim() || null,
            });

            entries = [...entries, created];

            closeCreateEntry();
        } catch (error) {
            if (
                error instanceof ApiError &&
                error.code === "authentication_required"
            ) {
                closeCreateEntry();
                onSessionExpired();
                return;
            }

            entryErrorMessage = describeError(error);
        } finally {
            entryBusy = false;
        }
    }

    function entryUrl(entry: CrumbEntry): string {
        if (event === null) {
            return "";
        }

        return `/e/${event.slug}/${entry.id}`;
    }

    async function copyEntryUrl(entry: CrumbEntry) {
        const url = `${window.location.origin}${entryUrl(entry)}`;

        try {
            await navigator.clipboard.writeText(url);

            copiedEntryId = entry.id;

            window.setTimeout(() => {
                if (copiedEntryId === entry.id) {
                    copiedEntryId = null;
                }
            }, 1500);
        } catch {
            entryErrorMessage = "The voting link could not be copied.";
        }
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

            entry_name_required: "Give the entry a name.",

            entry_name_too_long: "The entry name is too long.",

            entry_description_too_long: "The entry description is too long.",

            event_entries_locked:
                "Entries are locked after voting has been opened.",
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

                    <div class="mt-3 text-3xl font-semibold">
                        {entries.length}
                    </div>

                    <p class="mt-2 text-sm leading-6 text-slate-500">
                        {#if event.status === "draft"}
                            Add contestants before opening voting.
                        {:else}
                            The contestant list is locked while the event is or
                            has been live.
                        {/if}
                    </p>

                    <button
                        type="button"
                        disabled={event.status !== "draft"}
                        onclick={openCreateEntry}
                        class="mt-5 w-full rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-4 py-2.5 text-sm font-semibold text-white transition hover:brightness-110 disabled:cursor-not-allowed disabled:bg-none disabled:bg-white/[0.03] disabled:text-slate-600"
                    >
                        + Add entry
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

        <div
            class="mt-6 rounded-[2rem] border border-white/10 bg-white/[0.025] p-6 sm:p-8"
        >
            <div
                class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between"
            >
                <div>
                    <h2 class="text-xl font-semibold">Entries</h2>

                    <p class="mt-1 text-sm text-slate-500">
                        Contestants and their stable voting links.
                    </p>
                </div>

                <button
                    type="button"
                    disabled={event.status !== "draft"}
                    onclick={openCreateEntry}
                    class="w-fit rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm font-medium text-slate-300 transition hover:border-white/20 hover:bg-white/10 hover:text-white disabled:cursor-not-allowed disabled:opacity-40"
                >
                    + Add entry
                </button>
            </div>

            {#if entries.length === 0}
                <div
                    class="mt-6 rounded-3xl border border-dashed border-white/10 px-6 py-12 text-center"
                >
                    <div
                        class="mx-auto flex h-12 w-12 items-center justify-center rounded-2xl bg-white/5 text-xl"
                    >
                        #
                    </div>

                    <h3 class="mt-4 font-semibold">No entries yet</h3>

                    <p
                        class="mx-auto mt-2 max-w-md text-sm leading-6 text-slate-500"
                    >
                        Add the cakes or contestants that visitors will be able
                        to vote for.
                    </p>
                </div>
            {:else}
                <div class="mt-6 grid gap-4 lg:grid-cols-2">
                    {#each entries as entry (entry.id)}
                        <article
                            class="rounded-3xl border border-white/10 bg-slate-950/40 p-5"
                        >
                            <div class="flex items-start gap-4">
                                <div
                                    class="flex h-11 w-11 shrink-0 items-center justify-center rounded-2xl bg-violet-400/10 font-mono font-semibold text-violet-300"
                                >
                                    #{entry.number}
                                </div>

                                <div class="min-w-0 flex-1">
                                    <h3
                                        class="truncate font-semibold text-white"
                                    >
                                        {entry.name}
                                    </h3>

                                    <p
                                        class="mt-2 min-h-10 text-sm leading-5 text-slate-500"
                                    >
                                        {entry.description ?? "No description."}
                                    </p>
                                </div>
                            </div>

                            <div class="mt-5 border-t border-white/5 pt-4">
                                <div class="text-xs font-medium text-slate-500">
                                    Voting link
                                </div>

                                <div class="mt-2 flex items-center gap-2">
                                    <div
                                        class="min-w-0 flex-1 truncate rounded-xl bg-black/20 px-3 py-2.5 font-mono text-xs text-slate-500"
                                    >
                                        {entryUrl(entry)}
                                    </div>

                                    <button
                                        type="button"
                                        onclick={() => void copyEntryUrl(entry)}
                                        class="shrink-0 rounded-xl border border-white/10 bg-white/5 px-3 py-2.5 text-xs font-medium text-slate-300 transition hover:bg-white/10 hover:text-white"
                                    >
                                        {copiedEntryId === entry.id
                                            ? "Copied"
                                            : "Copy"}
                                    </button>
                                </div>
                            </div>
                        </article>
                    {/each}
                </div>
            {/if}
        </div>
    </section>

    {#if createEntryOpen}
        <div
            class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6"
        >
            <button
                type="button"
                aria-label="Close add entry dialog"
                disabled={entryBusy}
                onclick={closeCreateEntry}
                class="absolute inset-0 bg-slate-950/80 backdrop-blur-sm"
            ></button>

            <div
                role="dialog"
                aria-modal="true"
                aria-labelledby="create-entry-title"
                class="relative z-10 w-full max-w-xl rounded-[2rem] border border-white/10 bg-slate-900 p-6 shadow-2xl shadow-black/50 sm:p-8"
            >
                <div class="flex items-start justify-between gap-4">
                    <div>
                        <div
                            class="mb-3 inline-flex rounded-full border border-violet-400/20 bg-violet-400/10 px-3 py-1 text-xs font-medium text-violet-300"
                        >
                            New contestant
                        </div>

                        <h2
                            id="create-entry-title"
                            class="text-2xl font-semibold tracking-tight"
                        >
                            Add entry
                        </h2>

                        <p class="mt-2 leading-6 text-slate-400">
                            The entry number and permanent voting link are
                            assigned automatically.
                        </p>
                    </div>

                    <button
                        type="button"
                        aria-label="Close"
                        disabled={entryBusy}
                        onclick={closeCreateEntry}
                        class="rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-slate-400 transition hover:bg-white/10 hover:text-white disabled:opacity-50"
                    >
                        ✕
                    </button>
                </div>

                <form class="mt-7 space-y-5" onsubmit={handleCreateEntry}>
                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            Entry name
                        </span>

                        <input
                            bind:value={entryName}
                            type="text"
                            maxlength="120"
                            placeholder="Málnás csokitorta"
                            required
                            class="mt-2 w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition placeholder:text-slate-700 focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        />
                    </label>

                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            Description
                        </span>

                        <textarea
                            bind:value={entryDescription}
                            maxlength="2000"
                            rows="4"
                            placeholder="Optional description…"
                            class="mt-2 w-full resize-none rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition placeholder:text-slate-700 focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        ></textarea>

                        <div class="mt-2 text-right text-xs text-slate-600">
                            {entryDescription.length} / 2000
                        </div>
                    </label>

                    {#if entryErrorMessage}
                        <div
                            class="rounded-xl border border-red-400/15 bg-red-400/10 px-4 py-3 text-sm leading-6 text-red-200"
                        >
                            {entryErrorMessage}
                        </div>
                    {/if}

                    <div
                        class="flex flex-col-reverse gap-3 pt-2 sm:flex-row sm:justify-end"
                    >
                        <button
                            type="button"
                            disabled={entryBusy}
                            onclick={closeCreateEntry}
                            class="rounded-xl border border-white/10 bg-white/5 px-5 py-3 text-sm font-medium text-slate-300 transition hover:bg-white/10 hover:text-white disabled:opacity-50"
                        >
                            Cancel
                        </button>

                        <button
                            type="submit"
                            disabled={entryBusy}
                            class="rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-5 py-3 text-sm font-semibold text-white transition hover:brightness-110 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {entryBusy ? "Adding…" : "Add entry"}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    {/if}
{/if}
