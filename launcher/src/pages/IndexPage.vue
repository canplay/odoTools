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
      ref="status"
    >
      {{ status }}
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
    onResize() {
      // this.$refs.bg.style.width = document.querySelector("body").clientWidth;
      // this.$refs.bg.style.height = document.querySelector("body").clientHeight;
      console.log(this.$refs.status.style.margin);
    },

    onClose() {
      appWindow.close();
    },

    onStatus() {},
  },

  mounted() {
    addEventListener("resize", this.onResize());
    addEventListener("status", this.onStatus);

    appWindow.emit("test", { message: "Tauri is awesome!" });
  },

  onBeforeUnmount() {
    removeEventListener("resize", this.onResize);
    addEventListener("status", this.onStatus);
  },
});
</script>
