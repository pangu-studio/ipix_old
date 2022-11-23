<template>
    素材库
</template>
<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { invoke } from '@tauri-apps/api/tauri';

onMounted(() => {
    invoke("list_all_media_repo").then((res) => {
        console.log(res)
    }).catch((err) => {
        console.log(err)
    }).finally(() => {
        console.log("fetch repo list finished")
    });
})
</script>