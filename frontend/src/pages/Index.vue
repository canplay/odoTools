<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comtoolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <div class="row text-white">
          <q-card class="col transparent">
            <div class="text-h5 text-center">
              {{ $t("index.announcement") }}
            </div>

            <q-card-section class="fit">
              <q-scroll-area class="fit">
                <q-list>
                  <q-item-section>
                    <q-item v-for="item in announcements" :key="item" clickable>
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

            <q-card-section class="fit">
              <q-scroll-area class="fit">
                <q-list>
                  <q-item-section>
                    <q-item v-for="item in events" :key="item" clickable>
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

          <div class="col column">
            <q-carousel
              class="col"
              animated
              v-model="slide"
              navigation
              infinite
              :autoplay="autoplay"
              arrows
              transition-prev="slide-right"
              transition-next="slide-left"
              @mouseenter="autoplay = false"
              @mouseleave="autoplay = true"
              style="min-height: 240px"
            >
              <q-carousel-slide :name="1" img-src="imgs/no-imgs.png" />
              <q-carousel-slide :name="2" img-src="imgs/no-imgs.png" />
              <q-carousel-slide :name="3" img-src="imgs/no-imgs.png" />
              <q-carousel-slide :name="4" img-src="imgs/no-imgs.png" />
            </q-carousel>

            <div class="col-auto" style="height: 8px" />

            <q-form class="col" @submit="onSubmit">
              <q-input
                dark
                v-model="username"
                :label="$t('index.username')"
                filled
                lazy-rules
                :rules="[
                  (val) => (val && val.length > 0) || $t('error.noUsername'),
                ]"
              />

              <div style="height: 8px" />

              <q-input
                dark
                v-model="password"
                :label="$t('index.password')"
                filled
                :type="isPwd ? 'password' : 'text'"
                lazy-rules
                :rules="[
                  (val) => (val && val.length > 0) || $t('error.noPassword'),
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

              <div style="height: 8px" />

              <div class="row">
                <q-btn
                  class="col"
                  color="green"
                  :label="$t('index.login')"
                  type="submit"
                />

                <div class="col-auto" style="width: 8px" />

                <q-btn class="col" color="red" :label="$t('index.register')" />
              </div>
            </q-form>
          </div>
        </div>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import comtoolbar from "components/Toolbar.vue";

export default defineComponent({
  name: "PageIndex",

  components: {
    comtoolbar,
  },

  setup() {
    return {
      slide: ref(1),
      autoplay: ref(true),
      isPwd: ref(true),
      username: ref(""),
      password: ref(""),
      announcements: ref([]),
      events: ref([]),
    };
  },

  created() {
    for (let index = 0; index < 18; index++) {
      this.announcements.push({
        title: "1234567890abcdefghijklmnopq",
        date: "2022-01-01 00:00:00",
        author: "admin",
      });
    }

    for (let index = 0; index < 8; index++) {
      this.events.push({
        title: "abcdefghijklmnopq1234567890",
        date: "2022-01-01 00:00:00",
        author: "admin",
      });
    }
  },

  methods: {
    onSubmit() {
      this.$q.notify({
        color: "red-5",
        textColor: "white",
        icon: "warning",
        message: "error",
      });
    },
  },
});
</script>
