<script setup lang="ts">
import type { AddonWithStatus } from '../types/addon'
import defaultIcon from '../assets/skyliss_fr.png'

defineProps<{
  addon: AddonWithStatus
}>()

defineEmits<{
  install: []
  uninstall: []
}>()
</script>

<template>
  <div class="addon-card">
    <img
      :src="addon.icon || defaultIcon"
      :alt="addon.name"
      class="addon-icon"
      @error="($event.target as HTMLImageElement).src = defaultIcon"
    />
    <div class="addon-info">
      <div class="addon-header">
        <h3 class="addon-name">{{ addon.name }}</h3>
        <span class="addon-version">v{{ addon.version }}</span>
      </div>
      <p class="addon-description">{{ addon.description }}</p>
      <div class="addon-status" :class="addon.status">
        <template v-if="addon.status === 'installed'">
          Installed
        </template>
        <template v-else-if="addon.status === 'update-available'">
          Update available ({{ addon.installedVersion }} → {{ addon.version }})
        </template>
        <template v-else>
          Not installed
        </template>
      </div>
    </div>
    <div class="addon-actions">
      <button
        v-if="addon.status === 'not-installed'"
        class="btn btn-install"
        @click="$emit('install')"
      >
        Install
      </button>
      <button
        v-else-if="addon.status === 'update-available'"
        class="btn btn-update"
        @click="$emit('install')"
      >
        Update
      </button>
      <button
        v-if="addon.status !== 'not-installed'"
        class="btn btn-uninstall"
        @click="$emit('uninstall')"
      >
        Uninstall
      </button>
    </div>
  </div>
</template>

<style scoped>
.addon-card {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  padding: 1rem;
  background: #1a1a1a;
  border: 1px solid #2a2a2a;
  border-radius: 12px;
  transition: border-color 0.2s;
}

.addon-card:hover {
  border-color: #3a3a3a;
}

.addon-icon {
  width: 64px;
  height: 64px;
  border-radius: 8px;
  object-fit: cover;
  flex-shrink: 0;
}

.addon-info {
  flex: 1;
  min-width: 0;
}

.addon-header {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
  margin-bottom: 0.25rem;
}

.addon-name {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: #fff;
}

.addon-version {
  font-size: 0.85rem;
  color: #888;
}

.addon-description {
  margin: 0 0 0.5rem 0;
  font-size: 0.9rem;
  color: #aaa;
  line-height: 1.4;
}

.addon-status {
  font-size: 0.8rem;
  font-weight: 500;
}

.addon-status.installed {
  color: #4ade80;
}

.addon-status.update-available {
  color: #fbbf24;
}

.addon-status.not-installed {
  color: #666;
}

.addon-actions {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex-shrink: 0;
}

.btn {
  padding: 0.5rem 1rem;
  font-size: 0.85rem;
  font-weight: 500;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn:hover {
  opacity: 0.9;
}

.btn-install {
  background: #646cff;
  color: #fff;
}

.btn-update {
  background: #fbbf24;
  color: #000;
}

.btn-uninstall {
  background: transparent;
  border: 1px solid #666;
  color: #aaa;
}

.btn-uninstall:hover {
  border-color: #ef4444;
  color: #ef4444;
}
</style>
