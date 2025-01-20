<script lang="ts">
  import { getCategoryData } from "$lib/utils/data";
  import { scale } from "svelte/transition";
  import Element from "./Element.svelte";

  let { id, elements }: { id: string; elements: string[] } = $props();

  let data = getCategoryData(id) ?? {
    name: "NULL",
  };
</script>

<h2>{data.name}</h2>
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
