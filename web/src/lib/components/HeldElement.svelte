<script lang="ts">
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id }: { id: string } = $props();

  let mousePos = $state({ x: 0, y: 0 });

  function handleMouseMove(event: MouseEvent) {
    mousePos = { x: event.x, y: event.y };
  }
</script>

<svelte:document onpointermove={handleMouseMove} />

<div style="left: {mousePos.x}px; top: {mousePos.y}px">
  <ElementDisplay name={id} />
</div>

<style>
  div {
    position: fixed;
    pointer-events: none;
  }
</style>
