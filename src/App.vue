<script setup lang="ts">
import { onMounted, ref } from 'vue'
import SearchBar from './components/SearchBar.vue'
import AddonList from './components/AddonList.vue'
import { useAddons } from './composables/useAddons'
import { useAppUpdate } from './composables/useAppUpdate'

const {
  addons, searchQuery, loading, error,
  wowPath, pathMissing, initialize,
  installAddon, uninstallAddon, pickWowPath,
} = useAddons()

const {
  updateAvailable, updateVersion, updating, updateError,
  checkForUpdate, installUpdate, dismissUpdate,
} = useAppUpdate()

const pathBarOpen = ref(false)

onMounted(() => {
  initialize()
  checkForUpdate()
})
</script>

<template>
  <div class="app">
    <div v-if="updateAvailable" class="update-banner">
      <span v-if="!updating">
        Version <strong>{{ updateVersion }}</strong> available
      </span>
      <span v-else>Updating...</span>
      <span v-if="updateError" class="update-error">{{ updateError }}</span>
      <div v-if="!updating" class="update-actions">
        <button class="btn-update" @click="installUpdate">Update & Restart</button>
        <button class="btn-dismiss" @click="dismissUpdate">Later</button>
      </div>
    </div>
    <header class="header">
      <h1>Skyliss - Addon Manager</h1>
      <div class="path-row">
        <div class="path-bar" :class="{ open: pathBarOpen }" @click="pickWowPath">
          <div v-if="wowPath" class="path-info">
            <span class="path-label">AddOns:</span>
            <span class="path-value">{{ wowPath }}</span>
          </div>
          <div v-else class="path-info path-missing">
            <span class="path-label">No AddOns folder configured</span>
          </div>  
        </div>
        <button class="btn-toggle-path" @click="pathBarOpen = !pathBarOpen">
          Game Path
        </button>
      </div>
    </header>
    <main class="main">
      <div v-if="pathMissing" class="path-prompt">
        <p>To get started, select your WoW <strong>AddOns</strong> folder.</p>
        <p class="path-hint">
          It's usually located at:<br />
          <code>World of Warcraft/_retail_/Interface/AddOns</code>
        </p>
        <button class="btn-browse" @click="pickWowPath">Browse...</button>
      </div>
      <template v-else>
        <SearchBar v-model="searchQuery" />
        <div v-if="error" class="error">{{ error }}</div>
        <AddonList
          :addons="addons"
          :loading="loading"
          @install="installAddon"
          @uninstall="uninstallAddon"
        />
      </template>
    </main>
  </div>
</template>

<style scoped>
.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  padding: 1.5rem 2rem;
  border-bottom: 1px solid #2a2a2a;
}

.header h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: #fff;
}

.path-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-top: 0.75rem;
}

.btn-toggle-path {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 0.4rem;
  height: 2rem;
  padding: 0.35rem 0.75rem;
  font-size: 0.8rem;
  font-weight: 500;
  background: #2a2a2a;
  color: #888;
  border: 1px solid #3a3a3a;
  border-radius: 6px;
  cursor: pointer;
  transition: border-color 0.2s, color 0.2s;
}

.btn-toggle-path:hover {
  border-color: #646cff;
  color: #fff;
}

.path-bar {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0.75rem;
  background: #1a1a1a;
  border: 1px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  transform: scaleX(0);
  transform-origin: right center;
  opacity: 0;
  transition: transform 0.4s cubic-bezier(0.34, 1.56, 0.64, 1),
              opacity 0.25s cubic-bezier(0.34, 1.56, 0.64, 1),
              border-color 0.2s;
}

.path-bar:hover {
  border-color: #3a3a3a;
}

.path-bar.open {
  transform: scaleX(1);
  opacity: 1;
}

.path-info {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.85rem;
  color: #888;
}

.path-missing .path-label {
  color: #fbbf24;
}

.path-label {
  color: #666;
  margin-right: 0.5rem;
}

.path-value {
  color: #aaa;
}

.main {
  flex: 1;
  padding: 1.5rem 2rem;
  display: flex;
  flex-direction: column;
}

.path-prompt {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  padding: 3rem 2rem;
  text-align: center;
  color: #aaa;
}

.path-prompt p {
  margin: 0;
  font-size: 1rem;
  line-height: 1.5;
}

.path-hint {
  font-size: 0.9rem;
  color: #666;
}

.path-hint code {
  display: inline-block;
  margin-top: 0.25rem;
  padding: 0.2rem 0.5rem;
  background: #1a1a1a;
  border-radius: 4px;
  font-size: 0.85rem;
  color: #888;
}

.btn-browse {
  padding: 0.6rem 1.5rem;
  font-size: 0.95rem;
  font-weight: 500;
  background: #646cff;
  color: #fff;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn-browse:hover {
  opacity: 0.9;
}

.error {
  margin-bottom: 1rem;
  padding: 0.75rem 1rem;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid #ef4444;
  border-radius: 8px;
  color: #ef4444;
  font-size: 0.9rem;
}

.update-banner {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.6rem 1.25rem;
  background: rgba(100, 108, 255, 0.12);
  border-bottom: 1px solid rgba(100, 108, 255, 0.3);
  font-size: 0.85rem;
  color: #c4c7ff;
}

.update-actions {
  display: flex;
  gap: 0.5rem;
  margin-left: auto;
}

.btn-update {
  padding: 0.3rem 0.75rem;
  font-size: 0.8rem;
  font-weight: 500;
  background: #646cff;
  color: #fff;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn-update:hover {
  opacity: 0.85;
}

.btn-dismiss {
  padding: 0.3rem 0.75rem;
  font-size: 0.8rem;
  background: transparent;
  color: #888;
  border: 1px solid #3a3a3a;
  border-radius: 5px;
  cursor: pointer;
  transition: border-color 0.2s, color 0.2s;
}

.btn-dismiss:hover {
  border-color: #646cff;
  color: #fff;
}

.update-error {
  color: #ef4444;
  font-size: 0.8rem;
}
</style>
