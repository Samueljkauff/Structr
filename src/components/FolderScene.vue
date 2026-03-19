<template>
  <VueFlow :nodes="nodes" :edges="edges"></VueFlow>
</template>

<script lang="ts">
import { homeDir } from "@tauri-apps/api/path";
import { VueFlow } from "@vue-flow/core";
import type { Edge, Node } from "@vue-flow/core";
import { FolderNode } from "../interfaces/FolderNode";
import { invoke } from "@tauri-apps/api/core";

export default {
  data() {
    return {
      edges: [] as Edge[],
      nodes: [] as Node[],
    };
  },
  async mounted() {
    const rootPath = await homeDir();
    const homeNodes = await invoke<FolderNode[]>("load_children", {
      root: rootPath,
    });
    this.nodes = homeNodes.map((folder, index) => ({
      id: folder.path,
      position: {
        x: index * 100,
        y: 250,
      },
      data: {
        label: folder.name,
        folder,
      },
    }));
  },
  components: {
    VueFlow,
  },
};
</script>
