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
      path: "/autoreply",
      name: "autoreply",
      component: () => import("../views/autoreply.vue"),
    },
    {
      path: "/danmu/:room_id",
      name: "danmu",
      component: () => import("../views/danmu.vue"),
    },
  ],
});

export default router;
