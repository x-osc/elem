<script lang="ts">
  import { scale } from "svelte/transition";
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id }: { id: string | null } = $props();

  let currMousePos = $state({ x: 0, y: 0 });

  function handleMouseMove(event: MouseEvent) {
    currMousePos = { x: event.x, y: event.y };
  }
</script>

<svelte:document onpointermove={handleMouseMove} />

{#if id}
  <div
    style="left: {currMousePos.x}px; top: {currMousePos.y}px"
    transition:scale
  >
    <ElementDisplay name={id} />
  </div>
{/if}

<style>
  div {
    position: fixed;
    pointer-events: none;
  }
</style>
