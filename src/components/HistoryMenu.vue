<template>
  <button
    @click="openHistoryLog"
    class="fixed top-4 right-4 z-50 w-70 rounded-xl border border-[#B0E4CC] text-[#B0E4CC] backdrop-blur-xs hover:bg-[#B0E4CC] hover:text-[#091413] py-2 cursor-pointer"
  >
    View File Move History
  </button>

  <transition name="slide">
    <div
      v-if="dialogOpen"
      class="fixed top-4 right-4 h-[calc(100vh-25px)] w-74 z-100 border backdrop-blur-xl bg-[#091413]/30 border-[#B0E4CC] p-4 rounded-lg"
    >
      <div
        @click="closeHistoryLog"
        class="flex justify-center rounded-xl border border-[#B0E4CC] text-[#B0E4CC] h-10 w-10 hover:bg-[#B0E4CC] hover:text-[#091413] cursor-pointer"
      >
        <button class="cursor-pointer">X</button>
      </div>
      <div v-for="item in history" class="border-t border-t-[#B0E4CC] text-[#B0E4CC] my-4">
        <p class="text">{{ item.id }}</p>
        <p class="text">This file was moved to: {{ item.to_path }}</p>
      </div>
    </div>
  </transition>
</template>

<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { History } from '../interfaces/FileHistory';

export default {
  data() {
    return {
      dialogOpen: false,
      history : [] as History[],
    };
  },
  methods: {
    async openHistoryLog() {
      this.history = await invoke("get_recent_moves");
      this.dialogOpen = true;
    },
    closeHistoryLog() {
      this.dialogOpen = false;
    },
  },
};
</script>

<style>
.slide-enter-active,
.slide-leave-active {
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.slide-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.slide-enter-to {
  transform: translateX(0%);
  opacity: 1;
}

.slide-leave-from {
  transform: translateX(0%);
  opacity: 1;
}

.slide-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

.text {
    overflow: auto;
    overflow-wrap: break-word;
}
</style>