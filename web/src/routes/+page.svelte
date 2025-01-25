<script lang="ts">
  import MainGame from "$lib/components/MainGame.svelte";
  import { loadData } from "$lib/utils/data";
</script>

<svelte:head>
  <title>elm ent</title>
</svelte:head>

{#await loadData()}
  <p>loading...</p>
{:then _}
  <MainGame />
{/await}
