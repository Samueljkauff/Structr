<template>
  <VueFlow
    @node-click="handleNodeClick"
    :nodes="nodes"
    :edges="edges"
    :nodes-draggable="false"
  >
    <div class="fixed bottom-4 left-4 text-white z-50">
      <p>Current Path: {{ selectedPath }}</p>
    </div>
    <div class="fixed top-4 right-4 z-50 w-72">
  <div v-if="selectedNode" class="flex flex-col gap-3">
    <p class="text-white text-2xl">Folder: {{ selectedNode }}</p>
    <input
      v-model="contextText"
      class="backdrop-blur-xs text-white border border-[#B0E4CC] rounded-xl px-3 py-2"
      type="text"
      placeholder="Enter context (max 50 chars)"
      maxlength="50"
    />
    <div class="w-full flex justify-end">
      <button
        @click="addContext"
        class="rounded-xl border border-[#B0E4CC] text-[#B0E4CC] backdrop-blur-xs hover:bg-[#B0E4CC] hover:text-[#091413] py-2 w-20 cursor-pointer"
      >
        Add
      </button>
    </div>
  </div>
</div>
    <div v-if="dialog" class="dialog dialog-animation">
      <div
        @click="closeDialog"
        class="flex justify-center rounded-xl border border-[#B0E4CC] text-[#B0E4CC] h-10 w-10 hover:bg-[#B0E4CC] hover:text-[#091413] cursor-pointer"
      >
        <button class="cursor-pointer">X</button>
      </div>
      <div class="flex justify-items-end align-middle">
      <img class="mt-9 ml-3 mb-3 inline h-10 w-10" src="../icons/folder_icon.png"/>
      <p class="text-white mt-10 ml-1 mb-2 text-3xl">{{ selectedNode }}/</p>
      </div>
      <div
        class="flex-1 border rounded-xl border-[#B0E4CC] overflow-auto"
      ></div>
    </div>
    <MiniMap />
    <Background :variant="BackgroundVariant.Dots" />
    <Controls
      position="top-left"
      :show-fit-view="true"
      :show-interactive="false"
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

export default {
  data() {
    return {
      edges: [] as Edge[],
      nodes: [] as Node[],
      dialog: false,
      selectedNode: "",
      selectedPath: "",
      contextText: "",
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
    addContext() {

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
