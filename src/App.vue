<template>
  <button @click="test">test</button>  
</template>

<script setup lang="ts">
import { emit as appEmit } from '@tauri-apps/api/event'

const test = () => {
  appEmit('click', {
    message: 'test'
  })
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
