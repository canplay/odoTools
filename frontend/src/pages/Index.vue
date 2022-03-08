<template>
  <q-page padding>
    <q-card class="transparent no-box-shadow">
      <comtoolbar />

      <div style="height: 8px" />

      <q-card-section style="background-color: rgba(0, 0, 0, 0.8)">
        <q-scroll-area class="text-white" style="min-height: 500px" ref="area">
          <div class="row">
            <q-card class="col transparent">
              <q-list>
                <q-item-label class="text-bold text-h5 text-center">
                  {{ $t("announcement") }}
                </q-item-label>

                <q-item-section>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                </q-item-section>
              </q-list>
            </q-card>

            <div class="col-auto" style="width: 8px" />

            <q-card class="col transparent">
              <q-list>
                <q-item-label class="text-bold text-h5 text-center">
                  {{ $t("event") }}
                </q-item-label>

                <q-item-section>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
                  <q-item clickable> 123 </q-item>
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
                style="min-height: 320px"
              >
                <q-carousel-slide :name="1" img-src="imgs/no-imgs.png" />
                <q-carousel-slide :name="2" img-src="imgs/no-imgs.png" />
                <q-carousel-slide :name="3" img-src="imgs/no-imgs.png" />
                <q-carousel-slide :name="4" img-src="imgs/no-imgs.png" />
              </q-carousel>

              <div class="col-auto" style="height: 8px" />

              <q-form class="col">
                <q-input dark :label="$t('username')" />
                <q-input dark :label="$t('password')" />

                <div style="height: 8px" />

                <div class="row">
                  <q-btn class="col" color="green" :label="$t('login')" />
                  <div class="col-auto" style="width: 8px" />
                  <q-btn class="col" color="red" :label="$t('register')" />
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
    };
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
