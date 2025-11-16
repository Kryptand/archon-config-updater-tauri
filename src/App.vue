<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface Config {
  characters: Array<{
    name: string;
    class: string;
    specializations: string[];
  }>;
  raidDifficulties: string[];
  raidBosses: string[];
  dungeons: string[];
  clearPreviousBuilds: boolean;
  outputPath: string;
}

const configPath = ref<string>("");
const config = ref<Config | null>(null);
const isUpdating = ref(false);
const statusMessage = ref("");
const errorMessage = ref("");

const hasConfig = computed(() => config.value !== null);

async function selectConfigFile() {
  try {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "JSON",
          extensions: ["json"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      configPath.value = selected;
      await loadConfig();
    }
  } catch (error) {
    errorMessage.value = `Failed to select file: ${error}`;
  }
}

async function loadConfig() {
  try {
    errorMessage.value = "";
    const content = await invoke<string>("read_file", { path: configPath.value });
    config.value = JSON.parse(content);
    statusMessage.value = "Configuration loaded successfully";
  } catch (error) {
    errorMessage.value = `Failed to load config: ${error}`;
    config.value = null;
  }
}

async function updateTalents() {
  if (!configPath.value) {
    errorMessage.value = "Please select a configuration file first";
    return;
  }

  try {
    isUpdating.value = true;
    errorMessage.value = "";
    statusMessage.value = "Fetching talents from Archon.gg...";

    const result = await invoke<string>("update_talents", {
      configPath: configPath.value,
    });

    statusMessage.value = result;
    isUpdating.value = false;
  } catch (error) {
    errorMessage.value = `Update failed: ${error}`;
    statusMessage.value = "";
    isUpdating.value = false;
  }
}
</script>

<template>
  <main class="container">
    <header>
      <h1>🎮 Archon Talent Updater</h1>
      <p class="subtitle">
        Automatically fetch and update WoW talent builds from Archon.gg
      </p>
    </header>

    <section class="config-section">
      <h2>Configuration</h2>
      <div class="file-selector">
        <button @click="selectConfigFile" class="btn-primary">
          📁 Select settings.json
        </button>
        <span v-if="configPath" class="file-path">{{ configPath }}</span>
      </div>

      <div v-if="hasConfig" class="config-display">
        <h3>Loaded Configuration</h3>
        <div class="config-grid">
          <div class="config-item">
            <strong>Characters:</strong>
            <ul>
              <li v-for="char in config!.characters" :key="char.name">
                {{ char.name }} ({{ char.class }}) -
                {{ char.specializations.join(", ") }}
              </li>
            </ul>
          </div>
          <div class="config-item">
            <strong>Raid Difficulties:</strong>
            <span>{{ config!.raidDifficulties.join(", ") || "None" }}</span>
          </div>
          <div class="config-item">
            <strong>Raid Bosses:</strong>
            <span>{{ config!.raidBosses.join(", ") || "None" }}</span>
          </div>
          <div class="config-item">
            <strong>Dungeons:</strong>
            <span>{{ config!.dungeons.join(", ") || "None" }}</span>
          </div>
          <div class="config-item">
            <strong>Output Path:</strong>
            <span class="path">{{ config!.outputPath }}</span>
          </div>
          <div class="config-item">
            <strong>Clear Previous Builds:</strong>
            <span>{{ config!.clearPreviousBuilds ? "Yes" : "No" }}</span>
          </div>
        </div>
      </div>
    </section>

    <section class="action-section">
      <button
        @click="updateTalents"
        :disabled="!hasConfig || isUpdating"
        class="btn-update"
      >
        <span v-if="!isUpdating">🚀 Update Talents</span>
        <span v-else>⏳ Updating...</span>
      </button>
    </section>

    <section v-if="statusMessage || errorMessage" class="status-section">
      <div v-if="statusMessage" class="status-success">
        ✓ {{ statusMessage }}
      </div>
      <div v-if="errorMessage" class="status-error">✗ {{ errorMessage }}</div>
    </section>

    <footer>
      <p class="help-text">
        Need help? Create a
        <code>settings.json</code> file based on
        <code>settings.example.json</code>
      </p>
    </footer>
  </main>
</template>

<style scoped>
.container {
  max-width: 900px;
  margin: 0 auto;
  padding: 2rem;
}

header {
  text-align: center;
  margin-bottom: 3rem;
}

h1 {
  font-size: 2.5rem;
  margin-bottom: 0.5rem;
  color: #24c8db;
}

.subtitle {
  color: #666;
  font-size: 1.1rem;
}

.config-section,
.action-section,
.status-section {
  margin-bottom: 2rem;
}

h2 {
  font-size: 1.5rem;
  margin-bottom: 1rem;
  color: #333;
}

h3 {
  font-size: 1.2rem;
  margin-bottom: 0.5rem;
  color: #555;
}

.file-selector {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.file-path {
  font-size: 0.9rem;
  color: #666;
  font-family: monospace;
  word-break: break-all;
}

.btn-primary,
.btn-update {
  padding: 0.8rem 1.5rem;
  font-size: 1rem;
  font-weight: 600;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s;
}

.btn-primary {
  background-color: #24c8db;
  color: white;
}

.btn-primary:hover {
  background-color: #1da1b5;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(36, 200, 219, 0.3);
}

.btn-update {
  background-color: #4caf50;
  color: white;
  width: 100%;
  font-size: 1.2rem;
}

.btn-update:hover:not(:disabled) {
  background-color: #45a049;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(76, 175, 80, 0.3);
}

.btn-update:disabled {
  background-color: #ccc;
  cursor: not-allowed;
  transform: none;
}

.config-display {
  background: #f9f9f9;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1.5rem;
}

.config-grid {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.config-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.config-item strong {
  color: #333;
}

.config-item ul {
  margin: 0;
  padding-left: 1.5rem;
}

.config-item li {
  margin: 0.25rem 0;
}

.config-item .path {
  font-family: monospace;
  font-size: 0.9rem;
  color: #666;
  word-break: break-all;
}

.status-section {
  padding: 1rem;
  border-radius: 8px;
}

.status-success {
  background: #e8f5e9;
  color: #2e7d32;
  padding: 1rem;
  border-radius: 8px;
  border-left: 4px solid #4caf50;
}

.status-error {
  background: #ffebee;
  color: #c62828;
  padding: 1rem;
  border-radius: 8px;
  border-left: 4px solid #f44336;
}

footer {
  text-align: center;
  margin-top: 3rem;
  padding-top: 2rem;
  border-top: 1px solid #ddd;
}

.help-text {
  color: #666;
  font-size: 0.9rem;
}

code {
  background: #f5f5f5;
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
  font-family: monospace;
}

@media (prefers-color-scheme: dark) {
  h1 {
    color: #24c8db;
  }

  h2,
  h3 {
    color: #f6f6f6;
  }

  .subtitle,
  .file-path,
  .help-text {
    color: #aaa;
  }

  .config-display {
    background: #1a1a1a;
    border-color: #444;
  }

  .config-item strong {
    color: #f6f6f6;
  }

  .config-item .path {
    color: #aaa;
  }

  code {
    background: #333;
    color: #f6f6f6;
  }

  footer {
    border-top-color: #444;
  }
}
</style>
