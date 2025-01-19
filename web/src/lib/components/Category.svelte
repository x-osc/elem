<script lang="ts">
  import { scale } from "svelte/transition";
  import Element from "./Element.svelte";

  let { id, elements }: { id: string; elements: string[] } = $props();
</script>

<h2>{id}</h2>
<div>
  {#each elements as elementId}
    <div class="elem-wrapper" transition:scale>
      <Element id={elementId} />
    </div>
  {/each}
</div>

<style>
  .elem-wrapper {
    display: inline-block;
  }
</style>
