<template>
  <el-row :gutter="12">
    <el-col :span="8" class="repo-card-container">
      <el-card class="repo-card" shadow="hover">
        <el-row>
          <el-col :span="20">
            <span>博客</span>
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
                  <el-dropdown-item @click="handleRenameRepo">重命名</el-dropdown-item>
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
      <el-card class="repo-card" shadow="hover">
        <el-row>
          <el-col :span="20">
            <span>Anki</span>
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
                  <el-dropdown-item @click="handleRenameRepo">重命名</el-dropdown-item>
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
      <el-card class="repo-card" shadow="hover">
        <el-row>
          <el-col :span="20">
            <span>其他</span>
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
                  <el-dropdown-item @click="handleRenameRepo">重命名</el-dropdown-item>
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
      <el-card class="repo-card" shadow="hover">
        <el-row>
          <el-col :span="20">
            <span>博客</span>
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
                  <el-dropdown-item @click="handleRenameRepo">重命名</el-dropdown-item>
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
      <el-card class="repo-card" shadow="hover" style="text-align: center">
        <el-icon>
          <Plus />
        </el-icon>
      </el-card>
    </el-col>
  </el-row>

  <el-dialog v-model="editDialogFormVisible" title="重命名素材库">
    <el-form :model="editForm">
      <el-form-item label="素材库名称">
        <el-input v-model="editForm.name" autocomplete="off" />
      </el-form-item>
    </el-form>
    <template #footer>
      <span class="dialog-footer">
        <el-button @click="editDialogFormVisible = false">取消</el-button>
        <el-button type="primary" @click="editDialogFormVisible = false">
          保存
        </el-button>
      </span>
    </template>
  </el-dialog>
</template>
<script lang="ts" setup>
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { ElMessage, ElMessageBox } from "element-plus";
import { Repository } from "@/dto/repostiory";

let editDialogFormVisible = ref(false);

let editForm = ref({} as Repository);

function handleRenameRepo() {
  editDialogFormVisible.value = true;
  ElMessage({
    message: "功能开发中",
    type: "success",
  });
}

function handleDeleleRepo() {
  ElMessageBox.prompt("危险操作，请输入素材库名确认", "删除素材库", {
    confirmButtonText: "删除",
    cancelButtonText: "取消",
    inputPattern: /[\w!#$%&'*+/=?^_`{|}~-]+(?:\.[\w!#$%&'*+/=?^_`{|}~-]+)*@(?:[\w](?:[\w-]*[\w])?\.)+[\w](?:[\w-]*[\w])?/,
    inputErrorMessage: "Invalid Email",
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
onMounted(() => {
  invoke("list_all_media_repo")
    .then((res) => {
      console.log(res);
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
