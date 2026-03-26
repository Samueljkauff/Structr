<template>
  <VueFlow
    @node-click="handleNodeClick"
    :nodes="nodes"
    :edges="edges"
    :nodes-draggable="false"
  >
    <ContextMenu :selectedNode="selectedNode" />
    <MiniMap />
    <Background :variant="BackgroundVariant.Dots" />
    <Controls
      position="top-left"
      :show-fit-view="true"
      :show-interactive="false"
    />
    <div class="fixed bottom-4 left-4 text-white z-50">
      <p>Current Path: {{ selectedPath }}</p>
    </div>
    <Dialog
      :dialog="dialog"
      :selectedNode="selectedNode"
      :dialogChildren="dialogChildren"
      @close-dialog="closeDialog"
      @add-node="addNode"
    />
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
      selectedNode: null as Node | null,
      selectedPath: "",
      dialogChildren: [] as FolderNode[],
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
        layer: 0,
        description: "",
      },
    };
    this.nodes = [homeNode];

    const nodeWidth = 96;
    const nodeHeight = 96;

    this.$nextTick(() => {
      this.setCenter(
        homeNode.position.x + nodeWidth / 2,
        homeNode.position.y + nodeHeight / 2,
        { zoom: 1, duration: 0 },
      );
    });
  },
  setup() {
    const { setCenter } = useVueFlow();
    return { setCenter };
  },
  methods: {
    async handleNodeClick({ node }: NodeMouseEvent) {
      this.selectedPath = node.id as string;
      const layer = node.data.layer + 1;
      this.selectedNode = node;

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

      this.handleDialog(children);
    },
    pruneLayers(layer: number) {
      this.nodes = this.nodes.filter((n) => n.data.layer <= layer);

      const validNodeIds = new Set(this.nodes.map((n) => n.id));
      this.edges = this.edges.filter(
        (e) => validNodeIds.has(e.source) && validNodeIds.has(e.target),
      );
    },
    handleDialog(children: FolderNode[]) {
      this.dialog = true;
      this.dialogChildren = children;
    },
    closeDialog() {
      this.dialog = false;
    },
    addNode(folder: FolderNode) {
      const parentNode = this.nodes.find(
        (n) => n.data.label === this.selectedNode,
      );
      if (!parentNode) return;

      const layer = parentNode.data.layer + 1;
      const newX = 0;
      const newY = layer * 100;

      const newNode: Node = {
        id: folder.path,
        position: { x: newX, y: newY },
        data: {
          label: folder.name,
          folder,
          layer,
        },
      };

      const newEdge: Edge = {
        id: `${parentNode.id}-${newNode.id}`,
        source: parentNode.id,
        target: newNode.id,
        type: "smoothstep",
      };

      this.nodes.push(newNode);
      this.edges.push(newEdge);
      this.closeDialog();
    },
  },
  emits: ["closeDialog", "addNode"],
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
