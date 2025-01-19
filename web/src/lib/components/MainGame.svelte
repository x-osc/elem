<script lang="ts">
  import { getElementData, getResult } from "$lib/utils/data";
  import { loadGameFast, saveGame, type ElementState } from "$lib/utils/save";
  import { setContext } from "svelte";
  import Category from "./Category.svelte";
  import HeldElement from "./HeldElement.svelte";

  // game state

  const elementState: ElementState = $state(loadGameFast());

  function addElement(category: string, id: string) {
    if (!elementState[category]) {
      elementState[category] = [];
    }

    if (!elementState[category].includes(id)) {
      elementState[category].push(id);
    }
  }

  $effect(() => {
    saveGame(elementState);
  });

  // held element

  let heldElement: string | null = $state(null);

  function onElementClicked(event: MouseEvent, id: string) {
    event.stopPropagation();

    if (!heldElement) {
      heldElement = id;
    } else {
      const result = getResult(id, heldElement);
      heldElement = null;

      if (result === null) {
        console.log("cant combine these elemenets");
        return;
      }

      let category = getElementData(result)?.category;
      if (!category) {
        console.log(`element {${result}} doesnt exist (or has no category)`);
        return;
      }

      addElement(category, result);
    }
  }

  function onMouseClick() {
    heldElement = null;
  }

  setContext("onClickCallback", onElementClicked);

  const categories = $derived(Object.keys(elementState));
</script>

<svelte:document onclick={onMouseClick} />

<div>
  {#each categories as categoryId}
    <Category id={categoryId} elements={elementState[categoryId]} />
  {/each}

  <HeldElement id={heldElement} />
</div>
