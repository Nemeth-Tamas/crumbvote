<script lang="ts">
  import AdminPage from "./pages/AdminPage.svelte";
  import LandingPage from "./pages/LandingPage.svelte";
  import PublicEntryPage from "./pages/PublicEntryPage.svelte";
  import PublicResultsPage from "./pages/PublicResultsPage.svelte";

  const pathname = window.location.pathname.replace(/\/+$/, "") || "/";

  const adminEventMatch = pathname.match(/^\/admin\/events\/(\d+)$/);

  const adminEventId =
    adminEventMatch !== null ? Number.parseInt(adminEventMatch[1], 10) : null;

  const publicResultsMatch = pathname.match(/^\/e\/([a-z0-9-]+)\/results$/);

  const publicResultsSlug = publicResultsMatch?.[1] ?? null;

  const publicEntryMatch = pathname.match(/^\/e\/([a-z0-9-]+)\/(\d+)$/);

  const publicEventSlug = publicEntryMatch?.[1] ?? null;

  const publicEntryId =
    publicEntryMatch !== null ? Number.parseInt(publicEntryMatch[2], 10) : null;

  const isAdminRoute = pathname === "/admin" || adminEventId !== null;
</script>

{#if isAdminRoute}
  <AdminPage initialEventId={adminEventId} />
{:else if publicResultsSlug !== null}
  <PublicResultsPage eventSlug={publicResultsSlug} />
{:else if publicEventSlug !== null && publicEntryId !== null}
  <PublicEntryPage eventSlug={publicEventSlug} entryId={publicEntryId} />
{:else}
  <LandingPage />
{/if}
