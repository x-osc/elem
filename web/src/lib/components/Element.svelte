<script lang="ts">
  import { getContext } from "svelte";
  import ElementDisplay from "./ElementDisplay.svelte";

  let { id }: { id: string } = $props();

  let onClickCallback: (id: string) => void = getContext("onClickCallback");
</script>

<div onclick={() => onClickCallback(id)}><ElementDisplay name={id} /></div>

<style>
  div {
    display: inline-block;
    margin: 0.3rem;
  }
</style>
