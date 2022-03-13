<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comToolbar />

      <div style="height: 8px" />

      <q-card-section
        style="background-color: rgba(0, 0, 0, 0.8); min-height: 400px"
        class="text-h1 text-center text-white relative-position"
      >
        <div class="absolute-center">{{ $t("toolbar.market") }}</div>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import comToolbar from "components/Toolbar";

export default defineComponent({
  name: "PageMarket",

  components: {
    comToolbar,
  },
});
</script>
