<template>
  <div>
    <h1>Tauri + Vue.js + Laravel Integration</h1>
    <input v-model="laravelPath" placeholder="Path to Laravel project" />
    <textarea v-model="code" placeholder="Enter Laravel code"></textarea>
    <button @click="runLaravelCode">Run Laravel Code</button>
    <input v-model="artisanCommand" placeholder="Enter Artisan command" />
    <button @click="runArtisanCommand">Run Artisan Command</button>
    <pre>{{ output }}</pre>
  </div>
</template>

<script>
export default {
  data() {
    return {
      laravelPath: '',
      code: 'echo App\\Models\\User::count();',
      artisanCommand: 'migrate',
      output: '',
    };
  },
  methods: {
    async runLaravelCode() {
      try {
        this.output = await window.__TAURI__.invoke('execute_laravel_code', {
          code: this.code,
          laravelPath: this.laravelPath,
        });
      } catch (error) {
        this.output = `Error: ${error}`;
      }
    },
    async runArtisanCommand() {
      try {
        this.output = await window.__TAURI__.invoke('run_artisan_command', {
          command: this.artisanCommand,
          laravelPath: this.laravelPath,
        });
      } catch (error) {
        this.output = `Error: ${error}`;
      }
    },
  },
};
</script>

<style>
/* Tambahkan gaya CSS sesuai kebutuhan */
</style>
