<script lang="ts">
  import { getContext } from "svelte";
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id }: { id: string } = $props();

  let onClickCallback: (event: MouseEvent, id: string) => void =
    getContext("onClickCallback");
</script>

<div onclick={(e) => onClickCallback(e, id)}>
  <ElementDisplay name={id} />
</div>

<style>
  div {
    display: inline-block;
    margin: 0.3rem;
  }
</style>
