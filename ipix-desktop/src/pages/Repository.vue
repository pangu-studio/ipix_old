<template>
  <el-row :gutter="12">
    <el-col
      v-for="repo in mediaRepositoryStore.list"
      :span="8"
      class="repo-card-container"
    >
      <el-card class="repo-card" shadow="hover">
        <el-row>
          <el-col :span="20">
            <span>{{ repo.name }}</span>
          </el-col>
          <el-col class="btn-container" :span="4">
            <el-dropdown size="small" trigger="click">
              <span class="el-dropdown-link">
                <el-icon class="el-icon--right">
                  <!-- <Setting /> -->
                  <Tools />
                </el-icon>
              </span>
              <template #dropdown>
                <el-dropdown-menu>
                  <el-dropdown-item @click="handleSetDefault">设为默认</el-dropdown-item>
                  <el-dropdown-item
                    @click="handleAddMediaRepo(saveDialogTypes.edit, repo)"
                    >编辑</el-dropdown-item
                  >
                  <el-dropdown-item divided @click="handleDeleleRepo" class="delete-drop">
                    <template #>
                      <span class="delete-btn">删除</span>
                    </template>
                  </el-dropdown-item>
                  <!-- <el-dropdown-item divided class="delete-drop">删除</el-dropdown-item> -->
                </el-dropdown-menu>
              </template>
            </el-dropdown>
          </el-col>
        </el-row>
      </el-card>
    </el-col>

    <el-col :span="8">
      <el-card
        class="repo-card"
        shadow="hover"
        style="text-align: center"
        @click="handleAddMediaRepo(saveDialogTypes.add)"
      >
        <el-icon>
          <Plus />
        </el-icon>
      </el-card>
    </el-col>
  </el-row>

  <!-- create or update media repository -->

  <el-dialog v-model="addRepoDialogFormVisible" :title="saveDialogType.title">
    <el-form :model="saveRepoForm" label-width="100px" label-position="right">
      <el-form-item label="素材库名">
        <el-input v-model="saveRepoForm.name" autocomplete="off" />
      </el-form-item>
      <el-form-item label="素材库描述">
        <el-input v-model="saveRepoForm.description" autocomplete="off" />
      </el-form-item>

      <el-form-item label="媒体类型">
        <el-select
          v-model="saveRepoForm.repo_type"
          class="m-2"
          placeholder="请选择媒体类型"
        >
          <el-option
            v-for="item in repoTypeOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
      </el-form-item>
      <el-form-item label="存储桶">
        <el-input v-model="saveRepoAdditon.bucket" autocomplete="off" />
      </el-form-item>
      <el-form-item label="外链域名">
        <el-input v-model="saveRepoAdditon.host" autocomplete="off" />
      </el-form-item>
      <el-form-item label="路径前缀">
        <el-input v-model="saveRepoAdditon.key_prefix" autocomplete="off" />
      </el-form-item>
      <el-form-item label="key生成策略">
        <el-select v-model="saveRepoAdditon.key_policy" class="m-2" placeholder="Select">
          <el-option
            v-for="item in repoKeyOptions"
            :key="item.value"
            :label="item.label"
            :value="item.value"
          />
        </el-select>
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="addRepoDialogFormVisible = false">取消</el-button>
        <el-button type="primary" @click="saveRepository"> 保存 </el-button>
      </span>
    </template>
  </el-dialog>
</template>
<script lang="ts" setup>
import { onMounted, ref } from "vue";

import { ElMessage, ElMessageBox } from "element-plus";
import { Repository, KeyPolicy, RepoType } from "@/dto/repostiory";
import { useMediaRepositoryStore } from "~/stores/repository";

const mediaRepositoryStore = useMediaRepositoryStore();

let addRepoDialogFormVisible = ref(false);

// let editForm = ref({} as Repository);

let saveRepoForm = ref({} as Repository);

interface Addition {
  key_prefix: string;
  key_policy: KeyPolicy;
  bucket: string;
  host: string;
}
let saveRepoAdditon = ref({} as Addition);
const repoKeyValue = ref("");

const repoKeyOptions = [
  {
    value: KeyPolicy.Uuid,
    label: "UUID",
  },
  {
    value: KeyPolicy.DatetimeUuid,
    label: "日期时间/UUD",
  },
  {
    value: KeyPolicy.YyyyMmDdUuid,
    label: "年/月/日/UUID",
  },
];

// const repoTypeValue = ref("");

const repoTypeOptions = [
  {
    value: RepoType.Picture,
    label: "图片",
  },
  {
    value: RepoType.Audio,
    label: "音频",
  },
  {
    value: RepoType.Video,
    label: "视频",
  },
];

// delete repo
function handleDeleleRepo() {
  ElMessageBox.prompt("危险操作，请输入素材库名确认", "删除素材库", {
    confirmButtonText: "删除",
    cancelButtonText: "取消",
    // inputPattern: /[\w!#$%&'*+/=?^_`{|}~-]+(?:\.[\w!#$%&'*+/=?^_`{|}~-]+)*@(?:[\w](?:[\w-]*[\w])?\.)+[\w](?:[\w-]*[\w])?/,
    // inputErrorMessage: "Invalid Email",
  })
    .then(({ value }) => {
      ElMessage({
        type: "success",
        message: `Your email is:${value}`,
      });
    })
    .catch(() => {
      ElMessage({
        type: "info",
        message: "Input canceled",
      });
    });
}
interface SaveDialogType {
  type: "add" | "edit";
  title: string;
}
const saveDialogTypes = {
  add: {
    type: "add",
    title: "新增素材库",
  } as SaveDialogType,
  edit: {
    type: "edit",
    title: "编辑素材库",
  } as SaveDialogType,
};

let saveDialogType = ref(saveDialogTypes.add);
function handleAddMediaRepo(type: SaveDialogType, repo?: Repository) {
  if (type) {
    saveDialogType.value = type;
  }
  console.log("save dialog type: ", saveDialogType.value);
  addRepoDialogFormVisible.value = true;
  if (type.type === "edit") {
    console.log("===edit====");
    saveRepoForm.value = Object.assign({}, repo) as Repository;
    saveRepoAdditon.value = JSON.parse(repo?.addition || "{}");

    // mediaRepositoryStore.createMediaRepository(saveRepoForm.value);
  } else if (type.type === "add") {
    console.log("===add====");
    saveRepoForm.value = {} as Repository;
    saveRepoAdditon.value = {} as Addition;

    // mediaRepositoryStore.updateMediaRepository(saveRepoForm.value);
  }
}

function saveRepository() {
  // addition is a JSON string
  saveRepoForm.value.addition = JSON.stringify(saveRepoAdditon.value);
  //check dialog is add or edit
  if (saveDialogType.value.type === "add") {
    saveRepoForm.value.is_default = false;
    mediaRepositoryStore
      .createMediaRepository(saveRepoForm.value)
      .then(() => {
        console.log("success");
      })
      .catch((err) => {
        console.error(err);
      })
      .finally(() => {
        addRepoDialogFormVisible.value = false;
      });
  } else if (saveDialogType.value.type === "edit") {
    mediaRepositoryStore

      .updateMediaRepository(saveRepoForm.value)
      .then(() => {
        console.log("update success");
      })
      .catch((err) => {
        console.error(err);
      })
      .finally(() => {
        addRepoDialogFormVisible.value = false;
      });
  }
}

function handleSetDefault() {
  ElMessageBox.confirm("设当前素材库为默认存储库", "设为默认", {
    confirmButtonText: "确认",
    cancelButtonText: "取消",
    // inputPattern: '',
    // inputErrorMessage: "Invalid Email",
  })
    .then(({ value }) => {
      ElMessage({
        type: "success",
        message: `设置默认存储库成功`,
      });
    })
    .catch(() => {
      ElMessage({
        type: "info",
        message: "Input canceled",
      });
    });
}
onMounted(() => {
  mediaRepositoryStore
    .listMediaRepository()
    .then(() => {
      console.log(mediaRepositoryStore.list);
    })
    .catch((err) => {
      console.log(err);
    })
    .finally(() => {
      console.log("fetch repo list finished");
    });
});
</script>
<style lang="scss">
.repo-card-container {
  margin-bottom: 18px;
}

.repo-card {
  width: 100%;
  cursor: pointer;

  .btn-container {
    display: flex;
    justify-content: right;
    align-items: center;
    // align-content:center;
    // text-align: right;

    .el-icon--right {
      // line-height: 22px;
      display: flex;
      justify-content: right;
      align-items: center;
    }

    :not(:first-child) {
      margin-left: 0.3em;
      // justify-content: flex-end;
    }

    .btn-edit {
      color: var(--el-color-primary);
    }

    .btn-delete {
      color: var(--el-color-danger);
    }
  }
}

.delete-drop {
  // color: var(--el-color-danger);

  :focus {
    color: var(--el-color-danger);
  }
}

li.delete-drop:hover {
  color: rgb(251, 6, 6) !important;
}

span.delete-btn:hover {
  color: red;
}
</style>
