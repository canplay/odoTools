<template>
  <q-page
    style="
      background: url(bg.webp);
      background-size: 100% 100%;
      background-repeat: no-repeat;
    "
  >
    <div
      data-tauri-drag-region
      style="
        user-select: none;
        height: 36px;
        background-color: rgba(0, 0, 0, 0.5);
      "
    >
      <q-btn
        class="absolute-top-right text-white"
        flat
        label="X"
        @click="onClose"
      />
    </div>

    <div
      class="absolute-bottom text-white text-h4 justify-center items-center flex"
      style="background-color: #000; height: 110px"
    >
      {{ status === "" ? status : $t(status) }}
    </div>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";

export default defineComponent({
  name: "IndexPage",

  setup() {
    return {
      status: ref(""),
    };
  },

  methods: {
    onClose() {
      appWindow.close();
    },

    onStatus(event) {
      this.status = event.payload.message;
    },
  },

  created() {
    appWindow.listen("status", this.onStatus);
    appWindow.emit("js_ready");
  },
});
</script>
