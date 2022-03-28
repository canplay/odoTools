<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comToolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <q-splitter v-model="splitterModel">
          <template v-slot:before>
            <q-tabs
              v-model="tab"
              vertical
              class="text-warning"
              active-bg-color="grey-9"
            >
              <q-tab
                name="index"
                icon="img:imgs/overview.png"
                :label="$t('admin.overview')"
              />
              <q-tab
                name="website"
                icon="img:imgs/website.png"
                :label="$t('admin.website')"
              />
              <q-tab
                name="database"
                icon="img:imgs/database.png"
                :label="$t('admin.database')"
              />
              <q-tab
                name="loginserver"
                icon="img:imgs/loginserver.png"
                :label="$t('admin.loginserver')"
              />
              <q-tab
                name="gameserver"
                icon="img:imgs/gameserver.png"
                :label="$t('admin.gameserver')"
              />
            </q-tabs>
          </template>

          <template v-slot:after>
            <q-tab-panels
              class="transparent text-white"
              v-model="tab"
              animated
              swipeable
              vertical
              transition-prev="jump-up"
              transition-next="jump-up"
            >
              <q-tab-panel class="transparent" name="index">
                <comAdminOverview
                  :mongodb="info.mongodb"
                  :loginserver="info.loginserver"
                  :gameserver="info.gameserver"
                />
              </q-tab-panel>

              <q-tab-panel name="website">
                <comAdminWebsite />
              </q-tab-panel>

              <q-tab-panel name="database">
                <comAdminDb />
              </q-tab-panel>

              <q-tab-panel name="loginserver"> </q-tab-panel>

              <q-tab-panel name="gameserver"> </q-tab-panel>
            </q-tab-panels>
          </template>
        </q-splitter>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import { ConfigIniParser } from "config-ini-parser";
import comToolbar from "components/Toolbar";
import comAdminOverview from "components/AdminOverview";
import comAdminWebsite from "components/AdminWebsite";
import comAdminDb from "components/AdminDb";

export default defineComponent({
  name: "PageAdmin",

  components: {
    comToolbar,
    comAdminOverview,
    comAdminWebsite,
    comAdminDb,
  },

  setup() {
    return {
      splitterModel: ref(10),
      tab: ref("index"),
      info: ref({
        database: "",
        register: false,
        thread_num: 0,
        status: {
          mongodb: false,
          loginserver: false,
          gameserver: false,
        },
        dir: "",
        config: {
          web: "",
          loginserver: "",
          gameserver: "",
        },
      }),
    };
  },

  created() {
    this.$axios
      .get(this.$store.state.global.backend + "/api/info/server", {
        headers: {
          authorization: "Bearer " + this.$q.cookies.get("canplay-token"),
        },
      })
      .then((resp) => {
        this.info = resp.data.msg;

        let ini = new ConfigIniParser("\n");
        ini.parse(this.info.loginserver);
        console.log(ini.get(null, "login.game.service.type"));
      })
      .catch((e) => {
        console.log(e);
        this.$q.notify(this.$t("error.network"));
      });
  },
});
</script>
