<script lang="ts">
  import { setContext } from "svelte";
  import Category from "./Category.svelte";
  import HeldElement from "./HeldElement.svelte";

  // game state

  type ElementState = { [key: string]: string[] };

  const elementState: ElementState = $state({
    air: ["air"],
    earth: ["earth"],
    fire: ["fire"],
    water: ["water"],
  });

  function addElement(category: string, id: string) {
    elementState[category].push(id);
  }

  // held element

  let heldElement: string | null = $state(null);

  function onElementClicked(id: string) {
    heldElement = id;
  }

  setContext("onClickCallback", onElementClicked);

  const categories = $derived(Object.keys(elementState));
</script>

<div>
  {#each categories as categoryId}
    <Category id={categoryId} elements={elementState[categoryId]} />
  {/each}

  {#if heldElement}
    <HeldElement id={heldElement} />
  {/if}

  <button onclick={() => addElement("air", "air")}>big er</button>
</div>
