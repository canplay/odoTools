<template>
  <q-table
    grid
    hide-header
    :rows="rows"
    :columns="columns"
    :filter="filter"
    row-key="name"
    card-class="bg-primary text-white"
    class="transparent"
  >
    <template v-slot:top-right>
      <q-input
        dark
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

    <template v-slot:item="props">
      <q-card
        bordered
        dark
        class="q-ma-sm"
        style="max-width: 300px; min-width: 300px"
      >
        <div class="q-ma-sm" v-for="col in props.cols" :key="col.id">
          <div v-if="col.name === 'preview'">
            <q-img :src="col.value" />
          </div>

          <div v-if="col.name === 'name'" class="text-h4 text-center text-bold">
            {{ col.value }}
          </div>

          <div v-if="col.name === 'cash'" class="text-h6 row">
            <div class="col-3 text-bold text-warning">
              {{ $t("shop.cash") }}:
            </div>
            <div class="col">{{ col.value }}</div>
          </div>

          <div v-if="col.name === 'desc'" class="text-h6 row">
            <div class="col-3 text-bold text-warning">
              {{ $t("shop.desc") }}:
            </div>
            <div class="col">{{ col.value }}</div>
          </div>
        </div>
      </q-card>
    </template>
  </q-table>
</template>

<script>
import { defineComponent, ref, computed } from "vue";

export default defineComponent({
  name: "compShopTable",

  setup() {
    return {
      filter: ref(""),
    };
  },

  props: {
    columns: computed([]),
    rows: ref({}),
  },
});
</script>
