import { ref } from 'vue'
import { check, type Update } from '@tauri-apps/plugin-updater'

const updateAvailable = ref(false)
const latestVersion = ref('')
const updateBody = ref('')
const checking = ref(false)
const downloading = ref(false)
const downloadProgress = ref(0)
const error = ref('')
let pendingUpdate: Update | null = null
let totalContentLength = 0
let downloadedLength = 0

async function checkForUpdate() {
  checking.value = true
  error.value = ''
  try {
    const update = await check()
    if (update) {
      updateAvailable.value = true
      latestVersion.value = update.version
      updateBody.value = update.body ?? ''
      pendingUpdate = update
    } else {
      updateAvailable.value = false
      latestVersion.value = ''
      updateBody.value = ''
      pendingUpdate = null
    }
  } catch (e) {
    error.value = String(e)
    throw e
  } finally {
    checking.value = false
  }
}

async function downloadAndInstall() {
  if (!pendingUpdate) return
  downloading.value = true
  downloadProgress.value = 0
  error.value = ''
  downloadedLength = 0
  totalContentLength = 0
  try {
    await pendingUpdate.download((event) => {
      switch (event.event) {
        case 'Started':
          totalContentLength = event.data.contentLength ?? 0
          break
        case 'Progress':
          downloadedLength += event.data.chunkLength
          if (totalContentLength > 0) {
            downloadProgress.value = Math.round((downloadedLength / totalContentLength) * 100)
          }
          break
        case 'Finished':
          downloadProgress.value = 100
          break
      }
    })
    await pendingUpdate.install()
  } catch (e) {
    error.value = String(e)
    throw e
  } finally {
    downloading.value = false
  }
}

export function useUpdate() {
  return {
    updateAvailable,
    latestVersion,
    updateBody,
    checking,
    downloading,
    downloadProgress,
    error,
    checkForUpdate,
    downloadAndInstall,
  }
}
