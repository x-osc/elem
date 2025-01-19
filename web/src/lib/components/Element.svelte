<script lang="ts">
  import { getElementDataFallback } from "$lib/utils/data";
  import { getContext } from "svelte";
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id }: { id: string } = $props();

  const data = getElementDataFallback(id);

  let onClickCallback: (event: MouseEvent, id: string) => void =
    getContext("onClickCallback");
</script>

<div onclick={(e) => onClickCallback(e, id)}>
  <ElementDisplay {data} />
</div>

<style>
  div {
    display: inline-block;
    margin: 0.3rem;
  }
</style>
