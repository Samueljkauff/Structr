<template>
  <VueFlow :nodes="nodes"></VueFlow>
</template>

<script lang="ts">
import { VueFlow } from '@vue-flow/core';
import { FolderNode } from "../interfaces/FolderNode";

export default {
  data() {
    return {
      
    }
  },
  props: {
    nodes: {
      type: Object,
      required: true,
    }
  }
}
</script>