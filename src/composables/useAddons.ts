import { ref, computed } from 'vue'
import { refDebounced } from '@vueuse/core'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import type { Addon, InstalledAddon, AddonWithStatus } from '../types/addon'

const addons = ref<Addon[]>([])
const installedAddons = ref<InstalledAddon[]>([])
const searchQuery = ref('')
const debouncedQuery = refDebounced(searchQuery, 300)
const loading = ref(false)
const error = ref<string | null>(null)
const wowPath = ref<string | null>(null)
const pathMissing = ref(false)

export function useAddons() {
  const addonsWithStatus = computed<AddonWithStatus[]>(() => {
    return addons.value.map(addon => {
      const installed = installedAddons.value.find(i => i.id === addon.id)
      if (!installed) {
        return { ...addon, status: 'not-installed' as const }
      }
      if (installed.version !== addon.version) {
        return { ...addon, status: 'update-available' as const, installedVersion: installed.version }
      }
      return { ...addon, status: 'installed' as const, installedVersion: installed.version }
    })
  })

  const filteredAddons = computed(() => {
    if (!debouncedQuery.value) return addonsWithStatus.value
    const query = debouncedQuery.value.toLowerCase()
    return addonsWithStatus.value.filter(
      addon =>
        addon.name.toLowerCase().includes(query) ||
        addon.description.toLowerCase().includes(query)
    )
  })

  async function loadWowPath() {
    try {
      wowPath.value = await invoke<string>('get_wow_path')
      pathMissing.value = false
    } catch {
      wowPath.value = null
      pathMissing.value = true
    }
  }

  async function pickWowPath() {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select WoW AddOns folder',
    })
    if (!selected) return

    try {
      await invoke('set_wow_path', { path: selected })
      wowPath.value = selected
      pathMissing.value = false
      error.value = null
    } catch (e) {
      error.value = e as string
    }
  }

  async function fetchManifest() {
    loading.value = true
    error.value = null
    try {
      addons.value = await invoke<Addon[]>('fetch_manifest')
    } catch (e) {
      error.value = e as string
    } finally {
      loading.value = false
    }
  }

  async function loadInstalled() {
    try {
      installedAddons.value = await invoke<InstalledAddon[]>('get_installed_addons')
    } catch (e) {
      console.error('Failed to load installed addons:', e)
    }
  }

  async function installAddon(addonId: string) {
    const addon = addons.value.find(a => a.id === addonId)
    if (!addon) return

    try {
      await invoke('install_addon', { addon })
      await loadInstalled()
    } catch (e) {
      error.value = e as string
    }
  }

  async function uninstallAddon(addonId: string) {
    try {
      await invoke('uninstall_addon', { addonId })
      await loadInstalled()
    } catch (e) {
      error.value = e as string
    }
  }

  async function initialize() {
    await loadWowPath()
    await Promise.all([fetchManifest(), loadInstalled()])
  }

  return {
    addons: filteredAddons,
    searchQuery,
    loading,
    error,
    wowPath,
    pathMissing,
    initialize,
    installAddon,
    uninstallAddon,
    pickWowPath,
  }
}
