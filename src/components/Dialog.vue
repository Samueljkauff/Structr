<template>
  <div v-if="dialog" class="dialog dialog-animation">
    <div
      @click="closeDialog"
      class="flex justify-center rounded-xl border border-[#B0E4CC] text-[#B0E4CC] h-10 w-10 hover:bg-[#B0E4CC] hover:text-[#091413] cursor-pointer"
    >
      <button class="cursor-pointer">X</button>
    </div>
    <div class="flex justify-between items-center mb-1 mt-2">
      <p class="text-white ml-1 text-3xl">
        <img class="mb-1 inline h-10 w-10" src="../icons/folder_icon.png" />
        {{ selectedNode }}/
      </p>
      <input
        v-model="search"
        type="text"
        class="input h-10"
        placeholder="Search for a Folder"
      />
    </div>
    <div class="grid grid-cols-12 max-h-228 overflow-y-auto p-4">
      <div
        @click="selectNode(child)"
        class="dialog-node m-2 cursor-pointer"
        v-for="child in filteredChildren"
        :key="child.path"
      >
        {{ child.name }}
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { FolderNode } from "../interfaces/FolderNode";

export default {
  data() {
    return {
      search: "",
    };
  },
  props: {
    dialog: {
      type: Boolean,
      required: true,
      default: false,
    },
    selectedNode: {
      type: String,
      required: true,
      default: "",
    },
    dialogChildren: {
      type: Array as () => FolderNode[],
      required: true,
      default: () => [],
    },
  },
  methods: {
    closeDialog() {
      this.search = "";
      this.$emit("close-dialog");
    },
    selectNode(node: FolderNode) {
      this.search = "";
      this.$emit("add-node", node);
    },
  },
  computed: {
    filteredChildren(): FolderNode[] {
      if (!this.search) return this.dialogChildren;
      return this.dialogChildren.filter((child) =>
        child.name.toLowerCase().includes(this.search.toLowerCase()),
      );
    },
  },
};
</script>
