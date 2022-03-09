<template>
  <div>
    <q-toolbar
      class="rounded-borders"
      style="background-color: rgba(0, 0, 0, 0.8)"
    >
      <q-tabs class="text-white" shrink>
        <q-route-tab to="/" exact :label="$t('toolbar.news')" />
        <q-route-tab to="shop" exact :label="$t('toolbar.shop')" />
        <q-route-tab to="market" exact :label="$t('toolbar.market')" />
        <q-route-tab to="community" exact :label="$t('toolbar.community')" />
        <q-route-tab to="download" exact :label="$t('toolbar.download')" />
      </q-tabs>

      <q-space />

      <q-select
        v-model="locale"
        :options="localeOptions"
        :label="$t('toolbar.language')"
        label-color="white"
        borderless
        emit-value
        map-options
        dark
        filled
        square
        style="width: 150px; margin-left: 2%"
      />
    </q-toolbar>
  </div>
</template>

<script>
import { defineComponent, ref } from "vue";
import { useI18n } from "vue-i18n";

export default defineComponent({
  name: "compToolbar",

  setup() {
    const { locale } = useI18n({ useScope: "global" });
    locale.value = "zh-CN";

    return {
      locale,
      localeOptions: [
        { value: "zh-CN", label: "简体中文" },
        { value: "en-US", label: "English" },
      ],
    };
  },
});
</script>
