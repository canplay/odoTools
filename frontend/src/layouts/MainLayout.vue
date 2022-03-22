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
import jwt_decode from "jwt-decode";

export default defineComponent({
  name: "MainLayout",

  created() {
    if (
      this.$q.cookies.get("canplay-token") != null &&
      this.$q.cookies.get("canplay-token") != ""
    ) {
      let token = jwt_decode(this.$q.cookies.get("canplay-token"));

      this.$axios
        .get(this.$store.state.global.backend + "/api/user/" + token.username, {
          headers: {
            authorization: "Bearer " + this.$q.cookies.get("canplay-token"),
          },
        })
        .then((resp) => {
          this.$store.commit("global/login", true);

          this.$store.commit("global/user", {
            family: resp.data.msg.family,
            cash: resp.data.msg.cash,
            access_level: resp.data.msg.access_level,
            avatar: [
              {
                name: "",
                level: 0,
                types: "",
                pearl: 0,
              },
            ],
          });
        })
        .catch(() => {
          this.$store.commit("global/login", false);
          this.$q.cookies.remove("canplay-token");
        });
    }
  },
});
</script>
