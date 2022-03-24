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
              class="transparent text-white"
              v-model="tab"
              animated
              swipeable
              vertical
              transition-prev="jump-up"
              transition-next="jump-up"
            >
              <q-tab-panel class="transparent" name="index">
                <div class="row text-h6">
                  <div class="col">
                    数据库状态：{{ info.status.mongodb ? "启动" : "未启动" }}
                  </div>
                  <div class="col">
                    登录服务端状态：{{
                      info.status.loginserver ? "启动" : "未启动"
                    }}
                  </div>
                  <div class="col">
                    游戏服务端状态：{{
                      info.status.gameserver ? "启动" : "未启动"
                    }}
                  </div>
                </div>
              </q-tab-panel>

              <q-tab-panel name="website">
                <q-tabs
                  v-model="website.tab"
                  class="text-warning"
                  active-bg-color="grey-9"
                >
                  <q-tab
                    name="announcements"
                    icon="img:imgs/overview.png"
                    :label="$t('index.announcement')"
                  />
                  <q-tab
                    name="events"
                    icon="img:imgs/website.png"
                    :label="$t('index.event')"
                  />
                  <q-tab
                    name="carousel"
                    icon="img:imgs/database.png"
                    :label="$t('index.carousel')"
                  />
                  <q-tab
                    name="downloads"
                    icon="img:imgs/loginserver.png"
                    :label="$t('toolbar.downloads')"
                  />
                </q-tabs>

                <q-tab-panels
                  class="transparent text-white"
                  v-model="website.tab"
                  animated
                  swipeable
                  transition-prev="jump-up"
                  transition-next="jump-up"
                >
                  <q-tab-panel class="transparent" name="announcements">
                    <q-table
                      title="公告设置"
                      class="header-table"
                      :rows="website.announcements.rows"
                      :columns="website.announcements.columns"
                      row-key="title"
                      separator="cell"
                      selection="multiple"
                      v-model:selected="website.announcements.selected"
                    >
                      <template v-slot:top-right>
                        <q-btn
                          color="primary"
                          :disable="website.loading"
                          label="删除"
                          @click="onDelete('announcements')"
                        />
                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="website.loading"
                          label="保存"
                          @click="onSave('announcements')"
                        />
                      </template>

                      <template v-slot:body-cell="props">
                        <q-tr :props="props">
                          <q-td key="title" :props="props">
                            {{ props.row.title }}
                            <q-popup-edit
                              v-model="props.row.title"
                              v-slot="scope"
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>

                          <q-td key="date" :props="props">
                            {{ props.row.date }}
                            <q-popup-edit
                              v-model="props.row.date"
                              v-slot="scope"
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>

                          <q-td key="content" :props="props">
                            <div class="text-pre-wrap">
                              {{ props.row.content }}
                            </div>
                            <q-popup-edit
                              v-model="props.row.content"
                              v-slot="scope"
                            >
                              <q-input
                                type="textarea"
                                v-model="scope.value"
                                dense
                                autofocus
                              />
                            </q-popup-edit>
                          </q-td>
                        </q-tr>
                      </template>
                    </q-table>
                  </q-tab-panel>

                  <q-tab-panel class="transparent" name="events">
                    <q-table
                      title=""
                      :rows="website.events.rows"
                      :columns="website.announcements.columns"
                      row-key="title"
                    />
                  </q-tab-panel>

                  <q-tab-panel class="transparent" name="carousel">
                    <q-table
                      title=""
                      :rows="website.carousel.rows"
                      :columns="website.carousel.columns"
                      row-key="path"
                    />
                  </q-tab-panel>

                  <q-tab-panel class="transparent" name="downloads">
                    <q-table
                      title=""
                      :rows="website.downloads.rows"
                      :columns="website.downloads.columns"
                      row-key="name"
                    />
                  </q-tab-panel>
                </q-tab-panels>
              </q-tab-panel>

              <q-tab-panel name="database"> </q-tab-panel>

              <q-tab-panel name="loginserver"> </q-tab-panel>

              <q-tab-panel name="gameserver"> </q-tab-panel>
            </q-tab-panels>
          </template>
        </q-splitter>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<style lang="sass">
.header-table
  height: 310px

  .q-table__top,
  .q-table__bottom,
  thead tr th
    position: sticky
    z-index: 1
  thead tr:first-child th
    top: 0

  &.q-table--loading thead tr:last-child th
    top: 48px
</style>

<script>
import { defineComponent, ref } from "vue";
import comToolbar from "components/Toolbar";
import { ConfigIniParser } from "config-ini-parser";

export default defineComponent({
  name: "PageAdmin",

  components: {
    comToolbar,
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
      website: ref({
        loading: false,
        tab: "announcements",
        announcements: ref({
          selected: [],
          rows: [],
          columns: [
            {
              name: "title",
              align: "center",
              label: "标题",
              field: "title",
              sortable: true,
            },
            {
              name: "date",
              align: "center",
              label: "日期",
              field: "date",
              sortable: true,
            },
            {
              name: "content",
              align: "center",
              label: "内容",
              field: "content",
            },
          ],
        }),
        events: ref({
          selected: [],
          rows: [],
        }),
        carousel: ref({
          selected: [],
          rows: [],
          columns: [
            {
              name: "img",
              align: "center",
              label: "路径",
              field: "img",
            },
          ],
        }),
        downloads: ref({
          selected: [],
          rows: [],
          columns: [
            {
              name: "name",
              align: "center",
              label: "名称",
              field: "name",
            },
            {
              name: "desc",
              align: "center",
              label: "说明",
              field: "desc",
            },
            {
              name: "url",
              align: "center",
              label: "网址",
              field: "url",
            },
          ],
        }),
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

        for (
          let index = 0;
          index < this.info.config.web.announcements.length;
          index++
        ) {
          const element = this.info.config.web.announcements[index];
          this.website.announcements.rows.push(element);
        }

        for (
          let index = 0;
          index < this.info.config.web.events.length;
          index++
        ) {
          const element = this.info.config.web.events[index];
          this.website.events.rows.push(element);
        }

        for (
          let index = 0;
          index < this.info.config.web.carousel.length;
          index++
        ) {
          const element = this.info.config.web.carousel[index];
          this.website.carousel.rows.push(element);
        }

        for (
          let index = 0;
          index < this.info.config.web.downloads.length;
          index++
        ) {
          const element = this.info.config.web.downloads[index];
          this.website.downloads.rows.push(element);
        }

        let ini = new ConfigIniParser("\n");
        ini.parse(this.info.config.loginserver);
        console.log(ini.get(null, "login.game.service.type"));
      })
      .catch((e) => {
        console.log(e);
        this.$q.notify(this.$t("error.network"));
      });
  },
});
</script>
