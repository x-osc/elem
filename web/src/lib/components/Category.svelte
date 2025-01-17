<script lang="ts">
  let { id, elements }: { id: string; elements: string[] } = $props();
</script>

<h2>{id}</h2>
<div>
  {#each elements as elementId}
    <div>{elementId}</div>
  {/each}
</div>
