import { createWebHistory, createRouter } from "vue-router";
import Upload from "~/components/Upload.vue";
import Setting from "~/components/Setting.vue";
import Repository from "../components/Repository.vue";

const routes = [
    {
        path: "/",
        name: "Home",
        component: Upload,
    },
    {
        path: "/setting",
        name: "About",
        component: Setting,
    },
    {
        path: "/repo",
        name: "Repository",
        component: Repository,
    },
];

const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;