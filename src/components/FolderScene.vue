<template>
  <VueFlow :nodes="nodes" :edges="edges" :nodes-draggable="false">
    <Background :variant="BackgroundVariant.Dots" /> 
  </VueFlow>
</template>

<script lang="ts">
import { homeDir } from "@tauri-apps/api/path";
import { VueFlow } from "@vue-flow/core";
import type { Edge, Node } from "@vue-flow/core";
import { FolderNode } from "../interfaces/FolderNode";
import { invoke } from "@tauri-apps/api/core";
import { Background, BackgroundVariant } from '@vue-flow/additional-components';


export default {
  data() {
    return {
      edges: [] as Edge[],
      nodes: [] as Node[],
      BackgroundVariant,
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
        x: index * 125,
        y: 250,
      },
      data: {
        label: folder.name,
        folder,
      },
    } as Node));
  },
  components: {
    VueFlow,
    Background,
  },
};
</script>
