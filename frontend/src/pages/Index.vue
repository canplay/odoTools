<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comptoolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <q-scroll-area class="text-white" style="height: 400px">
          123
        </q-scroll-area>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent } from "vue";
import comptoolbar from "src/components/compToolbar.vue";

export default defineComponent({
  name: "PageIndex",

  setup() {},

  components: {
    comptoolbar,
  },
});
</script>