import { createWebHistory, createRouter } from "vue-router";
import Upload from "~/pages/Upload.vue";
import Account from "~/pages/Account.vue";
import Repository from "~/pages/Repository.vue";
import { appWindow } from '@tauri-apps/api/window';

const routes = [
    {
        path: "/",
        name: "Home",
        component: Upload,
        meta: {
            title: "上传文件"
        }
    },
    {
        path: "/setting",
        name: "About",
        component: Account,
        meta: {
            title: "存储账号"
        }
    },
    {
        path: "/repo",
        name: "Repository",
        component: Repository,
        meta: {
            title: "素材库"
        }
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});
const defaultTitle = 'iPix'
router.beforeEach((to, from, next) => {
 const title = (to.meta.title ? to.meta.title : defaultTitle) as string
  appWindow.setTitle(title)
  console.log(title)
  next()
})

export default router;