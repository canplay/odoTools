<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comToolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <q-card dark>
          <q-card-section class="row" v-for="item in list" :key="item">
            <div class="col text-h4">{{ item.title }}</div>

            <div class="col text-h6">{{ item.desc }}</div>

            <div class="col-auto text-center text-h6 text-red">
              <a :href="item.url" class="text-primary">{{ $t("download") }}</a>
            </div>
          </q-card-section>
        </q-card>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import comToolbar from "components/Toolbar";

export default defineComponent({
  name: "PageDownloads",

  components: {
    comToolbar,
  },

  setup() {
    return {
      list: ref([]),
    };
  },

  created() {
    this.$axios
      .get(this.$store.state.global.backend + "/website/downloads")
      .then((resp) => {
        for (let index = 0; index < resp.data.msg.length; index++) {
          const element = resp.data.msg[index];
          this.list.push(element);
        }
      })
      .catch(() => {
        this.$q.notify(this.$t("error.network"));
      });
  },
});
</script>
