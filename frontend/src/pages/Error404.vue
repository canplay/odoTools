<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comToolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <div style="font-size: 30vh">404</div>

        <div class="text-h2" style="opacity: 0.4">{{ $t("404") }}</div>

        <q-btn
          class="q-mt-xl"
          color="white"
          text-color="blue"
          unelevated
          to="/"
          :label="$t('back')"
          no-caps
        />
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent } from "vue";
import comToolbar from "components/Toolbar";

export default defineComponent({
  name: "error_404",

  components: {
    comToolbar,
  },
});
</script>
