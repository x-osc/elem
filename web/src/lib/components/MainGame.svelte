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
  let mouseClickPos = $state({ x: 0, y: 0 });

  function onElementClicked(event: MouseEvent, id: string) {
    event.stopPropagation();
    mouseClickPos = { x: event.x, y: event.y };

    if (!heldElement) {
      heldElement = id;
    } else {
      heldElement = null;
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

  {#if heldElement}
    <HeldElement id={heldElement} mousePos={mouseClickPos} />
  {/if}

  <button onclick={() => addElement("air", "air")}>big er</button>
</div>
