<script lang="ts">
  import { getResult } from "$lib/utils/data";
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
  let mouseClickPos = $state({ x: 0, y: 0 });

  function onElementClicked(event: MouseEvent, id: string) {
    event.stopPropagation();
    mouseClickPos = { x: event.x, y: event.y };

    if (!heldElement) {
      heldElement = id;
    } else {
      const result = getResult(id, heldElement);
      heldElement = null;

      if (result === null) {
        console.log("cant combine these elemenets");
        return;
      }

      addElement("air", result);
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
