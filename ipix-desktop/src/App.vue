<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from 'vue'
import { useDark, useToggle } from '@vueuse/core'
import {
  Document,
  Menu as IconMenu,
  Expand,
  Fold,
  Setting,
} from '@element-plus/icons-vue'

let activeClass = "el-menu--collapse"

const isCollapse = ref(true)
let isDark = useDark()
const handleOpen = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const handleClose = (key: string, keyPath: string[]) => {
  console.log(key, keyPath)
}
const toggleDark = useToggle(isDark)

// function toggleDark() {
//   useToggle(isDark)
// }
</script>

<template>
  <el-container class="container">
    <el-aside width="120px" class="aside" :class="isCollapse ? activeClass : ''">
      <el-menu default-active="2" class="el-menu-vertical-demo" :collapse="isCollapse" @open="handleOpen" router
        :collapse-transition="false" @close="handleClose">
        <el-menu-item index="0" class="logo">LOGO
        </el-menu-item>

        <el-menu-item index="/">
          <el-icon>
            <icon-menu />
          </el-icon>
          <template #title>图床</template>
        </el-menu-item>
        <el-menu-item index="3">
          <el-icon>
            <document />
          </el-icon>
          <template #title>素材</template>
        </el-menu-item>
        <el-menu-item index="/setting">
          <el-icon>
            <setting />
          </el-icon>
          <template #title>设置</template>
        </el-menu-item>
      </el-menu>
      <div class="bottom-tool" :class="isCollapse ? activeClass : 'bt-ex'">

        <el-switch class="dark-switch" inline-prompt active-text="黑" inactive-text="白" v-model="isDark"
          :change="useToggle()" size="small" />

        <el-button circle class="ex-btn" v-if="isCollapse" @click="isCollapse = !isCollapse" :icon="Expand" />
        <el-button circle class="ex-btn" v-if="!isCollapse" @click="isCollapse = !isCollapse" :icon="Fold" />
      </div>
    </el-aside>
    <el-main class="">
      <h1>iPix</h1>
      <!-- <Greet /> -->
      <router-view />
    </el-main>
  </el-container>

</template>

<style lang="scss" scoped>
.logo {
  justify-content: center;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

h1 {
  text-align: center;
}

.btn {
  width: 100px;
  margin: 0 auto;
}

.el-menu-vertical-demo {
  min-height: 100%;
}

.el-menu-vertical-demo:not(.el-menu--collapse) {
  width: 120px;
  min-height: 100%;
}

.aside {

  // height: 100%;
  .bt-ex {
    width: 120px;
  }

  .bottom-tool {
    position: fixed;
    bottom: 10px;
    overflow: hidden;
    z-index: 9999;
    position: fixed;
    text-align: center;
    margin: 0 auto;

    .dark-switch {
      margin: 0 auto;
      display: flex;
      justify-content: center;
      align-items: center;
    }

    .ex-btn {
      margin: 0 auto;
      border: none;
    }
  }



}

.container {
  height: 100%;
}
</style>
