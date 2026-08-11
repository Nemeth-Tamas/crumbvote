<script lang="ts">
    import LanguageSelector from "../components/LanguageSelector.svelte";
    import { locale, translate } from "../lib/i18n";

    let votingLink = "";
    let votingLinkInvalid = false;

    const features = [
        {
            title: "landing.featureVoteTitle",
            description: "landing.featureVoteDescription",
        },
        {
            title: "landing.featureManageTitle",
            description: "landing.featureManageDescription",
        },
        {
            title: "landing.featureAnalyseTitle",
            description: "landing.featureAnalyseDescription",
        },
    ] as const;

    const steps = [
        {
            title: "landing.stepScanTitle",
            description: "landing.stepScanDescription",
        },
        {
            title: "landing.stepReviewTitle",
            description: "landing.stepReviewDescription",
        },
        {
            title: "landing.stepVoteTitle",
            description: "landing.stepVoteDescription",
        },
    ] as const;

    function handleVotingLinkSubmit(submitEvent: SubmitEvent) {
        submitEvent.preventDefault();

        votingLinkInvalid = false;

        const path = publicCrumbVotePath(votingLink);

        if (path === null) {
            votingLinkInvalid = true;
            return;
        }

        window.location.assign(path);
    }

    function handleVotingLinkInput() {
        votingLinkInvalid = false;
    }

    function publicCrumbVotePath(value: string): string | null {
        let candidate = value.trim();

        if (candidate.length === 0) {
            return null;
        }

        if (candidate.startsWith("e/")) {
            candidate = `/${candidate}`;
        }

        let pathname: string;

        try {
            let parsed: URL;

            if (candidate.startsWith("/")) {
                parsed = new URL(candidate, `${window.location.origin}/`);
            } else if (candidate.includes("://")) {
                parsed = new URL(candidate);
            } else {
                parsed = new URL(`https://${candidate}`);
            }

            pathname = parsed.pathname.replace(/\/+$/, "");
        } catch {
            return null;
        }

        if (!/^\/e\/[a-z0-9-]+\/(?:\d+|results)$/.test(pathname)) {
            return null;
        }

        return pathname;
    }
</script>

<svelte:head>
    <title>CrumbVote</title>

    <meta
        name="description"
        content={translate($locale, "landing.metaDescription")}
    />
</svelte:head>

<main class="relative min-h-screen overflow-hidden bg-slate-950 text-white">
    <div
        class="pointer-events-none absolute left-1/2 top-[-18rem] h-[36rem] w-[36rem] -translate-x-1/2 rounded-full bg-violet-600/20 blur-[120px]"
    ></div>

    <div
        class="pointer-events-none absolute bottom-[-16rem] right-[-10rem] h-[32rem] w-[32rem] rounded-full bg-fuchsia-600/10 blur-[120px]"
    ></div>

    <div class="relative mx-auto max-w-6xl px-6 py-8 sm:px-10 lg:px-16">
        <nav class="flex items-center justify-between gap-4">
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

            <div class="flex items-center gap-2">
                <LanguageSelector />

                <a
                    href="/admin"
                    class="rounded-xl border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300 transition hover:border-white/20 hover:bg-white/10 hover:text-white"
                >
                    {translate($locale, "landing.admin")}
                </a>
            </div>
        </nav>

        <section
            class="grid min-h-[calc(100vh-8rem)] items-center gap-12 py-16 lg:grid-cols-[minmax(0,1.15fr)_minmax(22rem,0.85fr)]"
        >
            <div>
                <div
                    class="mb-6 inline-flex w-fit items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300 backdrop-blur"
                >
                    <span class="h-2 w-2 rounded-full bg-emerald-400"></span>

                    {translate($locale, "landing.badge")}
                </div>

                <h1
                    class="max-w-4xl text-5xl font-semibold tracking-tight text-white sm:text-6xl lg:text-7xl"
                >
                    {translate($locale, "landing.heroPrefix")}

                    <span
                        class="bg-gradient-to-r from-violet-400 to-pink-400 bg-clip-text text-transparent"
                    >
                        {translate($locale, "landing.heroAccent")}
                    </span>
                </h1>

                <p
                    class="mt-6 max-w-2xl text-lg leading-8 text-slate-400 sm:text-xl"
                >
                    {translate($locale, "landing.description")}
                </p>
            </div>

            <div
                class="rounded-[2rem] border border-white/10 bg-white/[0.045] p-6 shadow-2xl shadow-black/30 backdrop-blur-xl sm:p-7"
            >
                <div
                    class="inline-flex rounded-full border border-violet-400/15 bg-violet-400/10 px-3 py-1.5 text-xs font-medium text-violet-300"
                >
                    {translate($locale, "landing.linkEyebrow")}
                </div>

                <h2 class="mt-4 text-2xl font-semibold tracking-tight">
                    {translate($locale, "landing.linkTitle")}
                </h2>

                <p class="mt-2 text-sm leading-6 text-slate-400">
                    {translate($locale, "landing.linkDescription")}
                </p>

                <form class="mt-6" onsubmit={handleVotingLinkSubmit}>
                    <label
                        for="landing-voting-link"
                        class="text-sm font-medium text-slate-300"
                    >
                        {translate($locale, "landing.linkLabel")}
                    </label>

                    <div class="mt-2 flex flex-col gap-2 sm:flex-row">
                        <input
                            id="landing-voting-link"
                            data-testid="landing-link-input"
                            bind:value={votingLink}
                            oninput={handleVotingLinkInput}
                            type="text"
                            autocomplete="off"
                            spellcheck="false"
                            placeholder={translate(
                                $locale,
                                "landing.linkPlaceholder",
                            )}
                            aria-invalid={votingLinkInvalid}
                            class="min-w-0 flex-1 rounded-xl border border-white/10 bg-slate-950/70 px-4 py-3.5 font-mono text-sm text-white outline-none transition placeholder:text-slate-700 focus:border-violet-400/50 focus:ring-4 focus:ring-violet-400/10"
                        />

                        <button
                            data-testid="landing-open-link"
                            type="submit"
                            class="shrink-0 rounded-xl bg-gradient-to-r from-violet-500 to-fuchsia-500 px-5 py-3.5 text-sm font-semibold text-white shadow-lg shadow-violet-950/30 transition hover:brightness-110"
                        >
                            {translate($locale, "landing.openLink")}
                        </button>
                    </div>

                    {#if votingLinkInvalid}
                        <div
                            data-testid="landing-link-error"
                            class="mt-3 rounded-xl border border-red-400/15 bg-red-400/[0.08] px-4 py-3 text-sm text-red-200"
                        >
                            {translate($locale, "landing.invalidLink")}
                        </div>
                    {/if}
                </form>

                <div class="my-6 border-t border-white/10"></div>

                <div class="rounded-2xl bg-black/20 p-5">
                    <h3 class="font-semibold">
                        {translate($locale, "landing.organizerPrompt")}
                    </h3>

                    <p class="mt-2 text-sm leading-6 text-slate-500">
                        {translate($locale, "landing.organizerDescription")}
                    </p>

                    <a
                        href="/admin"
                        class="mt-4 inline-flex text-sm font-semibold text-violet-300 transition hover:text-violet-200"
                    >
                        {translate($locale, "landing.openAdmin")}
                    </a>
                </div>
            </div>
        </section>

        <section class="pb-16">
            <div
                class="rounded-[2rem] border border-white/10 bg-white/[0.025] p-6 sm:p-8"
            >
                <div class="max-w-2xl">
                    <h2 class="text-2xl font-semibold tracking-tight">
                        {translate($locale, "landing.howTitle")}
                    </h2>

                    <p class="mt-2 leading-7 text-slate-400">
                        {translate($locale, "landing.howDescription")}
                    </p>
                </div>

                <div class="mt-7 grid gap-4 md:grid-cols-3">
                    {#each steps as step}
                        <article
                            class="rounded-2xl border border-white/10 bg-slate-950/40 p-5"
                        >
                            <h3 class="font-semibold text-white">
                                {translate($locale, step.title)}
                            </h3>

                            <p class="mt-2 text-sm leading-6 text-slate-500">
                                {translate($locale, step.description)}
                            </p>
                        </article>
                    {/each}
                </div>
            </div>
        </section>

        <section class="pb-16">
            <div class="grid gap-4 md:grid-cols-3">
                {#each features as feature}
                    <article
                        class="rounded-3xl border border-white/10 bg-white/[0.04] p-6 backdrop-blur transition hover:-translate-y-1 hover:border-white/15 hover:bg-white/[0.06]"
                    >
                        <h2 class="text-xl font-semibold">
                            {translate($locale, feature.title)}
                        </h2>

                        <p class="mt-2 leading-7 text-slate-400">
                            {translate($locale, feature.description)}
                        </p>
                    </article>
                {/each}
            </div>

            <div class="mt-12 flex items-center gap-3 text-sm text-slate-500">
                <span>CrumbVote</span>
                <span>•</span>

                <span>
                    {translate($locale, "landing.developmentBuild")}
                </span>
            </div>
        </section>
    </div>
</main>
