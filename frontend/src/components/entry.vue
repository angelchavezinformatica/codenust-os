<script setup lang="ts">
import type { EntryI } from '@/types/entry'
import { computed, onMounted, ref, useTemplateRef } from 'vue'

interface Props extends EntryI {
  getNext: () => string | undefined
  getPrev: () => string | undefined
}

const props = defineProps<Props>()

const path = computed(() => {
  return props.path.replace(`/home/${props.user}`, '~')
})

const value = ref<string>(props.value)
const inputRef = useTemplateRef('inputRef')

const processKeyDown = async (e: KeyboardEvent) => {
  if (e.code === 'Enter') {
    await props.process(value.value)
  } else if (e.code === 'ArrowUp') {
    value.value = props.getPrev() || ''
  } else if (e.code === 'ArrowDown') {
    value.value = props.getNext() || ''
  }
}

onMounted(() => {
  inputRef.value?.focus()
})
</script>

<template>
  <div class="flex gap-2">
    <span v-if="isShell">[{{ user }}@codenus-os {{ path }}] ></span>

    <input
      v-if="isEditing"
      v-model="value"
      ref="inputRef"
      type="text"
      class="outline-0 flex-1"
      @keydown="processKeyDown"
    />
    <span v-else>{{ value }}</span>
  </div>
</template>
