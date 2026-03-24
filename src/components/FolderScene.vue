<template>
  <VueFlow
    @node-click="handleNodeClick"
    :nodes="nodes"
    :edges="edges"
    :nodes-draggable="false"
  >
  <ContextMenu :selectedNode="selectedNode" />
    <div class="fixed bottom-4 left-4 text-white z-50">
      <p>Current Path: {{ selectedPath }}</p>
    </div>
    <MiniMap />
    <Background :variant="BackgroundVariant.Dots" />
    <Controls
      position="top-left"
      :show-fit-view="true"
      :show-interactive="false"
    />
    <Dialog :dialog="dialog" :selectedNode="selectedNode" @close-dialog="closeDialog" />
  </VueFlow>
</template>

<script lang="ts">
import { basename, homeDir } from "@tauri-apps/api/path";
import { useVueFlow, VueFlow } from "@vue-flow/core";
import type { Edge, Node, NodeMouseEvent } from "@vue-flow/core";
import { FolderNode } from "../interfaces/FolderNode";
import { invoke } from "@tauri-apps/api/core";
import {
  Background,
  BackgroundVariant,
  MiniMap,
} from "@vue-flow/additional-components";
import { Controls } from "@vue-flow/controls";
import ContextMenu from "./ContextMenu.vue";
import Dialog from "./Dialog.vue";

export default {
  data() {
    return {
      edges: [] as Edge[],
      nodes: [] as Node[],
      dialog: false,
      selectedNode: "",
      selectedPath: "",
      BackgroundVariant,
    };
  },
  async mounted() {
    const rootPath = await homeDir();
    const rootName = await basename(rootPath);

    const homeNode: Node = {
      id: rootPath,
      position: { x: 0, y: 0 },
      data: {
        label: "Home",
        folder: { path: rootPath, name: rootName },
        layer: 1,
      },
    };
    this.nodes = [homeNode];
  },
  setup() {
    useVueFlow();
    return {};
  },
  methods: {
    async handleNodeClick({ node }: NodeMouseEvent) {
      this.selectedPath = node.id as string;
      const layer = node.data.layer + 1;
      this.selectedNode = node.data.label;

      const children = await invoke<FolderNode[]>("load_children", {
        root: this.selectedPath,
      });

      this.pruneLayers(node.data.layer);

      const childSpacing = 125;
      const totalWidth = (children.length - 1) * childSpacing;
      const startX = -totalWidth / 2;
      const Y = layer * 100;

      if (children.length <= 20) {
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
    pruneLayers(layer: number) {
      this.nodes = this.nodes.filter((n) => n.data.layer <= layer);

      const validNodeIds = new Set(this.nodes.map((n) => n.id));
      this.edges = this.edges.filter(
        (e) => validNodeIds.has(e.source) && validNodeIds.has(e.target),
      );
    },
    handleDialog() {
      this.dialog = true;
    },
    closeDialog() {
      this.dialog = false;
    },
  },
  emits: ['closeDialog'],
  components: {
    VueFlow,
    Background,
    MiniMap,
    Controls,
    ContextMenu,
    Dialog,
  },
};
</script>
