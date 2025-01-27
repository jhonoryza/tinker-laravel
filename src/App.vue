<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';

const phpPath = ref("/usr/local/bin/php");
const laravelPath = ref("/Users/fajar/Documents/php\-projects/testing/inertia\-react");
const code = ref('echo \'a\';');
const artisanCommand = ref('route:list');
const output = ref('');
async function runLaravelCode() {
  try {
    output.value = await invoke('execute_laravel_code', {
      code: code.value,
      laravelPath: laravelPath.value,
      bin: phpPath.value,
    });
  } catch (error) {
    output.value = `Error: ${error}`;
  }
}
async function runArtisanCommand() {
  try {
    output.value = await invoke('run_artisan_command', {
      command: artisanCommand.value,
      laravelPath: laravelPath.value,
      bin: phpPath.value,
    });
  } catch (error) {
    output.value = `Error: ${error}`;
  }
}

const MONACO_EDITOR_OPTIONS = {
  formatOnType: true,
  formatOnPaste: true,
  lineHeight: 32,
  fontSize: 14,
  // fontLigatures: true,
  fontFamily: 'monospace',
  contextmenu: false,
  smoothScrolling: true,
  scrollBeyondLastLine: true,
  wordWrap: 'on',
  wrappingStrategy: 'advance',
  automaticLayout: true,
  renderLineHighlight: 'none',
  renderWhitespace: false,
  scrollbar: {
    verticalScrollbarSize: 0,
    verticalSliderSize: 0
  },
  semanticHighlighting: {
    enabled: true
  },
  minimap: {
    enabled: false
  },
  lineNumbersWidth: 0,
}

function handleMount(editor) {
  // Tambahkan shortcut Cmd + Enter (atau Ctrl + Enter di Windows)
  editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, () => {
    runLaravelCode();
  });
}

async function openLaravelDirecttory() {
  await open({ directory: true }).then(res => {
    if (!res) {
      return false
    }
    laravelPath.value = res
  })
}

</script>

<template>
  <div class="font-sans h-screen flex flex-col space-y-2 bg-slate-800 overflow-hidden">
    <div class="flex flex-grow flex-shrink h-full overflow-scroll">
      <div class="flex w-full h-full">
        <div class="flex flex-col w-full h-full space-y-2 relative">
          <vue-monaco-editor class="border border-slate-100" v-model:value="code" theme="vs-dark" language="php"
            :options="MONACO_EDITOR_OPTIONS" @mount="handleMount" />
          <vue-monaco-editor class="border border-slate-100" v-model:value="output" theme="vs-dark" language="php"
            :options="MONACO_EDITOR_OPTIONS" @mount="handleMount" />
          <button
            class="absolute top-0 right-4 z-50 py-1 px-4 bg-lime-400 hover:bg-lime-600 rounded shadow text-slate-800"
            @click="runLaravelCode">
            Run
          </button>
        </div>
      </div>
    </div>
    <div class="flex flex-col gap-2 justify-center items-start p-2">
      <p class="text-white">PHP Path: </p>
      <input class="py-1 px-4 text-slate-800 rounded shadow flex-grow" v-model="phpPath" placeholder="Path to PHP" />
      <p class="text-white">Laravel Directory: </p>
      <div class="flex items-center space-x-2 w-full">
        <input class="py-1 px-4 text-slate-800 rounded shadow flex-grow" v-model="laravelPath"
          placeholder="Path to Laravel project" />
        <button class="py-1 px-4 bg-lime-400 hover:bg-lime-600 rounded shadow text-slate-800"
          @click="openLaravelDirecttory">
          Choose Path
        </button>
      </div>
      <p class="text-white">Artisan Command: </p>
      <div class="flex space-x-2 justify-center items-center w-full">
        <input class="flex-grow py-1 px-4 text-slate-800 rounded shadow" v-model="artisanCommand"
          placeholder="Enter Artisan command" />
        <button class="py-1 px-4 bg-lime-400 hover:bg-lime-600 rounded shadow text-slate-800"
          @click="runArtisanCommand">
          Run
        </button>
      </div>
    </div>
  </div>
</template>
