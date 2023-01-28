<template>
    <div class="q-pa-xs row justify-center">
        <q-layout view="lHh Lpr lFf" container style="height: 100vh;" class="shadow-2 rounded-borders">
            <q-header elevated>
                <q-toolbar data-tauri-drag-region>
                    <!-- <q-icon data-tauri-drag-region name="question_answer" color="green" size="32px" /> -->
                    <q-avatar size="32px">
                        <img data-tauri-drag-region :src="up_face">
                    </q-avatar>
                    <q-toolbar-title data-tauri-drag-region>
                        {{ up_name }}
                        <q-tooltip class="bg-indigo" anchor="bottom middle" self="top middle">
                           直播间房号: {{ room_id }}
                        </q-tooltip>
                    </q-toolbar-title>
                    <q-btn-dropdown flat round dense icon="menu" class="q-mr-sm">
                        <div class="row no-wrap q-pa-md">
                            <div class="column items-center">
                                <a :href="'https://live.bilibili.com/'+room_id" target="_blank">
                                    <q-avatar size="72px">
                                        <img :src="up_face" />
                                    </q-avatar>
                                </a>
                                <div class="text-subtitle1 q-mt-md q-mb-xs">{{ up_name }}</div>
                    
                                <q-btn color="primary" label="设置话痨" push size="sm" v-close-popup @click="open_chatterbox" />
                                <q-toggle v-model="chatterbox" checked-icon="check" unchecked-icon="clear" color="green" label="话痨模式" />
                            </div>
                            <q-separator vertical inset class="q-mx-sm" />
                            <div class="column">
                                <q-toggle v-model="show_join" checked-icon="check" unchecked-icon="clear" color="green" label="显示进入" />
                                <q-toggle v-model="show_face" checked-icon="check" unchecked-icon="clear" color="green" label="显示头像">
                                    <q-tooltip class="bg-indigo" anchor="top middle" self="center middle">
                                        如果瞬间有巨量弹幕显示头像会导致窗体卡顿<br />因为需要发起获取头像的请求
                                    </q-tooltip>
                                </q-toggle>
                                <q-toggle v-model="show_follow" checked-icon="check" unchecked-icon="clear" color="green" label="显示关注" />
                                <div class="q-pa-none q-gutter-xs">
                                    <q-toggle v-model="is_autoreply" checked-icon="check" unchecked-icon="clear" color="green"
                                        label="自动回复" />
                                    <q-btn round color="green" size="xs" icon="settings" @click="show_reply_setting"/>
                                </div>
                                <q-toggle v-model="wintop" checked-icon="check" unchecked-icon="clear" color="green" label="置顶" />
                                <q-separator spaced />
                                <q-btn color="green" icon="exit_to_app" label="关闭窗口" @click="onClose" />
                            </div>
                        </div>
                    </q-btn-dropdown>
                </q-toolbar>
            </q-header>
            <q-page-container>
                <q-page class="msger-chat">
                    <q-scroll-area style="width:100%; height: calc( 100vh - 94px);" ref="msger_chat" @scroll="onScroll">
                        <div class="q-pa-md row justify-center">
                            <div style="width: 100%;">
                                <template v-for="item in items" :key="item.id">
                                    <q-chat-message v-if="item.type == 'OPEN'" :label="item.msg" label-html @contextmenu.stop.prevent />
                                    <q-chat-message v-if="item.type == 'STARTLISTEN'" :label="item.msg + ' - ' +up_name" label-html @contextmenu.stop.prevent />
                                    <q-chat-message v-if="item.type == 'ERROR'" :label="item.msg" label-html @contextmenu.stop.prevent />
                                    <q-chat-message v-if="item.type == 'CLOSE'" :label="item.msg" label-html @contextmenu.stop.prevent />
                                    <q-chat-message v-if="event_settings.JOIN_ROOM.tip && item.type == 'INTERACT_WORD' && item.body.action == 'enter'"
                                        :label="'<span style=\'color:#8cd9ff;\'>' + item.body.user.uname + '</span> 进入直播间'" label-html @contextmenu.stop.prevent />
                                    <q-chat-message v-if="event_settings.FOLLOW_ROOM.tip && item.type == 'INTERACT_WORD' && item.body.action == 'follow'"
                                        :label="'<span style=\'color:#8cd9ff;\'>' + item.body.user.uname + '</span> <span style=\'color:red;\'>关注直播间</span>'"
                                        label-html @contextmenu.stop.prevent />
                                    <q-chat-message v-if="event_settings.LIKE_ROOM.tip && item.cmd == 'LIKE_INFO_V3_CLICK'" @contextmenu.stop.prevent
                                        :label="'<span style=\'color:#8cd9ff;\'>' + item.data.uname + '</span> <span style=\'color:green;\'>点赞了直播间</span>'"
                                        label-html />
                                    <q-chat-message v-if="event_settings.ROOM_ADMIN_ENTRANCE.tip && item.cmd == 'room_admin_entrance'" @contextmenu.stop.prevent
                                        :label="'恭喜<span style=\'color:#8cd9ff;\'>' + item.uname + '</span> <span style=\'color:green;\'>成为房管</span>'"
                                        label-html />
                                    <q-chat-message v-if="event_settings.ROOM_ADMIN_REVOKE.tip && item.cmd == 'ROOM_ADMIN_REVOKE'" @contextmenu.stop.prevent
                                        :label="'<span style=\'color:#8cd9ff;\'>' + item.uname + '</span> <span style=\'color:green;\'>被撤销房管</span>'"
                                        label-html />
                                    <q-chat-message v-if="item.type == 'DANMU_MSG'" @contextmenu.prevent="set_context_param(item,$event)"
                                        :name="(item.body.user.uid == up_uid ? ('<span style=\'color:red;\'>[主播]</span>'): '')+(item.body.user.identity.room_admin ? ('<span style=\'color:red;\'>[房]</span>'): '') + item.body.user.uname"
                                        name-html
                                        :avatar="show_face ? (item.body.user.face ? item.body.user.face : 'https://i0.hdslb.com/bfs/face/member/noface.jpg_48x48.jpg') : undefined"
                                        :stamp="new Date(item.timestamp).toLocaleTimeString().replace(/([\d]+:[\d]{2})(:[\d]{2})(.*)/, '$1$3')"
                                        :bg-color="item.body.user.uid == my_uid ? 'primary':'amber-7'"
                                        :text-color="item.body.user.uid == my_uid ? 'white':'black'" :sent="item.body.user.uid == my_uid">
                                        <img v-if="item.body.emoticon?.url != null"
                                            :style="'width: '+item.body.emoticon.width/3+'px; height: '+item.body.emoticon.height/3+'px;'"
                                            :src="item.body.emoticon.url" />
                                        <div class="selectable" v-else v-html="item.body.contentHtml"></div>
                                    </q-chat-message>
                                </template>
                                <q-menu touch-position auto-close context-menu>
                                    <q-list>
                                        <q-item v-close-popup clickable @click="clipboard_text(context_param.body.content)">
                                            <q-item-section>复制弹幕</q-item-section>
                                        </q-item>
                                        <q-item v-close-popup clickable @click="clipboard_text(context_param.body.user.uname)">
                                            <q-item-section>复制昵称</q-item-section>
                                        </q-item>
                                        <q-item v-close-popup clickable @click="open_borrower(context_param.body.user.uid)">
                                            <q-item-section>访问空间</q-item-section>
                                        </q-item>
                                        <q-item v-close-popup clickable @click="clipboard_text(context_param.body.user.uid)">
                                            <q-item-section>复制UID</q-item-section>
                                        </q-item>
                                        <q-item v-close-popup clickable @click="updateUserInfoByForce(context_param.body.user.uid)">
                                            <q-item-section>更新头像</q-item-section>
                                        </q-item>
                                    </q-list>
                                </q-menu>
                            </div>
                        </div>
                    </q-scroll-area>
                    <q-page-sticky :style="(scroll_sticky.show && scroll_sticky.unread!=0) ? '':'display: none;'" expand position="bottom" :offset="[0, 0]">
                        <q-btn size="xs" color="accent" icon="arrow_downward" :label="scroll_sticky.unread" @click="scrollBottom"/>
                    </q-page-sticky>
                </q-page>
            </q-page-container>
            <q-footer elevated>
                <q-input placeholder="请输入弹幕...回车也能直接发送" dense v-model="dmMsg" clearable :input-class="errtip ? 'errtip' : 'text-white'"
                    @animationend="errtip=false" @keyup.enter.exact="send_dm">
                    <template v-slot:after>
                        <q-btn round dense flat icon="send" color="positive" :loading="sending" @click="send_dm">
                            <template v-slot:loading>
                                <q-spinner-facebook />
                            </template>
                        </q-btn>
                    </template>
                </q-input>
            </q-footer>
        </q-layout>
    </div>
</template>

<script setup lang="ts">
//@ts-nocheck
import { library } from '@fortawesome/fontawesome-svg-core'
import { faHatWizard, faUserSecret, faCommentAlt } from '@fortawesome/free-solid-svg-icons'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { moveWindow, Position } from 'tauri-plugin-positioner-api'
import { emit,listen } from '@tauri-apps/api/event';
import { appWindow,WebviewWindow } from '@tauri-apps/api/window';
import { onMounted, onUnmounted, ref, nextTick, watch } from 'vue'
import { useRoute } from "vue-router";
import { startListen,UserActionMsg,User,DanmuMsg,GuardLevel } from 'blive-message-listener/browser'
import * as DataBase from '../assets/js/db.js';
import { useQuasar } from 'quasar'
import 'animate.css';
import * as bApi from '../assets/js/biliApi.js';
import * as rasterizeHTML from 'rasterizehtml';
import domtoimage from 'dom-to-image'
import { fetch, Body,ResponseType } from '@tauri-apps/api/http';
library.add(faHatWizard, faUserSecret, faCommentAlt)

moveWindow(Position.BottomRight)
// TODO: SC 礼物 展示
let db;
const $q = useQuasar()
const items = ref<Array<String>>([])
    const show_join = ref(true)
    const show_follow = ref(true)
    const wintop = ref(false)
    const chatterbox = ref(false)
    appWindow.setAlwaysOnTop(wintop.value);
    const show_face = ref(false)
    const my_uid = ref()
    const dmMsg = ref('')
    const up_name = ref('')
    const up_uid = ref()
    const up_face = ref('/noface.jpg')
    const is_autoreply = ref(false)
    const sending = ref(false)
    //滚动条
    const msger_chat = ref()
    const scroll_sticky = ref({show:false,unread:0})

const event_settings = ref(DataBase.default_event_settings)

let context_param,context_el;
let errtip = ref(false);
let unlisten: any;
let room_id ;
let danmuClient
onMounted(async () => {
    console.log(location.href);
    const route=useRoute()
    room_id = Number(route.params.room_id);
    db = await DataBase.init_db()
    bApi.getLiveRoomInfo(room_id).then(room =>{
        console.log("LiveRoomInfo",room);
        up_uid.value = room.data.data.uid
        bApi.getAccInfo(room.data.data.uid).then((info) => {
            console.log("AccInfo",info);
            up_name.value = info.data.data.name;
            up_face.value = info.data.data.face;
        });
    });


    bApi.getCookies().then((c)=> my_uid.value = c.DedeUserID)

    event_settings.value = await DataBase.get_event_settings(db, room_id)

    //接收全局消息
    unlisten = await listen<string>('EVENTMSG_reload_room_settings', async (event: any) => {
        console.log(`Got Msg:EVENTMSG_reload_room_settings ${JSON.stringify(event.payload)}`);
        const param = event.payload;
        switch (param.type) {
            case 'event_settings':
                event_settings.value = await DataBase.get_event_settings(db, room_id)
                break;
            case 'keyword_settings':
                // event_settings.value = await DataBase.get_event_settings(db, room_id)
                break;
            case 'robot_settings':
                // event_settings.value = await DataBase.get_event_settings(db, room_id)
                break;
            default:
                break;
        }
        
    });

    const handler: MsgHandler = {
        /** 连接成功 */
        onOpen: () => {
            addData({ type: 'OPEN', 'msg': '已连接直播弹幕服务器' })
        },
        /** 开始监听消息 */
        onStartListen: () => {
            addData({ type: 'STARTLISTEN', 'msg': `已连接直播间 ${room_id}` })
        },
        /** 连接关闭 */
        onClose: () => {
            addData({ type: 'CLOSE', 'msg': '连接已关闭' })
        },
        /** 连接错误 */
        onError: (e: Error) => {
            addData({ type: 'ERROR', 'msg': '连接失败,请检查网络或房间号.' })
        },
        onIncomeDanmu: (msg) => {
            console.log(msg.id, msg)
            msg.body.contentHtml = msg.body.content
            //解emoji表情的地址
            const extra = JSON.parse(msg.raw.info[0][15].extra)
            if(extra.emots){
                // 替换emoji表情
                console.log(extra.emots)
                for(let key in extra.emots){
                    let reg = new RegExp(key.replace('[','\\[').replace(']','\\]'),'gi')
                    msg.body.contentHtml = msg.body.contentHtml.replaceAll(reg, `<img style="width: 20px; height: 20px;" src="${extra.emots[key].url}" />`)
                }
            }
            addData(msg)
        },
        onIncomeSuperChat: (msg) => {
            console.log(msg.id, msg.body)
        },  
        onUserAction: (msg) => {
            const body: UserActionMsg = msg.body;
            if (body.action == 'enter') {
                if (!show_join.value) {
                    return;
                }
                addData(msg)
            } else if (body.action == 'follow') {
                if (!show_follow.value) {
                    return;
                }
                addData(msg)
            } else if (body.action == 'share') {
                addData(msg)
            }
            
        },
        onGift: (msg) => {
            console.log(msg.id, msg.body)
        },       
        onGuardBuy: (msg) => {
            //舰长上舰消息
            console.log('舰长上舰消息',msg);
        },
        raw: {
            // https://github.com/SocialSisterYi/bilibili-API-collect/issues/360
            // 点赞
            'LIKE_INFO_V3_CLICK': (msg) => {
                console.log(msg)
                addData(msg)
            },
            //设立房管 room_admin_entrance 
            'room_admin_entrance': (msg) => {
                // {
                //     "cmd": "room_admin_entrance",
                //     "dmscore": 45,
                //     "level": 1,
                //     "msg": "系统提示：你已被主播设为房管",
                //     "uid": 8212729
                // }
                console.log('设立房管',msg)
                addData(msg)
            },
            //撤销房管 ROOM_ADMIN_REVOKE 
            'ROOM_ADMIN_REVOKE': (msg) => {
                // {
                //     "cmd": "ROOM_ADMIN_REVOKE",
                //     "msg": "撤销房管",
                //     "uid": 8212729
                // }
                console.log('撤销房管',msg)
                addData(msg)
            },
            //发送红包 POPULARITY_RED_POCKET_NEW
            'POPULARITY_RED_POCKET_NEW': (msg) => {
                console.log('发送红包',msg)
                addData(msg)
            }
        }
    }

    danmuClient = startListen(room_id, handler)
    
})
let lastPosition = -1;
function onScroll ( { verticalPosition }) {
        if(lastPosition>verticalPosition){
            scroll_sticky.value.show = true;
        }
        // 10是误差,刚好滚到不容易~
        if(verticalPosition+10 >= (msger_chat.value?.getScroll().verticalSize-msger_chat.value.getScroll().verticalContainerSize)){
            scroll_sticky.value.show = false;
            scroll_sticky.value.unread = 0;
        }
        lastPosition = verticalPosition
}
function scrollBottom () {
    msger_chat.value?.setScrollPosition('vertical', msger_chat.value.getScroll().verticalSize, 300)
    scroll_sticky.value.show = false;
    scroll_sticky.value.unread = 0;
}
async function addData(data) {
    if (data.cmd == 'room_admin_entrance' || data.cmd == 'ROOM_ADMIN_REVOKE'){
        data.uname = '***'
        bApi.getAccInfo(data.uid).then((info) => {
            console.log("AccInfo",info);
            data.uname = info.data.data.name;
            autoreply(data)
        });
    }

    if(items.value?.length >= 100){
        items.value?.splice(0,1)
    }
    items.value?.push(data)
    if(scroll_sticky.value.show){
        scroll_sticky.value.unread++
    }else{
        scroll_sticky.value.unread = 0
        nextTick(() => {
            // msger_chat.value.scrollTo(0,msger_chat.value.scrollHeight);
            msger_chat.value.setScrollPosition('vertical', msger_chat.value.getScroll().verticalSize, 300)
        })
    }
    if (data.type == 'DANMU_MSG') {
        updateFace(data).then()
        data.body.upid = up_uid.value;
        data.body.upname = up_name.value;
        data.body.roomid = room_id;
        // emit('save_danmu_msg', data.body);
        DataBase.insert_danmu_msg(db, data.body);
    }
    if (data.cmd !== 'room_admin_entrance' && data.cmd !== 'ROOM_ADMIN_REVOKE'){
        autoreply(data)
    }
}
// 自动回复
async function autoreply(msg){
    if(!is_autoreply.value){
        return;
    }
    event_settings.value.JOIN_ROOM.replyText
    if (msg.type == 'INTERACT_WORD' && msg.body.action == 'follow'){
        send_danmu_with_notify(room_id,`[花]感谢${msg.body.user.uname.length>12 ? (msg.body.user.uname.substring(0,9) + '...') :msg.body.user.uname}的关注`)
    }else if (msg.cmd == 'LIKE_INFO_V3_CLICK'){
        // send_danmu_with_notify(room_id,`[哇]感谢${msg.data.uname.length>12 ? (msg.data.uname.substring(0,9) + '...') :msg.data.uname}的点赞`)
    }else if (msg.cmd == 'room_admin_entrance'){
        send_danmu_with_notify(room_id,`[爱]恭喜${msg.uname.length>11 ? (msg.uname.substring(0,8) + '...') :msg.uname}成为房管`)
    }else if (msg.cmd == 'ROOM_ADMIN_REVOKE'){
        send_danmu_with_notify(room_id,`[妙]逗比${msg.uname.length>10 ? (msg.uname.substring(0,7) + '...') :msg.uname}被撤销房管`)
    }
}
// 根据配置解析弹幕
function parseReplyDanmu(replyTemplate,uname,action){
    //返回['弹幕','提示(完整不截断)']
    const rst = []
    let reply = '';
    if(action){
        //感谢$$投喂的$$[哇]
        const tmp = replyTemplate.replaceAll(/(.*\$\$.+)(\$\$)(.*)/g,'$1'+action+'$3')
        reply = uname.length > 20-tmp.length-2 ? tmp.replace('$$', uname.slice(0,20-tmp.length-2-3)+'...') : tmp.replace('$$',uname)
    }else{
        reply = uname.length > 20-replyTemplate.length-2 ? replyTemplate.replace('$$', uname.slice(0,20-replyTemplate.length-2-3)+'...') : replyTemplate.replace('$$',uname)
    }
    rst.push(reply)
    //tip用于展示提示和tts语音播报
    let tip = replyTemplate.replace('$$',uname);
    if(action){
        tip = tip.replace('$$',action)
    }
    rst.push(tip)
    return rst;
}
function send_danmu_with_notify(room_id,msg){
    bApi.send_danmu(room_id, msg).then((resp: any) => {
        console.log('自动回复', resp);
        if (resp.data.message == '') {
        } else if (resp.data.message == 'f') {
            $q.notify({
                message: '您的弹幕被B站吞了<br/>请修改后重发',
                color: 'purple',
                html: true,
                timeout: 800,
                progress: true
            })
        } else if (resp.data.message == 'k') {
            $q.notify({
                message: '您的弹幕中有直播间违禁词<br/>这是由主播或房管设置的请修改后重发',
                color: 'purple',
                html: true,
                timeout: 800,
                progress: true
            })
        } else {
            $q.notify({
                message: resp.data.message,
                color: 'purple',
                html: true,
                timeout: 800,
                progress: true
            })
        }
    });
}
async function updateFace(data){
    // 即便禁用头像,也从尝试从缓存里获取头像
    let faces = localStorage.getItem('faces')
    if(faces == null){
        faces = "{}"
    }
    faces = JSON.parse(faces);
    if(faces[data.body.user.uid]){
        data.body.user.face = faces[data.body.user.uid]
    }else{
        if(show_face.value){
            bApi.getAccInfo(data.body.user.uid).then((info) => {
                console.log(info);
                faces[data.body.user.uid] = info.data.data.face;
                localStorage.setItem('faces',JSON.stringify(faces));
                data.body.user.face = info.data.data.face;
            });
        }
    }
}
//强制更新
async function updateUserInfoByForce(uid) {
    let faces = localStorage.getItem('faces')
    if (faces == null) {
        faces = "{}"
    }
    faces = JSON.parse(faces);
    bApi.getAccInfo(uid).then((info) => {
        console.log('info', info);
        faces[uid] = info.data.data.face;
        localStorage.setItem('faces', JSON.stringify(faces));
        items.value.forEach((data) => {
            if (data.body && data.body.user) {
                if (data.body.user.uid == uid) {
                    data.body.user.face = info.data.data.face;
                }
            }
        })
    });
}
async function send_dm(){
    if(dmMsg.value.trim().length == 0){
        return;
    }
    dmMsg.value = dmMsg.value.trim()
    sending.value=true
    bApi.send_danmu(room_id, `${dmMsg.value}`).then((resp: any) => {
        sending.value=false
        console.log(resp.data);
        if (resp.data.message == '') {
            dmMsg.value = ''
        } else if (resp.data.message == 'f') {
            errtip.value = true;
            $q.notify({
                message: '您的弹幕被B站吞了<br/>请修改后重发',
                color: 'purple',
                html: true,
                timeout: 800,
                progress: true
            })
        } else if (resp.data.message == 'k') {
            errtip.value = true;
            $q.notify({
                message: '您的弹幕中有直播间违禁词<br/>这是由主播或房管设置的请修改后重发',
                color: 'purple',
                html: true,
                timeout: 800,
                progress: true
            })
        } else {
            errtip.value = true;
            $q.notify({
                message: resp.data.message,
                color: 'purple',
                html: true,
                timeout: 800,
                progress: true
            })
        }
    });
}
const onClose = async (e: Event) => {
    appWindow.close();
}

function open_chatterbox() {
  let chatterboxWindow = WebviewWindow.getByLabel('chatterbox');
  if (chatterboxWindow) {
    chatterboxWindow.unminimize();
    chatterboxWindow.setFocus();
    return;
  }
  chatterboxWindow = new WebviewWindow('chatterbox', {
    url: `/danmu/chatterbox/${room_id}`,
    "fullscreen": false,
    "height": 600,
    "resizable": true,
    "title": "话痨模式",
    "width": 316,
    "visible": true,
    "skipTaskbar": false,
    "decorations": false,
    "transparent": true,
    "center":true
  });
  chatterboxWindow.once('tauri://created', function () {
    chatterboxWindow?.show();
  });
  chatterboxWindow.once('tauri://error', function (e) {
    console.error(e)
    chatterboxWindow.unminimize();
    chatterboxWindow.setFocus();
  });
}
function show_reply_setting(){
    let settingsReplyWindow = WebviewWindow.getByLabel('settingsReply');
  if (settingsReplyWindow) {
    settingsReplyWindow.unminimize();
    settingsReplyWindow.setFocus();
    return;
  }
  settingsReplyWindow = new WebviewWindow('settingsReply', {
    url: `/settings/reply/${room_id}`,
    "fullscreen": false,
    "height": 600,
    "resizable": true,
    "title": "设置回复",
    "width": 800,
    "visible": true,
    "skipTaskbar": false,
    "decorations": false,
    "transparent": true,
    "center":true
  });
  settingsReplyWindow.once('tauri://created', function () {
    settingsReplyWindow?.show();
  });
  settingsReplyWindow.once('tauri://error', function (e) {
    console.error(e)
    settingsReplyWindow.unminimize();
    settingsReplyWindow.setFocus();
  });
    
    // $q.dialog({
    //     title: '选择需要开启的自动回复',
    //     message: '请选择需要开启的自动回复.',
    //     options: {
    //         class:'text-center',
    //       type: 'toggle',
    //       model: [],
    //       isValid: model => model.includes('opt1') && model.includes('opt2'),
    //       inline: true,
    //     dense: true,
    //     color: 'green',
    //       items: [
    //         { label: '点赞回复', value: 'opt1' },
    //         { label: '关注回复', value: 'opt2' },
    //         { label: '礼物回复', value: 'opt3' },
    //         { label: '房管事件回复', value: 'opt3' },
    //         { label: '上舰回复', value: 'opt3' },
    //         { label: '红包事件回复', value: 'opt3' },
    //       ]
    //     },
    //     cancel: true,
    //     persistent: true
    //   }).onOk(data => {
    //     // console.log('>>>> OK, received', data)
    //   })
}

function set_context_param(item,evt){
    context_param = item
    context_el = evt.srcElement
    console.log('show_context>>>',item,evt)

}
// 复制元素图到剪贴板，透明的会展示成黑色。弃用
function clipboard_data(el) {
    // domtoimage.toBlob(el)
    // .then(function (blob) {
    //     console.log(blob)
    //     const imgSrc = window.URL.createObjectURL(blob);
    //     let imgtest = document.getElementById("imgtest")
    //     imgtest.src = imgSrc;
    //     // navigator.clipboard.write([
    //     //         new window.ClipboardItem({
    //     //             [blob['type']]: blob
    //     //         })
    //     //     ]);
    // })
    // .catch(function (error) {
    //     console.error('生成失败', error);
    // });
    domtoimage.toPng(el)
        .then(function (dataUrl) {
            console.log(dataUrl);
            navigator.clipboard.write([
                new window.ClipboardItem({
                    [blob['type']]: blob
                })
            ]);

        })
        .catch(function (error) {
            console.error('oops, something went wrong!', error);
        });
}
function clipboard_text(text){
    navigator.clipboard.writeText(text)
}

onUnmounted(async () => {
    await unlisten();
    danmuClient.close();
});
watch(wintop, (newTop) => {
    appWindow.setAlwaysOnTop(newTop);
})
let chatterbox_timer;
let msg_idx = 0;
watch(chatterbox, (newTop) => {
    if(newTop){
        DataBase.get_room_chatterbox(db, room_id).then(datas => {
            console.log(datas)
            if (datas.length!=0) {
                const chatters = JSON.parse(datas[0].chatterbox)
                clearTimeout(chatterbox_timer);
                (function loop_send_chatter() {
                    send_danmu_with_notify(room_id, chatters.chatterboxes[msg_idx++].msg);
                    if (msg_idx >= chatters.chatterboxes.length) {
                        msg_idx = 0
                        chatterbox_timer = setTimeout(loop_send_chatter, chatters.whole_interval*60*1000);
                    }else{
                        chatterbox_timer = setTimeout(loop_send_chatter, chatters.item_interval*1000);
                    }
                })();
            } else {
                $q.notify({
                    message: '请先设置话痨!',
                    color: 'red',
                    html: true,
                    timeout: 800,
                    progress: true
                })
                setTimeout(() => {
                    chatterbox.value = false;
                }, 200)
            }
        })
    }else{
        clearTimeout(chatterbox_timer)
    }
})

async function open_borrower(uid){
  var el = document.createElement("a");
  document.body.appendChild(el);
  el.href = `https://space.bilibili.com/${uid}`;
  el.target = '_blank'; 
  el.click();
  document.body.removeChild(el);
}
</script>

<style scoped>
:deep(.q-message-label) {
    margin: 0;
    text-align: center;
}

:deep(.errtip){
    animation: xcolor 1s linear forwards;
}
@keyframes xcolor {
    0%  {color: white;}
    25% {color: red;}
    100%    {color: white;}
}
.msger-chat {
    background-color: #fcfcfe;
    background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='260' height='260' viewBox='0 0 260 260'%3E%3Cg fill-rule='evenodd'%3E%3Cg fill='%23dddddd' fill-opacity='0.4'%3E%3Cpath d='M24.37 16c.2.65.39 1.32.54 2H21.17l1.17 2.34.45.9-.24.11V28a5 5 0 0 1-2.23 8.94l-.02.06a8 8 0 0 1-7.75 6h-20a8 8 0 0 1-7.74-6l-.02-.06A5 5 0 0 1-17.45 28v-6.76l-.79-1.58-.44-.9.9-.44.63-.32H-20a23.01 23.01 0 0 1 44.37-2zm-36.82 2a1 1 0 0 0-.44.1l-3.1 1.56.89 1.79 1.31-.66a3 3 0 0 1 2.69 0l2.2 1.1a1 1 0 0 0 .9 0l2.21-1.1a3 3 0 0 1 2.69 0l2.2 1.1a1 1 0 0 0 .9 0l2.21-1.1a3 3 0 0 1 2.69 0l2.2 1.1a1 1 0 0 0 .86.02l2.88-1.27a3 3 0 0 1 2.43 0l2.88 1.27a1 1 0 0 0 .85-.02l3.1-1.55-.89-1.79-1.42.71a3 3 0 0 1-2.56.06l-2.77-1.23a1 1 0 0 0-.4-.09h-.01a1 1 0 0 0-.4.09l-2.78 1.23a3 3 0 0 1-2.56-.06l-2.3-1.15a1 1 0 0 0-.45-.11h-.01a1 1 0 0 0-.44.1L.9 19.22a3 3 0 0 1-2.69 0l-2.2-1.1a1 1 0 0 0-.45-.11h-.01a1 1 0 0 0-.44.1l-2.21 1.11a3 3 0 0 1-2.69 0l-2.2-1.1a1 1 0 0 0-.45-.11h-.01zm0-2h-4.9a21.01 21.01 0 0 1 39.61 0h-2.09l-.06-.13-.26.13h-32.31zm30.35 7.68l1.36-.68h1.3v2h-36v-1.15l.34-.17 1.36-.68h2.59l1.36.68a3 3 0 0 0 2.69 0l1.36-.68h2.59l1.36.68a3 3 0 0 0 2.69 0L2.26 23h2.59l1.36.68a3 3 0 0 0 2.56.06l1.67-.74h3.23l1.67.74a3 3 0 0 0 2.56-.06zM-13.82 27l16.37 4.91L18.93 27h-32.75zm-.63 2h.34l16.66 5 16.67-5h.33a3 3 0 1 1 0 6h-34a3 3 0 1 1 0-6zm1.35 8a6 6 0 0 0 5.65 4h20a6 6 0 0 0 5.66-4H-13.1z'/%3E%3Cpath id='path6_fill-copy' d='M284.37 16c.2.65.39 1.32.54 2H281.17l1.17 2.34.45.9-.24.11V28a5 5 0 0 1-2.23 8.94l-.02.06a8 8 0 0 1-7.75 6h-20a8 8 0 0 1-7.74-6l-.02-.06a5 5 0 0 1-2.24-8.94v-6.76l-.79-1.58-.44-.9.9-.44.63-.32H240a23.01 23.01 0 0 1 44.37-2zm-36.82 2a1 1 0 0 0-.44.1l-3.1 1.56.89 1.79 1.31-.66a3 3 0 0 1 2.69 0l2.2 1.1a1 1 0 0 0 .9 0l2.21-1.1a3 3 0 0 1 2.69 0l2.2 1.1a1 1 0 0 0 .9 0l2.21-1.1a3 3 0 0 1 2.69 0l2.2 1.1a1 1 0 0 0 .86.02l2.88-1.27a3 3 0 0 1 2.43 0l2.88 1.27a1 1 0 0 0 .85-.02l3.1-1.55-.89-1.79-1.42.71a3 3 0 0 1-2.56.06l-2.77-1.23a1 1 0 0 0-.4-.09h-.01a1 1 0 0 0-.4.09l-2.78 1.23a3 3 0 0 1-2.56-.06l-2.3-1.15a1 1 0 0 0-.45-.11h-.01a1 1 0 0 0-.44.1l-2.21 1.11a3 3 0 0 1-2.69 0l-2.2-1.1a1 1 0 0 0-.45-.11h-.01a1 1 0 0 0-.44.1l-2.21 1.11a3 3 0 0 1-2.69 0l-2.2-1.1a1 1 0 0 0-.45-.11h-.01zm0-2h-4.9a21.01 21.01 0 0 1 39.61 0h-2.09l-.06-.13-.26.13h-32.31zm30.35 7.68l1.36-.68h1.3v2h-36v-1.15l.34-.17 1.36-.68h2.59l1.36.68a3 3 0 0 0 2.69 0l1.36-.68h2.59l1.36.68a3 3 0 0 0 2.69 0l1.36-.68h2.59l1.36.68a3 3 0 0 0 2.56.06l1.67-.74h3.23l1.67.74a3 3 0 0 0 2.56-.06zM246.18 27l16.37 4.91L278.93 27h-32.75zm-.63 2h.34l16.66 5 16.67-5h.33a3 3 0 1 1 0 6h-34a3 3 0 1 1 0-6zm1.35 8a6 6 0 0 0 5.65 4h20a6 6 0 0 0 5.66-4H246.9z'/%3E%3Cpath d='M159.5 21.02A9 9 0 0 0 151 15h-42a9 9 0 0 0-8.5 6.02 6 6 0 0 0 .02 11.96A8.99 8.99 0 0 0 109 45h42a9 9 0 0 0 8.48-12.02 6 6 0 0 0 .02-11.96zM151 17h-42a7 7 0 0 0-6.33 4h54.66a7 7 0 0 0-6.33-4zm-9.34 26a8.98 8.98 0 0 0 3.34-7h-2a7 7 0 0 1-7 7h-4.34a8.98 8.98 0 0 0 3.34-7h-2a7 7 0 0 1-7 7h-4.34a8.98 8.98 0 0 0 3.34-7h-2a7 7 0 0 1-7 7h-7a7 7 0 1 1 0-14h42a7 7 0 1 1 0 14h-9.34zM109 27a9 9 0 0 0-7.48 4H101a4 4 0 1 1 0-8h58a4 4 0 0 1 0 8h-.52a9 9 0 0 0-7.48-4h-42z'/%3E%3Cpath d='M39 115a8 8 0 1 0 0-16 8 8 0 0 0 0 16zm6-8a6 6 0 1 1-12 0 6 6 0 0 1 12 0zm-3-29v-2h8v-6H40a4 4 0 0 0-4 4v10H22l-1.33 4-.67 2h2.19L26 130h26l3.81-40H58l-.67-2L56 84H42v-6zm-4-4v10h2V74h8v-2h-8a2 2 0 0 0-2 2zm2 12h14.56l.67 2H22.77l.67-2H40zm13.8 4H24.2l3.62 38h22.36l3.62-38z'/%3E%3Cpath d='M129 92h-6v4h-6v4h-6v14h-3l.24 2 3.76 32h36l3.76-32 .24-2h-3v-14h-6v-4h-6v-4h-8zm18 22v-12h-4v4h3v8h1zm-3 0v-6h-4v6h4zm-6 6v-16h-4v19.17c1.6-.7 2.97-1.8 4-3.17zm-6 3.8V100h-4v23.8a10.04 10.04 0 0 0 4 0zm-6-.63V104h-4v16a10.04 10.04 0 0 0 4 3.17zm-6-9.17v-6h-4v6h4zm-6 0v-8h3v-4h-4v12h1zm27-12v-4h-4v4h3v4h1v-4zm-6 0v-8h-4v4h3v4h1zm-6-4v-4h-4v8h1v-4h3zm-6 4v-4h-4v8h1v-4h3zm7 24a12 12 0 0 0 11.83-10h7.92l-3.53 30h-32.44l-3.53-30h7.92A12 12 0 0 0 130 126z'/%3E%3Cpath d='M212 86v2h-4v-2h4zm4 0h-2v2h2v-2zm-20 0v.1a5 5 0 0 0-.56 9.65l.06.25 1.12 4.48a2 2 0 0 0 1.94 1.52h.01l7.02 24.55a2 2 0 0 0 1.92 1.45h4.98a2 2 0 0 0 1.92-1.45l7.02-24.55a2 2 0 0 0 1.95-1.52L224.5 96l.06-.25a5 5 0 0 0-.56-9.65V86a14 14 0 0 0-28 0zm4 0h6v2h-9a3 3 0 1 0 0 6H223a3 3 0 1 0 0-6H220v-2h2a12 12 0 1 0-24 0h2zm-1.44 14l-1-4h24.88l-1 4h-22.88zm8.95 26l-6.86-24h18.7l-6.86 24h-4.98zM150 242a22 22 0 1 0 0-44 22 22 0 0 0 0 44zm24-22a24 24 0 1 1-48 0 24 24 0 0 1 48 0zm-28.38 17.73l2.04-.87a6 6 0 0 1 4.68 0l2.04.87a2 2 0 0 0 2.5-.82l1.14-1.9a6 6 0 0 1 3.79-2.75l2.15-.5a2 2 0 0 0 1.54-2.12l-.19-2.2a6 6 0 0 1 1.45-4.46l1.45-1.67a2 2 0 0 0 0-2.62l-1.45-1.67a6 6 0 0 1-1.45-4.46l.2-2.2a2 2 0 0 0-1.55-2.13l-2.15-.5a6 6 0 0 1-3.8-2.75l-1.13-1.9a2 2 0 0 0-2.5-.8l-2.04.86a6 6 0 0 1-4.68 0l-2.04-.87a2 2 0 0 0-2.5.82l-1.14 1.9a6 6 0 0 1-3.79 2.75l-2.15.5a2 2 0 0 0-1.54 2.12l.19 2.2a6 6 0 0 1-1.45 4.46l-1.45 1.67a2 2 0 0 0 0 2.62l1.45 1.67a6 6 0 0 1 1.45 4.46l-.2 2.2a2 2 0 0 0 1.55 2.13l2.15.5a6 6 0 0 1 3.8 2.75l1.13 1.9a2 2 0 0 0 2.5.8zm2.82.97a4 4 0 0 1 3.12 0l2.04.87a4 4 0 0 0 4.99-1.62l1.14-1.9a4 4 0 0 1 2.53-1.84l2.15-.5a4 4 0 0 0 3.09-4.24l-.2-2.2a4 4 0 0 1 .97-2.98l1.45-1.67a4 4 0 0 0 0-5.24l-1.45-1.67a4 4 0 0 1-.97-2.97l.2-2.2a4 4 0 0 0-3.09-4.25l-2.15-.5a4 4 0 0 1-2.53-1.84l-1.14-1.9a4 4 0 0 0-5-1.62l-2.03.87a4 4 0 0 1-3.12 0l-2.04-.87a4 4 0 0 0-4.99 1.62l-1.14 1.9a4 4 0 0 1-2.53 1.84l-2.15.5a4 4 0 0 0-3.09 4.24l.2 2.2a4 4 0 0 1-.97 2.98l-1.45 1.67a4 4 0 0 0 0 5.24l1.45 1.67a4 4 0 0 1 .97 2.97l-.2 2.2a4 4 0 0 0 3.09 4.25l2.15.5a4 4 0 0 1 2.53 1.84l1.14 1.9a4 4 0 0 0 5 1.62l2.03-.87zM152 207a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm6 2a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm-11 1a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm-6 0a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm3-5a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm-8 8a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm3 6a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm0 6a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm4 7a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm5-2a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm5 4a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm4-6a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm6-4a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm-4-3a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm4-3a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm-5-4a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm-24 6a1 1 0 1 1 2 0 1 1 0 0 1-2 0zm16 5a5 5 0 1 0 0-10 5 5 0 0 0 0 10zm7-5a7 7 0 1 1-14 0 7 7 0 0 1 14 0zm86-29a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm19 9a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm-14 5a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm-25 1a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm5 4a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm9 0a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm15 1a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm12-2a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm-11-14a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm-19 0a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm6 5a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm-25 15c0-.47.01-.94.03-1.4a5 5 0 0 1-1.7-8 3.99 3.99 0 0 1 1.88-5.18 5 5 0 0 1 3.4-6.22 3 3 0 0 1 1.46-1.05 5 5 0 0 1 7.76-3.27A30.86 30.86 0 0 1 246 184c6.79 0 13.06 2.18 18.17 5.88a5 5 0 0 1 7.76 3.27 3 3 0 0 1 1.47 1.05 5 5 0 0 1 3.4 6.22 4 4 0 0 1 1.87 5.18 4.98 4.98 0 0 1-1.7 8c.02.46.03.93.03 1.4v1h-62v-1zm.83-7.17a30.9 30.9 0 0 0-.62 3.57 3 3 0 0 1-.61-4.2c.37.28.78.49 1.23.63zm1.49-4.61c-.36.87-.68 1.76-.96 2.68a2 2 0 0 1-.21-3.71c.33.4.73.75 1.17 1.03zm2.32-4.54c-.54.86-1.03 1.76-1.49 2.68a3 3 0 0 1-.07-4.67 3 3 0 0 0 1.56 1.99zm1.14-1.7c.35-.5.72-.98 1.1-1.46a1 1 0 1 0-1.1 1.45zm5.34-5.77c-1.03.86-2 1.79-2.9 2.77a3 3 0 0 0-1.11-.77 3 3 0 0 1 4-2zm42.66 2.77c-.9-.98-1.87-1.9-2.9-2.77a3 3 0 0 1 4.01 2 3 3 0 0 0-1.1.77zm1.34 1.54c.38.48.75.96 1.1 1.45a1 1 0 1 0-1.1-1.45zm3.73 5.84c-.46-.92-.95-1.82-1.5-2.68a3 3 0 0 0 1.57-1.99 3 3 0 0 1-.07 4.67zm1.8 4.53c-.29-.9-.6-1.8-.97-2.67.44-.28.84-.63 1.17-1.03a2 2 0 0 1-.2 3.7zm1.14 5.51c-.14-1.21-.35-2.4-.62-3.57.45-.14.86-.35 1.23-.63a2.99 2.99 0 0 1-.6 4.2zM275 214a29 29 0 0 0-57.97 0h57.96zM72.33 198.12c-.21-.32-.34-.7-.34-1.12v-12h-2v12a4.01 4.01 0 0 0 7.09 2.54c.57-.69.91-1.57.91-2.54v-12h-2v12a1.99 1.99 0 0 1-2 2 2 2 0 0 1-1.66-.88zM75 176c.38 0 .74-.04 1.1-.12a4 4 0 0 0 6.19 2.4A13.94 13.94 0 0 1 84 185v24a6 6 0 0 1-6 6h-3v9a5 5 0 1 1-10 0v-9h-3a6 6 0 0 1-6-6v-24a14 14 0 0 1 14-14 5 5 0 0 0 5 5zm-17 15v12a1.99 1.99 0 0 0 1.22 1.84 2 2 0 0 0 2.44-.72c.21-.32.34-.7.34-1.12v-12h2v12a3.98 3.98 0 0 1-5.35 3.77 3.98 3.98 0 0 1-.65-.3V209a4 4 0 0 0 4 4h16a4 4 0 0 0 4-4v-24c.01-1.53-.23-2.88-.72-4.17-.43.1-.87.16-1.28.17a6 6 0 0 1-5.2-3 7 7 0 0 1-6.47-4.88A12 12 0 0 0 58 185v6zm9 24v9a3 3 0 1 0 6 0v-9h-6z'/%3E%3Cpath d='M-17 191a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm19 9a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2H3a1 1 0 0 1-1-1zm-14 5a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm-25 1a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm5 4a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm9 0a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm15 1a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm12-2a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2H4zm-11-14a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm-19 0a1 1 0 0 0 0 2h2a1 1 0 0 0 0-2h-2zm6 5a1 1 0 0 1 1-1h2a1 1 0 0 1 0 2h-2a1 1 0 0 1-1-1zm-25 15c0-.47.01-.94.03-1.4a5 5 0 0 1-1.7-8 3.99 3.99 0 0 1 1.88-5.18 5 5 0 0 1 3.4-6.22 3 3 0 0 1 1.46-1.05 5 5 0 0 1 7.76-3.27A30.86 30.86 0 0 1-14 184c6.79 0 13.06 2.18 18.17 5.88a5 5 0 0 1 7.76 3.27 3 3 0 0 1 1.47 1.05 5 5 0 0 1 3.4 6.22 4 4 0 0 1 1.87 5.18 4.98 4.98 0 0 1-1.7 8c.02.46.03.93.03 1.4v1h-62v-1zm.83-7.17a30.9 30.9 0 0 0-.62 3.57 3 3 0 0 1-.61-4.2c.37.28.78.49 1.23.63zm1.49-4.61c-.36.87-.68 1.76-.96 2.68a2 2 0 0 1-.21-3.71c.33.4.73.75 1.17 1.03zm2.32-4.54c-.54.86-1.03 1.76-1.49 2.68a3 3 0 0 1-.07-4.67 3 3 0 0 0 1.56 1.99zm1.14-1.7c.35-.5.72-.98 1.1-1.46a1 1 0 1 0-1.1 1.45zm5.34-5.77c-1.03.86-2 1.79-2.9 2.77a3 3 0 0 0-1.11-.77 3 3 0 0 1 4-2zm42.66 2.77c-.9-.98-1.87-1.9-2.9-2.77a3 3 0 0 1 4.01 2 3 3 0 0 0-1.1.77zm1.34 1.54c.38.48.75.96 1.1 1.45a1 1 0 1 0-1.1-1.45zm3.73 5.84c-.46-.92-.95-1.82-1.5-2.68a3 3 0 0 0 1.57-1.99 3 3 0 0 1-.07 4.67zm1.8 4.53c-.29-.9-.6-1.8-.97-2.67.44-.28.84-.63 1.17-1.03a2 2 0 0 1-.2 3.7zm1.14 5.51c-.14-1.21-.35-2.4-.62-3.57.45-.14.86-.35 1.23-.63a2.99 2.99 0 0 1-.6 4.2zM15 214a29 29 0 0 0-57.97 0h57.96z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
}
</style>