<script setup>
import { StreamLanguage } from "@codemirror/language";
import { shell } from "@codemirror/legacy-modes/mode/shell";
import { oneDark } from "@codemirror/theme-one-dark";
import { invoke } from "@tauri-apps/api/core";
import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { getStore } from '@tauri-apps/plugin-store';
import { basicSetup, EditorView } from "codemirror";
import { onMounted, ref, watch } from "vue";

const artisanCommand = ref("");
const phpPath = ref("");
const laravelPath = ref("");
const output = ref('');
let outputEditor = null;

onMounted(async () => {
    const store = await getStore('store.json');
    artisanCommand.value = await store.get('artisanCommand') || 'route:list';
    phpPath.value = await store.get('phpPath');
    laravelPath.value = await store.get('laravelPath');
    output.value = await store.get('outputArtisan') || '';

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

    window.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            cancel();
        }
    })
})

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

async function runArtisanCommand() {
    try {
        output.value = await invoke('run_artisan_command', {
            command: artisanCommand.value,
            laravelPath: laravelPath.value,
            bin: phpPath.value,
        });
        await saveArtisanHistory();
    } catch (error) {
        output.value = `Error: ${error}`;
        await saveArtisanHistory();
    }
}

async function saveArtisanHistory() {
    const store = await getStore('store.json');
    await store.set('artisanCommand', artisanCommand.value);
    await store.save();
    await store.set('outputArtisan', output.value);
    await store.save();
}

async function cancel() {
    const currentWindow = WebviewWindow.getCurrent();
    await currentWindow.close();
}

</script>

<template>

    <div class="font-sans h-screen flex flex-col space-y-0 overflow-hidden bg-slate-800 px-4">
        <h1 class="text-2xl text-white font-bold">Artisan Command</h1>

        <div class="space-y-2 flex flex-col justify-start items-start">
            <input class="mt-4 flex-grow py-1 px-4 text-slate-800 rounded shadow" v-model="artisanCommand"
                placeholder="Enter Artisan command" />
            <div class="flex space-x-2">
                <button
                    class="py-1 px-4 bg-lime-400 hover:bg-lime-600 rounded shadow text-slate-800 flex items-center gap-2"
                    @click="runArtisanCommand">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M5.25 5.653c0-.856.917-1.398 1.667-.986l11.54 6.347a1.125 1.125 0 0 1 0 1.972l-11.54 6.347c-.75.412-1.667-.13-1.667-.986V5.653Z" />
                    </svg>
                    Play
                </button>
                <button @click="cancel"
                    class="py-1 px-4 bg-slate-600 hover:bg-slate-700 rounded shadow text-white flex items-center gap-2">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                        stroke="currentColor" class="w-5 h-5">
                        <path stroke-linecap="round" stroke-linejoin="round"
                            d="M9 15 3 9m0 0 6-6M3 9h12a6 6 0 0 1 0 12h-3" />
                    </svg>
                    Back
                </button>
            </div>
        </div>
        <div class="flex w-full h-full overflow-scroll">
            <div id="output-editor" class="h-full w-full"></div>
        </div>
        <div class="flex justify-between px-4">
            <p class="text-white">Directory: {{ laravelPath }}</p>
            <p class="text-white">PHP Path: {{ phpPath }}</p>
        </div>
    </div>
</template>