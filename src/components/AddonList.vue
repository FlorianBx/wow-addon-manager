<script setup lang="ts">
import type { AddonWithStatus } from '../types/addon'
import AddonCard from './AddonCard.vue'

defineProps<{
  addons: AddonWithStatus[]
  loading: boolean
}>()

defineEmits<{
  install: [addonId: string]
  uninstall: [addonId: string]
}>()
</script>

<template>
  <div class="addon-list">
    <div v-if="loading" class="loading">
      Loading addons...
    </div>
    <div v-else-if="addons.length === 0" class="empty">
      No addons found
    </div>
    <div v-else class="grid">
      <AddonCard
        v-for="addon in addons"
        :key="addon.id"
        :addon="addon"
        @install="$emit('install', addon.id)"
        @uninstall="$emit('uninstall', addon.id)"
      />
    </div>
  </div>
</template>

<style scoped>
.addon-list {
  flex: 1;
}

.loading,
.empty {
  text-align: center;
  padding: 3rem;
  color: #666;
  font-size: 1rem;
}

.grid {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>
