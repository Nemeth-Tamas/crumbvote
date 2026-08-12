<script lang="ts">
    import { onMount } from "svelte";
    import {
        ApiError,
        createAdminEntry,
        getAdminEvent,
        getAdminEventAnalytics,
        listAdminEntries,
        updateAdminEntry,
        updateAdminEvent,
        uploadAdminEntryImage,
        type AdminEventAnalytics,
        type CrumbEntry,
        type CrumbEvent,
        type EntryAnalytics,
        type EventStatus,
    } from "../lib/api";
    import { locale, translate, type TranslationKey } from "../lib/i18n";

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
    let analytics: AdminEventAnalytics | null = null;

    let createEntryOpen = false;
    let entryName = "";
    let entryDescription = "";
    let entryBusy = false;
    let entryErrorMessage = "";

    let copiedEntryId: number | null = null;

    let editEntryOpen = false;
    let editingEntry: CrumbEntry | null = null;
    let editEntryName = "";
    let editEntryDescription = "";
    let editEntryImageFile: File | null = null;
    let editEntryImagePreviewUrl: string | null = null;
    let editEntryBusy = false;
    let editEntryErrorMessage = "";

    onMount(() => {
        void loadWorkspace();
    });

    async function loadWorkspace() {
        loading = true;
        errorMessage = "";

        try {
            const [loadedEvent, loadedEntries, loadedAnalytics] =
                await Promise.all([
                    getAdminEvent(eventId),
                    listAdminEntries(eventId),
                    getAdminEventAnalytics(eventId),
                ]);

            applyEvent(loadedEvent);
            entries = loadedEntries;
            analytics = loadedAnalytics;
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

            successMessage = translate($locale, "workspace.changesSaved");
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

    function openEditEntry(entry: CrumbEntry) {
        clearEditEntryImage();

        editingEntry = entry;
        editEntryName = entry.name;
        editEntryDescription = entry.description ?? "";
        editEntryErrorMessage = "";
        editEntryOpen = true;
    }

    function closeEditEntry() {
        clearEditEntryImage();

        editEntryOpen = false;
        editingEntry = null;
        editEntryName = "";
        editEntryDescription = "";
        editEntryErrorMessage = "";
    }

    function clearEditEntryImage() {
        if (editEntryImagePreviewUrl !== null) {
            URL.revokeObjectURL(editEntryImagePreviewUrl);
        }

        editEntryImageFile = null;
        editEntryImagePreviewUrl = null;
    }

    function handleEditEntryImageChange(inputEvent: Event) {
        const input = inputEvent.currentTarget as HTMLInputElement;

        const file = input.files?.[0] ?? null;

        selectEditEntryImage(file);

        input.value = "";
    }

    function selectEditEntryImage(file: File | null) {
        if (file === null) {
            return;
        }

        if (!["image/jpeg", "image/png", "image/webp"].includes(file.type)) {
            editEntryErrorMessage = translate(
                $locale,
                "workspace.errorUnsupportedImageType",
            );
            return;
        }

        if (file.size > 8 * 1024 * 1024) {
            editEntryErrorMessage = translate(
                $locale,
                "workspace.errorImageTooLarge",
            );
            return;
        }

        clearEditEntryImage();

        editEntryImageFile = file;
        editEntryImagePreviewUrl = URL.createObjectURL(file);
        editEntryErrorMessage = "";
    }

    async function handleEditEntry(submitEvent: SubmitEvent) {
        submitEvent.preventDefault();

        if (event === null || editingEntry === null) {
            return;
        }

        editEntryBusy = true;
        editEntryErrorMessage = "";

        try {
            let updated = await updateAdminEntry(event.id, editingEntry.id, {
                name: editEntryName.trim(),
                description: editEntryDescription.trim() || null,
            });

            entries = entries.map((entry) =>
                entry.id === updated.id ? updated : entry,
            );

            editingEntry = updated;

            if (editEntryImageFile !== null) {
                updated = await uploadAdminEntryImage(
                    event.id,
                    updated.id,
                    editEntryImageFile,
                );

                entries = entries.map((entry) =>
                    entry.id === updated.id ? updated : entry,
                );

                editingEntry = updated;
            }

            closeEditEntry();
        } catch (error) {
            if (
                error instanceof ApiError &&
                error.code === "authentication_required"
            ) {
                closeEditEntry();
                onSessionExpired();
                return;
            }

            editEntryErrorMessage = describeError(error);
        } finally {
            editEntryBusy = false;
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
            entryErrorMessage = translate($locale, "workspace.errorCopyLink");
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
            return translate($locale, "workspace.errorNetwork");
        }

        const messages: Record<string, TranslationKey> = {
            event_not_found: "workspace.errorEventNotFound",

            title_required: "admin.errorTitleRequired",

            title_too_long: "admin.errorTitleTooLong",

            description_too_long: "admin.errorDescriptionTooLong",

            invalid_event_status: "workspace.errorInvalidEventStatus",

            invalid_status_transition: "workspace.errorInvalidStatusTransition",

            database_error: "admin.errorDatabase",

            entry_name_required: "workspace.errorEntryNameRequired",

            entry_name_too_long: "workspace.errorEntryNameTooLong",

            entry_description_too_long:
                "workspace.errorEntryDescriptionTooLong",

            event_entries_locked: "workspace.errorEntriesLocked",

            entry_not_found: "workspace.errorEntryNotFound",

            unsupported_image_type: "workspace.errorUnsupportedImageType",

            image_too_large: "workspace.errorImageTooLarge",

            image_empty: "workspace.errorImageEmpty",

            image_required: "workspace.errorImageRequired",

            invalid_image_upload: "workspace.errorInvalidImageUpload",

            invalid_image_data: "workspace.errorInvalidImageData",

            image_storage_error: "workspace.errorImageStorage",
        };

        const key = messages[error.code];

        if (key !== undefined) {
            return translate($locale, key);
        }

        return translate($locale, "common.requestFailed", {
            code: error.code,
        });
    }

    function statusLabel(status: EventStatus): string {
        switch (status) {
            case "draft":
                return translate($locale, "admin.statusDraft");

            case "open":
                return translate($locale, "admin.statusOpen");

            case "closed":
                return translate($locale, "admin.statusClosed");
        }
    }

    function conversionRate(): string {
        if (analytics === null || analytics.unique_visitors === 0) {
            return "0%";
        }

        const percentage =
            (analytics.current_votes / analytics.unique_visitors) * 100;

        return `${Math.round(percentage)}%`;
    }

    function rankedEntryAnalytics(): Array<{
        entry: CrumbEntry;
        analytics: EntryAnalytics;
    }> {
        if (analytics === null) {
            return [];
        }

        const byEntry = new Map(
            analytics.entries.map((entry) => [entry.entry_id, entry]),
        );

        return entries
            .map((entry) => ({
                entry,
                analytics: byEntry.get(entry.id) ?? {
                    entry_id: entry.id,
                    scans: 0,
                    unique_visitors: 0,
                    current_votes: 0,
                },
            }))
            .sort((left, right) => {
                return (
                    right.analytics.current_votes -
                        left.analytics.current_votes ||
                    right.analytics.unique_visitors -
                        left.analytics.unique_visitors ||
                    right.analytics.scans - left.analytics.scans ||
                    left.entry.number - right.entry.number
                );
            });
    }

    function voteShare(currentVotes: number): string {
        if (analytics === null || analytics.current_votes === 0) {
            return "0%";
        }

        const percentage = (currentVotes / analytics.current_votes) * 100;

        return `${Math.round(percentage)}%`;
    }

    function entryConversionRate(entryAnalytics: EntryAnalytics): string {
        if (entryAnalytics.unique_visitors === 0) {
            return entryAnalytics.current_votes === 0 ? "0%" : "—";
        }

        const percentage =
            (entryAnalytics.current_votes / entryAnalytics.unique_visitors) *
            100;

        return `${Math.round(percentage)}%`;
    }

    function activityKindLabel(kind: string): string {
        switch (kind) {
            case "scan":
                return "Scan / open";

            case "vote":
                return "Vote recorded";

            case "vote_change":
                return "Vote changed";

            default:
                return kind;
        }
    }

    function activityEntryLabel(entryId: number): string {
        const entry = entries.find((candidate) => candidate.id === entryId);

        if (entry === undefined) {
            return `Entry ${entryId}`;
        }

        return `#${entry.number} ${entry.name}`;
    }

    function signalLabel(code: string): string {
        switch (code) {
            case "high_scan_repeaters":
                return "Repeated scanning";

            case "frequent_vote_changers":
                return "Frequent vote changes";

            default:
                return code;
        }
    }

    function signalDescription(code: string): string {
        switch (code) {
            case "high_scan_repeaters":
                return "One or more browser identities opened entries unusually often.";

            case "frequent_vote_changers":
                return "One or more browser identities changed their vote repeatedly.";

            default:
                return "This activity may be worth reviewing.";
        }
    }

    function exportAnalyticsCsv() {
        if (event === null || analytics === null) {
            return;
        }

        const rows: Array<Array<string | number>> = [
            [
                "rank",
                "entry_number",
                "entry_name",
                "current_votes",
                "vote_share",
                "scans",
                "unique_visitors",
                "conversion",
            ],
        ];

        rankedEntryAnalytics().forEach((item, index) => {
            rows.push([
                index + 1,
                item.entry.number,
                item.entry.name,
                item.analytics.current_votes,
                voteShare(item.analytics.current_votes),
                item.analytics.scans,
                item.analytics.unique_visitors,
                entryConversionRate(item.analytics),
            ]);
        });

        const csv = rows.map((row) => row.map(csvCell).join(",")).join("\r\n");

        const blob = new Blob(["\uFEFF", csv], {
            type: "text/csv;charset=utf-8",
        });

        const url = URL.createObjectURL(blob);
        const link = document.createElement("a");

        link.href = url;
        link.download = `crumbvote-${event.slug}-analytics.csv`;

        document.body.appendChild(link);
        link.click();
        link.remove();

        window.setTimeout(() => {
            URL.revokeObjectURL(url);
        }, 0);
    }

    function csvCell(value: string | number): string {
        let text = String(value);

        if (/^[=+\-@]/.test(text)) {
            text = `'${text}`;
        }

        return `"${text.replace(/"/g, '""')}"`;
    }
</script>

{#if loading}
    <section class="flex flex-1 items-center justify-center py-24">
        <div class="text-center">
            <div
                class="mx-auto h-9 w-9 animate-spin rounded-full border-2 border-white/10 border-t-violet-400"
            ></div>

            <div class="mt-5 font-medium">
                {translate($locale, "workspace.loading")}
            </div>

            <div class="mt-2 text-sm text-slate-500">
                {translate($locale, "workspace.loadingDescription")}
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

            <h1 class="mt-5 text-2xl font-semibold">
                {translate($locale, "workspace.loadErrorTitle")}
            </h1>

            <p class="mt-3 leading-7 text-slate-400">
                {errorMessage}
            </p>

            <a
                href="/admin"
                class="mt-6 inline-flex rounded-xl bg-white px-5 py-3 text-sm font-semibold text-slate-950 transition hover:bg-slate-200"
            >
                {translate($locale, "workspace.backToEvents")}
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
                ← {translate($locale, "workspace.backToEvents")}
            </a>

            <div class="text-xs text-slate-600">
                {translate($locale, "workspace.eventNumber", {
                    number: event.id,
                })}
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

                                {translate($locale, "admin.statusOpen")}
                            </span>
                        {:else if event.status === "closed"}
                            <span
                                class="inline-flex items-center gap-2 rounded-full border border-slate-400/15 bg-slate-400/10 px-3 py-1.5 text-xs font-medium text-slate-300"
                            >
                                <span
                                    class="h-1.5 w-1.5 rounded-full bg-slate-400"
                                ></span>

                                {translate($locale, "admin.statusClosed")}
                            </span>
                        {:else}
                            <span
                                class="inline-flex items-center gap-2 rounded-full border border-violet-400/15 bg-violet-400/10 px-3 py-1.5 text-xs font-medium text-violet-300"
                            >
                                <span
                                    class="h-1.5 w-1.5 rounded-full bg-violet-400"
                                ></span>

                                {translate($locale, "admin.statusDraft")}
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
                        {event.description ??
                            translate($locale, "admin.noDescription")}
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
                            {translate($locale, "workspace.openVoting")}
                        </button>
                    {:else if event.status === "open"}
                        <button
                            type="button"
                            disabled={busy}
                            onclick={() => void handleStatusChange("closed")}
                            class="rounded-xl bg-amber-300 px-5 py-3 text-sm font-semibold text-amber-950 transition hover:bg-amber-200 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {translate($locale, "workspace.closeVoting")}
                        </button>
                    {:else}
                        <button
                            type="button"
                            disabled={busy}
                            onclick={() => void handleStatusChange("open")}
                            class="rounded-xl border border-emerald-400/20 bg-emerald-400/10 px-5 py-3 text-sm font-semibold text-emerald-300 transition hover:bg-emerald-400/15 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {translate($locale, "workspace.reopenVoting")}
                        </button>
                    {/if}
                </div>
            </div>
        </div>

        {#if analytics !== null}
            <section data-testid="analytics-summary" class="mt-6">
                <div class="mb-4 flex items-end justify-between gap-4">
                    <div>
                        <h2 class="text-lg font-semibold">Analytics</h2>

                        <p class="mt-1 text-sm text-slate-500">
                            Live activity collected for this event.
                        </p>
                    </div>
                </div>

                <div class="grid gap-3 sm:grid-cols-2 xl:grid-cols-5">
                    <article
                        class="rounded-2xl border border-white/10 bg-white/[0.025] p-5"
                    >
                        <div
                            class="text-xs font-medium uppercase tracking-wider text-slate-600"
                        >
                            Scans / opens
                        </div>

                        <div class="mt-3 text-3xl font-semibold">
                            {analytics.total_scans}
                        </div>
                    </article>

                    <article
                        class="rounded-2xl border border-white/10 bg-white/[0.025] p-5"
                    >
                        <div
                            class="text-xs font-medium uppercase tracking-wider text-slate-600"
                        >
                            Unique visitors
                        </div>

                        <div class="mt-3 text-3xl font-semibold">
                            {analytics.unique_visitors}
                        </div>
                    </article>

                    <article
                        class="rounded-2xl border border-white/10 bg-white/[0.025] p-5"
                    >
                        <div
                            class="text-xs font-medium uppercase tracking-wider text-slate-600"
                        >
                            Current votes
                        </div>

                        <div class="mt-3 text-3xl font-semibold">
                            {analytics.current_votes}
                        </div>
                    </article>

                    <article
                        class="rounded-2xl border border-white/10 bg-white/[0.025] p-5"
                    >
                        <div
                            class="text-xs font-medium uppercase tracking-wider text-slate-600"
                        >
                            Conversion
                        </div>

                        <div class="mt-3 text-3xl font-semibold">
                            {conversionRate()}
                        </div>
                    </article>

                    <article
                        class="rounded-2xl border border-white/10 bg-white/[0.025] p-5"
                    >
                        <div
                            class="text-xs font-medium uppercase tracking-wider text-slate-600"
                        >
                            Vote changes
                        </div>

                        <div class="mt-3 text-3xl font-semibold">
                            {analytics.vote_changes}
                        </div>
                    </article>
                </div>
            </section>
        {/if}

        <div class="mt-6 grid gap-6 xl:grid-cols-[minmax(0,1fr)_22rem]">
            <form
                onsubmit={handleSave}
                class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6 sm:p-8"
            >
                <div>
                    <h2 class="text-xl font-semibold">
                        {translate($locale, "workspace.eventSettings")}
                    </h2>

                    <p class="mt-1 text-sm text-slate-500">
                        {translate(
                            $locale,
                            "workspace.eventSettingsDescription",
                        )}
                    </p>
                </div>

                <div class="mt-7 space-y-6">
                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            {translate($locale, "admin.eventTitle")}
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
                            {translate($locale, "admin.description")}
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
                            {translate($locale, "admin.eventUrl")}
                        </span>

                        <div
                            class="mt-2 rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 font-mono text-sm text-slate-400"
                        >
                            /e/{event.slug}
                        </div>

                        <p class="mt-2 text-xs leading-5 text-slate-600">
                            {translate($locale, "workspace.eventUrlPermanent")}
                        </p>
                    </div>

                    <label
                        class="flex cursor-pointer items-start justify-between gap-5 rounded-2xl border border-white/10 bg-slate-950/40 p-5"
                    >
                        <div>
                            <div class="font-medium text-slate-200">
                                {translate($locale, "workspace.publicResults")}
                            </div>

                            <div
                                class="mt-1 max-w-md text-sm leading-6 text-slate-500"
                            >
                                {translate(
                                    $locale,
                                    "workspace.publicResultsDescription",
                                )}
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
                            {busy
                                ? translate($locale, "workspace.saving")
                                : translate($locale, "workspace.saveChanges")}
                        </button>
                    </div>
                </div>
            </form>

            <div class="space-y-6">
                <article
                    class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6"
                >
                    <div class="text-sm text-slate-500">
                        {translate($locale, "workspace.votingStatus")}
                    </div>

                    <div class="mt-3 text-2xl font-semibold">
                        {statusLabel(event.status)}
                    </div>

                    <p class="mt-2 text-sm leading-6 text-slate-500">
                        {#if event.status === "draft"}
                            {translate(
                                $locale,
                                "workspace.statusDraftDescription",
                            )}
                        {:else if event.status === "open"}
                            {translate(
                                $locale,
                                "workspace.statusOpenDescription",
                            )}
                        {:else}
                            {translate(
                                $locale,
                                "workspace.statusClosedDescription",
                            )}
                        {/if}
                    </p>
                </article>

                <article
                    class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6"
                >
                    <div class="text-sm text-slate-500">
                        {translate($locale, "workspace.entries")}
                    </div>

                    <div class="mt-3 text-3xl font-semibold">
                        {entries.length}
                    </div>

                    <p class="mt-2 text-sm leading-6 text-slate-500">
                        {#if event.status === "draft"}
                            {translate(
                                $locale,
                                "workspace.entriesDraftDescription",
                            )}
                        {:else}
                            {translate(
                                $locale,
                                "workspace.entriesLockedDescription",
                            )}
                        {/if}
                    </p>

                    <button
                        type="button"
                        disabled={event.status !== "draft"}
                        onclick={openCreateEntry}
                        class="mt-5 w-full rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-4 py-2.5 text-sm font-semibold text-white transition hover:brightness-110 disabled:cursor-not-allowed disabled:bg-none disabled:bg-white/[0.03] disabled:text-slate-600"
                    >
                        + {translate($locale, "workspace.addEntry")}
                    </button>
                </article>

                <article
                    class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6"
                >
                    <div class="text-sm text-slate-500">
                        {translate($locale, "workspace.lastUpdated")}
                    </div>

                    <div class="mt-3 text-sm font-medium text-slate-300">
                        {new Date(event.updated_at * 1000).toLocaleString(
                            $locale === "hu"
                                ? "hu-HU"
                                : $locale === "de"
                                  ? "de-DE"
                                  : "en-US",
                        )}
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
                    <h2 class="text-xl font-semibold">
                        {translate($locale, "workspace.entries")}
                    </h2>

                    <p class="mt-1 text-sm text-slate-500">
                        {translate(
                            $locale,
                            "workspace.entriesSectionDescription",
                        )}
                    </p>
                </div>

                <button
                    type="button"
                    disabled={event.status !== "draft"}
                    onclick={openCreateEntry}
                    class="w-fit rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm font-medium text-slate-300 transition hover:border-white/20 hover:bg-white/10 hover:text-white disabled:cursor-not-allowed disabled:opacity-40"
                >
                    + {translate($locale, "workspace.addEntry")}
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

                    <h3 class="mt-4 font-semibold">
                        {translate($locale, "workspace.noEntriesTitle")}
                    </h3>

                    <p
                        class="mx-auto mt-2 max-w-md text-sm leading-6 text-slate-500"
                    >
                        {translate($locale, "workspace.noEntriesDescription")}
                    </p>
                </div>
            {:else}
                <div class="mt-6 grid gap-4 lg:grid-cols-2">
                    {#each entries as entry (entry.id)}
                        <article
                            class="overflow-hidden rounded-3xl border border-white/10 bg-slate-950/40"
                        >
                            <div
                                class="relative aspect-[16/9] overflow-hidden bg-black/20"
                            >
                                {#if entry.image_url !== null}
                                    <img
                                        src={entry.image_url}
                                        alt={entry.name}
                                        class="h-full w-full object-cover"
                                    />
                                {:else}
                                    <div
                                        class="flex h-full items-center justify-center"
                                    >
                                        <div class="text-center">
                                            <div
                                                class="mx-auto flex h-14 w-14 items-center justify-center rounded-2xl bg-white/5 text-2xl text-slate-600"
                                            >
                                                ◇
                                            </div>

                                            <div
                                                class="mt-3 text-xs text-slate-600"
                                            >
                                                {translate(
                                                    $locale,
                                                    "common.noImage",
                                                )}
                                            </div>
                                        </div>
                                    </div>
                                {/if}

                                <div
                                    class="absolute left-4 top-4 rounded-xl border border-white/10 bg-slate-950/80 px-3 py-2 font-mono text-sm font-semibold text-white backdrop-blur"
                                >
                                    #{entry.number}
                                </div>
                            </div>

                            <div class="p-5">
                                <div
                                    class="flex items-start justify-between gap-4"
                                >
                                    <div class="min-w-0 flex-1">
                                        <h3
                                            class="truncate font-semibold text-white"
                                        >
                                            {entry.name}
                                        </h3>

                                        <p
                                            class="mt-2 min-h-10 text-sm leading-5 text-slate-500"
                                        >
                                            {entry.description ??
                                                translate(
                                                    $locale,
                                                    "workspace.noEntryDescription",
                                                )}
                                        </p>
                                    </div>

                                    <button
                                        type="button"
                                        disabled={event.status !== "draft"}
                                        onclick={() => openEditEntry(entry)}
                                        class="shrink-0 rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-xs font-medium text-slate-300 transition hover:border-violet-400/30 hover:bg-violet-400/10 hover:text-white disabled:cursor-not-allowed disabled:opacity-35"
                                    >
                                        {translate($locale, "workspace.edit")}
                                    </button>
                                </div>

                                <div class="mt-5 border-t border-white/5 pt-4">
                                    <div
                                        class="text-xs font-medium text-slate-500"
                                    >
                                        {translate(
                                            $locale,
                                            "workspace.votingLink",
                                        )}
                                    </div>

                                    <div class="mt-2 flex items-center gap-2">
                                        <div
                                            class="min-w-0 flex-1 truncate rounded-xl bg-black/20 px-3 py-2.5 font-mono text-xs text-slate-500"
                                        >
                                            {entryUrl(entry)}
                                        </div>

                                        <button
                                            type="button"
                                            onclick={() =>
                                                void copyEntryUrl(entry)}
                                            class="shrink-0 rounded-xl border border-white/10 bg-white/5 px-3 py-2.5 text-xs font-medium text-slate-300 transition hover:bg-white/10 hover:text-white"
                                        >
                                            {copiedEntryId === entry.id
                                                ? translate(
                                                      $locale,
                                                      "workspace.copied",
                                                  )
                                                : translate(
                                                      $locale,
                                                      "workspace.copy",
                                                  )}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </article>
                    {/each}
                </div>
            {/if}
        </div>

        {#if analytics !== null}
            <section
                aria-labelledby="analyse-title"
                class="mt-6 rounded-[2rem] border border-white/10 bg-white/[0.025] p-6 sm:p-8"
            >
                <div
                    class="flex flex-col gap-4 sm:flex-row sm:items-end sm:justify-between"
                >
                    <div>
                        <div
                            class="mb-3 inline-flex rounded-full border border-fuchsia-400/15 bg-fuchsia-400/10 px-3 py-1 text-xs font-medium text-fuchsia-300"
                        >
                            Event intelligence
                        </div>

                        <h2
                            id="analyse-title"
                            class="text-2xl font-semibold tracking-tight"
                        >
                            Analyse
                        </h2>

                        <p
                            class="mt-2 max-w-2xl text-sm leading-6 text-slate-500"
                        >
                            Entry performance, recent activity and lightweight
                            signals that may be worth reviewing.
                        </p>
                    </div>

                    <button
                        type="button"
                        onclick={exportAnalyticsCsv}
                        class="w-fit shrink-0 rounded-xl border border-white/10 bg-white/5 px-4 py-2.5 text-sm font-medium text-slate-300 transition hover:bg-white/10 hover:text-white"
                    >
                        Export CSV
                    </button>
                </div>

                <div
                    class="mt-7 grid gap-6 xl:grid-cols-[minmax(0,1.35fr)_minmax(20rem,0.65fr)]"
                >
                    <div>
                        <h3 class="text-lg font-semibold">Entry performance</h3>

                        <p class="mt-1 text-sm text-slate-500">
                            Ranked by current votes, then visitors and scans.
                        </p>

                        {#if entries.length === 0}
                            <div
                                class="mt-5 rounded-2xl border border-dashed border-white/10 px-5 py-10 text-center text-sm text-slate-600"
                            >
                                No entries to analyse yet.
                            </div>
                        {:else}
                            <div class="mt-5 space-y-3">
                                {#each rankedEntryAnalytics() as item, index (item.entry.id)}
                                    <article
                                        data-testid={"analytics-entry-" +
                                            item.entry.id}
                                        class="rounded-2xl border border-white/10 bg-slate-950/40 p-5"
                                    >
                                        <div
                                            class="flex items-start justify-between gap-4"
                                        >
                                            <div
                                                class="flex min-w-0 items-center gap-3"
                                            >
                                                <div
                                                    class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-violet-400/10 font-mono text-sm font-semibold text-violet-300"
                                                >
                                                    {index + 1}
                                                </div>

                                                <div class="min-w-0">
                                                    <div
                                                        class="truncate font-semibold"
                                                    >
                                                        #{item.entry.number}
                                                        {item.entry.name}
                                                    </div>

                                                    <div
                                                        class="mt-1 text-xs text-slate-600"
                                                    >
                                                        Entry #{item.entry.id}
                                                    </div>
                                                </div>
                                            </div>
                                        </div>

                                        <div
                                            class="mt-5 grid grid-cols-2 gap-3 sm:grid-cols-5"
                                        >
                                            <div
                                                class="rounded-xl bg-black/20 p-3"
                                            >
                                                <div
                                                    class="text-xl font-semibold"
                                                >
                                                    {item.analytics
                                                        .current_votes}
                                                </div>

                                                <div
                                                    class="mt-1 text-xs text-slate-600"
                                                >
                                                    Current votes
                                                </div>
                                            </div>

                                            <div
                                                class="rounded-xl bg-black/20 p-3"
                                            >
                                                <div
                                                    class="text-xl font-semibold"
                                                >
                                                    {voteShare(
                                                        item.analytics
                                                            .current_votes,
                                                    )}
                                                </div>

                                                <div
                                                    class="mt-1 text-xs text-slate-600"
                                                >
                                                    Vote share
                                                </div>
                                            </div>

                                            <div
                                                class="rounded-xl bg-black/20 p-3"
                                            >
                                                <div
                                                    class="text-xl font-semibold"
                                                >
                                                    {item.analytics.scans}
                                                </div>

                                                <div
                                                    class="mt-1 text-xs text-slate-600"
                                                >
                                                    Scans
                                                </div>
                                            </div>

                                            <div
                                                class="rounded-xl bg-black/20 p-3"
                                            >
                                                <div
                                                    class="text-xl font-semibold"
                                                >
                                                    {item.analytics
                                                        .unique_visitors}
                                                </div>

                                                <div
                                                    class="mt-1 text-xs text-slate-600"
                                                >
                                                    Visitors
                                                </div>
                                            </div>

                                            <div
                                                class="col-span-2 rounded-xl bg-black/20 p-3 sm:col-span-1"
                                            >
                                                <div
                                                    class="text-xl font-semibold"
                                                >
                                                    {entryConversionRate(
                                                        item.analytics,
                                                    )}
                                                </div>

                                                <div
                                                    class="mt-1 text-xs text-slate-600"
                                                >
                                                    Conversion
                                                </div>
                                            </div>
                                        </div>
                                    </article>
                                {/each}
                            </div>
                        {/if}
                    </div>

                    <div class="space-y-6">
                        <article
                            class="rounded-2xl border border-white/10 bg-slate-950/40 p-5"
                        >
                            <div
                                class="flex items-center justify-between gap-3"
                            >
                                <div>
                                    <h3 class="font-semibold">
                                        Review signals
                                    </h3>

                                    <p
                                        class="mt-1 text-xs leading-5 text-slate-600"
                                    >
                                        Heuristics only — not proof of abuse.
                                    </p>
                                </div>

                                <div
                                    class="rounded-xl bg-white/5 px-3 py-2 font-mono text-sm text-slate-400"
                                >
                                    {analytics.signals.length}
                                </div>
                            </div>

                            {#if analytics.signals.length === 0}
                                <div
                                    class="mt-5 rounded-xl border border-emerald-400/10 bg-emerald-400/[0.06] px-4 py-3"
                                >
                                    <div
                                        class="text-sm font-medium text-emerald-300"
                                    >
                                        No unusual activity signals.
                                    </div>

                                    <p
                                        class="mt-1 text-xs leading-5 text-slate-500"
                                    >
                                        Nothing currently crosses the review
                                        thresholds.
                                    </p>
                                </div>
                            {:else}
                                <div class="mt-5 space-y-3">
                                    {#each analytics.signals as signal}
                                        <div
                                            class="rounded-xl border border-amber-400/15 bg-amber-400/[0.07] p-4"
                                        >
                                            <div
                                                class="flex items-start justify-between gap-3"
                                            >
                                                <div
                                                    class="font-medium text-amber-200"
                                                >
                                                    {signalLabel(signal.code)}
                                                </div>

                                                <div
                                                    class="shrink-0 font-mono text-xs text-amber-300"
                                                >
                                                    {signal.affected_visitors}
                                                    {signal.affected_visitors ===
                                                    1
                                                        ? "visitor"
                                                        : "visitors"}
                                                </div>
                                            </div>

                                            <p
                                                class="mt-2 text-xs leading-5 text-slate-500"
                                            >
                                                {signalDescription(signal.code)}
                                            </p>
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </article>

                        <article
                            class="rounded-2xl border border-white/10 bg-slate-950/40 p-5"
                        >
                            <h3 class="font-semibold">Recent activity</h3>

                            <p class="mt-1 text-xs leading-5 text-slate-600">
                                Latest scans and voting actions.
                            </p>

                            {#if analytics.recent_activity.length === 0}
                                <div
                                    class="mt-5 rounded-xl border border-dashed border-white/10 px-4 py-8 text-center text-sm text-slate-600"
                                >
                                    No public activity yet.
                                </div>
                            {:else}
                                <div class="mt-5 space-y-2">
                                    {#each analytics.recent_activity as activity}
                                        <div
                                            class="rounded-xl border border-white/5 bg-black/20 p-3"
                                        >
                                            <div
                                                class="flex items-start justify-between gap-3"
                                            >
                                                <div class="min-w-0">
                                                    <div
                                                        class="text-sm font-medium text-slate-300"
                                                    >
                                                        {activityKindLabel(
                                                            activity.kind,
                                                        )}
                                                    </div>

                                                    <div
                                                        class="mt-1 truncate text-xs text-slate-600"
                                                    >
                                                        {activityEntryLabel(
                                                            activity.entry_id,
                                                        )}
                                                    </div>
                                                </div>

                                                <time
                                                    class="shrink-0 text-right text-[0.7rem] text-slate-700"
                                                >
                                                    {new Date(
                                                        activity.created_at *
                                                            1000,
                                                    ).toLocaleString()}
                                                </time>
                                            </div>
                                        </div>
                                    {/each}
                                </div>
                            {/if}
                        </article>
                    </div>
                </div>
            </section>
        {/if}
    </section>

    {#if createEntryOpen}
        <div
            class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6"
        >
            <button
                type="button"
                aria-label={translate($locale, "workspace.closeAddEntryDialog")}
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
                            {translate($locale, "workspace.newContestant")}
                        </div>

                        <h2
                            id="create-entry-title"
                            class="text-2xl font-semibold tracking-tight"
                        >
                            {translate($locale, "workspace.addEntry")}
                        </h2>

                        <p class="mt-2 leading-6 text-slate-400">
                            {translate(
                                $locale,
                                "workspace.addEntryDescription",
                            )}
                        </p>
                    </div>

                    <button
                        type="button"
                        aria-label={translate($locale, "admin.close")}
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
                            {translate($locale, "workspace.entryName")}
                        </span>

                        <input
                            bind:value={entryName}
                            type="text"
                            maxlength="120"
                            placeholder={translate(
                                $locale,
                                "workspace.entryNamePlaceholder",
                            )}
                            required
                            class="mt-2 w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition placeholder:text-slate-700 focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        />
                    </label>

                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            {translate($locale, "admin.description")}
                        </span>

                        <textarea
                            bind:value={entryDescription}
                            maxlength="2000"
                            rows="4"
                            placeholder={translate(
                                $locale,
                                "workspace.optionalDescriptionPlaceholder",
                            )}
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
                            {translate($locale, "admin.cancel")}
                        </button>

                        <button
                            type="submit"
                            disabled={entryBusy}
                            class="rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-5 py-3 text-sm font-semibold text-white transition hover:brightness-110 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {entryBusy
                                ? translate($locale, "workspace.adding")
                                : translate($locale, "workspace.addEntry")}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    {/if}

    {#if editEntryOpen && editingEntry !== null}
        <div
            class="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6"
        >
            <button
                type="button"
                aria-label={translate(
                    $locale,
                    "workspace.closeEditEntryDialog",
                )}
                disabled={editEntryBusy}
                onclick={closeEditEntry}
                class="absolute inset-0 bg-slate-950/80 backdrop-blur-sm"
            ></button>

            <div
                role="dialog"
                aria-modal="true"
                aria-labelledby="edit-entry-title"
                class="relative z-10 max-h-[calc(100vh-2rem)] w-full max-w-2xl overflow-y-auto rounded-[2rem] border border-white/10 bg-slate-900 p-6 shadow-2xl shadow-black/50 sm:p-8"
            >
                <div class="flex items-start justify-between gap-4">
                    <div>
                        <div
                            class="mb-3 inline-flex rounded-full border border-violet-400/20 bg-violet-400/10 px-3 py-1 text-xs font-medium text-violet-300"
                        >
                            {translate($locale, "entry.number", {
                                number: editingEntry.number,
                            })}
                        </div>

                        <h2
                            id="edit-entry-title"
                            class="text-2xl font-semibold tracking-tight"
                        >
                            {translate($locale, "workspace.editEntry")}
                        </h2>

                        <p class="mt-2 leading-6 text-slate-400">
                            {translate(
                                $locale,
                                "workspace.editEntryDescription",
                            )}
                        </p>
                    </div>

                    <button
                        type="button"
                        aria-label={translate($locale, "admin.close")}
                        disabled={editEntryBusy}
                        onclick={closeEditEntry}
                        class="rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-slate-400 transition hover:bg-white/10 hover:text-white disabled:opacity-50"
                    >
                        ✕
                    </button>
                </div>

                <form class="mt-7 space-y-5" onsubmit={handleEditEntry}>
                    <div>
                        <span class="text-sm font-medium text-slate-300">
                            {translate($locale, "workspace.entryImage")}
                        </span>

                        <div
                            class="mt-2 overflow-hidden rounded-2xl border border-white/10 bg-slate-950/50"
                        >
                            <div class="aspect-[16/9] bg-black/20">
                                {#if editEntryImagePreviewUrl !== null}
                                    <img
                                        src={editEntryImagePreviewUrl}
                                        alt={translate(
                                            $locale,
                                            "workspace.selectedPreview",
                                        )}
                                        class="h-full w-full object-cover"
                                    />
                                {:else if editingEntry.image_url !== null}
                                    <img
                                        src={editingEntry.image_url}
                                        alt={editingEntry.name}
                                        class="h-full w-full object-cover"
                                    />
                                {:else}
                                    <div
                                        class="flex h-full items-center justify-center text-sm text-slate-600"
                                    >
                                        {translate(
                                            $locale,
                                            "workspace.noImageYet",
                                        )}
                                    </div>
                                {/if}
                            </div>

                            <label
                                class="flex cursor-pointer items-center justify-between gap-4 border-t border-white/10 px-4 py-3 transition hover:bg-white/[0.03]"
                            >
                                <div>
                                    <div
                                        class="text-sm font-medium text-slate-300"
                                    >
                                        {editingEntry.image_url !== null
                                            ? translate(
                                                  $locale,
                                                  "workspace.replaceImage",
                                              )
                                            : translate(
                                                  $locale,
                                                  "workspace.chooseImage",
                                              )}
                                    </div>

                                    <div class="mt-1 text-xs text-slate-600">
                                        {translate(
                                            $locale,
                                            "workspace.imageRequirements",
                                        )}
                                    </div>
                                </div>

                                <span
                                    class="rounded-xl border border-white/10 bg-white/5 px-3 py-2 text-xs font-medium text-slate-300"
                                >
                                    {translate($locale, "workspace.browse")}
                                </span>

                                <input
                                    type="file"
                                    accept="image/jpeg,image/png,image/webp"
                                    disabled={editEntryBusy}
                                    onchange={handleEditEntryImageChange}
                                    class="sr-only"
                                />
                            </label>
                        </div>

                        {#if editEntryImageFile !== null}
                            <div class="mt-2 text-xs text-slate-500">
                                {translate($locale, "workspace.selectedFile", {
                                    name: editEntryImageFile.name,
                                })}
                            </div>
                        {/if}
                    </div>

                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            {translate($locale, "workspace.entryName")}
                        </span>

                        <input
                            bind:value={editEntryName}
                            type="text"
                            maxlength="120"
                            required
                            class="mt-2 w-full rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        />
                    </label>

                    <label class="block">
                        <span class="text-sm font-medium text-slate-300">
                            {translate($locale, "admin.description")}
                        </span>

                        <textarea
                            bind:value={editEntryDescription}
                            maxlength="2000"
                            rows="4"
                            class="mt-2 w-full resize-none rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3.5 text-white outline-none transition focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        ></textarea>

                        <div class="mt-2 text-right text-xs text-slate-600">
                            {editEntryDescription.length} / 2000
                        </div>
                    </label>

                    <div>
                        <span class="text-sm font-medium text-slate-300">
                            {translate(
                                $locale,
                                "workspace.permanentVotingLink",
                            )}
                        </span>

                        <div
                            class="mt-2 rounded-xl border border-white/10 bg-slate-950/60 px-4 py-3 font-mono text-xs text-slate-500"
                        >
                            {entryUrl(editingEntry)}
                        </div>
                    </div>

                    {#if editEntryErrorMessage}
                        <div
                            class="rounded-xl border border-red-400/15 bg-red-400/10 px-4 py-3 text-sm leading-6 text-red-200"
                        >
                            {editEntryErrorMessage}
                        </div>
                    {/if}

                    <div
                        class="flex flex-col-reverse gap-3 pt-2 sm:flex-row sm:justify-end"
                    >
                        <button
                            type="button"
                            disabled={editEntryBusy}
                            onclick={closeEditEntry}
                            class="rounded-xl border border-white/10 bg-white/5 px-5 py-3 text-sm font-medium text-slate-300 transition hover:bg-white/10 hover:text-white disabled:opacity-50"
                        >
                            {translate($locale, "admin.cancel")}
                        </button>

                        <button
                            type="submit"
                            disabled={editEntryBusy}
                            class="rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-5 py-3 text-sm font-semibold text-white transition hover:brightness-110 disabled:cursor-not-allowed disabled:opacity-50"
                        >
                            {editEntryBusy
                                ? translate($locale, "workspace.saving")
                                : translate($locale, "workspace.saveEntry")}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    {/if}
{/if}
