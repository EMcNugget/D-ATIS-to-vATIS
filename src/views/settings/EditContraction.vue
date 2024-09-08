<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { computed, ref } from "vue";
import { use_store } from "../../lib/stores";
import { type TAlert } from "../../lib/types";

const store = use_store();

const contractions = computed({
  get: () => store.get_contractions(),
  set: (v) => store.set_contractions(v),
});

const message = computed({
  get: () => store.get_alert(),
  set: (v) => store.set_alert(v),
});

const new_contraction = ref({
  string: "",
  spoken: "",
});

const hovered_row = ref<number | null>(null);

const save_contractions = () => {
  invoke("set_contractions", { contractions: contractions.value }).then((k) => {
    message.value = k as TAlert;
  });
};

const add = () => {
  contractions.value.notam_contractions = {
    ...contractions.value.notam_contractions,
    [new_contraction.value.string]: new_contraction.value.spoken,
  };

  new_contraction.value = { string: "", spoken: "" };
};

const remove_contraction = (index: number) => {
  const remove = (index: number) => {
    const keys = Object.keys(contractions.value.notam_contractions);

    const key = keys[index];
    const { [key]: removed, ...rest } = contractions.value.notam_contractions;

    contractions.value.notam_contractions = rest;
  };

  message.value = {
    message: "Are you sure you want to delete this contraction?",
    alert_type: "warn",
    confirm: () => remove(index),
  };
};
</script>

<template>
  <div class="overflow-auto h-96 border-2 border-base-300 rounded-lg">
    <table class="table border-collapse rounded-lg">
      <thead class="sticky top-0 shadow-md bg-accent z-10">
        <tr>
          <th></th>
          <th class="text-base-content">Contraction</th>
          <th class="text-base-content">Spoken</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(string, spoken, index) in contractions.notam_contractions"
          :key="index"
          @mouseenter="hovered_row = index"
          @mouseleave="hovered_row = null"
        >
          <th>
            <button
              class="btn btn-circle btn-sm"
              v-if="hovered_row === index"
              @click="remove_contraction(index)"
            >
              <img src="/trash.svg" alt="Delete" class="w-4 h-4" />
            </button>
          </th>
          <td>{{ string }}</td>
          <td>{{ spoken }}</td>
        </tr>
      </tbody>
    </table>
  </div>
  <div class="flex flex-row items-center justify-center space-x-4">
    <button
      class="btn btn-active btn-primary mt-8 w-1/4 mr-4"
      onclick="my_modal_1.showModal()"
    >
      Add
    </button>
    <button
      class="btn btn-active btn-primary mt-8 w-1/4"
      @click="save_contractions()"
    >
      Save
    </button>
  </div>
  <dialog id="my_modal_1" class="modal">
    <div class="modal-box">
      <h3 class="text-lg font-bold">Add Contraction</h3>
      <div class="flex flex-col space-y-4 mt-8 items-center">
        <input
          type="text"
          v-model="new_contraction.string"
          placeholder="Contraction..."
          class="input input-bordered w-full"
          @input="new_contraction.string = new_contraction.string.toUpperCase()"
        />
        <input
          type="text"
          v-model="new_contraction.spoken"
          placeholder="Spoken..."
          class="input input-bordered w-full"
          @input="new_contraction.spoken = new_contraction.spoken.toUpperCase()"
        />
        <button
          class="btn btn-active btn-neutral w-1/4"
          onclick="my_modal_1.close()"
          @click="add()"
        >
          Save
        </button>
      </div>
      <div class="modal-action">
        <form method="dialog">
          <button
            class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
          >
            ✕
          </button>
        </form>
      </div>
    </div>
  </dialog>
</template>
