<script setup>
import { open } from "@tauri-apps/plugin-dialog";
import { onMounted, ref } from "vue";
import { getStore } from '@tauri-apps/plugin-store';
import { WebviewWindow } from '@tauri-apps/api/webviewWindow';

const layout = ref("");
const phpPath = ref("");
const laravelPath = ref("");

onMounted(async () => {
    const store = await getStore('store.json');
    layout.value = await store.get('layout') || "Vertical";
    phpPath.value = await store.get('phpPath') || "/usr/local/bin/php";
    laravelPath.value = await store.get('laravelPath') || "/Users/fajar/Documents/php-projects/testing/inertia-react";

    window.addEventListener('keydown', (e) => {
        if (e.key === 'Escape') {
            cancel();
        }
    })
})

async function openLaravelDirectory() {
    await open({ directory: true }).then(res => {
        if (!res) {
            return false
        }
        laravelPath.value = res
    })
}

async function save() {
    const store = await getStore('store.json');
    await store.set('layout', layout.value);
    await store.save();
    await store.set('phpPath', phpPath.value);
    await store.save();
    await store.set('laravelPath', laravelPath.value);
    await store.save();

    // Reload main window
    const mainWindow = await WebviewWindow.getByLabel('main');
    if (mainWindow) {
        await mainWindow.close();
        const newMainWindow = new WebviewWindow('main', {
            url: 'index.html',
            title: 'Tinker Laravel',
            label: "main",
            width: 800,
            height: 600,
            resizable: true,
            fullscreen: false,
        });
        newMainWindow.once('tauri://error', (e) => {
            console.error('reload main window error: ', e)
        })
    };

    // Close settings window
    const currentWindow = WebviewWindow.getCurrent();
    await currentWindow.close();
}

async function cancel() {
    const currentWindow = WebviewWindow.getCurrent();
    await currentWindow.close();
}

</script>

<template>
    <div class="min-h-screen bg-slate-800 p-4">
        <div class="max-w-2xl mx-auto space-y-4">
            <h1 class="text-2xl text-white font-bold">Settings</h1>

            <div class="space-y-4">
                <div class="space-y-2">
                    <label class="text-white">Default Layout</label>
                    <select v-model="layout" class="w-full py-1 px-4 text-slate-800 rounded shadow">
                        <option value="Vertical">Vertical</option>
                        <option value="Horizontal">Horizontal</option>
                    </select>
                </div>

                <div class="space-y-2">
                    <label class="text-white">PHP Path</label>
                    <input v-model="phpPath" class="w-full py-1 px-4 text-slate-800 rounded shadow"
                        placeholder="/usr/local/bin/php" />
                </div>

                <div class="space-y-2">
                    <label class="text-white">Default Laravel Directory</label>
                    <input v-model="laravelPath" class="w-full py-1 px-4 text-slate-800 rounded shadow"
                        placeholder="Path to Laravel project" />
                    <button
                        class="py-1 px-4 bg-slate-600 hover:bg-slate-700 rounded shadow text-white flex items-center gap-2"
                        @click="openLaravelDirectory">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                            stroke="currentColor" class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                d="M3.75 9.776c.112-.017.227-.026.344-.026h15.812c.117 0 .232.009.344.026m-16.5 0a2.25 2.25 0 0 0-1.883 2.542l.857 6a2.25 2.25 0 0 0 2.227 1.932H19.05a2.25 2.25 0 0 0 2.227-1.932l.857-6a2.25 2.25 0 0 0-1.883-2.542m-16.5 0V6A2.25 2.25 0 0 1 6 3.75h3.879a1.5 1.5 0 0 1 1.06.44l2.122 2.12a1.5 1.5 0 0 0 1.06.44H18A2.25 2.25 0 0 1 20.25 9v.776" />
                        </svg>
                        Change
                    </button>
                </div>

                <div class="flex space-x-2">

                    <button @click="save"
                        class="w-full py-1 px-4 bg-lime-400 hover:bg-lime-600 rounded shadow text-slate-800 flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                            stroke="currentColor" class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                d="M9 3.75H6.912a2.25 2.25 0 0 0-2.15 1.588L2.35 13.177a2.25 2.25 0 0 0-.1.661V18a2.25 2.25 0 0 0 2.25 2.25h15A2.25 2.25 0 0 0 21.75 18v-4.162c0-.224-.034-.447-.1-.661L19.24 5.338a2.25 2.25 0 0 0-2.15-1.588H15M2.25 13.5h3.86a2.25 2.25 0 0 1 2.012 1.244l.256.512a2.25 2.25 0 0 0 2.013 1.244h3.218a2.25 2.25 0 0 0 2.013-1.244l.256-.512a2.25 2.25 0 0 1 2.013-1.244h3.859M12 3v8.25m0 0-3-3m3 3 3-3" />
                        </svg>
                        Save
                    </button>
                    <button @click="cancel"
                        class="w-full py-1 px-4 bg-slate-600 hover:bg-slate-700 rounded shadow text-white flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5"
                            stroke="currentColor" class="w-5 h-5">
                            <path stroke-linecap="round" stroke-linejoin="round"
                                d="M9 15 3 9m0 0 6-6M3 9h12a6 6 0 0 1 0 12h-3" />
                        </svg>
                        Cancel
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>