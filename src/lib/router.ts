import Atis from "../views/Atis.vue";
import EditContraction from "../views/EditContraction.vue";
import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
  {
    path: "/atis",
    name: "D-ATIS to vATIS",
    component: Atis,
  },
  {
    path: "/contractions",
    name: "Edit Contractions",
    component: EditContraction,
  }
];

export const router = createRouter({
  routes,
  history: createWebHashHistory(),
});
