<script lang="ts">
  import AdminPage from "./pages/AdminPage.svelte";
  import LandingPage from "./pages/LandingPage.svelte";

  const pathname = window.location.pathname.replace(/\/+$/, "") || "/";
</script>

{#if pathname === "/admin"}
  <AdminPage />
{:else}
  <LandingPage />
{/if}
