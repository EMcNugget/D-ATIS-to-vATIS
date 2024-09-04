import Atis from "../views/Atis.vue";
import RecordAtis from "../views/RecordAtis.vue";
import EditContractions from "../views/EditContractions.vue";
import { createRouter, createWebHashHistory } from "vue-router";

const routes = [
  {
    path: "/atis",
    name: "D-ATIS to vATIS",
    component: Atis,
  },
  {
    path: "/record",
    name: "Record ATIS",
    component: RecordAtis,
  },
  {
    path: "/contractions",
    name: "Edit Contractions",
    component: EditContractions,
  }
];

export const router = createRouter({
  routes,
  history: createWebHashHistory(),
});
