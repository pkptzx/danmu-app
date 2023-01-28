<template>
    <Window icon="/vite.svg" bg-color="bg-light-blue-1" :title="'设置回复 ' + roomid" no-space>
  
        <div class="q-gutter-y-md" >
            <q-card>
                <q-tabs v-model="tab" dense class="text-grey" active-color="primary" indicator-color="primary"
                    align="justify">
                    <q-tab name="event_reply" label="事件回复" />
                    <q-tab name="keyword_reply" label="关键字回复" />
                    <q-tab name="robot_reply" label="机器人回复" />
                </q-tabs>

                <q-separator />

                <q-tab-panels v-model="tab" animated>
                    <q-tab-panel name="event_reply">
                            <q-list bordered padding>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>进入直播间</q-item-label>
                                        <q-item-label caption>暂时包括所有人</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.JOIN_ROOM.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.JOIN_ROOM.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.JOIN_ROOM.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.JOIN_ROOM.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>

                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>关注直播间</q-item-label>
                                        <q-item-label caption>暂时包括所有人</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.FOLLOW_ROOM.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.FOLLOW_ROOM.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.FOLLOW_ROOM.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.FOLLOW_ROOM.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>收到礼物</q-item-label>
                                        <q-item-label caption>暂时包括所有礼物</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.GIFT.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.GIFT.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.GIFT.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.GIFT.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>点赞直播间</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.LIKE_ROOM.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.LIKE_ROOM.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.LIKE_ROOM.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.LIKE_ROOM.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>发送红包</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.RED_POCKET.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.RED_POCKET.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.RED_POCKET.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.RED_POCKET.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>上舰</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.GUARD_BUY.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.GUARD_BUY.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.GUARD_BUY.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.GUARD_BUY.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>设立房管</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.ROOM_ADMIN_ENTRANCE.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.ROOM_ADMIN_ENTRANCE.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.ROOM_ADMIN_ENTRANCE.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.ROOM_ADMIN_ENTRANCE.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>撤销房管</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.ROOM_ADMIN_REVOKE.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.ROOM_ADMIN_REVOKE.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.ROOM_ADMIN_REVOKE.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.ROOM_ADMIN_REVOKE.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>开播</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.LIVE_OPEN.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.LIVE_OPEN.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.LIVE_OPEN.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.LIVE_OPEN.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>下播</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.LIVE_CLOSE.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.LIVE_CLOSE.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.LIVE_CLOSE.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.LIVE_CLOSE.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                                <q-item tag="div">
                                    <q-item-section></q-item-section>
                                    <q-item-section side>
                                        <q-btn color="secondary" icon="close" label="关闭" @click="onClose" />
                                    </q-item-section>
                                    <q-item-section side>                                    
                                    <q-btn color="primary" icon="save" label="保存" @click="on_save_event_settings" />
                                </q-item-section>
                                </q-item>
                            </q-list>
                    </q-tab-panel>
                    <q-tab-panel name="keyword_reply">
                            <q-list bordered padding>
                                <q-item tag="label" v-ripple>
                                    <q-item-section>
                                        <q-item-label>进入直播间</q-item-label>
                                        <q-item-label caption>暂时包括所有人</q-item-label>
                                    </q-item-section>
                                    <q-item-section>
                                        <input type="text" v-model="event_settings.JOIN_ROOM.replyText" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.JOIN_ROOM.tip" label="提示" />
                                    </q-item-section>
                                    <q-item-section side style="">
                                        <q-toggle color="blue" dense v-model="event_settings.JOIN_ROOM.reply" label="回复" />
                                    </q-item-section>
                                    <q-item-section side>
                                        <q-toggle color="blue" dense v-model="event_settings.JOIN_ROOM.tts" label="语音播报" />
                                    </q-item-section>
                                </q-item>
                            </q-list>
                    </q-tab-panel>
                    <q-tab-panel name="robot_reply">
                            暂时懒得实现...<br/>
                            对接机器人接口<br/>
                            用途: 比如用户发送 @xx 时间,@xx 什么是std算法等....
                    </q-tab-panel>
                </q-tab-panels>
            </q-card>
        </div>
   
</Window>
</template>

<script setup lang="ts">
//@ts-nocheck
import { computed,onBeforeMount, onMounted, onUnmounted, ref, reactive, watch } from "vue";
import { useRoute } from "vue-router";
import Window from '../components/Window.vue';
import * as DataBase from '../assets/js/db.js';
const event_settings = ref(DataBase.default_event_settings)
import { appWindow } from "@tauri-apps/api/window";
import { emit,listen } from '@tauri-apps/api/event';
import { useQuasar } from 'quasar'
const $q = useQuasar()
const tab = ref('event_reply')
let db;
const roomid = ref()
onMounted(async () => {
    const route = useRoute()
    roomid.value = Number(route.params.room_id);
    db = await DataBase.init_db()
    event_settings.value = await DataBase.get_event_settings(db, roomid.value)
})

async function on_save_event_settings(){
    DataBase.save_event_settings(db, roomid.value ,event_settings.value).then(count => {
        if (count > 0) {
            emit('EVENTMSG_reload_room_settings',{type: 'event_settings'})
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
                    message: `保存失败前检查`,
                    type: 'negative',
                    html: true,
                    timeout: 800,
                    progress: true
                })
            }
        }
    });
}
async function onClose() {
    appWindow.close()
}
</script>

<style scoped>

</style>