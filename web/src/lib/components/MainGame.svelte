<script lang="ts">
  import Category from "./Category.svelte";
  import HeldElement from "./HeldElement.svelte";

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

  const categories = $derived(Object.keys(elementState));
</script>

<div>
  {#each categories as categoryId}
    <Category id={categoryId} elements={elementState[categoryId]} />
  {/each}

  <HeldElement id="air" />

  <button onclick={() => addElement("air", "air")}>big er</button>
</div>
