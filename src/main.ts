import "./styles.css";
import App from "./App.svelte";
import { register } from "@tauri-apps/api/globalShortcut";
import { appWindow } from "@tauri-apps/api/window";

const targetElement = document.getElementById("app");
const app = new App({
  target: targetElement!,
});

export async function listenForHotkey(shortcut: string) {
  await register(shortcut, async () => {
    if (document.hasFocus()) {
      await appWindow.hide();
    } else {
      await appWindow.show();
      await appWindow.center();
      await appWindow.setFocus();
      document.getElementById('searchbar')!.focus();
    }
  })
}

export default app;
