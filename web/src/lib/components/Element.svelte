<script lang="ts">
  import { getElementData } from "$lib/utils/data";
  import { getContext } from "svelte";
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id }: { id: string } = $props();

  const data = getElementData(id) ?? {
    name: `NULL id: {${id}}`,
    category: "NULL",
    tier: 1,
    color: "#f700ff",
  };

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
