<script lang="ts">

  import Search from './lib/Search.svelte'
  import Searchresult from './lib/Searchresult.svelte'
  import { appWindow, LogicalSize } from "@tauri-apps/api/window"
  import { invoke } from "@tauri-apps/api/tauri"

  let results: string[] = [];
  let exec_time: number = 0;
  let type: number = 0;

  const onBlur = async () => {
    await appWindow.hide();
  }

  const search = async (prompt: string) => {
    [results, exec_time, type] = await invoke('handle_input', {
      input: prompt,
    });

    if (results.length === 0) {
      return;
    }
  }

  document.addEventListener('keydown', async (event: KeyboardEvent) => {
    if (event.key === "Enter") {
      event.preventDefault();
      if(event.target?.value?.startsWith('/')) {
        search(event.target.value);
      }
    }
  })

  const handle_input = async (event: KeyboardEvent) => {
    if (event.target.value === '' ) {
      results = [];
      return;
    }

    if (event.target.value.startsWith('/')) {
      return;
    }

    search(event.target.value);
  }

</script>

<svelte:window on:blur={onBlur} />

<main class="container">
  <!-- svelte-ignore empty-block -->
  {#await appWindow.setSize(new LogicalSize(750, 100))}{/await}
  <Search on:input={handle_input}/>
  <Searchresult bind:results />
</main>

<style>
  .container {
    height: 100%;
    width: 750px;
    background: var(--primary-bg);
    border-radius: 8px;
  }
</style>