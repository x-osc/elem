<script lang="ts">
  type ElementState = { [key: string]: string[] };

  const elementState: ElementState = $state({
    air: ["air"],
  });
</script>

<div>
  {#each elementState["air"] as element}
    <div>air</div>
  {/each}

  <button onclick={() => elementState["air"].push("air")}>big er</button>
</div>
