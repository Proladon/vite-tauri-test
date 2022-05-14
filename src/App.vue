<template>
  <button @click="test">test</button>  
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import { dialog } from '@tauri-apps/api'

const test = async() => {
  const res = await invoke('my_custom_command', {
    data: {
      msg1: 'hello',
      msg2: 'world'
    },
  })
  // const res = await dialog.open()
  console.log(res)
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
