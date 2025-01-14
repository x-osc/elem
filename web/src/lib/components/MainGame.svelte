<script lang="ts">
  type ElementState = { [key: string]: string[] };

  const elementState: ElementState = $state({
    air: ["air"],
    earth: ["earth"],
    fire: ["fire"],
    water: ["water"],
  });

  const categories = $derived(Object.keys(elementState));
</script>

<div>
  {#each categories as category}
    <div>{category}</div>
  {/each}

  <button onclick={() => elementState["air"].push("air")}>big er</button>
</div>
