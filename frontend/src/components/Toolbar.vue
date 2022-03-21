<template>
  <q-toolbar
    class="rounded-borders"
    style="background-color: rgba(0, 0, 0, 0.8)"
  >
    <q-tabs class="text-white" shrink>
      <q-route-tab to="/" exact :label="$t('toolbar.index')" />
      <q-route-tab to="shop" exact :label="$t('toolbar.shop')" />
      <q-route-tab to="market" exact :label="$t('toolbar.market')" />
      <q-route-tab to="community" exact :label="$t('toolbar.community')" />
      <q-route-tab to="downloads" exact :label="$t('toolbar.downloads')" />
      <q-route-tab
        v-if="$store.state.global.loginin"
        to="admin"
        exact
        :label="$t('toolbar.admin')"
      />
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
</template>

<script>
import { defineComponent } from "vue";
import { useI18n } from "vue-i18n";

export default defineComponent({
  name: "compToolbar",

  setup() {
    const { locale } = useI18n({
      useScope: "global",
      fallbackLocale: "en-US",
    });
    locale.value = navigator.languages[0];

    return {
      locale,
      localeOptions: [
        { value: "en-US", label: "English" },
        { value: "zh-CN", label: "简体中文" },
      ],
    };
  },
});
</script>
