<script lang="ts">
  import type { ElementData } from "$lib/utils/data";
  import { colorIsDark } from "$lib/utils/text";

  let { data }: { data: ElementData } = $props();

  let isDark = colorIsDark(data.color);
</script>

<div
  class="elem {isDark ? 'dark' : 'light'}"
  style="background-color: {data.color};"
>
  {data.name}
</div>

<style>
  .elem {
    display: inline-block;
    justify-content: center;
    text-align: center;
    box-sizing: border-box;

    font-size: 0.8rem;
    width: 70px;
    height: 70px;
    max-width: 70px;
    max-height: 70px;
    border-radius: 7px;
    padding-top: 0.3rem;
    padding-bottom: 0.2rem;
    padding-left: 0.2rem;
    padding-right: 0.2rem;

    box-shadow: 1px 2px 3px rgba(0, 0, 0, 0.35);

    user-select: none;
    overflow-wrap: break-word;
    overflow-x: hidden;
    overflow-y: auto;
  }

  .elem:hover {
    background-image: linear-gradient(rgba(146, 146, 146, 0.1) 0 0);
  }

  .elem:active {
    background-image: linear-gradient(rgba(146, 146, 146, 0.3) 0 0);
  }

  .light {
    color: rgba(0, 0, 0, 0.8);
  }

  .dark {
    color: rgba(255, 255, 255, 0.8);
  }
</style>
