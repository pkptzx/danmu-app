<template>
    <Window icon="/vite.svg" bg-color="bg-light-blue-1" :title="'设置回复'" no-space>
  
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
                                        <q-item-label caption>暂时包括所有人</q-item-label>
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
                                        <q-item-label caption>暂时包括所有人</q-item-label>
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
                                        <q-item-label caption>暂时包括所有人</q-item-label>
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
                                        <q-item-label caption>暂时包括所有人</q-item-label>
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
                                        <q-item-label caption>暂时包括所有人</q-item-label>
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
                                        <q-item-label caption>暂时包括所有人</q-item-label>
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
import Window from '../components/Window.vue';
import * as DataBase from '../assets/js/db.js';
const event_settings = ref(DataBase.default_event_settings)
const tab = ref('event_reply')
let db;
let roomid = ref(3796382)
onBeforeMount(async () => {
    db = await DataBase.init_db()
    event_settings.value = await DataBase.get_event_settings(db, roomid.value)
})
onMounted(async () => {
    // db = await DataBase.init_db()
    // event_settings.value = await DataBase.get_event_settings(db, roomid.value)
})
</script>

<style scoped>

</style>