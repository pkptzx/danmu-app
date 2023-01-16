<template>
  <Window title="弹幕查询" :space="false" icon="/vite.svg">
    <div class="q-pa-none">
    <q-table
    class="my-sticky-dynamic selectable"
    dense
    ref="tableRef"
      title="弹幕查询"
      :rows="rows"
      :columns="columns"
      row-key="id"
      v-model:pagination="pagination"
      :loading="loading"
      :filter="filter"
      binary-state-sort
      @request="onRequest"
    >
    <!-- @virtual-scroll="onScroll" -->
    <!-- :dense="$q.screen.lt.md"
      virtual-scroll
      :virtual-scroll-item-size="48"
      :virtual-scroll-sticky-size-start="48" -->
    <template v-slot:top-right>
        <q-input borderless dense debounce="300" v-model="filter" placeholder="Search">
          <template v-slot:append>
            <q-icon name="search" />
          </template>
        </q-input>
      </template>
      </q-table>
  </div>

  </Window>
</template>
 
<script setup lang="ts"> 
//@ts-nocheck
import { reactive, onMounted,onUnmounted, ref,computed, nextTick } from "vue";
import Window from '../components/Window.vue';
import { fetch,Body } from '@tauri-apps/api/http';
import * as DataBase from '../assets/js/db.js';
import { date } from 'quasar'

let db
const columns = [
  {
    name: 'id',
    label: 'id',
    field: 'id'
  },
  {
    name: 'uid',
    required: true,
    label: '用户ID',
    align: 'left',
    field: row => row.uid,
    format: val => val,
    sortable: true
  },
  { name: 'uname', label: '用户昵称', field: 'uname', align: 'left',sortable: true},
  { name: 'content', label: '弹幕', field: 'content', align: 'left',sortable: true},
  {
    name: 'ts',
    required: true,
    label: '时间',
    align: 'left',
    field: row => row.ts,
    format: val => date.formatDate(new Date(val), 'YYYY-MM-DD HH:mm:ss.SSS'),
    sortable: true
  },
  {
    name: 'upname',
    field: 'upname',
    label: '主播昵称',
    align: 'left',
    sortable: true
  },
  {
    name: 'roomid',
    field: 'roomid',
    label: '直播间ID',
    align: 'left',
    sortable: true
  },
]
const tableRef = ref()
const nextPage = ref(2)
const loading = ref(false)
const filter = ref('')
const pagination = ref({
      sortBy: 'desc',
      descending: false,
      page: 1,
      rowsPerPage: 50,
      rowsNumber: 10
    })
const lastPage = 9

const rows = ref([])

// function onScroll({ to, ref }) {
//   const lastIndex = rows.value.length - 1
//   console.log('onScroll', to, lastIndex, loading.value, nextPage.value, lastPage);
//   if (loading.value !== true && nextPage.value < lastPage && to === lastIndex) {
//     loading.value = true
//     const sql = `select * from danmu_msg limit ${(nextPage.value - 2) * pageSize},${pageSize}`;
//     console.log(sql);
//     console.log('db', db);
//     db.select(sql).then(rst => {
//       rows.value = rst;
//       nextPage.value++
//       nextTick(() => {
//         ref.refresh()
//         loading.value = false
//       })
//     });
//   }
// }


onMounted(async () => {
  db = await DataBase.init_db()
  tableRef.value.requestServerInteraction()
})
onUnmounted(async () => {
  await db.close();
});

function onRequest(props) {
  console.log(props)
  const { page, rowsPerPage, sortBy, descending } = props.pagination
      const filter = props.filter
  loading.value = true

  DataBase.get_danmu_msg(db, filter, page, rowsPerPage).then(rst => {
    console.log('弹幕查询结果:', rst);
    rows.value = rst.rows;
    nextPage.value++
    pagination.value.rowsNumber = rst.count;
    // don't forget to update local pagination object
    pagination.value.page = page
    pagination.value.rowsPerPage = rowsPerPage
    pagination.value.sortBy = sortBy
    pagination.value.descending = descending

    // ...and turn of loading indicator
    loading.value = false
  });
}



</script>

<style scoped>
:deep(.my-sticky-dynamic) {
  /* height or max-height is important */
  height: calc(100vh - 42px);
}

:deep(.my-sticky-dynamic thead tr:first-child th) {
  background-color: #fff ;
}

:deep(.my-sticky-dynamic thead tr th) {
  position: sticky;
  z-index: 1;
}

/* this will be the loading indicator */
:deep(.my-sticky-dynamic thead tr:last-child th) {
  /* height of all previous header rows */
  top: 48px;
}

:deep(.my-sticky-dynamic thead tr:first-child th) {
  top: 0;
}
</style>
