<template>
  <VueFlow @node-click="handleNodeClick" :nodes="nodes" :edges="edges" :nodes-draggable="false">
    <div v-if="dialog" class="dialog">
      <button @click="closeDialog">X</button>
    </div>
    <MiniMap />
    <Background :variant="BackgroundVariant.Dots" />
    <Controls position="top-left" :show-fit-view="true" :show-interactive="false" />
  </VueFlow>
</template>

<script lang="ts">
import { homeDir } from "@tauri-apps/api/path";
import { useVueFlow, VueFlow } from "@vue-flow/core";
import type { Edge, Node, NodeMouseEvent } from "@vue-flow/core";
import { FolderNode } from "../interfaces/FolderNode";
import { invoke } from "@tauri-apps/api/core";
import { Background, BackgroundVariant, MiniMap } from '@vue-flow/additional-components';
import { Controls } from '@vue-flow/controls';


export default {
  data() {
    return {
      edges: [] as Edge[],
      nodes: [] as Node[],
      dialog: false,
      BackgroundVariant,
    };
  },
  async mounted() {
    const rootPath = await homeDir();

    const homeNode: Node = {
      id: rootPath,
      position: { x: 0, y: 0 },
      data: { label: "Home", folder: { path: rootPath, name: "Home" }, layer: 1 },
    };
    this.nodes = [homeNode];
  },
  setup() {
    useVueFlow();
    return {};
  },
  methods: {
    async handleNodeClick({ node }: NodeMouseEvent) {
      const path = node.id as string;
      const layer = node.data.layer + 1;
      const children = await invoke<FolderNode[]>("load_children", {root:path});

      const childSpacing = 125;
      const totalWidth = (children.length - 1) * childSpacing;
      const startX = -totalWidth / 2;
      const Y = layer*100;

      if(children.length <= 20) {
        const childNodes: Node[] = children.map((folder, index) => ({
          id: folder.path,
          position: {
            x: startX + index * childSpacing,
            y: Y,
          },
          data: { label: folder.name, folder, layer },
        }));

        const childEdges: Edge[] = childNodes.map((child) => ({
          id: `${node.id}-${child.id}`,
          source: node.id,
          target: child.id,
          type: "smoothstep",
        }));

      this.nodes = [...this.nodes, ...childNodes];
      this.edges = [...this.edges, ...childEdges];

      return;
      }

      this.handleDialog();
    },
    handleDialog() {
      this.dialog = true;
    },
    closeDialog() {
      this.dialog = false;
    },
  },
  components: {
    VueFlow,
    Background,
    MiniMap,
    Controls,
  },
};
</script>
