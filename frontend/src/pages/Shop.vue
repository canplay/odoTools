<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comToolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <q-tabs
          v-model="tab"
          class="text-white"
          active-color="primary"
          indicator-color="primary"
          align="justify"
          narrow-indicator
        >
          <q-tab class="tabs-bold" name="hot" :label="$t('shop.hot')" />
          <q-tab class="tabs-bold" name="vip" :label="$t('shop.vip')" />
          <q-tab class="tabs-bold" name="item" :label="$t('shop.items')" />
          <q-tab class="tabs-bold" name="outfit" :label="$t('shop.outfit')" />
        </q-tabs>

        <q-separator />

        <q-tab-panels v-model="tab" animated class="transparent">
          <q-tab-panel name="hot">
            <compShopTable :columns="columns" :rows="rows.hot" />
          </q-tab-panel>

          <q-tab-panel name="vip">
            <compShopTable :columns="columns" :rows="rows.vip" />
          </q-tab-panel>

          <q-tab-panel name="item">
            <compShopTable :columns="columns" :rows="rows.item" />
          </q-tab-panel>

          <q-tab-panel name="outfit">
            <compShopTable :columns="columns" :rows="rows.outfit" />
          </q-tab-panel>
        </q-tab-panels>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<style lang="sass">
.tabs-bold
  .q-tab__content
    .q-tab__label
      font-size: 20px
      font-weight: 900
</style>

<script>
import { defineComponent, ref } from "vue";
import comToolbar from "components/Toolbar";
import compShopTable from "components/ShopTable";

export default defineComponent({
  name: "PageShop",

  components: {
    comToolbar,
    compShopTable,
  },

  setup() {
    return {
      tab: ref("hot"),
      rows: ref({
        hot: [],
        vip: [],
        item: [],
        outfit: [],
      }),
      filter: ref(""),
    };
  },

  computed: {
    columns() {
      return [
        {
          name: "preview",
          align: "center",
          label: this.$t("shop.preview"),
          field: "preview",
        },
        {
          name: "name",
          align: "center",
          label: this.$t("shop.name"),
          field: "name",
          sortable: true,
        },
        {
          name: "cash",
          align: "center",
          label: this.$t("shop.cash"),
          field: "cash",
          sortable: true,
        },
        {
          name: "desc",
          align: "center",
          label: this.$t("shop.desc"),
          field: "desc",
        },
      ];
    },
  },

  created() {
    this.$axios
      .get(this.$store.state.global.backend + "/website/shop")
      .then((resp) => {
        for (let index = 0; index < resp.data.msg.length; index++) {
          const element = resp.data.msg[index];

          switch (element.type) {
            case "hot":
              this.rows.hot.push(element);
              break;
            case "vip":
              this.rows.vip.push(element);
              break;
            case "item":
              this.rows.item.push(element);
              break;
            case "outfit":
              this.rows.outfit.push(element);
              break;
          }
        }
      })
      .catch(() => {
        this.$q.notify(this.$t("error.network"));
      });
  },
});
</script>
