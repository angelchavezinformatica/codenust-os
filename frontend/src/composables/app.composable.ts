import { WS_BACKEND } from '@/constants'
import type { EntryI } from '@/types/entry'
import type { ResponseWS } from '@/types/ws'
import { nextTick, reactive, ref } from 'vue'

export const useAppComposable = () => {
  const user = ref<string>('')
  const path = ref<string>('')
  let buffer = reactive<EntryI[]>([])
  const isInit = ref<boolean>(false)

  const history = ref<string[]>([])
  const historyIndex = ref<number>(-1)

  const ws = new WebSocket(WS_BACKEND)

  ws.onmessage = (e: MessageEvent<string>) => {
    const data = JSON.parse(e.data) as ResponseWS

    path.value = data.path

    if (data.type === 'message') {
      const lines = data.data.split('\n')
      lines.map((line) => createEntry(line, false, false))
      createEntry()
    }

    if (data.type === 'init') {
      user.value = data.data.user
      document.title = `${user.value}@codenust-os`
      if (!isInit.value) {
        createEntry()
        isInit.value = true
      }
    }
  }

  const sendMessage = async (message: string) => {
    if (message === 'clear') {
      buffer.splice(0)
      await nextTick()
      createEntry()
    } else ws.send(message)
  }

  const processEntry = async (value: string) => {
    const index = buffer.length - 1
    if (buffer[index]) buffer[index].isEditing = false

    if (value.trim()) {
      history.value.push(value)
      historyIndex.value = history.value.length
    }

    await sendMessage(value)
  }

  const createEntry = (value: string = '', isShell: boolean = true, isEditing: boolean = true) => {
    buffer.push({
      value,
      isShell,
      isEditing,
      path: path.value,
      user: user.value,
      process: processEntry,
    })
  }

  const getPrevCommand = () => {
    if (history.value.length === 0) return ''
    if (historyIndex.value > 0) historyIndex.value--
    return history.value[historyIndex.value] ?? ''
  }

  const getNextCommand = () => {
    if (history.value.length === 0) return ''
    if (historyIndex.value < history.value.length - 1) {
      historyIndex.value++
      return history.value[historyIndex.value]
    }
    historyIndex.value = history.value.length
    return ''
  }

  return {
    buffer,
    createEntry,
    getPrevCommand,
    getNextCommand,
  }
}
