<template>
  <el-select v-model="value" class="m-2" placeholder="请选择素材库" size="">
    <el-option
      v-for="item in options"
      :key="item.value"
      :label="item.label"
      :value="item.value"
    />
  </el-select>
  <!-- <el-button type="primary" style="margin-left: 0.5em">设为默认</el-button> -->
  <div class="pic-uploader">
    <!-- <h1>iPix</h1> -->
    <div class="middle-con" :class="[isHover ? 'drag-hover' : '']">
      <div class="upload-icon">
        <el-icon>
          <UploadFilled />
        </el-icon>
        <span>拖拽文件上传</span>
      </div>
    </div>
    <el-progress class="progress" :percentage="progress" />
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted, h } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { writeText } from "@tauri-apps/api/clipboard";
import { ElMessage } from "element-plus";
import "element-plus/theme-chalk/el-message.css";

import { appWindow } from "@tauri-apps/api/window";
import { UnlistenFn, listen } from "@tauri-apps/api/event";
import { Store } from "tauri-plugin-store-api";
import { appDataDir } from "@tauri-apps/api/path";

const value = ref("");

const options = [
  {
    value: "Option1",
    label: "博客",
  },
  {
    value: "oo",
    label: "音乐",
  },
];

interface QiniuSettings {
  ak: "";
  sk: "";
  bucket: "";
  prefix: "";
  host: "";
}
interface UploadReturn {
  hash: string;
  key: string;
}
let qiniuSettings = ref<QiniuSettings>({
  ak: "",
  sk: "",
  bucket: "",
  prefix: "",
  host: "",
});
const qiniuTokenKey = "qiniu:token";
let store: Store;
async function initStore() {
  if (store == undefined || store == null) {
    const appDirPath = await appDataDir();
    store = new Store(appDirPath + import.meta.env.APP_DB_NAME);
  }
}
let progress = ref<any>(0);
async function get(): Promise<QiniuSettings> {
  await initStore();
  let val = await store.get<QiniuSettings>(qiniuTokenKey);
  console.log(val);
  if (val) {
    qiniuSettings.value = val;
    return val;
  }
  return { ak: "", sk: "", bucket: "", prefix: "", host: "" };
}
let unlisten: UnlistenFn;

let isHover = ref(false);

onMounted(async () => {
  await listen("upload_process", (event) => {
    if (progress.value != event.payload) {
      progress.value = event.payload;
    }
  });
  unlisten = await appWindow.onFileDropEvent(async (event) => {
    if (event.payload.type === "hover") {
      progress.value = 0;
      isHover.value = true;
      console.log(isHover);
      console.log("User hovering", event.payload.paths);
    } else if (event.payload.type === "drop") {
      progress.value = 0;
      isHover.value = false;
      console.log("User dropped", event.payload.paths);

      //调用rust方法上传文件
      //rust上传支持大文件，同时可以回传文件上传进度
      let setting = await get();
      let retStr: string = await invoke("upload_file", {
        key: event.payload.paths[0],
        accessKey: setting.ak,
        secretKey: setting.sk,
        bucketName: setting.bucket,
        prefix: setting.prefix,
      });

      let ret: UploadReturn = JSON.parse(retStr);
      const url = setting.host + "/" + ret.key;
      await writeText(url);
      console.log(url);
      openVn();
    } else {
      console.log("File drop cancelled");
      isHover.value = false;
    }
  });
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});

const openVn = () => {
  ElMessage({
    message: "上传成功,URL：已复制到剪切板",
    type: "success",
  });
};
</script>
<style lang="scss" scoped>
.pic-uploader {
  margin-top: 1em;
  text-align: center;
  display: block;
}

.upload-icon {
  // width: 48px;
  height: 100%;
  font-size: 48px;
  display: block;
  margin-top: 50px;
  margin-left: auto;
  align-items: center;
  span {
    font-size: 12px;
    height: 16px;
    display: block;
    // // align-items: stretch;
    // justify-items: flex-end;
    // justify-content: right;
  }
}

html.dark {
  .upload-icon {
    color: green;
  }
}

.middle-con {
  border: 1.5px rgb(6, 106, 247) dashed;
  border-radius: 5px;
  height: 200px;
  width: 80%;
  margin: 0 auto;
}

.progress {
  width: 80%;
  margin: 0 auto;
  margin-top: 10px;
}

.drag-hover {
  border: 2px #409eff dashed !important;
}

html.dark {
  .middle-con {
    border: 1.5px rgb(102, 226, 99) dashed;
  }

  .drag-hover {
    border: 2px rgb(17, 187, 5) dashed !important;
  }
}
</style>
