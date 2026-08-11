<script lang="ts">
    import LanguageSelector from "../components/LanguageSelector.svelte";
    import { locale, translate } from "../lib/i18n";

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

        <div
            class="flex min-h-[calc(100vh-8rem)] flex-col justify-center py-16"
        >
            <div
                class="mb-6 inline-flex w-fit items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300 backdrop-blur"
            >
                <span class="h-2 w-2 rounded-full bg-emerald-400"></span>

                {translate($locale, "landing.badge")}
            </div>

            <div class="max-w-3xl">
                <h1
                    class="text-5xl font-semibold tracking-tight text-white sm:text-6xl lg:text-7xl"
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

            <div class="mt-12 grid gap-4 md:grid-cols-3">
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
        </div>
    </div>
</main>
