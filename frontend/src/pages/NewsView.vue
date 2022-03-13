<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comToolbar />

      <div style="height: 8px" />

      <q-card-section
        class="text-white"
        style="background-color: rgba(0, 0, 0, 0.8)"
      >
        <div class="text-h4 text-bold text-center">{{ view.title }}</div>
        <div class="text-center">{{ view.date }}</div>
        <div v-html="view.content" />
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import comToolbar from "components/Toolbar";

export default defineComponent({
  name: "PageNewsView",

  components: {
    comToolbar,
  },

  setup() {
    return {
      view: ref({}),
    };
  },

  created() {
    if (this.$route.params.view) {
      this.view = JSON.parse(this.$route.params.view);
    } else {
      this.$router.push("/");
    }
  },
});
</script>
