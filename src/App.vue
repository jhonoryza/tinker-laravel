<script setup>
import { ref, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { EditorView, basicSetup } from 'codemirror';
import { php } from '@codemirror/lang-php';
import { StreamLanguage } from '@codemirror/language';
import { shell } from '@codemirror/legacy-modes/mode/shell';
import { oneDark } from '@codemirror/theme-one-dark';
import { keymap } from '@codemirror/view';
import { indentWithTab } from '@codemirror/commands';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getStore, load } from '@tauri-apps/plugin-store';

const layout = ref("");
const phpPath = ref("");
const laravelPath = ref("");
const code = ref("");
const output = ref('');
let outputEditor = null;

onMounted(async () => {
  const store = await load('store.json', { autoSave: false });
  layout.value = await store.get('layout');
  phpPath.value = await store.get('phpPath');
  laravelPath.value = await store.get('laravelPath');
  code.value = await store.get('code') || 'time();';
  output.value = await store.get('output') || '';

  new EditorView({
    doc: code.value,
    extensions: [
      basicSetup,
      php({
        baseLanguage: null,
        plain: true
      }),
      oneDark,
      keymap.of([
        indentWithTab,
        {
          mac: 'Cmd-Shift-Enter',
          win: 'Ctrl-Shift-Enter',
          linux: 'Ctrl-Shift-Enter',
          run: () => {
            runLaravelCode();
            return true;
          }
        }
      ]),
      EditorView.updateListener.of((update) => {
        if (update.docChanged) {
          code.value = update.state.doc.toString();
        }
      })
    ],
    parent: document.getElementById('code-editor')
  });

  outputEditor = new EditorView({
    doc: output.value,
    extensions: [
      basicSetup,
      StreamLanguage.define(shell),
      oneDark,
      EditorView.editable.of(false)
    ],
    parent: document.getElementById('output-editor')
  });
});

// Add watcher for output
watch(output, (newValue) => {
  if (outputEditor) {
    outputEditor.dispatch({
      changes: {
        from: 0,
        to: outputEditor.state.doc.length,
        insert: newValue
      }
    });
  }
});

async function runLaravelCode() {
  try {
    output.value = await invoke('execute_laravel_code', {
      code: code.value,
      laravelPath: laravelPath.value,
      bin: phpPath.value,
    });
    await saveCodeHistory();
  } catch (error) {
    output.value = `Error: ${error}`;
    await saveCodeHistory();
  }
}

async function saveCodeHistory() {
  const store = await getStore('store.json');
  await store.set('code', code.value);
  await store.save();
  await store.set('output', output.value);
  await store.save();
}

async function openArtisanWindow() {
  const artisanWindow = new WebviewWindow('artisan', {
    url: 'artisan.html',
    title: 'Artisan',
  });
  artisanWindow.once('tauri://error', (e) => {
    console.error('Artisan window error:', e);
  });
}

async function openSettingsWindow() {
  const settingsWindow = new WebviewWindow('settings', {
    url: 'settings.html',
    title: 'Settings',
  });
  settingsWindow.once('tauri://error', (e) => {
    console.error('Settings window error:', e);
  });
}

</script>

<template>
  <div class="font-sans h-screen flex flex-col space-y-0 overflow-hidden bg-slate-800">
    <div class="text-slate-100 p-2 flex flex-col space-y-2">
      <div class="flex space-x-2">
        <button class="py-1 px-4 bg-lime-400 hover:bg-lime-600 rounded shadow text-slate-800 flex items-center gap-2"
          @click="runLaravelCode">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
            stroke="currentColor" class="w-5 h-5">
            <path stroke-linecap="round" stroke-linejoin="round"
              d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347c-.75.412-1.667-.13-1.667-.986V5.653Z" />
          </svg>
          Play
        </button>
        <button class="py-1 px-4 bg-lime-400 hover:bg-lime-600 rounded shadow text-slate-800 flex items-center gap-2"
          @click="openArtisanWindow">
          <svg viewBox="0 0 50 52" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 fill-current">
            <path
              d="M49.626 11.564a.809.809 0 0 1 .028.209v10.972a.8.8 0 0 1-.402.694l-9.209 5.302V39.25c0 .286-.152.55-.4.694L20.42 51.01c-.044.025-.092.041-.14.058-.018.006-.035.017-.054.022a.805.805 0 0 1-.41 0c-.022-.006-.042-.018-.063-.026-.044-.016-.09-.03-.132-.054L.402 39.944A.801.801 0 0 1 0 39.25V6.334c0-.072.01-.142.028-.21.006-.023.02-.044.028-.067.015-.042.029-.085.051-.124.015-.026.037-.047.055-.071.023-.032.044-.065.071-.093.023-.023.053-.04.079-.06.029-.024.055-.05.088-.069h.001l9.61-5.533a.802.802 0 0 1 .8 0l9.61 5.533h.002c.032.02.059.045.088.068.026.02.055.038.078.06.028.029.048.062.072.094.017.024.04.045.054.071.023.04.036.082.052.124.008.023.022.044.028.068a.809.809 0 0 1 .028.209v20.559l8.008-4.611v-10.51c0-.07.01-.141.028-.208.007-.024.02-.045.028-.068.016-.042.03-.085.052-.124.015-.026.037-.047.054-.071.024-.032.044-.065.072-.093.023-.023.052-.04.078-.06.03-.024.056-.05.088-.069h.001l9.611-5.533a.801.801 0 0 1 .8 0l9.61 5.533c.034.02.06.045.09.068.025.02.054.038.077.06.028.029.048.062.072.094.018.024.04.045.054.071.023.039.036.082.052.124.009.023.022.044.028.068zm-1.574 10.718v-9.124l-3.363 1.936-4.646 2.675v9.124l8.01-4.611zm-9.61 16.505v-9.13l-4.57 2.61-13.05 7.448v9.216l17.62-10.144zM1.602 7.719v31.068L19.22 48.93v-9.214l-9.204-5.209-.003-.002-.004-.002c-.031-.018-.057-.044-.086-.066-.025-.02-.054-.036-.076-.058l-.002-.003c-.026-.025-.044-.056-.066-.084-.02-.027-.044-.05-.06-.078l-.001-.003c-.018-.03-.029-.066-.042-.1-.013-.03-.03-.058-.038-.09v-.001c-.01-.038-.012-.078-.016-.117-.004-.03-.012-.06-.012-.09v-.002-21.481L4.965 9.654 1.602 7.72zm8.81-5.994L2.405 6.334l8.005 4.609 8.006-4.61-8.006-4.608zm4.164 28.764l4.645-2.674V7.719l-3.363 1.936-4.646 2.675v20.096l3.364-1.937zM39.243 7.164l-8.006 4.609 8.006 4.609 8.005-4.61-8.005-4.608zm-.801 10.605l-4.646-2.675-3.363-1.936v9.124l4.645 2.674 3.364 1.937v-9.124zM20.02 38.33l11.743-6.704 5.87-3.35-8-4.606-9.211 5.303-8.395 4.833 7.993 4.524z" />
          </svg>
          Artisan
        </button>
        <button class="py-1 px-4 bg-lime-400 hover:bg-lime-600 rounded shadow text-slate-800 flex items-center gap-2"
          @click="openSettingsWindow">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
            stroke="currentColor" class="w-5 h-5">
            <path stroke-linecap="round" stroke-linejoin="round"
              d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.432l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.432l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.248a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281Z" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
          </svg>
          Settings
        </button>
      </div>
    </div>
    <div :class="`flex ${layout === 'Vertical' ? 'flex-row' : 'flex-col'} w-full h-full overflow-scroll`">
      <div id="code-editor" class="h-full w-full"></div>
      <div id="output-editor" class="h-full w-full"></div>
    </div>
    <div class="flex justify-between px-4">
      <p class="text-white">Directory: {{ laravelPath }}</p>
      <p class="text-white">PHP Path: {{ phpPath }}</p>
    </div>
  </div>
</template>
