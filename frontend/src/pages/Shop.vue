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
          <q-tab name="hot" :label="$t('shop.hot')" />
          <q-tab name="vip" :label="$t('shop.vip')" />
          <q-tab name="items" :label="$t('shop.items')" />
          <q-tab name="outfit" :label="$t('shop.outfit')" />
          <q-tab name="equip" :label="$t('shop.equip')" />
        </q-tabs>

        <q-separator />

        <q-tab-panels v-model="tab" animated class="transparent">
          <q-tab-panel name="hot">
            <compShopTable :columns="columns" :rows="rows.hot" />
          </q-tab-panel>

          <q-tab-panel name="vip">
            <compShopTable :columns="columns" :rows="rows.vip" />
          </q-tab-panel>

          <q-tab-panel name="items">
            <compShopTable :columns="columns" :rows="rows.items" />
          </q-tab-panel>

          <q-tab-panel name="outfit">
            <compShopTable :columns="columns" :rows="rows.outfit" />
          </q-tab-panel>

          <q-tab-panel name="equip">
            <compShopTable :columns="columns" :rows="rows.equip" />
          </q-tab-panel>
        </q-tab-panels>
      </q-card-section>
    </q-card>
  </q-page>
</template>

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
        items: [],
        outfit: [],
        equip: [],
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
      .get(this.$store.state.global.database.shop)
      .then((resp) => {
        for (let index = 0; index < resp.data.hot.length; index++) {
          const element = resp.data.hot[index];
          this.rows.hot.push(element);
        }

        for (let index = 0; index < resp.data.vip.length; index++) {
          const element = resp.data.vip[index];
          this.rows.vip.push(element);
        }

        for (let index = 0; index < resp.data.items.length; index++) {
          const element = resp.data.items[index];
          this.rows.items.push(element);
        }

        for (let index = 0; index < resp.data.outfit.length; index++) {
          const element = resp.data.outfit[index];
          this.rows.outfit.push(element);
        }

        for (let index = 0; index < resp.data.equip.length; index++) {
          const element = resp.data.equip[index];
          this.rows.equip.push(element);
        }
      })
      .catch(() => {
        this.$q.notify(this.$t("error.network"));
      });
  },
});
</script>
