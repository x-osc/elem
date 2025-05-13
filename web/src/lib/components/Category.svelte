<script lang="ts">
  import { getCategoryData } from "$lib/utils/data";
  import { scale } from "svelte/transition";
  import Element from "./Element.svelte";

  let { id, elements }: { id: string; elements: string[] } = $props();

  let length = $derived(elements.length);

  let data = getCategoryData(id) ?? {
    name: "NULL",
    amount: 0,
  };
</script>

<div class="header-row">
  <h2 class="title">{data.name}</h2>
  <h2 class="amount">{length} / {data.amount}</h2>
</div>
<div>
  {#each elements as elementId}
    <div class="elem-wrapper" transition:scale>
      <Element id={elementId} />
    </div>
  {/each}
</div>

<style>
  .header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;

    padding-left: 0.5em;
    padding-right: 2em;
  }

  .title {
    text-align: left;
  }

  .amount {
    text-align: right;
  }

  .elem-wrapper {
    display: inline-block;
  }
</style>
