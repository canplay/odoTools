<template>
  <div class="row text-h6">
    <div class="col text-right">
      {{ $t("admin.database") }}{{ $t("status") }}：{{
        mongodb ? $t("runing") : $t("stoping")
      }}
    </div>
    <div class="col text-center">
      {{ $t("admin.loginserver") }}{{ $t("status") }}：{{
        loginserver ? $t("runing") : $t("stoping")
      }}
    </div>
    <div class="col">
      {{ $t("admin.gameserver") }}{{ $t("status") }}：{{
        gameserver ? $t("runing") : $t("stoping")
      }}
    </div>
  </div>
</template>

<script>
import { defineComponent, ref } from "vue";

export default defineComponent({
  name: "compAdminOverview",

  setup() {
    return {};
  },

  props: {
    mongodb: ref([]),
    loginserver: ref({}),
    gameserver: ref([]),
  },
});
</script>
