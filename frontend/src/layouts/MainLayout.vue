<template>
  <q-layout
    view="lHh Lpr lFf"
    style="
      background: url('imgs/bg.jpg') no-repeat;
      background-size: cover;
      background-position: center center;
    "
  >
    <q-header class="transparent">
      <q-img src="imgs/logo.png" width="30%" style="margin: 2% 0 0 2%" />
    </q-header>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script>
import { defineComponent } from "vue";

export default defineComponent({
  name: "MainLayout",
});
</script>
