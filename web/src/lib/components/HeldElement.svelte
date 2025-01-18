<script lang="ts">
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id, mousePos }: { id: string; mousePos: { x: number; y: number } } =
    $props();

  let currMousePos = $state(mousePos);

  function handleMouseMove(event: MouseEvent) {
    currMousePos = { x: event.x, y: event.y };
  }
</script>

<svelte:document onpointermove={handleMouseMove} />

<div style="left: {currMousePos.x}px; top: {currMousePos.y}px">
  <ElementDisplay name={id} />
</div>

<style>
  div {
    position: fixed;
    pointer-events: none;
  }
</style>
