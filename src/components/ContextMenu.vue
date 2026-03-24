<template>
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
</template>

<script lang="ts">
    export default {
        data() {
            return {
                contextText: "",
            }
        },
        props: {
            selectedNode: {
                type: String,
                required: true,
                default: "",
            }
        },
        methods: {
            addContext() {

            },
        }
    }
</script>
