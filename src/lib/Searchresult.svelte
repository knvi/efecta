<script lang="ts">
    export let results: string[];

    import { afterUpdate } from "svelte";
    import { appWindow, LogicalSize } from "@tauri-apps/api/window";
    import { invoke } from "@tauri-apps/api/tauri";
    import { getFileName } from "../utils/path";

    afterUpdate(async () => {
        const height = document.getElementsByClassName("container")[0].clientHeight;
        await appWindow.setSize(new LogicalSize(750, height));

        if (results.length > 0 && results[0] != '') {
            const firstResult = document.getElementById(results[0]);
            firstResult?.classList.add("selected");
            await firstResult?.focus();
        }
    });

    async function handleKeyinput(event: KeyboardEvent) {
        if (event.key == "ArrowUp" || event.key == "ArrowDown") {
            const current = document.activeElement as HTMLElement | null;
            const items = [...document.getElementsByClassName("result")] as | HTMLElement[] | null;

            const currIndex = items?.indexOf(current as HTMLElement);
            let nextIndex;

            if (currIndex === -1) {
                nextIndex = 0;
            } else {
                if (event.key == "ArrowUp") {
                    nextIndex = (currIndex! + items!.length - 1) % items!.length;
                } else {
                    nextIndex = (currIndex! + 1) % items!.length;
                }                
            }

            if (current !== null && items![nextIndex] !== null) {
                items![nextIndex].classList.add("selected");
                current.classList.remove("selected");
                current.blur();
                items![nextIndex].focus();
            }

        } else if (event.key == "Enter") {
            const current = document.activeElement as HTMLElement | null;
            if (current !== null) {
                current.click();
            }
        } else {
            const input = document.getElementById("searchbar") as HTMLInputElement | null;
            input?.focus();
        }
    };

    async function clicked(event: any) {
        await invoke('open', { path: event.target.id });
        const searchbar = document.getElementById("searchbar") as HTMLInputElement;

        results = [];
        searchbar.value = "";

        await appWindow.hide();
    }
</script>

<svelte:window on:keydown={handleKeyinput} />

<div class="searchResults">
    {#if results.length > 0 && results[0] != ' '}
        {#each results.slice(0, 5) as result}
            <button on:click={clicked} class="searchResult" id={result}>
                <div class="resultContent">
                    <p class="fileName">{getFileName(result)}</p>
                </div>
            </button>
        {/each}
    {/if}
</div>

<style>
    .searchResult {
    margin-top: 7px;
    margin-left: 12px;
    box-sizing: border-box;
    display: flex;
    flex-direction: row;
    align-items: center;
    padding: 0px 12px;
    width: 100%;
    height: 43px;
    background: transparent;
    border: none;
    border-radius: 8px;
    flex: none;
    order: 1;
    flex-grow: 0;
  }

  .resultContent {
    display: flex;
    flex-direction: row;
    align-items: center;
    flex: none;
    order: 0;
    flex-grow: 0;
    margin-right: auto;
  }

  .fileName {
    font-family: Helvetica;
    font-style: normal;
    font-weight: 500;
    font-size: 14px;
    line-height: 20px;
    margin: 0;
    color: var(--primary-text-color);
    text-align: left;
    width: 250px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

    :global(.selected) {
        background: var(--highlight-overlay) !important;
        outline: 0 !important;
        border-radius: 8px;
    }
</style>