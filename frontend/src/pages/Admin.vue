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
                label="Login Server"
              />
              <q-tab
                name="gameserver"
                icon="img:imgs/gameserver.png"
                label="Game Server"
              />
            </q-tabs>
          </template>

          <template v-slot:after>
            <q-tab-panels
              v-model="tab"
              animated
              swipeable
              vertical
              transition-prev="jump-up"
              transition-next="jump-up"
            >
              <q-tab-panel name="index">
                <q-card dark>
                  <q-card-section> IP地址： </q-card-section>
                </q-card>
              </q-tab-panel>

              <q-tab-panel name="website">
                <div class="text-h4 q-mb-md">Mails</div>
              </q-tab-panel>

              <q-tab-panel name="database">
                <div class="text-h4 q-mb-md">Alarms</div>
              </q-tab-panel>

              <q-tab-panel name="loginserver">
                <div class="text-h4 q-mb-md">Movies</div>
              </q-tab-panel>

              <q-tab-panel name="gameserver">
                <div class="text-h4 q-mb-md">Movies</div>
              </q-tab-panel>
            </q-tab-panels>
          </template>
        </q-splitter>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import comToolbar from "components/Toolbar";

export default defineComponent({
  name: "PageAdmin",

  components: {
    comToolbar,
  },

  setup() {
    return {
      splitterModel: ref(10),
      tab: ref("index"),
    };
  },
});
</script>
