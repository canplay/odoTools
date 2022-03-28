<template>
  <q-tabs v-model="tab" class="text-primary" active-bg-color="grey-9">
    <q-tab name="announcements" :label="$t('index.announcement')" />
    <q-tab name="events" :label="$t('index.event')" />
    <q-tab name="carousel" :label="$t('index.carousel')" />
    <q-tab name="downloads" :label="$t('toolbar.downloads')" />
  </q-tabs>

  <q-tab-panels
    class="transparent text-white"
    v-model="tab"
    animated
    swipeable
    transition-prev="jump-up"
    transition-next="jump-up"
  >
    <q-tab-panel class="transparent" name="announcements">
      <q-table
        :title="$t('index.announcement') + $t('list')"
        :rows="announcements.rows"
        :columns="column_news"
        row-key="title"
        separator="cell"
        selection="multiple"
        v-model:selected="announcements.selected"
        style="min-height: 406px"
      >
        <template v-slot:top-right>
          <q-btn
            class="q-ml-sm"
            color="primary"
            :disable="loading"
            :label="$t('add')"
            @click="onAdd('announcements')"
          />

          <q-btn
            class="q-ml-sm"
            color="red"
            :disable="loading || announcements.selected.length === 0"
            :label="$t('delete')"
            @click="onDelete('announcements')"
          />

          <q-btn
            class="q-ml-sm"
            color="primary"
            :disable="loading || announcements.selected.length === 0"
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
              <q-popup-edit v-model="props.row.title" v-slot="scope" auto-save>
                <q-input v-model="scope.value" dense autofocus />
              </q-popup-edit>
            </q-td>

            <q-td key="date" :props="props">
              {{ props.row.date }}
              <q-popup-edit v-model="props.row.date" v-slot="scope" auto-save>
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
        :rows="events.rows"
        :columns="column_news"
        row-key="title"
        separator="cell"
        selection="multiple"
        v-model:selected="events.selected"
        style="min-height: 406px"
      >
        <template v-slot:top-right>
          <q-btn
            class="q-ml-sm"
            color="primary"
            :disable="loading"
            :label="$t('add')"
            @click="onAdd('events')"
          />

          <q-btn
            class="q-ml-sm"
            color="red"
            :disable="loading || events.selected.length === 0"
            :label="$t('delete')"
            @click="onDelete('events')"
          />

          <q-btn
            class="q-ml-sm"
            color="primary"
            :disable="loading || events.selected.length === 0"
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
              <q-popup-edit v-model="props.row.title" v-slot="scope" auto-save>
                <q-input v-model="scope.value" dense autofocus />
              </q-popup-edit>
            </q-td>

            <q-td key="date" :props="props">
              {{ props.row.date }}
              <q-popup-edit v-model="props.row.date" v-slot="scope" auto-save>
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
        :rows="carousel.rows"
        :columns="column_carousel"
        row-key="img"
        separator="cell"
        selection="multiple"
        v-model:selected="carousel.selected"
        style="min-height: 406px"
      >
        <template v-slot:top-right>
          <q-btn
            class="q-ml-sm"
            color="primary"
            :disable="loading"
            :label="$t('add')"
            @click="onAdd('carousel')"
          />

          <q-btn
            class="q-ml-sm"
            color="red"
            :disable="loading || carousel.selected.length === 0"
            :label="$t('delete')"
            @click="onDelete('carousel')"
          />

          <q-btn
            class="q-ml-sm"
            color="primary"
            :disable="loading || carousel.selected.length === 0"
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
              <q-popup-edit v-model="props.row.img" v-slot="scope" auto-save>
                <q-input v-model="scope.value" dense autofocus />
              </q-popup-edit>
            </q-td>

            <q-td key="url" :props="props">
              {{ props.row.url }}
              <q-popup-edit v-model="props.row.url" v-slot="scope" auto-save>
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
        :rows="downloads.rows"
        :columns="column_downloads"
        row-key="title"
        separator="cell"
        selection="multiple"
        v-model:selected="downloads.selected"
        style="min-height: 406px"
      >
        <template v-slot:top-right>
          <q-btn
            class="q-ml-sm"
            color="primary"
            :disable="loading"
            :label="$t('add')"
            @click="onAdd('downloads')"
          />

          <q-btn
            class="q-ml-sm"
            color="red"
            :disable="loading || downloads.selected.length === 0"
            :label="$t('delete')"
            @click="onDelete('downloads')"
          />

          <q-btn
            class="q-ml-sm"
            color="primary"
            :disable="loading || downloads.selected.length === 0"
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
              <q-popup-edit v-model="props.row.title" v-slot="scope" auto-save>
                <q-input v-model="scope.value" dense autofocus />
              </q-popup-edit>
            </q-td>

            <q-td key="desc" :props="props">
              {{ props.row.desc }}
              <q-popup-edit v-model="props.row.desc" v-slot="scope" auto-save>
                <q-input v-model="scope.value" dense autofocus />
              </q-popup-edit>
            </q-td>

            <q-td key="url" :props="props">
              <div class="text-pre-wrap">
                {{ props.row.url }}
              </div>
              <q-popup-edit v-model="props.row.url" v-slot="scope" auto-save>
                <q-input v-model="scope.value" dense autofocus />
              </q-popup-edit>
            </q-td>
          </q-tr>
        </template>
      </q-table>
    </q-tab-panel>
  </q-tab-panels>
</template>

<script>
import { defineComponent, ref } from "vue";
import { date } from "quasar";

export default defineComponent({
  name: "compAdminOverview",

  setup() {
    return {
      loading: ref(false),
      tab: ref("announcements"),
      announcements: ref({
        selected: [],
        rows: [],
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
    };
  },

  computed: {
    column_news() {
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

    column_carousel() {
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

    column_downloads() {
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
            this.announcements.rows.push(element);
          } else if (element.type === "events") {
            this.events.rows.push(element);
          }
        }
      });

    this.$axios
      .get(this.$store.state.global.backend + "/website/carousel")
      .then((resp) => {
        for (let index = 0; index < resp.data.msg.length; index++) {
          const element = resp.data.msg[index];
          this.carousel.rows.push(element);
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
          this.downloads.rows.push(element);
        }
      })
      .catch(() => {
        this.$q.notify(this.$t("error.network"));
      });
  },

  methods: {
    onAdd(val) {
      switch (val) {
        case "announcements":
          this.announcements.rows.push({
            _id: {
              $oid: "t" + this.announcements.rows.length,
            },
            title: "New Announcement",
            date: date.formatDate(Date.now(), "YYYY-MM-DD"),
            type: "announcements",
            content: "New Announcement",
            delete: false,
          });
          break;
        case "events":
          this.events.rows.push({
            _id: {
              $oid: "t" + this.events.rows.length,
            },
            title: "New Event",
            date: date.formatDate(Date.now(), "YYYY-MM-DD"),
            type: "events",
            content: "New Event",
            delete: false,
          });
          break;
        case "carousel":
          this.carousel.rows.push({
            _id: {
              $oid: "t" + this.carousel.rows.length,
            },
            img: "img/",
            url: "",
            delete: false,
          });
          break;
        case "downloads":
          this.downloads.rows.push({
            _id: {
              $oid: "t" + this.downloads.rows.length,
            },
            title: "New Download",
            desc: "",
            url: "",
            delete: false,
          });
          break;
      }
    },

    onSave(val) {
      let selected;

      switch (val) {
        case "announcements":
          selected = this.announcements.selected;
          break;
        case "events":
          selected = this.events.selected;
          break;
        case "carousel":
          selected = this.carousel.selected;
          break;
        case "downloads":
          selected = this.downloads.selected;
          break;
      }

      selected.forEach((element) => {
        let data;
        switch (val) {
          case "announcements":
            data = {
              method: "news",
              id: element._id.$oid,
              title: element.title,
              date: element.date,
              type: "announcements",
              content: element.content,
              delete: false,
            };
            break;
          case "events":
            data = {
              method: "news",
              id: element._id.$oid,
              title: element.title,
              date: element.date,
              type: "events",
              content: element.content,
              delete: false,
            };
            break;
          case "carousel":
            data = {
              method: "events",
              id: element._id.$oid,
              img: element.img,
              url: element.url,
              delete: false,
            };
            break;
          case "downloads":
            data = {
              method: "downloads",
              id: element._id.$oid,
              title: element.title,
              desc: element.desc,
              url: element.url,
              delete: false,
            };
            break;
        }

        this.$axios
          .post(this.$store.state.global.backend + "/api/website", data, {
            headers: {
              authorization: "Bearer " + this.$q.cookies.get("canplay-token"),
            },
          })
          .then((resp) => {
            if (resp.data.status === 1) {
              this.$q.notify(this.$t("success"));
            } else {
              this.$q.notify(this.$t("failed"));
            }
          })
          .catch((e) => {
            console.log(e);
            this.$q.notify(this.$t("error.network"));
          });
      });
    },

    onDelete(val) {
      let selected;

      switch (val) {
        case "announcements":
          selected = this.announcements.selected;
          break;
        case "events":
          selected = this.events.selected;
          break;
        case "carousel":
          selected = this.carousel.selected;
          break;
        case "downloads":
          selected = this.downloads.selected;
          break;
      }

      console.log(data.id.indexOf("t"));
      return;

      selected.forEach((element) => {
        let data;
        switch (val) {
          case "announcements":
            data = {
              method: "news",
              id: element._id.$oid,
              title: element.title,
              date: element.date,
              type: "announcements",
              content: element.content,
              delete: true,
            };

            if (data.id.indexOf("t") === 1) {
              let el = this.announcements.rows.filter((e) => {
                return e.id === element.id;
              });

              let index = this.announcements.rows.indexOf(el[0]);

              index > -1 && this.announcements.rows.splice(index, 1);
            }
            break;
          case "events":
            data = {
              method: "events",
              id: element._id.$oid,
              title: element.title,
              date: element.date,
              type: "events",
              content: element.content,
              delete: true,
            };
            break;
          case "carousel":
            data = {
              method: "events",
              id: element._id.$oid,
              img: element.img,
              url: element.url,
              delete: true,
            };
            break;
          case "downloads":
            data = {
              method: "downloads",
              id: element._id.$oid,
              title: element.title,
              desc: element.desc,
              url: element.url,
              delete: true,
            };
            break;
        }

        this.$axios
          .post(this.$store.state.global.backend + "/api/website", data, {
            headers: {
              authorization: "Bearer " + this.$q.cookies.get("canplay-token"),
            },
          })
          .then((resp) => {
            console.log(resp);
          })
          .catch((e) => {
            console.log(e);
            this.$q.notify(this.$t("error.network"));
          });
      });
    },
  },
});
</script>
