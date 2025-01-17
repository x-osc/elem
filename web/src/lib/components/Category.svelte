<script lang="ts">
  import Element from "./Element.svelte";

  let { id, elements }: { id: string; elements: string[] } = $props();
</script>

<h2>{id}</h2>
<div>
  {#each elements as elementId}
    <Element id={elementId} />
  {/each}
</div>
