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
                <div class="row text-h6">
                  <div class="col">
                    {{ $t("admin.database") }}{{ $t("status") }}：{{
                      info.status.mongodb ? $t("runing") : $t("stoping")
                    }}
                  </div>
                  <div class="col">
                    {{ $t("admin.loginserver") }}{{ $t("status") }}：{{
                      info.status.loginserver ? $t("runing") : $t("stoping")
                    }}
                  </div>
                  <div class="col">
                    {{ $t("admin.gameserver") }}{{ $t("status") }}：{{
                      info.status.gameserver ? $t("runing") : $t("stoping")
                    }}
                  </div>
                </div>
              </q-tab-panel>

              <q-tab-panel name="website">
                <q-tabs
                  v-model="website.tab"
                  class="text-primary"
                  active-bg-color="grey-9"
                >
                  <q-tab
                    name="announcements"
                    :label="$t('index.announcement')"
                  />
                  <q-tab name="events" :label="$t('index.event')" />
                  <q-tab name="carousel" :label="$t('index.carousel')" />
                  <q-tab name="downloads" :label="$t('toolbar.downloads')" />
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
                      :title="$t('index.announcement') + $t('list')"
                      class="header-table"
                      :rows="website.announcements.rows"
                      :columns="news"
                      row-key="title"
                      separator="cell"
                      selection="multiple"
                      v-model:selected="website.announcements.selected"
                      style="min-height: 406px"
                    >
                      <template v-slot:top-right>
                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="website.loading"
                          :label="$t('add')"
                          @click="onAdd('announcements')"
                        />

                        <q-btn
                          class="q-ml-sm"
                          color="red"
                          :disable="
                            website.loading ||
                            website.announcements.selected.length === 0
                          "
                          :label="$t('delete')"
                          @click="onDelete('announcements')"
                        />

                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="
                            website.loading ||
                            website.announcements.selected.length === 0
                          "
                          :label="$t('save')"
                          @click="onSave('announcements')"
                        />
                      </template>

                      <template v-slot:body="props">
                        <q-tr :props="props">
                          <q-td class="text-center">
                            <q-checkbox dense v-model="props.selected" />
                          </q-td>

                          <q-td key="title" :props="props">
                            {{ props.row.title }}
                            <q-popup-edit
                              v-model="props.row.title"
                              v-slot="scope"
                              auto-save
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>

                          <q-td key="date" :props="props">
                            {{ props.row.date }}
                            <q-popup-edit
                              v-model="props.row.date"
                              v-slot="scope"
                              auto-save
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
                              auto-save
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
                      :title="$t('index.event') + $t('list')"
                      class="header-table"
                      :rows="website.events.rows"
                      :columns="news"
                      row-key="title"
                      separator="cell"
                      selection="multiple"
                      v-model:selected="website.events.selected"
                      style="min-height: 406px"
                    >
                      <template v-slot:top-right>
                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="website.loading"
                          :label="$t('add')"
                          @click="onAdd('events')"
                        />

                        <q-btn
                          class="q-ml-sm"
                          color="red"
                          :disable="
                            website.loading ||
                            website.events.selected.length === 0
                          "
                          :label="$t('delete')"
                          @click="onDelete('events')"
                        />

                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="
                            website.loading ||
                            website.events.selected.length === 0
                          "
                          :label="$t('save')"
                          @click="onSave('events')"
                        />
                      </template>

                      <template v-slot:body="props">
                        <q-tr :props="props">
                          <q-td class="text-center">
                            <q-checkbox dense v-model="props.selected" />
                          </q-td>

                          <q-td key="title" :props="props">
                            {{ props.row.title }}
                            <q-popup-edit
                              v-model="props.row.title"
                              v-slot="scope"
                              auto-save
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>

                          <q-td key="date" :props="props">
                            {{ props.row.date }}
                            <q-popup-edit
                              v-model="props.row.date"
                              v-slot="scope"
                              auto-save
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
                              auto-save
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

                  <q-tab-panel class="transparent" name="carousel">
                    <q-table
                      :title="$t('index.carousel') + $t('list')"
                      class="header-table"
                      :rows="website.carousel.rows"
                      :columns="carousel"
                      row-key="img"
                      separator="cell"
                      selection="multiple"
                      v-model:selected="website.carousel.selected"
                      style="min-height: 406px"
                    >
                      <template v-slot:top-right>
                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="website.loading"
                          :label="$t('add')"
                          @click="onAdd('carousel')"
                        />

                        <q-btn
                          class="q-ml-sm"
                          color="red"
                          :disable="
                            website.loading ||
                            website.carousel.selected.length === 0
                          "
                          :label="$t('delete')"
                          @click="onDelete('carousel')"
                        />

                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="
                            website.loading ||
                            website.carousel.selected.length === 0
                          "
                          :label="$t('save')"
                          @click="onSave('carousel')"
                        />
                      </template>

                      <template v-slot:body="props">
                        <q-tr :props="props">
                          <q-td class="text-center">
                            <q-checkbox dense v-model="props.selected" />
                          </q-td>

                          <q-td key="img" :props="props">
                            {{ props.row.img }}
                            <q-popup-edit
                              v-model="props.row.img"
                              v-slot="scope"
                              auto-save
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>

                          <q-td key="url" :props="props">
                            {{ props.row.url }}
                            <q-popup-edit
                              v-model="props.row.url"
                              v-slot="scope"
                              auto-save
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>
                        </q-tr>
                      </template>
                    </q-table>
                  </q-tab-panel>

                  <q-tab-panel class="transparent" name="downloads">
                    <q-table
                      :title="$t('toolbar.downloads') + $t('list')"
                      class="header-table"
                      :rows="website.downloads.rows"
                      :columns="downloads"
                      row-key="title"
                      separator="cell"
                      selection="multiple"
                      v-model:selected="website.downloads.selected"
                      style="min-height: 406px"
                    >
                      <template v-slot:top-right>
                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="website.loading"
                          :label="$t('add')"
                          @click="onAdd('downloads')"
                        />

                        <q-btn
                          class="q-ml-sm"
                          color="red"
                          :disable="
                            website.loading ||
                            website.downloads.selected.length === 0
                          "
                          :label="$t('delete')"
                          @click="onDelete('downloads')"
                        />

                        <q-btn
                          class="q-ml-sm"
                          color="primary"
                          :disable="
                            website.loading ||
                            website.downloads.selected.length === 0
                          "
                          :label="$t('save')"
                          @click="onSave('downloads')"
                        />
                      </template>

                      <template v-slot:body="props">
                        <q-tr :props="props">
                          <q-td class="text-center">
                            <q-checkbox dense v-model="props.selected" />
                          </q-td>

                          <q-td key="title" :props="props">
                            {{ props.row.title }}
                            <q-popup-edit
                              v-model="props.row.title"
                              v-slot="scope"
                              auto-save
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>

                          <q-td key="desc" :props="props">
                            {{ props.row.desc }}
                            <q-popup-edit
                              v-model="props.row.desc"
                              v-slot="scope"
                              auto-save
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>

                          <q-td key="url" :props="props">
                            <div class="text-pre-wrap">
                              {{ props.row.url }}
                            </div>
                            <q-popup-edit
                              v-model="props.row.url"
                              v-slot="scope"
                              auto-save
                            >
                              <q-input v-model="scope.value" dense autofocus />
                            </q-popup-edit>
                          </q-td>
                        </q-tr>
                      </template>
                    </q-table>
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
          selected: ref([]),
          rows: ref([]),
        }),
        events: ref({
          selected: [],
          rows: [],
        }),
        carousel: ref({
          selected: [],
          rows: [],
        }),
        downloads: ref({
          selected: [],
          rows: [],
        }),
      }),
    };
  },

  computed: {
    news() {
      return [
        {
          name: "title",
          align: "center",
          label: this.$t("admin.table.title"),
          field: "title",
          sortable: true,
        },
        {
          name: "date",
          align: "center",
          label: this.$t("admin.table.date"),
          field: "date",
          sortable: true,
        },
        {
          name: "content",
          align: "center",
          label: this.$t("admin.table.content"),
          field: "content",
        },
      ];
    },

    carousel() {
      return [
        {
          name: "img",
          align: "center",
          label: this.$t("admin.table.img"),
          field: "img",
        },
        {
          name: "url",
          align: "center",
          label: this.$t("admin.table.url"),
          field: "url",
        },
      ];
    },

    downloads() {
      return [
        {
          name: "title",
          align: "center",
          label: this.$t("admin.table.title"),
          field: "title",
        },
        {
          name: "desc",
          align: "center",
          label: this.$t("admin.table.desc"),
          field: "desc",
        },
        {
          name: "url",
          align: "center",
          label: this.$t("admin.table.url"),
          field: "url",
        },
      ];
    },
  },

  created() {
    this.$axios
      .get(this.$store.state.global.backend + "/website/news")
      .then((resp) => {
        for (let index = 0; index < resp.data.msg.length; index++) {
          const element = resp.data.msg[index];

          if (element.type === "announcements") {
            this.website.announcements.rows.push(element);
          } else if (element.type === "events") {
            this.website.events.rows.push(element);
          }
        }
      });

    this.$axios
      .get(this.$store.state.global.backend + "/website/carousel")
      .then((resp) => {
        for (let index = 0; index < resp.data.msg.length; index++) {
          const element = resp.data.msg[index];
          this.website.carousel.rows.push(element);
        }
      })
      .catch((error) => {
        console.log(error);
        this.$q.notify(this.$t("error.network"));
      });

    this.$axios
      .get(this.$store.state.global.backend + "/website/downloads")
      .then((resp) => {
        for (let index = 0; index < resp.data.msg.length; index++) {
          const element = resp.data.msg[index];
          this.website.downloads.rows.push(element);
        }
      })
      .catch(() => {
        this.$q.notify(this.$t("error.network"));
      });

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

  methods: {
    onAdd(val) {
      switch (val) {
        case "announcements":
          break;
        case "events":
          break;
        case "carousel":
          break;
        case "downloads":
          break;
      }
    },

    onSave(val) {
      switch (val) {
        case "announcements":
          console.log(this.website.announcements.selected);
          // return;

          this.website.announcements.selected.forEach((element) => {
            this.$axios
              .post(
                this.$store.state.global.backend + "/api/website/news",
                {
                  method: "add",
                  id: element._id.$oid,
                  title: element.title,
                  date: element.date,
                  type: element.type,
                  content: element.content,
                },
                {
                  headers: {
                    authorization:
                      "Bearer " + this.$q.cookies.get("canplay-token"),
                  },
                }
              )
              .then((resp) => {
                console.log(resp);
              })
              .catch((e) => {
                console.log(e);
                this.$q.notify(this.$t("error.network"));
              });
          });
          break;
        case "events":
          break;
        case "carousel":
          break;
        case "downloads":
          break;
      }
    },

    onDelete(val) {
      switch (val) {
        case "announcements":
          break;
        case "events":
          break;
        case "carousel":
          break;
        case "downloads":
          break;
      }
    },
  },
});
</script>
