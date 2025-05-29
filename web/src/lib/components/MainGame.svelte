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
<div class="padding"></div>
<div class="links">
  <a class="source" target="_blank" href="https://github.com/x-osc/elem">SOURCE</a>
  <a class="suggest" target="_blank" href="https://github.com/x-osc/elem/issues/new/choose">SUGGESTIONS</a>
</div>

<style>
  .maingame {
    padding: 4px;
  }

  .amount {
    padding-left: 0.5em;
  }

  .padding {
    height: 3em
  }

  .links {
    position: fixed;
    bottom: 0;
    right: 0;
    display: flex;
  }

  .links a {
    padding: 0.5em 0.8em;
    font-size: 1.2em;
    text-decoration: none;
    color: white;
  }

  .source {
    background-color: #5fbf56;
  }

  .suggest {
    background-color: #5693bf;
  }
</style>
