import { ref } from 'vue'
import { check, type Update } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'

const updateAvailable = ref(false)
const updateVersion = ref('')
const updating = ref(false)
const updateProgress = ref(0)
const updateError = ref<string | null>(null)

export function useAppUpdate() {
  async function checkForUpdate() {
    try {
      const update: Update | null = await check()
      if (update) {
        updateAvailable.value = true
        updateVersion.value = update.version
      }
    } catch (e) {
      console.error('Update check failed:', e)
    }
  }

  async function installUpdate() {
    updating.value = true
    updateError.value = null
    try {
      const update = await check()
      if (!update) return

      await update.downloadAndInstall((event) => {
        if (event.event === 'Started' && event.data.contentLength) {
          updateProgress.value = 0
        } else if (event.event === 'Progress') {
          updateProgress.value += event.data.chunkLength
        } else if (event.event === 'Finished') {
          updateProgress.value = 100
        }
      })

      await relaunch()
    } catch (e) {
      updateError.value = e instanceof Error ? e.message : String(e)
      updating.value = false
    }
  }

  function dismissUpdate() {
    updateAvailable.value = false
  }

  return {
    updateAvailable,
    updateVersion,
    updating,
    updateProgress,
    updateError,
    checkForUpdate,
    installUpdate,
    dismissUpdate,
  }
}
