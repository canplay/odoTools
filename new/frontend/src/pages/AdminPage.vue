<template>
  <q-page>
    <q-card square>
      <q-splitter
        v-model="splitterModel"
        style="height: calc(100vh - 56px - 50px)"
      >
        <template v-slot:before>
          <q-scroll-area class="fit">
            <q-tabs v-model="tab" vertical class="text-teal">
              <q-tab name="index" icon="home" label="概览" />
              <q-tab name="web" icon="analytics" label="前端管理" />
              <q-tab name="user" icon="groups" label="用户管理" />
              <q-tab name="shop" icon="store" label="商城管理" />
              <q-tab name="system" icon="settings" label="系统设置" />
            </q-tabs>
          </q-scroll-area>
        </template>

        <template v-slot:after>
          <q-scroll-area class="fit">
            <q-tab-panels
              v-model="tab"
              animated
              swipeable
              vertical
              transition-prev="jump-up"
              transition-next="jump-up"
            >
              <q-tab-panel name="index">
                <div class="row text-h5">
                  <q-card class="col">
                    <q-card-section>
                      <div>
                        服务器IP地址(外网):
                        {{ system.ip_external }}
                      </div>

                      <div>
                        服务器IP地址(内网):
                        {{ system.ip_interior }}
                      </div>

                      <div>
                        Web端口:
                        {{ system.port }}
                      </div>
                    </q-card-section>
                  </q-card>

                  <div class="col-auto" style="width: 8px" />

                  <q-card class="col">
                    <q-card-section>
                      <div>
                        数据库地址:
                        {{ system.database.host }}
                      </div>

                      <div>
                        数据库账号:
                        {{ system.database.username }}
                      </div>

                      <div>
                        数据库密码:
                        {{ system.database.password }}
                      </div>
                    </q-card-section>
                  </q-card>
                </div>
              </q-tab-panel>

              <q-tab-panel name="web">
                <q-table
                  :rows="table.news.rows"
                  :columns="table.news.columns"
                  row-key="id"
                  separator="cell"
                  v-model:pagination="table.news.pagination"
                  :filter="table.news.filter"
                  @request="onTableNewsRequest"
                >
                  <template v-slot:top>
                    <div class="fit row">
                      <div class="col text-h5 flex items-center">新闻公告</div>

                      <q-space class="col-auto" />

                      <q-btn
                        class="col-2"
                        color="primary"
                        label="查询"
                        @click="onTableNewsQuery"
                      />

                      <div class="col-auto" style="width: 8px" />

                      <q-btn
                        class="col-2"
                        color="red"
                        label="新增"
                        @click="onTableNewsNew"
                      />
                    </div>
                  </template>

                  <template v-slot:header-cell="props">
                    <q-th
                      :props="props"
                      style="font: bold 15px arial, sans-serif"
                    >
                      {{ props.col.label }}
                    </q-th>
                  </template>

                  <template v-slot:body-cell="props">
                    <q-td v-if="props.col.name === 'id'" :props="props">
                      <q-btn
                        color="primary"
                        :label="props.value"
                        @click="onView('news', props.row)"
                      />
                    </q-td>

                    <q-td v-else :props="props">
                      <div
                        style="
                          max-width: 400px;
                          overflow: hidden;
                          text-overflow: ellipsis;
                          white-space: nowrap;
                        "
                      >
                        {{ props.value }}
                      </div>
                    </q-td>
                  </template>
                </q-table>

                <div style="height: 8px" />

                <q-table
                  :rows="table.slide.rows"
                  :columns="table.slide.columns"
                  row-key="id"
                  separator="cell"
                  v-model:pagination="table.slide.pagination"
                  :filter="table.slide.filter"
                  @request="onTableSlideRequest"
                >
                  <template v-slot:top>
                    <div class="fit row">
                      <div class="col text-h5 flex items-center">滚动图</div>

                      <q-space class="col-auto" />

                      <q-btn
                        class="col-2"
                        color="primary"
                        label="查询"
                        @click="onTableSlideQuery"
                      />
                    </div>
                  </template>

                  <template v-slot:header-cell="props">
                    <q-th
                      :props="props"
                      style="font: bold 15px arial, sans-serif"
                    >
                      {{ props.col.label }}
                    </q-th>
                  </template>

                  <template v-slot:body-cell="props">
                    <q-td v-if="props.col.name === 'id'" :props="props">
                      <q-btn
                        color="primary"
                        :label="props.value"
                        @click="onView('slide', props.row)"
                      />
                    </q-td>

                    <q-td v-else :props="props">
                      <div
                        style="
                          max-width: 400px;
                          overflow: hidden;
                          text-overflow: ellipsis;
                          white-space: nowrap;
                        "
                      >
                        {{ props.value }}
                      </div>
                    </q-td>
                  </template>
                </q-table>

                <div style="height: 8px" />

                <q-table
                  :rows="table.link.rows"
                  :columns="table.link.columns"
                  row-key="id"
                  separator="cell"
                  v-model:pagination="table.link.pagination"
                  :filter="table.link.filter"
                  @request="onTableLinkRequest"
                >
                  <template v-slot:top>
                    <div class="fit row">
                      <div class="col text-h5 flex items-center">固定链接</div>

                      <q-space class="col-auto" />

                      <q-btn
                        class="col-2"
                        color="primary"
                        label="查询"
                        @click="onTableLinkQuery"
                      />
                    </div>
                  </template>

                  <template v-slot:header-cell="props">
                    <q-th
                      :props="props"
                      style="font: bold 15px arial, sans-serif"
                    >
                      {{ props.col.label }}
                    </q-th>
                  </template>

                  <template v-slot:body-cell="props">
                    <q-td v-if="props.col.name === 'id'" :props="props">
                      <q-btn
                        color="primary"
                        :label="props.value"
                        @click="onView('link', props.row)"
                      />
                    </q-td>

                    <q-td v-else :props="props">
                      <div
                        style="
                          max-width: 400px;
                          overflow: hidden;
                          text-overflow: ellipsis;
                          white-space: nowrap;
                        "
                      >
                        {{ props.value }}
                      </div>
                    </q-td>
                  </template>
                </q-table>
              </q-tab-panel>

              <q-tab-panel name="user">
                <q-table
                  :rows="table.user.rows"
                  :columns="table.user.columns"
                  row-key="userId"
                  separator="cell"
                  v-model:pagination="table.user.pagination"
                  :filter="table.user.filter"
                  @request="onTableUserRequest"
                >
                  <template v-slot:top>
                    <div class="fit row">
                      <div class="col text-h5 flex items-center">用户管理</div>

                      <q-space class="col-auto" />

                      <q-btn
                        class="col-2"
                        color="primary"
                        label="查询"
                        @click="onTableUserQuery"
                      />

                      <div class="col-auto" style="width: 8px" />

                      <q-btn
                        class="col-2"
                        color="red"
                        label="新增"
                        @click="onTableNewsNew"
                      />
                    </div>
                  </template>

                  <template v-slot:header-cell="props">
                    <q-th
                      :props="props"
                      style="font: bold 15px arial, sans-serif"
                    >
                      {{ props.col.label }}
                    </q-th>
                  </template>

                  <template v-slot:body-cell="props">
                    <q-td v-if="props.col.name === 'userNo'" :props="props">
                      <q-btn
                        color="primary"
                        :label="props.value"
                        @click="onView('user', props.row)"
                      />
                    </q-td>

                    <q-td
                      v-else-if="props.col.name === 'characters'"
                      :props="props"
                    >
                      <q-list bordered v-for="item in props.value" :key="item">
                        <q-item class="text-left">
                          <q-item-section>
                            <q-item-label overline>
                              角色名：{{ item.characterName }}
                            </q-item-label>

                            <q-item-label overline>
                              职业：{{ item.classType }}
                            </q-item-label>

                            <q-item-label caption>
                              负重：{{ item.variedWeight / 100000 }}
                            </q-item-label>

                            <q-item-label caption>
                              游玩时长：{{ item.totalPlayTime }}
                            </q-item-label>
                          </q-item-section>

                          <q-item-section side top>
                            <q-item-label caption>
                              等级：{{ item.level }}
                            </q-item-label>

                            <q-item-label caption>
                              攻击力：{{ item.offenceValue }}
                            </q-item-label>

                            <q-item-label caption>
                              防御力：{{ item.defenceValue }}
                            </q-item-label>

                            <q-item-label caption>
                              觉醒攻击力：{{ item.awakenValue }}
                            </q-item-label>
                          </q-item-section>
                        </q-item>
                      </q-list>
                    </q-td>

                    <q-td v-else :props="props">
                      <div
                        style="
                          max-width: 400px;
                          overflow: hidden;
                          text-overflow: ellipsis;
                          white-space: nowrap;
                        "
                      >
                        {{ props.value }}
                      </div>
                    </q-td>
                  </template>
                </q-table>
              </q-tab-panel>
            </q-tab-panels>
          </q-scroll-area>
        </template>
      </q-splitter>
    </q-card>
  </q-page>

  <q-dialog v-model="dialog.news.show">
    <q-card style="min-width: 80vw">
      <q-card-section class="text-h6">编辑新闻</q-card-section>

      <q-card-section>
        <div class="row">
          <q-input class="col" v-model="dialog.news.title" label="标题" />

          <div class="col-auto" style="width: 8px" />

          <q-input class="col" v-model="dialog.news.date" label="日期">
            <template v-slot:prepend>
              <q-icon name="event" class="cursor-pointer">
                <q-popup-proxy transition-show="scale" transition-hide="scale">
                  <q-date v-model="dialog.news.date" mask="YYYY-MM-DD HH:mm:ss">
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="关闭" color="primary" flat />
                    </div>
                  </q-date>
                </q-popup-proxy>
              </q-icon>
            </template>

            <template v-slot:append>
              <q-icon name="access_time" class="cursor-pointer">
                <q-popup-proxy transition-show="scale" transition-hide="scale">
                  <q-time
                    v-model="dialog.news.date"
                    mask="YYYY-MM-DD HH:mm:ss"
                    format24h
                  >
                    <div class="row items-center justify-end">
                      <q-btn v-close-popup label="关闭" color="primary" flat />
                    </div>
                  </q-time>
                </q-popup-proxy>
              </q-icon>
            </template>
          </q-input>

          <div class="col-auto" style="width: 8px" />

          <q-input v-model="dialog.news.author" label="作者" />
        </div>

        <div style="height: 8px" />

        <q-input
          class="fit"
          v-model="dialog.news.content"
          type="textarea"
          filled
          input-style="height: 60vh"
        />
      </q-card-section>

      <q-card-actions class="row">
        <q-btn
          class="col"
          label="删除"
          color="negative"
          @click="onDel('news')"
        />
        <q-btn
          class="col"
          label="保存"
          color="primary"
          @click="onSave('news')"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <q-dialog v-model="dialog.slide.show">
    <q-card style="min-width: 80vw">
      <q-card-section class="text-h6">编辑滚动图</q-card-section>

      <q-card-section>
        <div class="row">
          <q-input class="col" v-model="dialog.slide.title" label="标题" />

          <div class="col-auto" style="width: 8px" />

          <q-input class="col" v-model="dialog.slide.desc" label="简介" />

          <div class="col-auto" style="width: 8px" />

          <q-input class="col" v-model="dialog.slide.link" label="链接" />
        </div>

        <div class="col-auto" style="height: 8px" />

        <q-input v-model="dialog.slide.img" label="图片链接" />
      </q-card-section>

      <q-card-actions class="row">
        <q-btn
          class="col"
          label="保存"
          color="primary"
          @click="onSave('slide')"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <q-dialog v-model="dialog.link.show">
    <q-card style="min-width: 80vw">
      <q-card-section class="text-h6">编辑滚动图</q-card-section>

      <q-card-section>
        <div class="row">
          <q-input class="col" v-model="dialog.link.title" label="标题" />

          <div class="col-auto" style="width: 8px" />

          <q-input class="col" v-model="dialog.link.link" label="链接" />
        </div>
      </q-card-section>

      <q-card-actions class="row">
        <q-btn
          class="col"
          label="保存"
          color="primary"
          @click="onSave('link')"
        />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { date, useQuasar, QTableProps } from 'quasar';
import { useStore } from 'stores/store';
import useFetch from 'src/components/fetch';
import { Base64 } from 'js-base64';

const $q = useQuasar();
const store = useStore();

const splitterModel = ref(8);
const tab = ref('index');

const system = ref({
  ip_external: '0.0.0.0',
  ip_interior: '0.0.0.0',
  port: '',
  database: {
    host: '',
    username: '',
    password: '',
  },
});

const table = ref({
  news: {
    pagination: {
      page: 1,
      rowsNumber: 0,
      rowsPerPage: 7,
      sortBy: null,
      descending: true,
    } as any,
    filter: '',
    columns: [
      {
        name: 'id',
        label: 'ID',
        field: 'id',
        align: 'center',
      },
      {
        name: 'title',
        label: '标题',
        field: 'title',
        align: 'center',
        sortable: true,
      },
      {
        name: 'date',
        label: '日期',
        field: 'date',
        align: 'center',
        sortable: true,
      },
      {
        name: 'author',
        label: '作者',
        field: 'author',
        align: 'center',
        sortable: true,
      },
      {
        name: 'content',
        label: '内容',
        field: 'content',
        align: 'center',
        sortable: true,
      },
      {
        name: 'delete',
        label: '是否删除',
        field: 'delete',
        align: 'center',
        sortable: true,
      },
      {
        name: 'create_date',
        label: '创建日期',
        field: 'create_date',
        align: 'center',
        sortable: true,
      },
      {
        name: 'create_user',
        label: '创建人',
        field: 'create_user',
        align: 'center',
        sortable: true,
      },
      {
        name: 'update_date',
        label: '更新日期',
        field: 'update_date',
        align: 'center',
        sortable: true,
      },
      {
        name: 'update_user',
        label: '更新人',
        field: 'update_user',
        align: 'center',
        sortable: true,
      },
    ] as QTableProps['columns'],
    rows: [] as any,
  },
  link: {
    pagination: {
      page: 1,
      rowsNumber: 0,
      rowsPerPage: 7,
      sortBy: null,
      descending: true,
    } as any,
    filter: '',
    columns: [
      {
        name: 'id',
        label: 'ID',
        field: 'id',
        align: 'center',
      },
      {
        name: 'title',
        label: '标题',
        field: 'title',
        align: 'center',
        sortable: true,
      },
      {
        name: 'link',
        label: '链接',
        field: 'link',
        align: 'center',
        sortable: true,
      },
      {
        name: 'create_date',
        label: '创建日期',
        field: 'create_date',
        align: 'center',
        sortable: true,
      },
      {
        name: 'create_user',
        label: '创建人',
        field: 'create_user',
        align: 'center',
        sortable: true,
      },
      {
        name: 'update_date',
        label: '更新日期',
        field: 'update_date',
        align: 'center',
        sortable: true,
      },
      {
        name: 'update_user',
        label: '更新人',
        field: 'update_user',
        align: 'center',
        sortable: true,
      },
    ] as QTableProps['columns'],
    rows: [] as any,
  },
  slide: {
    pagination: {
      page: 1,
      rowsNumber: 0,
      rowsPerPage: 7,
      sortBy: null,
      descending: true,
    } as any,
    filter: '',
    columns: [
      {
        name: 'id',
        label: 'ID',
        field: 'id',
        align: 'center',
      },
      {
        name: 'title',
        label: '标题',
        field: 'title',
        align: 'center',
        sortable: true,
      },
      {
        name: 'desc',
        label: '简介',
        field: 'desc',
        align: 'center',
        sortable: true,
      },
      {
        name: 'link',
        label: '链接',
        field: 'link',
        align: 'center',
        sortable: true,
      },
      {
        name: 'img',
        label: '图像',
        field: 'img',
        align: 'center',
        sortable: true,
      },
      {
        name: 'create_date',
        label: '创建日期',
        field: 'create_date',
        align: 'center',
        sortable: true,
      },
      {
        name: 'create_user',
        label: '创建人',
        field: 'create_user',
        align: 'center',
        sortable: true,
      },
      {
        name: 'update_date',
        label: '更新日期',
        field: 'update_date',
        align: 'center',
        sortable: true,
      },
      {
        name: 'update_user',
        label: '更新人',
        field: 'update_user',
        align: 'center',
        sortable: true,
      },
    ] as QTableProps['columns'],
    rows: [] as any,
  },
  user: {
    pagination: {
      page: 1,
      rowsNumber: 0,
      rowsPerPage: 7,
      sortBy: null,
      descending: true,
    } as any,
    filter: '',
    columns: [
      {
        name: 'userNo',
        label: '编号',
        field: 'userNo',
        align: 'center',
        sortable: true,
      },
      {
        name: 'userId',
        label: 'ID',
        field: 'userId',
        align: 'center',
        sortable: true,
      },
      {
        name: 'userNickname',
        label: '家族名',
        field: 'userNickname',
        align: 'center',
        sortable: true,
      },
      {
        name: 'lastLoginTime',
        label: '最近上线时间',
        field: 'lastLoginTime',
        align: 'center',
        sortable: true,
      },
      {
        name: 'lastLogoutTime',
        label: '最近下线时间',
        field: 'lastLogoutTime',
        align: 'center',
        sortable: true,
      },
      {
        name: 'totalPlayTime',
        label: '游玩时间',
        field: 'totalPlayTime',
        align: 'center',
        sortable: true,
      },
      {
        name: 'membershipType',
        label: '会员类型',
        field: 'membershipType',
        align: 'center',
        sortable: true,
      },
      {
        name: 'pcroom',
        label: '是否网吧加成',
        field: 'pcroom',
        align: 'center',
        sortable: true,
      },
      {
        name: 'valid',
        label: '是否启用',
        field: 'valid',
        align: 'center',
        sortable: true,
      },
      {
        name: 'characters',
        label: '游戏角色',
        field: 'characters',
        align: 'center',
        sortable: true,
      },
    ] as QTableProps['columns'],
    rows: [] as any,
  },
});

const dialog = ref({
  news: {
    show: false,
    id: '',
    title: '',
    author: '',
    date: '',
    content: '',
  },
  slide: {
    show: false,
    id: '',
    title: '',
    desc: '',
    link: '',
    img: '',
  },
  link: {
    show: false,
    id: '',
    title: '',
    link: '',
  },
});

const onTableNewsRequest = (props: any) => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  let { page, rowsPerPage, sortBy, descending, rowsNumber } = props.pagination;

  table.value.news.rows = [];

  useFetch()
    .post(
      store.backend + '/api/news/info',
      {
        curPage: (page - 1) * rowsPerPage,
        maxPage: rowsPerPage === 0 ? rowsNumber : rowsPerPage,
        sortBy: sortBy === null ? 'create_date' : sortBy,
        descending: descending,
      },
      $q.cookies.get('canplay_token')
    )
    .then((resp) => {
      if (resp.data.status != 0) {
        for (let i = 0; i < resp.data.msg.length; ++i) {
          if (resp.data.msg[i].id === '') break;

          table.value.news.rows.push({
            id: resp.data.msg[i].id,
            title: resp.data.msg[i].title,
            date: resp.data.msg[i].date.replace(' +0800', ''),
            author: resp.data.msg[i].author,
            content: Base64.decode(resp.data.msg[i].content),
            create_date: resp.data.msg[i].create_date.replace(' +0800', ''),
            create_user: resp.data.msg[i].create_user,
            create_id: resp.data.msg[i].create_id,
            update_date: resp.data.msg[i].update_date.replace(' +0800', ''),
            update_user: resp.data.msg[i].update_user,
            update_id: resp.data.msg[i].update_id,
            delete: resp.data.msg[i].delete,
          });
        }
      } else {
        $q.notify('网络错误，请稍后重试');
      }

      table.value.news.pagination.page = page;
      table.value.news.pagination.rowsPerPage = rowsPerPage;
      table.value.news.pagination.sortBy = sortBy;
      table.value.news.pagination.descending = descending;

      $q.loading.hide();
      clearTimeout(time);
    })
    .catch(() => {
      $q.notify('网络错误，请稍后重试');
      $q.loading.hide();
      clearTimeout(time);
    });
};

const onTableNewsQuery = () => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  useFetch()
    .get(store.backend + '/api/news/count', $q.cookies.get('canplay_token'))
    .then((resp) => {
      table.value.news.pagination.rowsNumber = parseInt(resp.data.msg);
      onTableNewsRequest({ pagination: table.value.news.pagination });

      $q.loading.hide();
      clearTimeout(time);
    })
    .catch(() => {
      $q.notify('网络错误，请稍后重试');
      $q.loading.hide();
      clearTimeout(time);
    });
};

const onTableNewsNew = () => {
  dialog.value.news.show = true;
};

const onTableLinkRequest = (props: any) => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  let { page, rowsPerPage, sortBy, descending, rowsNumber } = props.pagination;

  table.value.link.rows = [];

  useFetch()
    .post(
      store.backend + '/api/link/info',
      {
        curPage: (page - 1) * rowsPerPage,
        maxPage: rowsPerPage === 0 ? rowsNumber : rowsPerPage,
        sortBy: sortBy === null ? 'create_date' : sortBy,
        descending: descending,
      },
      $q.cookies.get('canplay_token')
    )
    .then((resp) => {
      if (resp.data.status != 0) {
        for (let i = 0; i < resp.data.msg.length; ++i) {
          if (resp.data.msg[i].id === '') break;

          table.value.link.rows.push({
            id: resp.data.msg[i].id,
            title: resp.data.msg[i].title,
            link: resp.data.msg[i].link,
            create_date: resp.data.msg[i].create_date.replace(' +0800', ''),
            create_user: resp.data.msg[i].create_user,
            create_id: resp.data.msg[i].create_id,
            update_date: resp.data.msg[i].update_date.replace(' +0800', ''),
            update_user: resp.data.msg[i].update_user,
            update_id: resp.data.msg[i].update_id,
          });
        }
      } else {
        $q.notify('网络错误，请稍后重试');
      }

      table.value.link.pagination.page = page;
      table.value.link.pagination.rowsPerPage = rowsPerPage;
      table.value.link.pagination.sortBy = sortBy;
      table.value.link.pagination.descending = descending;

      $q.loading.hide();
      clearTimeout(time);
    })
    .catch(() => {
      $q.notify('网络错误，请稍后重试');
      $q.loading.hide();
      clearTimeout(time);
    });
};

const onTableLinkQuery = () => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  useFetch()
    .get(store.backend + '/api/link/count', $q.cookies.get('canplay_token'))
    .then((resp) => {
      table.value.link.pagination.rowsNumber = parseInt(resp.data.msg);
      onTableLinkRequest({ pagination: table.value.link.pagination });

      $q.loading.hide();
      clearTimeout(time);
    })
    .catch(() => {
      $q.notify('网络错误，请稍后重试');
      $q.loading.hide();
      clearTimeout(time);
    });
};

const onTableSlideRequest = (props: any) => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  let { page, rowsPerPage, sortBy, descending, rowsNumber } = props.pagination;

  table.value.slide.rows = [];

  useFetch()
    .post(
      store.backend + '/api/slide/info',
      {
        curPage: (page - 1) * rowsPerPage,
        maxPage: rowsPerPage === 0 ? rowsNumber : rowsPerPage,
        sortBy: sortBy === null ? 'create_date' : sortBy,
        descending: descending,
      },
      $q.cookies.get('canplay_token')
    )
    .then((resp) => {
      if (resp.data.status != 0) {
        for (let i = 0; i < resp.data.msg.length; ++i) {
          if (resp.data.msg[i].id === '') break;

          table.value.slide.rows.push({
            id: resp.data.msg[i].id,
            title: resp.data.msg[i].title,
            desc: resp.data.msg[i].desc,
            img: resp.data.msg[i].img,
            link: resp.data.msg[i].link,
            create_date: resp.data.msg[i].create_date.replace(' +0800', ''),
            create_user: resp.data.msg[i].create_user,
            create_id: resp.data.msg[i].create_id,
            update_date: resp.data.msg[i].update_date.replace(' +0800', ''),
            update_user: resp.data.msg[i].update_user,
            update_id: resp.data.msg[i].update_id,
          });
        }
      } else {
        $q.notify('网络错误，请稍后重试');
      }

      table.value.slide.pagination.page = page;
      table.value.slide.pagination.rowsPerPage = rowsPerPage;
      table.value.slide.pagination.sortBy = sortBy;
      table.value.slide.pagination.descending = descending;

      $q.loading.hide();
      clearTimeout(time);
    })
    .catch(() => {
      $q.notify('网络错误，请稍后重试');
      $q.loading.hide();
      clearTimeout(time);
    });
};

const onTableSlideQuery = () => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  useFetch()
    .get(store.backend + '/api/slide/count', $q.cookies.get('canplay_token'))
    .then((resp) => {
      table.value.slide.pagination.rowsNumber = parseInt(resp.data.msg);
      onTableSlideRequest({ pagination: table.value.slide.pagination });

      $q.loading.hide();
      clearTimeout(time);
    })
    .catch(() => {
      $q.notify('网络错误，请稍后重试');
      $q.loading.hide();
      clearTimeout(time);
    });
};

const onTableUserRequest = (props: any) => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  let { page, rowsPerPage, sortBy, descending, rowsNumber } = props.pagination;

  table.value.user.rows = [];

  useFetch()
    .post(
      store.backend + '/api/user/info',
      {
        curPage: (page - 1) * rowsPerPage,
        maxPage: rowsPerPage === 0 ? rowsNumber : rowsPerPage,
        sortBy: sortBy === null ? '_userNo' : sortBy,
        descending: descending,
      },
      $q.cookies.get('canplay_token')
    )
    .then((resp) => {
      if (resp.data.status != 0) {
        for (let i = 0; i < resp.data.msg.length; ++i) {
          if (resp.data.msg[i].id === '') break;

          table.value.user.rows.push({
            registerDate: resp.data.msg[i].registerDate,
            valid: resp.data.msg[i].valid,
            userNo: resp.data.msg[i].userNo,
            userId: resp.data.msg[i].userId,
            userNickname: resp.data.msg[i].userNickname,
            lastLoginTime: resp.data.msg[i].lastLoginTime.replace(' +0800', ''),
            lastLogoutTime: resp.data.msg[i].lastLogoutTime.replace(
              ' +0800',
              ''
            ),
            totalPlayTime: resp.data.msg[i].totalPlayTime,
            membershipType: resp.data.msg[i].membershipType,
            pcroom: resp.data.msg[i].pcroom,
            characters: resp.data.msg[i].characters,
          });
        }
      } else {
        $q.notify('网络错误，请稍后重试');
      }

      table.value.user.pagination.page = page;
      table.value.user.pagination.rowsPerPage = rowsPerPage;
      table.value.user.pagination.sortBy = sortBy;
      table.value.user.pagination.descending = descending;

      $q.loading.hide();
      clearTimeout(time);
    })
    .catch(() => {
      $q.notify('网络错误，请稍后重试');
      $q.loading.hide();
      clearTimeout(time);
    });
};

const onTableUserQuery = () => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  useFetch()
    .get(store.backend + '/api/user/count', $q.cookies.get('canplay_token'))
    .then((resp) => {
      table.value.user.pagination.rowsNumber = parseInt(resp.data.msg);
      onTableUserRequest({ pagination: table.value.user.pagination });

      $q.loading.hide();
      clearTimeout(time);
    })
    .catch(() => {
      $q.notify('网络错误，请稍后重试');
      $q.loading.hide();
      clearTimeout(time);
    });
};

const onView = (val1: string, val2: any) => {
  switch (val1) {
    case 'news':
      dialog.value.news.show = true;
      dialog.value.news.id = val2.id;
      dialog.value.news.title = val2.title;
      dialog.value.news.date = val2.date;
      dialog.value.news.author = val2.author;
      dialog.value.news.content = val2.content;
      break;
    case 'slide':
      dialog.value.slide.show = true;
      dialog.value.slide.id = val2.id;
      dialog.value.slide.title = val2.title;
      dialog.value.slide.desc = val2.desc;
      dialog.value.slide.link = val2.link;
      dialog.value.slide.img = val2.img;
      break;
    case 'link':
      dialog.value.link.show = true;
      dialog.value.link.id = val2.id;
      dialog.value.link.title = val2.title;
      dialog.value.link.link = val2.link;
      break;
  }
};

const onDel = (val: string) => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  switch (val) {
    case 'news':
      dialog.value.news.show = false;

      useFetch()
        .post(
          store.backend + '/api/news/update',
          {
            id: dialog.value.news.id,
            title: dialog.value.news.title,
            date: dialog.value.news.date,
            author: dialog.value.news.author,
            content: Base64.encode(dialog.value.news.content),
            update_user: store.user.familyname,
            update_id: store.user.username,
            delete: 1,
          },
          $q.cookies.get('canplay_token')
        )
        .then((resp) => {
          if (resp.data.status === 1) {
            onTableNewsQuery();
          }

          $q.loading.hide();
          clearTimeout(time);
        })
        .catch(() => {
          $q.notify('网络错误，请稍后重试');
          $q.loading.hide();
          clearTimeout(time);
        });
      break;
  }
};

const onSave = (val: string) => {
  let time = setTimeout(() => {
    $q.loading.hide();
    clearTimeout(time);
  }, 120000);

  $q.loading.show();

  switch (val) {
    case 'news':
      dialog.value.news.show = false;

      if (dialog.value.news.id === '') {
        useFetch()
          .post(
            store.backend + '/api/news/add',
            {
              title: dialog.value.news.title,
              date: dialog.value.news.date,
              author: dialog.value.news.author,
              content: Base64.encode(dialog.value.news.content),
              create_user: store.user.familyname,
              create_id: store.user.username,
              update_user: store.user.familyname,
              update_id: store.user.username,
            },
            $q.cookies.get('canplay_token')
          )
          .then((resp) => {
            if (resp.data.status === 1) {
              onTableNewsQuery();
            }

            $q.loading.hide();
            clearTimeout(time);
          })
          .catch(() => {
            $q.notify('网络错误，请稍后重试');
            $q.loading.hide();
            clearTimeout(time);
          });
      } else {
        useFetch()
          .post(
            store.backend + '/api/news/update',
            {
              id: dialog.value.news.id,
              title: dialog.value.news.title,
              date: dialog.value.news.date,
              author: dialog.value.news.author,
              content: Base64.encode(dialog.value.news.content),
              update_user: store.user.familyname,
              update_id: store.user.username,
              delete: 0,
            },
            $q.cookies.get('canplay_token')
          )
          .then((resp) => {
            if (resp.data.status === 1) {
              onTableNewsQuery();
            }

            $q.loading.hide();
            clearTimeout(time);
          })
          .catch(() => {
            $q.notify('网络错误，请稍后重试');
            $q.loading.hide();
            clearTimeout(time);
          });
      }
      break;
    case 'slide':
      dialog.value.slide.show = false;

      useFetch()
        .post(
          store.backend + '/api/slide/update',
          {
            id: dialog.value.slide.id,
            title: dialog.value.slide.title,
            desc: dialog.value.slide.desc,
            link: dialog.value.slide.link,
            img: dialog.value.slide.img,
            update_user: store.user.familyname,
            update_id: store.user.username,
          },
          $q.cookies.get('canplay_token')
        )
        .then((resp) => {
          if (resp.data.status === 1) {
            onTableSlideQuery();
          }

          $q.loading.hide();
          clearTimeout(time);
        })
        .catch(() => {
          $q.notify('网络错误，请稍后重试');
          $q.loading.hide();
          clearTimeout(time);
        });
      break;
    case 'link':
      dialog.value.link.show = false;

      useFetch()
        .post(
          store.backend + '/api/link/update',
          {
            id: dialog.value.link.id,
            title: dialog.value.link.title,
            link: dialog.value.link.link,
            update_user: store.user.familyname,
            update_id: store.user.username,
          },
          $q.cookies.get('canplay_token')
        )
        .then((resp) => {
          if (resp.data.status === 1) {
            onTableLinkQuery();
          }

          $q.loading.hide();
          clearTimeout(time);
        })
        .catch(() => {
          $q.notify('网络错误，请稍后重试');
          $q.loading.hide();
          clearTimeout(time);
        });
      break;
  }
};
</script>
