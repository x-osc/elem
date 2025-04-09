<script lang="ts">
  import { getElementData, getResult } from "$lib/utils/data";
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
  let err: string | null = $state(null); //Help Im going insane

  function onElementClicked(event: MouseEvent, id: string) {
    event.stopPropagation();

    if (!heldElement) {
      heldElement = id;
    } else {
      const result = getResult(id, heldElement);
      heldElement = null;

      if (result === null) {
        console.log("cant combine these elemenets");
        err = "Yeah that's not gonna work";

        setTimeout(() => {
          err = null;
        }, 2000);
        return;
      }

      //I need to make an edit to check if git is commiting

      let category = getElementData(result)?.category;
      if (!category) {
        err = 'yeah buddy thats not gonna work';
        console.log(`element {${result}} doesnt exist (or has no category)`);
        return;
      }
      
      err = null;
      addElement(category, result);
    }
  }

  function onMouseClick() {
    heldElement = null;
    err = null;
  }

  setContext("onClickCallback", onElementClicked);

  const categories = $derived(Object.keys(elementState));
</script>

<svelte:document onclick={onMouseClick} />

<button type="button" on:click={onElementClicked}>
  {#if err}
    <div class="error-message">{err}</div>
  {/if}
</button>  
<div>

  {#each categories as categoryId}
    <Category id={categoryId} elements={elementState[categoryId]} />
  {/each}

  <HeldElement id={heldElement} />
</div>

<style>
  .error-message {
  color: orange;
  font-weight: bold;
  font-size: 1em;
  }
</style>
