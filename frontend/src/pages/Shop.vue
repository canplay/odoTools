<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comtoolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <q-scroll-area class="text-white" style="height: 400px">
          {{ $t("shop") }}
        </q-scroll-area>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent } from "vue";
import comtoolbar from "components/Toolbar.vue";

export default defineComponent({
  name: "PageShop",

  components: {
    comtoolbar,
  },
});
</script>
