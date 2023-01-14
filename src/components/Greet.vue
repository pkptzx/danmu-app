<script setup lang="ts">
//@ts-nocheck
import { computed, onMounted, onUnmounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow,WebviewWindow,getAll,PhysicalPosition,LogicalPosition,currentMonitor } from '@tauri-apps/api/window';
import { confirm } from '@tauri-apps/api/dialog';
import { register,isRegistered,unregister  } from '@tauri-apps/api/globalShortcut';
import { fetch,Body } from '@tauri-apps/api/http';
import { appCacheDir, resourceDir, appDataDir,runtimeDir, resolveResource, resolve } from '@tauri-apps/api/path';
import { exists, readDir, BaseDirectory } from '@tauri-apps/api/fs';
import Window from './Window.vue';
import { emit,listen } from '@tauri-apps/api/event';
import * as bApi from '../assets/js/biliApi.js';
import * as DataBase from '../assets/js/db.js';
import 'animate.css';


const greetMsg = ref("");
const rooms = ref([]);
const hot_rooms = ref([]);
const followrooms = ref([]);
const input = ref("");
let db;
let listen_save_danmu_msg;
const uidOptions = ref([{'uid':'8094023','uname':'码之魂'},{'uid':'3796382','uname':'冬灰条'}])
let roomid = ref(3796382)//self 8094023 dht 3796382//点击获取直播间房号时会被覆盖
let unlisten:any;
onMounted(async () => {
  //接收窗体自己给自己发的消息
  unlisten = await appWindow.listen<string>('state-changed', (event) => {
    console.log(`Got error: ${event.payload.msg}`);
  });
  get_live_ups()
  db = await DataBase.init_db()
  //接收全局消息,可以多次订阅全局消息,订阅了都能收到
  await listen<string>('dmMsg', (event: any) => {
    // console.log(`Got2 dmMsg: ${event.payload.msg}`);
  });
  listen_save_danmu_msg = await listen<string>('save_danmu_msg', (event: any) => {
    DataBase.insert_danmu_msg(db, event.payload);
  });
  await listen<string>('send_dm', (event: any) => {
    bApi.send_danmu(roomid.value, event.payload.msg)
      .then((resp: any) => {
        console.log(resp);
        greetMsg.value = JSON.stringify(resp);
      });
  });
});
onUnmounted(async () => {
  await unlisten();
  await listen_save_danmu_msg();
  await db.close();
});
async function testdb(){
  const rst = await db.select("select * from danmu_msg");
  console.log(rst);
  const rst2 = await db.select("select * from danmu_msg where gradient not null");
  console.log(rst2);
}
async function hot_rank_list(){
  bApi.getHotRankList().then((v)=>{
    hot_rooms.value.length=0
    console.log(v.data);
    const uids = [];
     v.data.data.list.forEach((room) => {
      uids.push(room.uid)
    })
    bApi.getLivesInfo(uids).then((vv)=>{
      console.log(vv.data);
      for (let uid in vv.data.data){
        hot_rooms.value.push(vv.data.data[uid]);
    }
    })
  });
}
async function guard_rank_list(){
  bApi.getGuardRankList().then((v)=>{
    rooms.value.length=0
    console.log(v.data);
    const uids = [];
     v.data.data.forEach((room) => {
      uids.push(room.uid)
    })
    bApi.getLivesInfo(uids).then((vv)=>{
      console.log(vv.data);
      for (let uid in vv.data.data){
        rooms.value.push(vv.data.data[uid]);
    }
    })
  });
}
async function open_borrower(room_id){
  var el = document.createElement("a");
  document.body.appendChild(el);
  el.href = `https://live.bilibili.com/${room_id}`;
  el.target = '_blank'; 
  el.click();
  document.body.removeChild(el);
}
 
async function my_custom_shortcut_command() {
  await invoke("my_custom_shortcut_command");
  greetMsg.value = "快试试按下 Ctrl+K";
}
async function get_cookies() {
  const cookies = await bApi.getCookies();
  const cookiesPlainText = bApi.toCookiesPlainText(cookies); 
  //避免直播测试输出 , 仅展示长度,折叠输出到控制台
  greetMsg.value = "获得Bilibili的Cookies长度:" + cookiesPlainText.length; 
  console.log(cookies)
}
//发送私信
async function send_msg(){
  if(input.value.trim().length == 0) {
      //[懒]需要 替换成友好的对话框 
      const confirmed = await confirm('请输入私信内容', { title: '提示', type: 'warning' });
      document.getElementById("greet-input")?.focus();
      return;
    }
  bApi.send_msg(roomid.value,input.value)
  .then((resp)=>{
    console.log(resp);
    greetMsg.value = JSON.stringify(resp.data);})
}
async function test_fs(){
  const current_path = await invoke('get_current_path');
  console.log('current_path',current_path);
  const appDataDirPath = await appDataDir();
  console.log('appDataDirPath',appDataDirPath);
  const path = await resolve(appDataDirPath, '..', 'users', 'tauri', 'avatar.png');
  console.log('appDataDirPath拼接',path);
  const resourcePath = await resolveResource('script.sh');
  console.log('resourcePath拼接',resourcePath);
  const resourceDirPath = await resourceDir();
  console.log('resourceDirPath',resourceDirPath);
  
  //遍历文件夹
  const entries = await readDir('EBWebView', { dir: BaseDirectory.AppLocalData, recursive: true });
  function processEntries(entries) {
    for (const entry of entries) {
      console.log(`Entry: ${entry.path}`);
      if (entry.children) {
        processEntries(entry.children)
      }
    }
  }
  processEntries(entries);
}
async function get_live_roomid() {
  const cookies = await bApi.getCookies();
  console.log(cookies);
  const bid = cookies.DedeUserID
  //通过bid获取直播间id
  bApi.getAccInfo(bid).then((resp: any)=>{
    console.log(resp);
    let log = `主播信息:<br/>名字: ${resp.data.data.name} 用户id:${bid}<br/>直播间信息:ID:${resp.data.data.live_room.roomid} <br/>直播间标题:${resp.data.data.live_room.title} <br/> 当前是否直播:${resp.data.data.live_room.liveStatus === 0 ? '未开播' : '直播中'} <br/> 当前路径: ${current_path}`;
    roomid.value = resp.data.data.live_room.roomid
    greetMsg.value = log;
  });
}
async function get_live_ups() {
  const cookies = await bApi.getCookies();
  const bid = cookies.DedeUserID
  bApi.getAccInfo(bid).then((resp: any)=>{
    console.log(resp);
    let log = `主播信息:<br/>名字: ${resp.data.data.name} 用户id:${bid}<br/>直播间信息:ID:${resp.data.data.live_room.roomid} <br/>直播间标题:${resp.data.data.live_room.title} <br/> 当前是否直播:${resp.data.data.live_room.liveStatus === 0 ? '未开播' : '直播中'}`;
    greetMsg.value = log;
  });
  bApi.getLiveUps().then((resp: any)=>{
    console.log(resp);
    followrooms.value = resp.data.data.list
  });
}
async function send_danmu() {
    if(input.value.trim().length == 0) {
      //[懒]需要 替换成友好的对话框 
      const confirmed = await confirm('请输入弹幕内容', { title: '提示', type: 'warning' });
      document.getElementById("greet-input")?.focus();
      return;
    }
    bApi.send_danmu(roomid.value,input.value)
  .then((resp: any)=>{
    console.log(resp);
    greetMsg.value = JSON.stringify(resp.data);
  });
}
async function shortcut() {
  const registered = await isRegistered('CommandOrControl+L');
  if(registered){
    greetMsg.value = "你已经设置过快捷键 Ctrl+L";
    return;
  }
  await register('CommandOrControl+L', () => {
    greetMsg.value = "你按了Ctrl+L " +new Date().getMilliseconds();
  });
  greetMsg.value = "快试试按下 Ctrl+L";
}
async function unshortcut() {
  const registered = await isRegistered('CommandOrControl+L');
  if(!registered){
    greetMsg.value = "你已经取消过快捷键 Ctrl+L";
    return;
  }
  await unregister('CommandOrControl+L');
  greetMsg.value = "取消快捷键 Ctrl+L , 再按没反应喽";
}

//新窗口示例
function open_chatgtp() {
  let chatgtpWindow = WebviewWindow.getByLabel('chatgtp');
  if (chatgtpWindow) {
    chatgtpWindow.unminimize();
    chatgtpWindow.setFocus();
    return;
  }
  chatgtpWindow = new WebviewWindow('chatgtp', {
    url: '/chatgtp',
    "fullscreen": false,
    "height": 300,
    "resizable": true,
    "title": "设置 - 自动回复",
    "width": 400,
    "visible": false,
    "skipTaskbar": false,
    "decorations": false,
    "transparent": true,
    "center":true
  });
  chatgtpWindow.once('tauri://created', function () {
    chatgtpWindow?.show();
  });
  chatgtpWindow.once('tauri://error', function (e) {
    console.error(e)
    chatgtpWindow.unminimize();
    chatgtpWindow.setFocus();
  });
}
async function show_danmu(room_id) {
  let danmuWindow = WebviewWindow.getByLabel(`danmu-${room_id}`);
  console.log(`danmuWindow: ${danmuWindow}`);
  if (danmuWindow) {
    danmuWindow.unminimize();
    danmuWindow.setFocus();
    return;
  }
  danmuWindow = new WebviewWindow(`danmu-${room_id}`, {
    url: `/danmu/${room_id}`,
    "fullscreen": false,
    "height": 600,
    "resizable": true,
    "title": `弹幕-${room_id}`,
    "width": 400,
    "visible": false,
    "skipTaskbar": false,
    "decorations": false,
    "transparent": true,
    "center":true,
    "alwaysOnTop": false
  });
  danmuWindow.once('tauri://created', function () {
    danmuWindow?.show();
  });
  danmuWindow.once('tauri://error', function (e) {
    //如果窗口已经打开了,会报label重复的错误.所以需要先getByLabel查找窗体
    //但,在开发的时候如果刷新了页面,会导致getByLabel返回null
    //这是当前版本官方确认的bug,所以才需要在这补上已经存在的窗口的处理.
    //如果tauri更新了,就不需要这个方法了
    console.error(e)
    danmuWindow.unminimize();
    danmuWindow.setFocus();
  });
}
async function layout_danmu_win() {
  //获取屏幕大小
  const monitor = await currentMonitor();
  const m_size = monitor?.size;
  const scaleFactor = monitor?.scaleFactor
  console.log(m_size)
  // 注意!!! 在开发模式中刷新界面了,跨窗口就找不到了.
  const wins = getAll();
  let x = 0
  let y = 0;
  wins.forEach(win=>{
    if(win.label.startsWith('danmu')){
       win.outerSize().then(size=>{
        if(x+size.width > m_size.width){
          y += size.height
          x = 0
        }
        win.setPosition(new PhysicalPosition(x, y).toLogical(scaleFactor));
        win.unminimize();
        win.setFocus();
        x += size.width
       })
      
    }
  })
}
let count = 0;
async function send_event_to_danmu() {
  await emit('dmMsg', { msg: '消息消息消息'+count++ });
}

async function send_event_to_danmu2() {
  await appWindow.emit('state-changed', { msg: '消息消息消息'+count++ });
}
</script>

<template>
   <Window title="弹幕君">
    <div> 
        <div class="card"> 
          直播间房号: <input type="text" v-model="roomid" />  
          <button class="x" type="button" @click="show_danmu(roomid)">打开弹幕窗口</button>
          <button class="x" type="button" @click="get_live_roomid()">通过Cookies获取主播直播间房号</button>
 
      </div>
          <div>
          <input id="greet-input" v-model="input" placeholder="Enter a name...或输入弹幕点击发送弹幕" style="width:30vw"/>
          <button class="x" type="button" @click="send_danmu()">发送弹幕</button>
          <button class="x" type="button" @click="send_msg()">发送私信</button>
        </div>
        <div class="selectable" style="overflow-wrap: break-word;height: 80px;overflow-y: auto;">
          <div class="selectable" v-html="greetMsg"></div> 
        </div>
         
        <div> 
          <button class="x" type="button" @click="shortcut()">JS设置快捷键Ctrl+L</button>
          <button class="x" type="button" @click="unshortcut()">JS取消快捷键Ctrl+L</button>
          <button class="x" type="button" @click="testdb">testdb</button>
          <button class="x" type="button" @click="open_chatgtp()">ChatGTP示例</button>
        </div> 
        <div> 
          <button class="x" type="button" @click="send_event_to_danmu()">测试发送全局消息给弹幕窗口</button>
          <button class="x" type="button" @click="send_event_to_danmu2()">测试发送消息只能当前窗口接收</button>
          
        </div>
        <div>
          <button class="x" type="button" @click="hot_rank_list()">获取人气榜</button>
          <button class="x" type="button" @click="guard_rank_list()">获取大航海榜</button>
          <button class="x" type="button" @click="layout_danmu_win()">自动排列弹幕窗口</button>
        </div>
        
      </div>
        <q-scroll-area style="height: calc( 100vh - 250px);"> 
          <div class="q-pa-xs row items-center justify-center q-gutter-xs flex-container">
          <q-card class="my-card" v-for="room in hot_rooms" :key="room.room_id">
            <q-img :src="room.cover_from_user" @mouseenter="$event.target.children[1].children[0].src=room.keyframe" @mouseleave="$event.target.children[1].children[0].src=room.cover_from_user">
              <div class="absolute-full text-subtitle2 flex flex-center show-caption">
              </div>
            </q-img>
            <q-item>
              <q-item-section avatar>
                <q-avatar>
                  <img :src="room.face">
                </q-avatar>
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ room.title }}</q-item-label>
                <q-item-label caption>{{ room.uname }}</q-item-label>
                <q-btn dense color="secondary" @click="show_danmu(room.room_id)" class="absolute"
                  style="bottom: 8px; right: 8px; ">打开弹幕</q-btn>
                  <q-btn dense color="green" class="absolute show-go"
                  style="top: -120%; right: 35%; " :href="'https://live.bilibili.com/'+room.room_id" target="_blank">浏览器打开</q-btn>
              </q-item-section> 
            </q-item>
          </q-card>
        </div> 
        <div class="q-pa-xs row items-center justify-center q-gutter-xs flex-container">
          <q-card class="my-card" v-for="room in rooms" :key="room.room_id">
            <q-img :src="room.cover_from_user" @mouseenter="$event.target.children[1].children[0].src=room.keyframe" @mouseleave="$event.target.children[1].children[0].src=room.cover_from_user">
              <div class="absolute-full text-subtitle2 flex flex-center show-caption">
              </div>
            </q-img>
            <q-item>
              <q-item-section avatar>
                <q-avatar>
                  <img :src="room.face">
                </q-avatar>
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ room.title }}</q-item-label>
                <q-item-label caption>{{ room.uname }}</q-item-label>
                <q-btn dense color="secondary" @click="show_danmu(room.room_id)" class="absolute"
                  style="bottom: 8px; right: 8px; ">打开弹幕</q-btn>
                  <q-btn dense color="green" class="absolute show-go"
                  style="top: -120%; right: 35%; " :href="'https://live.bilibili.com/'+room.room_id" target="_blank">浏览器打开</q-btn>
              </q-item-section> 
            </q-item>
          </q-card>
        </div> 
        <div class="q-pa-xs row items-center justify-center q-gutter-xs flex-container">
          <q-card class="my-card" v-for="room in followrooms" :key="room.roomid">
            <q-img :src="room.pic" @mouseenter="$event.target.children[1].children[0].src=room.cover" @mouseleave="$event.target.children[1].children[0].src=room.pic">
              <div class="absolute-full text-subtitle2 flex flex-center show-caption">
              </div>
            </q-img>
            <q-item>
              <q-item-section avatar>
                <q-avatar>
                  <img :src="room.face">
                </q-avatar>
              </q-item-section>
              <q-item-section>
                <q-item-label>{{ room.title }}</q-item-label>
                <q-item-label caption>{{ room.uname }}</q-item-label>
                <q-btn dense color="secondary" @click="show_danmu(room.roomid)" class="absolute"
                  style="bottom: 8px; right: 8px; ">打开弹幕</q-btn>
                  <q-btn dense color="green" class="absolute show-go"
                  style="top: -120%; right: 35%; " :href="'https://live.bilibili.com/'+room.roomid" target="_blank">浏览器打开</q-btn>
              </q-item-section> 
            </q-item>
          </q-card>
        </div>
      </q-scroll-area> 
      </Window>
</template>
<style scoped>
.my-card{
  width: 100%;
  max-width: 250px;
} 
.my-card:hover{
  background: radial-gradient(circle, #35a2ff 0%, #014a88 100%)
} 
.show-go{
  display: none;
}
.my-card:hover .show-go{
  display: inherit;
} 
.my-card:hover .show-caption{
  display: none;
} 

.flex-container::after {
    content: '';
    flex: auto;    /* 或者flex: 1 */
}
button.x{
 --color: var(--q-primary);
 font-family: inherit;
 display: inline-block;
 /* width: 8em; */
 height: 2em;
 line-height: 1.6em;
 margin: 0px;
 position: relative;
 overflow: hidden;
 border: 2px solid var(--color);
 transition: color .5s;
 z-index: 1;
 font-size: 12px;
 border-radius: 6px;
 font-weight: 500;
 color: var(--color);
}

button.x:before {
 content: "";
 position: absolute;
 z-index: -1;
 background: var(--color);
 height: 150px;
 width: 200px;
 border-radius: 90%;
}

button.x:hover {
 color: #fff;
}

button.x:before {
 top: 100%;
 left: 100%;
 transition: all .7s;
}

button.x:hover:before {
 top: -30px;
 left: -30px;
}

button.x:active:before {
 background: #3a0ca3;
 transition: background 0s;
}
</style>