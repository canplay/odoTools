<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comptoolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <q-scroll-area class="text-white" style="min-height: 400px" ref="area">
          <div class="row">
            <q-card class="col" style="background-color: rgba(0, 0, 0, 0.5)">
              <q-list>
                <q-item-label class="text-bold text-h6">
                  {{ $t("announcement") }}
                </q-item-label>

                <q-item-section>
                  <q-item clickable> 123 </q-item>
                </q-item-section>
              </q-list>
            </q-card>

            <div class="col-auto" style="width: 8px" />

            <q-card class="col" style="background-color: rgba(0, 0, 0, 0.5)">
              <q-list>
                <q-item-label class="text-bold text-h6">
                  {{ $t("event") }}
                </q-item-label>

                <q-item-section>
                  <q-item clickable> 123 </q-item>
                </q-item-section>
              </q-list>
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
              >
                <q-carousel-slide
                  :name="1"
                  img-src="https://cdn.quasar.dev/img/mountains.jpg"
                />
                <q-carousel-slide
                  :name="2"
                  img-src="https://cdn.quasar.dev/img/parallax1.jpg"
                />
                <q-carousel-slide
                  :name="3"
                  img-src="https://cdn.quasar.dev/img/parallax2.jpg"
                />
                <q-carousel-slide
                  :name="4"
                  img-src="https://cdn.quasar.dev/img/quasar.jpg"
                />
              </q-carousel>

              <div class="col-auto" style="height: 8px" />

              <q-form class="col">
                <q-input dark label="用户名" />
                <q-input dark label="密码" />

                <div style="height: 8px" />

                <div class="row">
                  <q-btn class="col" color="green" label="登录" />
                  <div class="col-auto" style="width: 8px" />
                  <q-btn class="col" color="red" label="注册" />
                </div>
              </q-form>
            </div>
          </div>
        </q-scroll-area>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script>
import { defineComponent, ref } from "vue";
import comptoolbar from "src/components/Toolbar.vue";

export default defineComponent({
  name: "PageIndex",

  setup() {
    return {
      slide: ref(1),
      autoplay: ref(true),
    };
  },

  components: {
    comptoolbar,
  },

  mounted() {
    window.addEventListener("resize", this.resize());
  },

  beforeUnmount() {
    window.removeEventListener("resize", this.resize());
  },

  methods: {
    resize() {
      if (this.$refs.area) {
        this.$refs.area.$el.style.height =
          document.querySelector(
            "#q-app > div > div > main > div > div.q-card__section.q-card__section--vert"
          ).clientHeight -
          32 +
          "px";
      }
    },
  },
});
</script>
