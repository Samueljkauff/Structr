<template>
  <div class="fixed top-4 right-4 z-50 w-72">
    <div v-if="selectedNode" class="flex flex-col gap-3">
      <p class="text-white text-2xl overflow-hidden truncate">Folder: {{ selectedNode.data.label }}</p>
      <p class="text-gray-400">What files belongs in {{ selectedNode.data.label }}?</p>
      <input
        v-model="contextText"
        class="input"
        type="text"
        placeholder="Enter context (max 50 chars)"
        maxlength="100"
      />
      <div class="text-gray-400">Description: <p class="inline" :class="selectedNode.data.description ? 'text-green-600' : 'text-red-600'">{{ selectedNode.data.description ? selectedNode.data.description : 'None' }}</p></div>
      <div class="w-full flex justify-end">
        <button
          v-if="selectedNode.data.description"
          @click="removeContext"
          class="mr-2 rounded-xl border border-red-200 text-red-200 backdrop-blur-xs hover:bg-red-200 hover:text-[#091413] py-2 w-20 cursor-pointer"
        >
          Clear
        </button>
        <button
          @click="addContext"
          class="rounded-xl border border-[#B0E4CC] text-[#B0E4CC] backdrop-blur-xs hover:bg-[#B0E4CC] hover:text-[#091413] py-2 w-20 cursor-pointer"
        >
          Add
        </button>
      </div>
    </div>
    <div class="flex flex-col gap-2" v-else>
      <p class="text-white text-3xl">Welcome to Structrs!</p>
      <p class="text-gray-400">
        Click a node to begin exploring your folder structure and giving nodes
        context!
      </p>
    </div>
  </div>
</template>

<script lang="ts">
import type { Node } from "@vue-flow/core";

export default {
  data() {
    return {
      contextText: "",
    };
  },
  props: {
    selectedNode: {
      type: Object as () => Node | null,
      required: false,
      default: {},
    },
  },
  methods: {
    addContext() {
      this.$emit("add-context", this.contextText);
      this.contextText = "";
    },
    removeContext() {
      this.$emit("remove-context");
    },
  },
};
</script>
