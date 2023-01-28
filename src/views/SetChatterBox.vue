<template>
    <Window :title="'话痨模式' + room_id" icon="/vite.svg" hide-maximize hide-minimize>
        <div class="q-pa-xs" style="max-width: 300px">
            <div class="q-gutter-sm">
                <div class="row justify-around">
                <q-input v-model.number="whole_interval" prefix="整体间隔:" suffix="分钟" dense type="text" style="max-width: 121px"></q-input>
                <q-input v-model.number="item_interval" prefix="每条间隔:" suffix="秒" dense type="text" mask="###" no-error-icon :rules="[ val => val >= 1 || '小与1秒没什么意义']" style="max-width: 110px"></q-input>
            </div>
                <template v-for="(item, index) in chatterboxes" :key="index">
                    <q-input hide-bottom-space ref="inputsRef" v-model="item.msg" filled type="text" :rules="[ val => val.length <= 20 || '最多只能输入20个字符']">
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
import { appWindow } from "@tauri-apps/api/window";
const $q = useQuasar()
let room_id = ref();
let db;
const chatterboxes = ref([])
const inputsRef = ref([])
const whole_interval = ref(5)
const item_interval = ref(1)
onMounted(async () => {
    console.log(location.href);
    const route = useRoute()
    room_id.value = Number(route.params.room_id);
    db = await DataBase.init_db()
    DataBase.get_room_chatterbox(db, room_id.value).then(datas => {
        console.log(datas)
        if (datas.length!=0) {
            const chatters = JSON.parse(datas[0].chatterbox)
            whole_interval.value = chatters.whole_interval
            item_interval.value = chatters.item_interval
            chatterboxes.value = chatters.chatterboxes
        } else {
            chatterboxes.value = [{
                roomid: room_id.value,
                msg: '关注我不迷路',
                enable: 'true'
            }, {
                roomid: room_id.value,
                msg: '关注我福利不断~',
                enable: 'true'
            }, {
                roomid: room_id.value,
                msg: '上舰长,包您满意~',
                enable: 'true'
            }]
        }
    })
})
async function onAdd(){
    chatterboxes.value.push({roomid:room_id.value,msg:'进群,你懂得~',enable:'true'})
}
async function onSave() {
    let hasErr = false
    inputsRef.value.forEach(input=>{
        input.validate()
        hasErr = input.hasError 
    })
    if(hasErr){
        $q.notify({
            message: `请检查输入项是否合规`,
            type: 'negative',
            html: true,
            timeout: 800,
            progress: true
        })
        return;
    }
    chatterboxes.value = chatterboxes.value.filter(f => f.msg.trim() != '')
    DataBase.save_room_chatterbox(db, room_id.value, {whole_interval:whole_interval.value,item_interval:item_interval.value,'chatterboxes':chatterboxes.value}).then(count => {
        if (count > 0) {
            $q.notify({
                message: `保存成功`,
                type: 'positive',
                html: true,
                timeout: 300,
                progress: true,
                onDismiss: () => {
                    //appWindow.close()
                }
            })
        } else {
            if (chatterboxes.value.length != 0) {
                $q.notify({
                    message: `保存失败`,
                    type: 'negative',
                    html: true,
                    timeout: 800,
                    progress: true
                })
            }
        }
    })
}
const show_add =  computed(() => {
    return chatterboxes.value.length < 5
})
</script>