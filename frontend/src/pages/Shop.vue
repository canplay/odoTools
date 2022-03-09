<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comtoolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <!-- <q-scroll-area class="text-white" style="height: 400px">
          {{ $t("shop") }}
        </q-scroll-area> -->

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

        <q-tab-panels v-model="tab" animated>
          <q-tab-panel name="hot">
            <q-table
              grid
              hide-header
              :rows="rows"
              :columns="columns"
              :filter="filter"
              row-key="name"
            >
              <template v-slot:top-right>
                <q-input
                  dense
                  debounce="300"
                  v-model="filter"
                  :placeholder="$t('search')"
                >
                  <template v-slot:append>
                    <q-icon name="search" />
                  </template>
                </q-input>
              </template>
            </q-table>
          </q-tab-panel>

          <q-tab-panel name="vip"> </q-tab-panel>

          <q-tab-panel name="items"> </q-tab-panel>

          <q-tab-panel name="outfit"> </q-tab-panel>

          <q-tab-panel name="equip"> </q-tab-panel>
        </q-tab-panels>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import comtoolbar from "components/Toolbar.vue";

export default defineComponent({
  name: "PageShop",

  components: {
    comtoolbar,
  },

  setup() {
    return {
      tab: ref("hot"),
      columns: [
        {
          name: "name",
          align: "center",
          label: "名称",
          field: "name",
          sortable: true,
        },
        {
          name: "cash",
          align: "center",
          label: "价格",
          field: "cash",
          sortable: true,
        },
        {
          name: "info",
          align: "center",
          label: "说明",
          field: "info",
        },
      ],
      rows: ref([
        {
          name: "KitKat",
          cash: 518,
          info: "123",
        },
      ]),
      filter: ref(""),
    };
  },
});
</script>
