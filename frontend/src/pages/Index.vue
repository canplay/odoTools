<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comToolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <div class="row text-white">
          <q-card class="col transparent">
            <div class="text-h5 text-center">
              {{ $t("index.announcement") }}
            </div>

            <q-card-section>
              <q-scroll-area ref="container1">
                <q-list>
                  <q-item-section>
                    <q-item
                      v-for="item in announcements"
                      :key="item"
                      clickable
                      @click="onView(item)"
                    >
                      <q-item-section class="ellipsis">
                        {{ item.title }}
                      </q-item-section>
                      <q-item-section side>{{ item.date }}</q-item-section>
                    </q-item>
                  </q-item-section>
                </q-list>
              </q-scroll-area>
            </q-card-section>
          </q-card>

          <div class="col-auto" style="width: 8px" />

          <q-card class="col transparent">
            <div class="text-h5 text-center">
              {{ $t("index.event") }}
            </div>

            <q-card-section>
              <q-scroll-area ref="container2">
                <q-list>
                  <q-item-section>
                    <q-item
                      v-for="item in events"
                      :key="item"
                      clickable
                      @click="onView(item)"
                    >
                      <q-item-section class="ellipsis">
                        {{ item.title }}
                      </q-item-section>
                      <q-item-section side>{{ item.date }}</q-item-section>
                    </q-item>
                  </q-item-section>
                </q-list>
              </q-scroll-area>
            </q-card-section>
          </q-card>

          <div class="col-auto" style="width: 8px" />

          <q-card class="col transparent">
            <q-card-section>
              <q-carousel
                class="col"
                animated
                v-model="carousel.index"
                :autoplay="carousel.autoplay"
                infinite
                thumbnails
                transition-prev="slide-right"
                transition-next="slide-left"
                @mouseenter="carousel.autoplay = false"
                @mouseleave="carousel.autoplay = true"
                style="min-height: 240px"
              >
                <q-carousel-slide
                  :name="index"
                  v-for="(item, index) in carousel.items"
                  :key="index"
                  :img-src="item.img"
                />
              </q-carousel>

              <div class="col-auto" style="height: 8px" />

              <q-form class="col" @submit="onSubmit">
                <div v-if="!$store.state.global.loginin">
                  <q-input
                    dark
                    v-model="username"
                    :label="$t('index.username')"
                    filled
                    lazy-rules
                    :rules="[
                      (val) =>
                        (val && val.length > 0) || $t('error.noUsername'),
                    ]"
                  />

                  <q-input
                    v-if="!$store.state.global.loginin"
                    dark
                    v-model="password"
                    :label="$t('index.password')"
                    filled
                    :type="isPwd ? 'password' : 'text'"
                    lazy-rules
                    :rules="[
                      (val) =>
                        (val && val.length > 0) || $t('error.noPassword'),
                    ]"
                  >
                    <template v-slot:append>
                      <q-icon
                        :name="isPwd ? 'visibility_off' : 'visibility'"
                        class="cursor-pointer"
                        @click="isPwd = !isPwd"
                      />
                    </template>
                  </q-input>
                </div>

                <div v-else>
                  <q-card dark>
                    <q-card-section>
                      <div class="row">
                        <div class="col text-h6">
                          {{ $t("index.username") }}: {{ this.username }}
                        </div>
                        <div class="col text-h6">
                          {{ $t("user.family") }}:
                          {{ $store.state.global.user.family }}
                        </div>
                      </div>

                      <div class="col-auto" style="height: 8px" />

                      <div class="row">
                        <div class="col text-h6">
                          {{ $t("user.access_level") }}:
                          {{ $store.state.global.user.access_level }}
                        </div>
                        <div class="col text-h6">
                          {{ $t("user.cash") }}:
                          {{ $store.state.global.user.cash }}
                        </div>
                      </div>
                    </q-card-section>
                  </q-card>

                  <div class="col-auto" style="height: 8px" />
                </div>

                <div v-if="!$store.state.global.loginin" class="row">
                  <q-btn
                    class="col"
                    color="green"
                    :label="$t('index.login')"
                    type="submit"
                  />

                  <div class="col-auto" style="width: 8px" />

                  <q-btn
                    class="col"
                    color="red"
                    :label="$t('index.register')"
                  />
                </div>

                <div v-else class="row">
                  <q-btn
                    class="col"
                    color="green"
                    :label="$t('index.run')"
                    @click="onRun"
                  />

                  <div class="col-auto" style="width: 8px" />

                  <q-btn
                    class="col"
                    color="red"
                    :label="$t('index.loginout')"
                    @click="onLoginout"
                  />
                </div>
              </q-form>
            </q-card-section>
          </q-card>
        </div>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import comToolbar from "components/Toolbar";
import jwt_decode from "jwt-decode";

export default defineComponent({
  name: "PageIndex",

  components: {
    comToolbar,
  },

  setup() {
    return {
      carousel: ref({
        autoplay: ref(true),
        index: 1,
        items: [],
      }),
      isPwd: ref(true),
      username: ref(""),
      password: ref(""),
      announcements: ref([]),
      events: ref([]),
    };
  },

  created() {
    this.$axios
      .get(this.$store.state.global.database.config)
      .then((resp) => {
        for (let index = 0; index < resp.data.announcements.length; index++) {
          const element = resp.data.announcements[index];
          this.announcements.push(element);
        }

        for (let index = 0; index < resp.data.events.length; index++) {
          const element = resp.data.events[index];
          this.events.push(element);
        }

        this.carousel.items = resp.data.carousel;
      })
      .catch((error) => {
        console.log(error);
        this.$q.notify(this.$t("error.network"));
      });

    if (
      this.$q.sessionStorage.getItem("canplay") != null &&
      this.$q.sessionStorage.getItem("canplay") != ""
    ) {
      let token = jwt_decode(this.$q.sessionStorage.getItem("canplay"));
      this.username = token.username;
      this.password = token.password;
    }

    this.$nextTick(() => {
      window.addEventListener("resize", this.onResize());
    });
  },

  beforeUnmount() {
    window.removeEventListener("resize", this.onResize());
  },

  methods: {
    onResize() {
      if (this.$refs.container1) {
        this.$refs.container1.$el.style.height =
          document.querySelector(
            "#q-app > div > div > main > div > div.q-card__section.q-card__section--vert > div > div:nth-child(5)"
          ).clientHeight -
          document.querySelector(
            "#q-app > div > div > main > div > div.q-card__section.q-card__section--vert > div > div:nth-child(1) > div.text-h5.text-center"
          ).clientHeight -
          32 +
          "px";
      }

      if (this.$refs.container2) {
        this.$refs.container2.$el.style.height =
          document.querySelector(
            "#q-app > div > div > main > div > div.q-card__section.q-card__section--vert > div > div:nth-child(5)"
          ).clientHeight -
          document.querySelector(
            "#q-app > div > div > main > div > div.q-card__section.q-card__section--vert > div > div:nth-child(3) > div.text-h5.text-center"
          ).clientHeight -
          32 +
          "px";
      }
    },

    onSubmit() {
      this.$axios
        .post(this.$store.state.global.backend + "/user/login", {
          username: this.username,
          password: this.password,
        })
        .then((resp) => {
          try {
            if (resp.data.msg === "loginin") {
              this.$store.commit("global/login", true);
              this.$q.sessionStorage.set("canplay", resp.data.token);

              try {
                this.$axios
                  .get(
                    this.$store.state.global.backend +
                      "/api/user/" +
                      this.username,
                    {
                      headers: {
                        authorization:
                          "Bearer " + this.$q.sessionStorage.getItem("canplay"),
                      },
                    }
                  )
                  .then((resp) => {
                    this.$store.commit("global/user", {
                      family: resp.data.msg.family,
                      cash: resp.data.msg.cash,
                      access_level: resp.data.msg.access_level,
                      avatar: {
                        name: "",
                        level: 0,
                        types: "",
                        pearl: 0,
                      },
                    });
                  });
              } catch (error) {
                console.log(error);
              }
            } else {
              this.$q.notify(this.$t("error.network"));
            }
          } catch (error) {
            this.$q.notify(this.$t("error.network"));
          }
        })
        .catch(() => {
          this.$q.notify(this.$t("error.network"));
        });
    },

    onLoginout() {
      this.$store.commit("global/login", false);
      this.$q.sessionStorage.set("canplay", "");
    },

    onRun() {
      this.$axios
        .get(this.$store.state.global.database.config)
        .then((resp) => {
          window.location.href =
            "odolauncher://" +
            resp.data.server +
            "&" +
            this.username +
            "&" +
            this.password;
        })
        .catch(() => {
          this.$q.notify(this.$t("error.launcher"));
        });
    },

    onView(val) {
      this.$router.push({
        name: "newsview",
        params: { view: JSON.stringify(val) },
      });
    },
  },
});
</script>
