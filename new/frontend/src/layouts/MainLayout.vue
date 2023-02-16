<template>
  <q-layout
    view="hHh LpR fFf"
    style="
      background-image: url(imgs/bg.jpg);
      background-position: right bottom, left top;
      background-repeat: no-repeat, no-repeat;
      background-size: 100% 100%;
    "
  >
    <q-header elevated>
      <q-toolbar class="bg-grey-10 text-white row">
        <q-img class="col-auto" src="imgs/logo.png" width="240px" />

        <div class="col-auto" style="width: 30px" />

        <q-tabs class="col" v-model="tab">
          <q-route-tab
            name="index"
            :label="$t('index')"
            @click="onGoto('index')"
          />
          <q-route-tab
            name="news"
            :label="$t('news')"
            @click="onGoto('news')"
          />
          <q-route-tab
            name="shop"
            :label="$t('shop.title')"
            @click="onGoto('shop.title')"
          />
          <q-route-tab
            name="downloads"
            :label="$t('downloads')"
            @click="onGoto('downloads')"
          />
          <q-route-tab
            name="social"
            :label="$t('social')"
            @click="onGoto('social')"
          />
        </q-tabs>

        <div class="col-auto" style="width: 30px" />

        <q-select
          dark
          class="col-auto"
          v-model="locale"
          :options="localeOptions"
          label="Global"
          map-options
          emit-value
        />
      </q-toolbar>
    </q-header>

    <q-footer elevated>
      <q-toolbar class="bg-grey-10 text-center">
        <q-toolbar-title style="font-size: medium">
          Powered by CaNplay
        </q-toolbar-title>
      </q-toolbar>
    </q-footer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';

const { locale } = useI18n({ useScope: 'global' });
const localeOptions = [
  { value: 'en-US', label: 'English' },
  { value: 'zh-CN', label: '简体中文' },
];

const tab = ref('index');

const onGoto = (val: string) => {
  document.getElementById(val)?.scrollIntoView();
};
</script>
