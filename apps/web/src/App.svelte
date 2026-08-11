<script lang="ts">
  const features = [
    {
      title: "Vote",
      description: "Fast, mobile-first voting from individual entry links.",
    },
    {
      title: "Manage",
      description: "Create events and manage entries from the admin dashboard.",
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

<main class="min-h-screen bg-slate-950 px-6 py-12 text-white sm:px-10 lg:px-16">
  <div
    class="mx-auto flex min-h-[calc(100vh-6rem)] max-w-6xl flex-col justify-center"
  >
    <div
      class="mb-6 inline-flex w-fit items-center gap-2 rounded-full border border-white/10 bg-white/5 px-4 py-2 text-sm text-slate-300"
    >
      <span class="h-2 w-2 rounded-full bg-emerald-400"></span>
      CrumbVote
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

      <p class="mt-6 max-w-2xl text-lg leading-8 text-slate-400 sm:text-xl">
        Simple event voting, beautiful participant pages and useful analytics
        without forcing visitors to install an app.
      </p>
    </div>

    <div class="mt-12 grid gap-4 md:grid-cols-3">
      {#each features as feature}
        <article
          class="rounded-3xl border border-white/10 bg-white/[0.04] p-6 backdrop-blur"
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
</main>
