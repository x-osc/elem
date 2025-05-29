<script lang="ts">
  import { getElementData, getResult, getTotalAmount } from "$lib/utils/data";
  import { loadGameSlow, saveGame, type ElementState } from "$lib/utils/save";
  import { setContext } from "svelte";
  import Category from "./Category.svelte";
  import HeldElement from "./HeldElement.svelte";

  // game state

  const elementState: ElementState = $state(loadGameSlow());

  function addElement(category: string, id: string) {
    if (!elementState[category]) {
      elementState[category] = [id];
      return;
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

  // total elements

  const categories = $derived(Object.keys(elementState));

  const totalAmount = getTotalAmount();

  const elemAmount = $derived(
    categories.reduce(
      (total, categoryId) => total + elementState[categoryId].length,
      0,
    ),
  );
</script>

<svelte:document onclick={onMouseClick} />

<div class="maingame">
  <h2 class="amount">{elemAmount} / {totalAmount}</h2>

  {#each categories as categoryId}
    <Category id={categoryId} elements={elementState[categoryId]} />
  {/each}

  <HeldElement id={heldElement} />
</div>

<style>
  .maingame {
    padding: 4px;
  }

  .amount {
    padding-left: 0.5em;
  }
</style>
