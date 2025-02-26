<script setup>
import { ref, onMounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { getVersion } from "@tauri-apps/api/app";
import { EditorView, basicSetup } from 'codemirror';
import { php } from '@codemirror/lang-php';
import { oneDark } from '@codemirror/theme-one-dark';
import { keymap } from '@codemirror/view';
import { indentWithTab } from '@codemirror/commands';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';
import { getStore, load } from '@tauri-apps/plugin-store';
import { message } from "@tauri-apps/plugin-dialog";
import Prism from 'prismjs';
import 'prismjs/themes/prism-okaidia.min.css';
import 'prismjs/components/prism-php';
import 'prismjs/components/prism-bash';
import 'prismjs/components/prism-markup-templating';

const layout = ref("");
const phpPath = ref("");
const laravelPath = ref("");
const code = ref("");
const outputs = ref([]);

onMounted(async () => {
  const store = await load('store.json', { autoSave: false });
  layout.value = await store.get('layout');
  phpPath.value = await store.get('phpPath');
  laravelPath.value = await store.get('laravelPath');
  code.value = await store.get('code') || 'time();';
  outputs.value = await store.get('outputs') || [];

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

  await nextTick();
  Prism.highlightAll();
});

async function runLaravelCode() {
  try {
    const message = await invoke('execute_laravel_code', {
      code: code.value,
      laravelPath: laravelPath.value,
      bin: phpPath.value,
    });
    outputs.value.push(message);
    outputs.value.reverse();

    await saveCodeHistory();

    await nextTick();
    Prism.highlightAll();
  } catch (error) {
    outputs.value.push(`Error: ${error}`);
    await saveCodeHistory();
  }
}

async function saveCodeHistory() {
  const store = await getStore('store.json');
  await store.set('code', code.value);
  await store.save();
  await store.set('outputs', outputs.value);
  await store.save();
}

async function openArtisanWindow() {
  const artisanWindow = new WebviewWindow('artisan', {
    url: 'artisan.html',
    title: 'Artisan',
    label: 'artisan',
    width: 1200,
    height: 1000,
    resizable: true,
    fullscreen: false,
    focus: true
  });
  artisanWindow.once('tauri://error', (e) => {
    console.error('Artisan window error:', e);
  });
}

async function openSettingsWindow() {
  const settingsWindow = new WebviewWindow('settings', {
    url: 'settings.html',
    title: 'Settings',
    label: 'settings',
    width: 800,
    height: 600,
    resizable: true,
    fullscreen: false,
    focus: true
  });
  settingsWindow.once('tauri://error', (e) => {
    console.error('Settings window error:', e);
  });
}

async function showHelp() {
  const version = await getVersion();
  await message(
    `version: ${version}

    Copyright © 2025 Labkita. All rights reserved.

• Cmd/Ctrl + Enter: Play
• Esc: Close Window`,
    { title: 'Tinker Laravel', type: 'info' }
  );
}

function clearOutputs() {
  outputs.value = [];
  saveCodeHistory();
}

</script>

<template>
  <div class="font-sans h-screen flex flex-col justify-between bg-slate-800">
    <div class="p-2 flex flex-col space-y-2">
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
        <button class="py-1 px-4 bg-slate-600 hover:bg-slate-700 rounded shadow text-white flex items-center gap-2"
          @click="openArtisanWindow">
          <svg viewBox="0 0 50 52" xmlns="http://www.w3.org/2000/svg" class="w-5 h-5 fill-current">
            <path
              d="M49.626 11.564a.809.809 0 0 1 .028.209v10.972a.8.8 0 0 1-.402.694l-9.209 5.302V39.25c0 .286-.152.55-.4.694L20.42 51.01c-.044.025-.092.041-.14.058-.018.006-.035.017-.054.022a.805.805 0 0 1-.41 0c-.022-.006-.042-.018-.063-.026-.044-.016-.09-.03-.132-.054L.402 39.944A.801.801 0 0 1 0 39.25V6.334c0-.072.01-.142.028-.21.006-.023.02-.044.028-.067.015-.042.029-.085.051-.124.015-.026.037-.047.055-.071.023-.032.044-.065.071-.093.023-.023.053-.04.079-.06.029-.024.055-.05.088-.069h.001l9.61-5.533a.802.802 0 0 1 .8 0l9.61 5.533h.002c.032.02.059.045.088.068.026.02.055.038.078.06.028.029.048.062.072.094.017.024.04.045.054.071.023.04.036.082.052.124.008.023.022.044.028.068a.809.809 0 0 1 .028.209v20.559l8.008-4.611v-10.51c0-.07.01-.141.028-.208.007-.024.02-.045.028-.068.016-.042.03-.085.052-.124.015-.026.037-.047.054-.071.024-.032.044-.065.072-.093.023-.023.052-.04.078-.06.03-.024.056-.05.088-.069h.001l9.611-5.533a.801.801 0 0 1 .8 0l9.61 5.533c.034.02.06.045.09.068.025.02.054.038.077.06.028.029.048.062.072.094.018.024.04.045.054.071.023.039.036.082.052.124.009.023.022.044.028.068zm-1.574 10.718v-9.124l-3.363 1.936-4.646 2.675v9.124l8.01-4.611zm-9.61 16.505v-9.13l-4.57 2.61-13.05 7.448v9.216l17.62-10.144zM1.602 7.719v31.068L19.22 48.93v-9.214l-9.204-5.209-.003-.002-.004-.002c-.031-.018-.057-.044-.086-.066-.025-.02-.054-.036-.076-.058l-.002-.003c-.026-.025-.044-.056-.066-.084-.02-.027-.044-.05-.06-.078l-.001-.003c-.018-.03-.029-.066-.042-.1-.013-.03-.03-.058-.038-.09v-.001c-.01-.038-.012-.078-.016-.117-.004-.03-.012-.06-.012-.09v-.002-21.481L4.965 9.654 1.602 7.72zm8.81-5.994L2.405 6.334l8.005 4.609 8.006-4.61-8.006-4.608zm4.164 28.764l4.645-2.674V7.719l-3.363 1.936-4.646 2.675v20.096l3.364-1.937zM39.243 7.164l-8.006 4.609 8.006 4.609 8.005-4.61-8.005-4.608zm-.801 10.605l-4.646-2.675-3.363-1.936v9.124l4.645 2.674 3.364 1.937v-9.124zM20.02 38.33l11.743-6.704 5.87-3.35-8-4.606-9.211 5.303-8.395 4.833 7.993 4.524z" />
          </svg>
          Artisan
        </button>
        <button class="py-1 px-4 bg-slate-600 hover:bg-slate-700 rounded shadow text-white flex items-center gap-2"
          @click="openSettingsWindow">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
            stroke="currentColor" class="w-5 h-5">
            <path stroke-linecap="round" stroke-linejoin="round"
              d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.325.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 0 1 1.37.49l1.296 2.247a1.125 1.125 0 0 1-.26 1.431l-1.003.827c-.293.241-.438.613-.43.992a7.723 7.723 0 0 1 0 .255c-.008.378.137.75.43.991l1.004.827c.424.35.534.955.26 1.432l-1.298 2.247a1.125 1.125 0 0 1-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.47 6.47 0 0 1-.22.128c-.331.183-.581.495-.644.869l-.213 1.281c-.09.543-.56.94-1.11.94h-2.594c-.55 0-1.019-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 0 1-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 0 1-1.369-.49l-1.297-2.247a1.125 1.125 0 0 1 .26-1.432l1.004-.827c.292-.24.437-.613.43-.991a6.932 6.932 0 0 1 0-.255c.007-.38-.138-.751-.43-.992l-1.004-.827a1.125 1.125 0 0 1-.26-1.43l1.297-2.248a1.125 1.125 0 0 1 1.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281Z" />
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z" />
          </svg>
          Settings
        </button>
        <button @click="showHelp"
          class="py-1 px-4 bg-slate-600 hover:bg-slate-700 rounded shadow text-white flex items-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
            stroke="currentColor" class="w-5 h-5">
            <path stroke-linecap="round" stroke-linejoin="round"
              d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9 5.25h.008v.008H12v-.008Z" />
          </svg>
        </button>
        <button @click="clearOutputs"
          class="py-1 px-4 bg-slate-600 hover:bg-slate-700 rounded shadow text-white flex items-center gap-2">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
            stroke="currentColor" class="w-5 h-5">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
          </svg>
          Clear
        </button>
      </div>
    </div>
    <div :class="`flex grow ${layout === 'Vertical' ? 'flex-row' : 'flex-col'} gap-2 p-2 overflow-auto`">
      <div id="code-editor"
        :class="`${layout === 'Vertical' ? 'w-1/2' : 'h-1/2'} border-4 border-lime-500 bg-slate-800 p-2 shadow-3xl rounded overflow-auto text-lg`">
      </div>
      <h1 :class="`text-sm text-white ${layout === 'Vertical' ? 'hidden' : 'block'}`">Output:</h1>
      <div
        :class="`${layout === 'Vertical' ? 'w-1/2' : 'h-1/2'} flex flex-col overflow-auto bg-slate-800 shadow-3xl rounded`">
        <div class="" v-for="output in outputs">
          <pre class="border-4 border-lime-500 py-0 px-4 language-bash">
          <code>{{ output }}</code>
        </pre>
        </div>
      </div>
    </div>
    <div class="flex justify-between pb-2 px-4 text-xs text-white">
      <p class="">Directory: {{ laravelPath }}</p>
      <p class="">PHP Path: {{ phpPath }}</p>
    </div>
  </div>
</template>
