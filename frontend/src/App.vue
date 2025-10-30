<script setup lang="ts">
import Entry from './components/entry.vue'
import { useAppComposable } from './composables/app.composable'

const app = useAppComposable()
</script>

<template>
  <div class="flex flex-col w-full gap-1 p-4">
    <Entry
      v-for="(item, index) in app.buffer"
      v-bind="item"
      :key="index"
      :get-prev="app.getPrevCommand"
      :get-next="app.getNextCommand"
    />
  </div>
</template>
