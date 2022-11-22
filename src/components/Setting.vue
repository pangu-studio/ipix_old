<template>
    <div>
        七牛云存储配置
    </div>
    <p />
    <el-row>
        <el-col :span="2">
            <span>AK:</span>
        </el-col>
        <el-col :span="22">
            <el-input class="w-50 m-2" v-model="qiniuSettings.ak" placeholder="qiniu ak" />
        </el-col>
    </el-row>
    <p />
    <el-row>
        <el-col :span="2">
            <span>SK:</span>
        </el-col>
        <el-col :span="22">
            <el-input v-model="qiniuSettings.sk" placeholder="qiniu sk" />
        </el-col>
    </el-row>
    <p />
    <el-row>
        <el-col :span="2">
            <span>Bucket:</span>
        </el-col>
        <el-col :span="22">
            <el-input v-model="qiniuSettings.bucket" placeholder="qiniu bucket" />
        </el-col>
    </el-row>
    <p />
    <el-row>
        <el-col :span="2">
            <span>存储路径前缀:</span>
        </el-col>
        <el-col :span="22">
            <el-input v-model="qiniuSettings.prefix" placeholder="prefix" />
        </el-col>
    </el-row>
    <p />
    <el-row>
        <el-col :span="2">
            <span>域名:</span>
        </el-col>
        <el-col :span="22">
            <el-input v-model="qiniuSettings.host" placeholder="host" />
        </el-col>
    </el-row>
    <p />
    <el-button type="primary" @click="saveAkSk">保存</el-button>
    <!-- <el-button @click="get">查询</el-button> -->

</template>

<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { Store } from 'tauri-plugin-store-api';
import { appDir } from '@tauri-apps/api/path';

interface QiniuSettings { ak: '', sk: '', bucket: '', prefix: '', host: '' }
let qiniuSettings = ref<QiniuSettings>({ ak: '', sk: '', bucket: '', prefix: '', host: '' });

let store: Store;
async function initStore() {
    if (store == undefined || store == null) {
        const appDirPath = await appDir();
        store = new Store(appDirPath + import.meta.env.APP_DB_NAME);
    }
}
const qiniuTokenKey = "qiniu:token"
async function saveAkSk() {
    await initStore();
    await store.set(qiniuTokenKey, {
        ak: qiniuSettings.value.ak, sk: qiniuSettings.value.sk, bucket: qiniuSettings.value.bucket,
        prefix: qiniuSettings.value.prefix, host: qiniuSettings.value.host
    });
    const val = await store.get(qiniuTokenKey);
    console.log(val);
}
async function get() {
    await initStore();
    let val = await store.get<QiniuSettings>(qiniuTokenKey);
    console.log(val);
    if (val) {
        qiniuSettings.value = val;
    }
}
onMounted(() => {
    get();
})

</script>