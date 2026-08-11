<script lang="ts">
    const features = [
        {
            title: "Vote",
            description:
                "Fast, mobile-first voting from individual entry links.",
        },
        {
            title: "Manage",
            description:
                "Create events and manage entries from the admin dashboard.",
        },
        {
            title: "Analyse",
            description:
                "Watch turnout, results and suspicious activity in real time.",
        },
    ];
</script>

<svelte:head>
    <title>CrumbVote</title>
    <meta name="description" content="Simple, beautiful event voting." />
</svelte:head>

<main class="relative min-h-screen overflow-hidden bg-slate-950 text-white">
    <div
        class="pointer-events-none absolute left-1/2 top-[-18rem] h-[36rem] w-[36rem] -translate-x-1/2 rounded-full bg-violet-600/20 blur-[120px]"
    ></div>

    <div
        class="pointer-events-none absolute bottom-[-16rem] right-[-10rem] h-[32rem] w-[32rem] rounded-full bg-fuchsia-600/10 blur-[120px]"
    ></div>

    <div class="relative mx-auto max-w-6xl px-6 py-8 sm:px-10 lg:px-16">
        <nav class="flex items-center justify-between">
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

            <a
                href="/admin"
                class="rounded-xl border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300 transition hover:border-white/20 hover:bg-white/10 hover:text-white"
            >
                Admin
            </a>
        </nav>

        <div
            class="flex min-h-[calc(100vh-8rem)] flex-col justify-center py-16"
        >
            <div
                class="mb-6 inline-flex w-fit items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300 backdrop-blur"
            >
                <span class="h-2 w-2 rounded-full bg-emerald-400"></span>
                Simple event voting
            </div>

            <div class="max-w-3xl">
                <h1
                    class="text-5xl font-semibold tracking-tight text-white sm:text-6xl lg:text-7xl"
                >
                    Voting without
                    <span
                        class="bg-gradient-to-r from-violet-400 to-pink-400 bg-clip-text text-transparent"
                    >
                        the nonsense.
                    </span>
                </h1>

                <p
                    class="mt-6 max-w-2xl text-lg leading-8 text-slate-400 sm:text-xl"
                >
                    Simple event voting, beautiful participant pages and useful
                    analytics without forcing visitors to install an app.
                </p>
            </div>

            <div class="mt-12 grid gap-4 md:grid-cols-3">
                {#each features as feature}
                    <article
                        class="rounded-3xl border border-white/10 bg-white/[0.04] p-6 backdrop-blur transition hover:-translate-y-1 hover:border-white/15 hover:bg-white/[0.06]"
                    >
                        <h2 class="text-xl font-semibold">
                            {feature.title}
                        </h2>

                        <p class="mt-2 leading-7 text-slate-400">
                            {feature.description}
                        </p>
                    </article>
                {/each}
            </div>

            <div class="mt-12 flex items-center gap-3 text-sm text-slate-500">
                <span>CrumbVote</span>
                <span>•</span>
                <span>Development build</span>
            </div>
        </div>
    </div>
</main>
