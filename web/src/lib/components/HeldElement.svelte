<script lang="ts">
  import { getElementData } from "$lib/utils/data";
  import { scale } from "svelte/transition";
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id }: { id: string | null } = $props();

  let data = $derived(id ? getElementData(id) : null);

  let currMousePos = $state({ x: 0, y: 0 });

  function handleMouseMove(event: MouseEvent) {
    currMousePos = { x: event.x, y: event.y };
  }
</script>

<svelte:document onpointermove={handleMouseMove} />

{#if data}
  <div
    style="left: {currMousePos.x}px; top: {currMousePos.y}px"
    transition:scale={{ duration: 300 }}
  >
    <ElementDisplay {data} />
  </div>
{/if}

<style>
  div {
    position: fixed;
    pointer-events: none;
  }
</style>
