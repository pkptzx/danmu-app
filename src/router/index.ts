import { createRouter, createWebHistory } from "vue-router";
import Greet from "../components/Greet.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "home",
      component: Greet,
    },
    {
      path: "/chatgtp",
      name: "chatgtp",
      component: () => import("../views/chatgtp.vue"),
    },
    {
      path: "/danmu/:room_id",
      name: "danmu",
      component: () => import("../views/danmu.vue"),
    },
    {
      path: "/danmu/query",
      name: "danmuQuery",
      component: () => import("../views/danmuQuery.vue"),
    },
    {
      path: "/danmu/chatterbox",
      name: "setChatterBox",
      component: () => import("../views/SetChatterBox.vue"),
    },
  ],
});

export default router;
