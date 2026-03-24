<template>
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
</template>

<script lang="ts">
    export default {
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
            }
        },
        methods: {
            closeDialog() {
                this.$emit('closeDialog');
            }
        }
    }
</script>