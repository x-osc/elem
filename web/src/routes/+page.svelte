<script lang="ts">
  import MainGame from "$lib/components/MainGame.svelte";
  import { loadData } from "$lib/utils/data";
</script>

<svelte:head>
  <title>elm ent</title>
</svelte:head>

{#await loadData()}
  <div class="loading">
    <p>loading...</p>
  </div>
{:then _}
  <MainGame />
{/await}

<style>
  .loading {
    margin: 1em;
  }
</style>
