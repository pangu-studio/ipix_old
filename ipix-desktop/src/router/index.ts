import { createWebHistory, createRouter } from "vue-router";
import Upload from "~/components/Upload.vue";
import Account from "~/components/Account.vue";
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
        component: Account,
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