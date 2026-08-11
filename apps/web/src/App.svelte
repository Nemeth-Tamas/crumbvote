<script lang="ts">
  import AdminPage from "./pages/AdminPage.svelte";
  import LandingPage from "./pages/LandingPage.svelte";

  const pathname = window.location.pathname.replace(/\/+$/, "") || "/";

  const adminEventMatch = pathname.match(/^\/admin\/events\/(\d+)$/);

  const adminEventId =
    adminEventMatch !== null ? Number.parseInt(adminEventMatch[1], 10) : null;

  const isAdminRoute = pathname === "/admin" || adminEventId !== null;
</script>

{#if isAdminRoute}
  <AdminPage initialEventId={adminEventId} />
{:else}
  <LandingPage />
{/if}
