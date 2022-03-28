<template>
  <q-splitter v-model="splitterModel">
    <template v-slot:before>
      <q-list>
        <q-item clickable>登录数据库</q-item>
      </q-list>
    </template>

    <template v-slot:after>
      <q-table
        title=""
        :rows="rows"
        :columns="column"
        row-key="title"
        separator="cell"
        selection="multiple"
        v-model:selected="selected"
        style="min-height: 406px"
      />
    </template>
  </q-splitter>
</template>

<script>
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "compAdminDb",

  setup() {
    return {
      splitterModel: ref(10),
      rows: ref([]),
      selected: ref([]),
    };
  },

  computed: {
    column() {
      return [
        {
          name: "id",
          align: "center",
          label: "ID",
          field: "id",
          sortable: true,
          headerStyle: "width: 20%",
        },
        {
          name: "content",
          align: "center",
          label: this.$t("admin.table.content"),
          field: "content",
        },
      ];
    },
  },
});
</script>
