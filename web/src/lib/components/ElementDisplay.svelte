<script lang="ts">
  import type { ElementData } from "$lib/utils/data";
  import { colorIsDark } from "$lib/utils/text";

  let { data }: { data: ElementData } = $props();

  let isDark = colorIsDark(data.color);
</script>

<div class={isDark ? "dark" : "light"} style="background-color: {data.color};">
  {data.name}
</div>

<style>
  div {
    display: inline-block;
    justify-content: center;
    text-align: center;
    box-sizing: border-box;

    font-size: 0.9rem;
    width: 70px;
    height: 70px;
    max-width: 70px;
    max-height: 70px;
    border-radius: 7px;
    padding-top: 0.3rem;
    padding-bottom: 0.2rem;
    padding-left: 0.2rem;
    padding-right: 0.2rem;

    background-color: lightblue;
    box-shadow: 1px 2px 3px rgba(0, 0, 0, 0.35);

    user-select: none;
    overflow-wrap: break-word;
    overflow-x: hidden;
    overflow-y: auto;
  }

  .light {
    color: #000000;
  }

  .dark {
    color: #fafafa;
  }
</style>
