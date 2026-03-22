<template>
  <VueFlow :nodes="nodes" :edges="edges" :nodes-draggable="false">
    <MiniMap />
    <Background :variant="BackgroundVariant.Dots" />
    <Controls position="top-left" :show-fit-view="true" :show-interactive="false" />
  </VueFlow>
</template>

<script lang="ts">
import { homeDir } from "@tauri-apps/api/path";
import { useVueFlow, VueFlow } from "@vue-flow/core";
import type { Edge, Node } from "@vue-flow/core";
import { FolderNode } from "../interfaces/FolderNode";
import { invoke } from "@tauri-apps/api/core";
import { Background, BackgroundVariant, MiniMap } from '@vue-flow/additional-components';
import { Controls } from '@vue-flow/controls';


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

    const homeNode: Node = {
      id: rootPath,
      position: { x: 0, y: 0 },
      data: { label: "Home", folder: { path: rootPath, name: "Home" } },
    };

    const childSpacing = 125;
    const totalWidth = (homeNodes.length - 1) * childSpacing;
    const startX = -totalWidth / 2;

    const childNodes: Node[] = homeNodes.map((folder, index) => ({
      id: folder.path,
      position: {
        x: startX + index * childSpacing,
        y: 250,
      },
      data: { label: folder.name, folder },
    }));

    this.nodes = [homeNode, ...childNodes];

    this.edges = childNodes.map((child) => ({
      id: `${rootPath}-${child.id}`,
      source: rootPath,
      target: child.id,
      type: "smoothstep",
    }));
  },
  setup() {
    useVueFlow();
    return {};
  },
  components: {
    VueFlow,
    Background,
    MiniMap,
    Controls,
  },
};
</script>
