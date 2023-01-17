<template>
    <Window :title="'话痨模式' + room_id" icon="/vite.svg">
        <div class="q-pa-md" style="max-width: 300px">
            <div class="q-gutter-md">
                <template v-for="(item, index) in chatterboxs" :key="index">
                    <q-input v-model="item.msg" filled type="text" :rules="[ val => val.length <= 20 || '最多只能输入20个字符']">
                        <template v-slot:append>
                            <q-toggle v-model="item.enable" true-value="true" false-value="false" color="green" />
                        </template>
                    </q-input>
                </template>
            </div>
            <q-btn v-if="show_add" color="secondary" icon="add" label="新增" @click="onAdd" style="float:left;margin: 12px 0;"/>
            <q-btn color="primary" icon="save" label="保存" @click="onSave" style="float:right;margin: 12px 0;"/>
        </div>
    </Window>
</template>
 
<script setup lang="ts">
//@ts-nocheck
import { reactive, onMounted, ref, computed } from "vue";
import Window from '../components/Window.vue';
import { useRoute } from "vue-router";
import * as DataBase from '../assets/js/db.js';
import { useQuasar } from 'quasar'
const $q = useQuasar()
let room_id = ref();
let db;
const chatterboxs = ref([])
onMounted(async () => {
    console.log(location.href);
    const route = useRoute()
    room_id.value = Number(route.params.room_id);
    db = await DataBase.init_db()
    DataBase.get_chatterbox(db, room_id.value).then(datas => {
        if(datas.length > 0){
        chatterboxs.value = datas
    }else{
        chatterboxs.value = [{
            roomid:room_id.value,
            msg: '关注我不迷路',
            enable: 'true'
        },{
            roomid:room_id.value,
            msg: '关注我福利不断~',
            enable: 'true'
        },{
            roomid:room_id.value,
            msg: '上舰长,包您满意~',
            enable: 'true'
        }]
    }
    })
})
async function onAdd(){
    chatterboxs.value.push({roomid:room_id.value,msg:'',enable:'true'})
}
async function onSave() {
    chatterboxs.value = chatterboxs.value.filter(f => f.msg.trim() != '')
    DataBase.save_chatterbox(db, room_id.value, chatterboxs.value).then(count => {
        if (count > 0) {
            $q.notify({
                message: `保存成功: ${count} 条`,
                color: 'purple',
                html: true,
                timeout: 800,
                progress: true
            })
        } else {
            if (chatterboxs.value.length != 0) {
                $q.notify({
                    message: `保存失败: 成功 ${count} 条`,
                    color: 'red',
                    html: true,
                    timeout: 800,
                    progress: true
                })
            }
        }
    })
}
const show_add =  computed(() => {
    return chatterboxs.value.length < 5
})
</script>