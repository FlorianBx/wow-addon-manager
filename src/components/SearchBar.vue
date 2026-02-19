<script setup lang="ts">
defineProps<{
  modelValue: string
}>()

defineEmits<{
  'update:modelValue': [value: string]
}>()
</script>

<template>
  <div class="search-bar">
    <input
      type="text"
      :value="modelValue"
      @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
      placeholder="Search addons..."
    />
  </div>
</template>

<style scoped>
.search-bar {
  margin-bottom: 1.5rem;
}

.search-bar input {
  width: 100%;
  padding: 0.75rem 1rem;
  font-size: 1rem;
  border: 1px solid #3a3a3a;
  border-radius: 8px;
  background: #1a1a1a;
  color: #fff;
  outline: none;
  transition: border-color 0.2s;
}

.search-bar input:focus {
  border-color: #646cff;
}

.search-bar input::placeholder {
  color: #666;
}
</style>
