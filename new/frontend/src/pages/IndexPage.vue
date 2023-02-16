<template>
  <q-page padding>
    <q-scroll-area style="height: calc(100vh - 56px - 50px - 32px)">
      <div class="row" id="index">
        <q-carousel
          dark
          arrows
          animated
          infinite
          autoplay
          class="col rounded-borders"
          height="410px"
          v-model="slide.model"
        >
          <q-carousel-slide
            v-for="items in slide.list"
            :key="items"
            :name="items.title"
            :img-src="items.img"
          >
            <div class="absolute-bottom custom-caption">
              <div class="text-h2">{{ items.title }}</div>
              <div class="text-subtitle1">{{ items.desc }}</div>
            </div>
          </q-carousel-slide>
        </q-carousel>

        <div class="col-auto" style="width: 8px" />

        <q-card dark class="col" id="news">
          <q-scroll-area class="fit">
            <q-list dark v-for="items in news" :key="items">
              <q-item clickable @click="onNews(items)">
                <q-item-section>
                  <q-item-label>{{ items.title }}</q-item-label>
                  <q-item-label caption lines="2">
                    {{ items.desc }}
                  </q-item-label>
                </q-item-section>

                <q-item-section side top>
                  <q-item-label caption>
                    {{ items.date }}
                  </q-item-label>
                </q-item-section>
              </q-item>
            </q-list>
          </q-scroll-area>
        </q-card>
      </div>

      <div style="height: 8px" />

      <q-card dark>
        <q-card-section>
          <q-form @submit="onSignup" @reset="onSignin" class="row">
            <div class="col" v-if="!store.user.signin">
              <q-input
                dark
                filled
                v-model="username"
                :label="$t('username')"
                lazy-rules
                :rules="[(val) => (val && val.length > 0) || '不能为空']"
              />

              <q-input
                dark
                filled
                v-model="password"
                :label="$t('password')"
                lazy-rules
                :rules="[(val) => (val && val.length > 0) || '不能为空']"
              />

              <q-toggle
                dark
                v-model="accept"
                class="text-white"
                :label="$t('accept')"
              />
            </div>

            <div class="col" v-else>
              <div class="text-h4">用户名：{{ store.user.username }}</div>

              <div class="text-h4">家族名：{{ store.user.familyname }}</div>

              <div class="text-h4">
                会员等级：{{ store.user.membershipType }}
              </div>

              <div class="text-h4">
                游玩时间：{{ store.user.totalPlayTime }}
              </div>
            </div>

            <div class="col-auto" style="width: 8px" />

            <div class="col row" v-if="!store.user.signin">
              <q-btn
                dark
                flat
                class="col text-positive text-bold"
                size="48px"
                type="submit"
              >
                <div class="absolute-center">{{ $t('signup') }}</div>
                <div
                  class="fit relative-position"
                  style="
                    background-image: url(imgs/slide/4.jpg);
                    background-position: right bottom, left top;
                    background-repeat: no-repeat, no-repeat;
                    background-size: 100% 100%;
                    z-index: -1;
                    filter: blur(3px);
                  "
                />
              </q-btn>

              <div class="col-auto" style="width: 8px" />

              <q-btn
                dark
                flat
                class="col text-white text-bold"
                size="48px"
                type="reset"
              >
                <div class="absolute-center">{{ $t('signin') }}</div>
                <div
                  class="fit relative-position"
                  style="
                    background-image: url(imgs/slide/5.jpg);
                    background-position: right bottom, left top;
                    background-repeat: no-repeat, no-repeat;
                    background-size: 100% 100%;
                    z-index: -1;
                    filter: blur(3px);
                  "
                />
              </q-btn>
            </div>

            <div class="col row" v-else>
              <q-btn
                dark
                flat
                class="col text-positive text-bold"
                size="48px"
                type="submit"
              >
                <div class="absolute-center">{{ $t('signout') }}</div>
                <div
                  class="fit relative-position"
                  style="
                    background-image: url(imgs/slide/4.jpg);
                    background-position: right bottom, left top;
                    background-repeat: no-repeat, no-repeat;
                    background-size: 100% 100%;
                    z-index: -1;
                    filter: blur(3px);
                  "
                />
              </q-btn>

              <div class="col-auto" style="width: 8px" />

              <q-btn
                dark
                flat
                class="col text-white text-bold"
                size="48px"
                type="reset"
              >
                <div class="absolute-center">{{ $t('launcher') }}</div>
                <div
                  class="fit relative-position"
                  style="
                    background-image: url(imgs/slide/5.jpg);
                    background-position: right bottom, left top;
                    background-repeat: no-repeat, no-repeat;
                    background-size: 100% 100%;
                    z-index: -1;
                    filter: blur(3px);
                  "
                />
              </q-btn>
            </div>
          </q-form>
        </q-card-section>
      </q-card>

      <div style="height: 8px" />

      <q-img
        class="rounded-borders"
        src="imgs/slide/2.jpg"
        height="300px"
        id="shop"
        @click="onShop"
      >
        <div class="absolute-bottom custom-caption">
          <div class="text-h2">{{ $t('shop.hot') }}</div>
          <div class="text-subtitle1">{{ $t('shop.hotDesc') }}</div>
        </div>
      </q-img>

      <div style="height: 8px" />

      <q-scroll-area class="rounded-borders" style="width: 100%; height: 300px">
        <q-table
          dark
          separator="cell"
          :columns="columns"
          :rows="rows"
          row-key="class"
          hide-bottom
        >
          <template v-slot:top>
            <div class="fit text-center text-h5 text-bold">
              {{ $t('system.title') }}
            </div>
          </template>

          <template v-slot:header-cell="props">
            <q-th :props="props" style="font: bold 18px arial, sans-serif">
              {{ $t(props.col.label) }}
            </q-th>
          </template>

          <template v-slot:body="props">
            <q-tr :props="props">
              <q-td
                key="class"
                :props="props"
                style="font: bold 18px arial, sans-serif"
              >
                {{ $t(props.row.class) }}
              </q-td>

              <q-td key="min" :props="props">
                {{ props.row.min }}
              </q-td>

              <q-td key="rec" :props="props">
                {{ props.row.rec }}
              </q-td>

              <q-td key="rem" :props="props">
                {{ props.row.rem }}
              </q-td>

              <q-td key="max" :props="props">
                {{ props.row.max }}
              </q-td>
            </q-tr>
          </template>
        </q-table>
      </q-scroll-area>

      <div style="height: 8px" />

      <div class="row">
        <q-btn
          dark
          flat
          class="col text-white text-bold"
          size="24px"
          to="https://www.nvidia.com/Download/index.aspx"
        >
          <div class="absolute-center">{{ $t('nvidia') }}</div>
          <div
            class="fit relative-position"
            style="
              background-image: url(imgs/slide/5.jpg);
              background-position: right bottom, left top;
              background-repeat: no-repeat, no-repeat;
              background-size: 100% 100%;
              z-index: -1;
              filter: blur(3px);
            "
          />
        </q-btn>

        <div style="width: 8px" />

        <q-btn
          dark
          flat
          class="col text-white text-bold"
          size="24px"
          to="https://www.amd.com/en/support"
        >
          <div class="absolute-center">{{ $t('amd') }}</div>
          <div
            class="fit relative-position"
            style="
              background-image: url(imgs/slide/5.jpg);
              background-position: right bottom, left top;
              background-repeat: no-repeat, no-repeat;
              background-size: 100% 100%;
              z-index: -1;
              filter: blur(3px);
            "
          />
        </q-btn>
      </div>

      <div style="height: 8px" />

      <div class="row" id="downloads">
        <q-btn dark flat class="col text-white text-bold" size="48px">
          <div class="absolute-center">{{ $t('client') }}</div>
          <div
            class="fit relative-position"
            style="
              background-image: url(imgs/slide/5.jpg);
              background-position: right bottom, left top;
              background-repeat: no-repeat, no-repeat;
              background-size: 100% 100%;
              z-index: -1;
              filter: blur(3px);
            "
          />
        </q-btn>
      </div>

      <div style="height: 8px" />

      <q-card dark class="text-h6 text-center" id="social">
        <q-card-section>
          {{ $t('social') }}
          <div class="row">
            <q-btn dark flat class="col" icon="img:imgs/weibo.svg" />
            <q-btn dark flat class="col" icon="img:imgs/qq.svg" />
            <q-btn dark flat class="col" icon="img:imgs/wechat.svg" />
            <q-btn dark flat class="col" icon="img:imgs/discord.svg" />
            <q-btn dark flat class="col" icon="img:imgs/telegram.svg" />
            <q-btn dark flat class="col" icon="img:imgs/discord.svg" />
          </div>
        </q-card-section>
      </q-card>
    </q-scroll-area>
  </q-page>

  <q-dialog v-model="dialog.news.show">
    <q-card dark style="width: 80vw; height: 80vh">
      <q-card-section>
        <div class="text-h4 text-bold">{{ dialog.news.title }}</div>

        <div style="height: 8px" />

        <div class="text-h6">{{ dialog.news.date }}</div>

        <div style="height: 8px" />

        <div>{{ dialog.news.content }}</div>
      </q-card-section>
    </q-card>
  </q-dialog>

  <q-dialog v-model="dialog.shop.show">
    <q-card dark style="width: 80vw; height: 80vh">
      <q-card-section>
        <q-table
          dark
          :rows="dialog.shop.rows"
          :columns="dialog.shop.columns"
          row-key="name"
          selection="multiple"
          v-model:selected="dialog.shop.selected"
          :filter="dialog.shop.filter"
          v-model:pagination="dialog.shop.pagination"
          grid
          hide-header
        >
          <template v-slot:top>
            <div class="row fit">
              <div class="col text-h4 text-bold">{{ $t('shop.title') }}</div>

              <q-input
                class="col-auto"
                dark
                borderless
                dense
                debounce="300"
                v-model="dialog.shop.filter"
                :placeholder="$t('search')"
              >
                <template v-slot:append>
                  <q-icon name="search" />
                </template>
              </q-input>
            </div>
          </template>

          <template v-slot:item="props">
            <div
              class="q-pa-xs col-xs-12 col-sm-6 col-md-4 col-lg-3 grid-style-transition"
              :style="props.selected ? 'transform: scale(0.95);' : ''"
            >
              <q-card dark :class="props.selected ? 'bg-grey-2' : ''">
                <q-card-section>
                  <q-checkbox
                    dense
                    v-model="props.selected"
                    :label="props.row.name"
                  />
                </q-card-section>
                <q-separator />
                <q-list dense>
                  <q-item
                    v-for="col in props.cols.filter(
                      (col) => col.name !== 'desc'
                    )"
                    :key="col.name"
                  >
                    <q-item-section>
                      <q-item-label>{{ col.label }}</q-item-label>
                    </q-item-section>
                    <q-item-section side>
                      <q-item-label caption>{{ col.value }}</q-item-label>
                    </q-item-section>
                  </q-item>
                </q-list>
              </q-card>
            </div>
          </template>
        </q-table>
      </q-card-section>
    </q-card>
  </q-dialog>
</template>

<style lang="sass" scoped>
.custom-caption
  text-align: center
  padding: 12px
  color: white
  background-color: rgba(0, 0, 0, .3)
</style>

<script setup lang="ts">
import { ref } from 'vue';
import { useQuasar, QTableProps } from 'quasar';
import { useStore } from 'src/stores/store';
import useFetch from 'components/fetch';
import electron from 'electron';

const $q = useQuasar();
const store = useStore();

const slide = ref({
  model: '1',
  list: [] as any,
});
const news = ref([] as any);
const username = ref('');
const password = ref('');
const accept = ref(false);

const columns = [
  {
    name: 'class',
    label: 'system.class',
    field: 'class',
    align: 'center',
    classes: 'text-bold',
  },
  {
    name: 'min',
    label: 'system.min',
    field: 'min',
    align: 'center',
  },
  {
    name: 'rec',
    label: 'system.rec',
    field: 'rec',
    align: 'center',
  },
  {
    name: 'rem',
    label: 'system.rem',
    field: 'rem',
    align: 'center',
  },
  {
    name: 'max',
    label: 'system.max',
    field: 'max',
    align: 'center',
  },
] as QTableProps['columns'];

const rows = ref([
  {
    class: 'system.cpu',
    min: 'Intel Core i3',
    rec: 'Intel Core i5',
    rem: 'Intel Core i7-8700',
    max: 'Intel Core i7-8700K',
  },
  {
    class: 'system.disk',
    min: '~41GB',
    rec: '~41GB',
    rem: '~41GB',
    max: '~41GB',
  },
  {
    class: 'system.ram',
    min: '4GB',
    rec: '8GB',
    rem: '16GB',
    max: '32GB',
  },
  {
    class: 'system.graphics',
    min: 'NVIDIA GTS 250、AMD HD 3870 X2',
    rec: 'NVIDIA GTX 970、AMD RX 480',
    rem: 'NVIDIA GTX 1070 8GB',
    max: 'NVIDIA GTX 1080ti 11GB',
  },
  {
    class: 'system.os',
    min: 'Windows 10 x64',
    rec: 'Windows 10 x64',
    rem: 'Windows 10 x64',
    max: 'Windows 10 x64',
  },
] as any);

const dialog = ref({
  news: {
    show: false,
    title: '',
    date: '',
    content: '',
  },
  shop: {
    show: false,
    pagination: {
      page: 1,
      rowsNumber: 0,
      rowsPerPage: 7,
      sortBy: null,
      descending: true,
    } as any,
    columns: [
      {
        name: 'name',
        required: true,
        label: 'Dessert (100g serving)',
        align: 'left',
      },
      {
        name: 'calories',
        align: 'center',
        label: 'Calories',
        field: 'calories',
        sortable: true,
      },
      { name: 'fat', label: 'Fat (g)', field: 'fat', sortable: true },
      { name: 'carbs', label: 'Carbs (g)', field: 'carbs' },
      { name: 'protein', label: 'Protein (g)', field: 'protein' },
      { name: 'sodium', label: 'Sodium (mg)', field: 'sodium' },
      {
        name: 'calcium',
        label: 'Calcium (%)',
        field: 'calcium',
        sortable: true,
      },
      { name: 'iron', label: 'Iron (%)', field: 'iron', sortable: true },
    ] as QTableProps['columns'],
    rows: [
      {
        name: 'Frozen Yogurt',
        calories: 159,
        fat: 6.0,
        carbs: 24,
        protein: 4.0,
        sodium: 87,
        calcium: '14%',
        iron: '1%',
      },
    ] as any,
    filter: '',
    selected: [] as any,
  },
});

for (let index = 0; index < 8; index++) {
  news.value.push({
    title: '新闻标题' + index,
    desc: '新闻简介',
    date: '2023-2-15',
    link: '',
  });
}

for (let index = 0; index < 5; index++) {
  slide.value.list.push({
    title: '滚动图标题' + index,
    desc: '滚动图简介',
    img: 'imgs/slide/' + (index + 1) + '.jpg',
  });
}

const onSignin = () => {
  if (store.user.signin) {
    electron.ipcRenderer.send('launcher', username.value, password.value);
  } else {
    if (!accept.value) {
      $q.notify('需要同意用户协议才能注册');
      return;
    }

    let time = setTimeout(() => {
      $q.loading.hide();
      clearTimeout(time);
    }, 180000);

    $q.loading.show();

    useFetch()
      .post(store.backend + '/api/user/signin', {
        username: username.value,
        password: password.value,
      })
      .then((resp) => {
        if (resp.data.status === 0) {
          $q.loading.hide();
          clearTimeout(time);
          $q.notify(resp.data.msg);
          return;
        }

        $q.cookies.set('canplay_token', resp.data.msg.token);

        store.user = {
          signin: true,
          username: username.value,
          password: password.value,
          familyname: resp.data.msg.userNickname,
          totalPlayTime: resp.data.msg.totalPlayTime,
          membershipType: resp.data.msg.membershipType,
        };

        $q.loading.hide();
        clearTimeout(time);
      })
      .catch(() => {
        $q.loading.hide();
        clearTimeout(time);
        $q.notify('网络错误，请稍后重试');
      });
  }
};

const onSignup = () => {
  if (store.user.signin) {
    $q.cookies.remove('canplay_token');
    store.user = {
      signin: false,
      username: '',
      password: '',
      familyname: '',
      totalPlayTime: 0,
      membershipType: 0,
    };
  } else {
    let time = setTimeout(() => {
      $q.loading.hide();
      clearTimeout(time);
    }, 180000);

    $q.loading.show();

    useFetch()
      .post(store.backend + '/api/user/signup', {
        username: username.value,
        password: password.value,
      })
      .then((resp) => {
        if (resp.data.status === 0) {
          $q.loading.hide();
          clearTimeout(time);
          $q.notify(resp.data.msg);
          return;
        }

        $q.cookies.set('canplay_token', resp.data.msg.token);

        store.user = {
          signin: true,
          username: username.value,
          password: password.value,
          familyname: resp.data.msg.userNickname,
          totalPlayTime: resp.data.msg.totalPlayTime,
          membershipType: resp.data.msg.membershipType,
        };

        $q.loading.hide();
        clearTimeout(time);
      })
      .catch(() => {
        $q.loading.hide();
        clearTimeout(time);
        $q.notify('网络错误，请稍后重试');
      });
  }
};

const onNews = (val: any) => {
  dialog.value.news = {
    show: true,
    title: val.title,
    date: val.date,
    content: val.desc,
  };
};

const onShop = (val: any) => {
  dialog.value.shop.show = true;
};
</script>
