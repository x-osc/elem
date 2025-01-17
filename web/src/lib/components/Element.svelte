<script lang="ts">
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id }: { id: string } = $props();
</script>

<div><ElementDisplay name={id} /></div>
