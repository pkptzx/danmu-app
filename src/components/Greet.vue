<script setup lang="ts">
//@ts-nocheck
import { computed, onMounted, onUnmounted, ref,reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { getVersion } from '@tauri-apps/api/app';
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

const version = ref('');
const greetMsg = ref("");
const rooms = ref([]);
const hot_rooms = ref([]);
const followrooms = ref([]);
const input = ref("");
const my_info = reactive({"uid":0,roomid:0,"face": "","uname": "",sign:"",sex:"保密",level:0,is_senior_member:0,img_label_uri_hans_static:"",follower:0,following:0,"coredata":{
  'popularity':0,
  'fans':0,
  'watchTime':0,
  'broadcast':0,
  'barrage':0,
  'viewingDuration':0,
  'viewingNumber':0,
  'goldConsumption':0,
  'goldConsumptionForFans':0,
  'income':0,
}});
let db;
let listen_save_danmu_msg;
let roomid = ref(3796382)//self 8094023 dht 3796382//点击获取直播间房号时会被覆盖
let unlisten:any;
let coredata_Interval;
onMounted(async () => {
  version.value = await getVersion();
  //接收窗体自己给自己发的消息
  unlisten = await appWindow.listen<string>('state-changed', (event) => {
    console.log(`Got error: ${event.payload.msg}`);
  });
  init()
  db = await DataBase.init_db()
  //接收全局消息,可以多次订阅全局消息,订阅了都能收到
  await listen<string>('dmMsg', (event: any) => {
    // console.log(`Got2 dmMsg: ${event.payload.msg}`);
  });
  // 获取主播核心数据
  getCoreData()
  coredata_Interval = setInterval(getCoreData,10000)
});
onUnmounted(async () => {
  clearInterval(coredata_Interval);
  await unlisten();
  await listen_save_danmu_msg();
  await db.close();
});
function getCoreData(){
    bApi.getCoreData(1).then((resp)=>{
      console.log("CoreData",resp.data);
      resp.data.data.list.forEach((data)=>{
        switch (data.index) {
          case "income"://收益
            my_info.coredata.income = data.total;
            break;
          case "popularity"://最高人气
            my_info.coredata.popularity = data.total;
            break;
          case "fans"://新增关注
            my_info.coredata.fans = data.total;
            break;
          case "watchTime"://有效观看时长
            my_info.coredata.watchTime = data.total;
            break;
          case "broadcast"://直播时长
            my_info.coredata.broadcast = data.total;
            break;
          case "barrage"://弹幕数量
            my_info.coredata.barrage = data.total;
            break;
          case "viewingDuration"://总观看时长
            my_info.coredata.viewingDuration = data.total;
            break;
          case "viewingNumber"://总观看人数
            my_info.coredata.viewingNumber = data.total;
            break;
          case "goldConsumption"://收益-送礼-金瓜子消费总人数
            my_info.coredata.goldConsumption = data.total;
            break;
          case "goldConsumptionForFans"://收益-送礼-金瓜子消费粉丝总人数
            my_info.coredata.goldConsumptionForFans = data.total;
            break;
        }
      });
    })
  }
async function open_danmu_query(){
  
  let danmuQueryWindow = WebviewWindow.getByLabel('danmuQuery');
  if (danmuQueryWindow) {
    danmuQueryWindow.unminimize();
    danmuQueryWindow.setFocus();
    return;
  }
  danmuQueryWindow = new WebviewWindow(`danmuQuery`, {
    url: `/danmu/query`,
    "fullscreen": false,
    "height": 600,
    "resizable": true,
    "title": `弹幕查询`,
    "width": 800,
    "visible": true,
    "skipTaskbar": false,
    "decorations": false,
    "transparent": true,
    "center":true,
    "alwaysOnTop": false
  });
  danmuQueryWindow.once('tauri://created', function () {
    danmuQueryWindow?.show();
  });
  danmuQueryWindow.once('tauri://error', function (e) {
    //如果窗口已经打开了,会报label重复的错误.所以需要先getByLabel查找窗体
    //但,在开发的时候如果刷新了页面,会导致getByLabel返回null
    //这是当前版本官方确认的bug,所以才需要在这补上已经存在的窗口的处理.
    //如果tauri更新了,就不需要这个方法了
    console.error(e)
    danmuQueryWindow.unminimize();
    danmuQueryWindow.setFocus();
  });
}
async function get_my_follow_live_rooms(){
  bApi.getLiveUps().then((resp: any)=>{
    console.log(resp);
    followrooms.value = resp.data.data.list
  });
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
    console.log('send_msg',resp);
    if(resp.data.code != 0){
      greetMsg.value = '发送失败: ' + resp.data.message
    } else{
      greetMsg.value = `发送成功[${resp.data.data.msg_key}]: ${resp.data.data.msg_content}`
    }
  })
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
async function init() {
  const cookies = await bApi.getCookies();
  const uid = cookies.DedeUserID
  bApi.getAccInfo(uid).then((resp: any)=>{
    console.log(resp);
    my_info.uid =  resp.data.data.mid;
    my_info.roomid = resp.data.data.live_room.roomid;
    my_info.face = resp.data.data.face;
    my_info.uname =  resp.data.data.name;
    my_info.sign =  resp.data.data.sign;
    my_info.sex =  resp.data.data.sex;
    my_info.level =  resp.data.data.level;
    my_info.is_senior_member =  resp.data.data.is_senior_member;
    my_info.img_label_uri_hans_static = resp.data.data.vip.label.img_label_uri_hans_static;
    let log = `主播信息:<br/>名字: ${resp.data.data.name} 用户id:${uid}<br/>直播间信息:ID:${resp.data.data.live_room.roomid} <br/>直播间标题:${resp.data.data.live_room.title} <br/> 当前是否直播:${resp.data.data.live_room.liveStatus === 0 ? '未开播' : '直播中'}`;
    greetMsg.value = log;
  });
  bApi.getStat(uid).then((resp: any)=>{
    console.log('stat',resp);
    my_info.follower = resp.data.data.follower
    my_info.following = resp.data.data.following
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
    if(resp.data.code != 0){
      greetMsg.value = '发送失败: ' + resp.data.message
    } else if(resp.data.message == 'f'){
      greetMsg.value = `发送失败: 弹幕被吞,请检查是否包含不友好的词汇`
    }else{
      greetMsg.value = `发送成功`
    }
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
    "visible": true,
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
    "visible": true,
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
  <Window icon="/vite.svg" bg-color="bg-light-blue-1" :title="'弹幕君 v' + version">
    <div class="q-pa-sx row items-start q-gutter-md">
      <q-item clickable v-ripple style="width:300px">
        <q-item-section side>
          <q-avatar size="48px">
            <img :src="my_info.face" />
          </q-avatar>
          ID:{{ my_info.roomid }}
        </q-item-section>
        <q-item-section>
          <q-item-label><span class="text-weight-bold">{{ my_info.uname }}</span>
            <q-badge v-if="my_info.sex != '保密'" :color="my_info.sex == '男' ? 'light-blue' : 'pink-12'">
              <q-icon :name="my_info.sex == '男' ? 'male' : 'female'" color="white" />
            </q-badge>
            &nbsp;
            <q-badge :style="my_info.level == 6 ? 'background-color:rgb(255, 0, 0)' : 'background-color:rgb(238, 103, 42)'">
              &nbsp; LV{{ my_info.level }} <q-icon v-if="my_info.is_senior_member" name="electric_bolt" />&nbsp;
            </q-badge>
          </q-item-label>
          <q-item-label caption>{{ my_info.sign }}</q-item-label>
        </q-item-section>
      </q-item>
      <q-card class="data-card text-white" style="background: #c7c7ff;">
        <q-item>
          <q-item-section class="text-center items-center">
            <q-icon name="diversity_3" size="48px" style="color: rgba(149,149,255,1);" />
            <span class="text-weight-bolder" style="font-size: 1.4em;">{{ my_info.follower }}</span>
            <strong style="color: rgba(149,149,255,1);">粉丝数量</strong>
          </q-item-section>
        </q-item>
      </q-card>
      <q-card class="data-card text-white" style="background: #ffd8be;">
        <q-item>
          <q-item-section class="text-center items-center">
            <q-icon name="trending_up" size="48px" style="color: rgba(252,161,71,1);" />
            <span class="text-weight-bolder" style="font-size: 1.4em;">{{ my_info.coredata.fans }}</span>
            <strong style="color: rgba(252,161,71,1);">新增粉丝</strong>
          </q-item-section>
        </q-item>
      </q-card>
      <q-card class="data-card text-white" style="background: #a9ecbf;">
        <q-item>
          <q-item-section class="text-center items-center">
            <q-icon name="card_giftcard" size="48px" style="color: rgba(66,193,110,1);" />
            <span class="text-weight-bolder" style="font-size: 1.4em;">{{ my_info.coredata.goldConsumption }}</span>
            <strong style="color: rgba(66,193,110,1);">本场礼物</strong>
          </q-item-section>
        </q-item>
      </q-card>
      <q-card class="data-card text-white" style="background: #f3bbe1;">
        <q-item>
          <q-item-section class="text-center items-center">
            <q-icon name="visibility" size="48px" style="color: rgba(220,91,183,1);" />
            <span class="text-weight-bolder" style="font-size: 1.4em;">{{ my_info.coredata.barrage }}</span>
            <strong style="color: rgba(220,91,183,1);">弹幕数量</strong>
          </q-item-section>
        </q-item>
      </q-card>
    </div>
    <div style="margin-top:8px;">
      <div>
        直播间房号: <input type="text" v-model="roomid" />
        <button class="x" type="button" @click="show_danmu(roomid)">打开弹幕窗口</button>
        <button class="x" type="button" @click="show_danmu('8094023')">打开码之魂弹幕窗口</button>
        <button class="x" type="button" @click="show_danmu('3796382')">打开冬灰条弹幕窗口</button>
  
      </div>
      <div>
        <input id="greet-input" v-model="input" placeholder="请输入弹幕或私信内容..." style="width:30vw" />
        <button class="x" type="button" @click="send_danmu()">发送弹幕</button>
        <button class="x" type="button" @click="send_msg()">发送私信</button>
      </div>
      <div style="overflow-wrap: break-word;height: 80px;overflow-y: auto;">
        <div class="selectable" v-html="greetMsg"></div>
      </div>
  
      <!-- <div> 
            <button class="x" type="button" @click="shortcut()">JS设置快捷键Ctrl+L</button>
            <button class="x" type="button" @click="unshortcut()">JS取消快捷键Ctrl+L</button>
            <button class="x" type="button" @click="send_event_to_danmu()">测试发送全局消息给弹幕窗口</button>
            <button class="x" type="button" @click="send_event_to_danmu2()">测试发送消息只能当前窗口接收</button>
            
          </div> -->
      <div>
        <button class="x" type="button" @click="get_my_follow_live_rooms()">刷新我关注的在播直播间</button>
        <button class="x" type="button" @click="hot_rank_list()">获取人气榜</button>
        <button class="x" type="button" @click="guard_rank_list()">获取大航海榜</button>
        <button class="x" type="button" @click="layout_danmu_win()">自动排列弹幕窗口</button>
        <button class="x" type="button" @click="open_danmu_query">弹幕查询</button>
        <button class="x" type="button" @click="open_chatgtp()">ChatGTP示例</button>
      </div>
  
    </div>
    <q-scroll-area style="height: calc( 100vh - 320px);">
      <div class="q-pa-xs row items-center justify-center q-gutter-xs flex-container">
        <q-card class="my-card" v-for="room in hot_rooms" :key="room.room_id">
          <q-img :src="room.cover_from_user" @mouseenter="$event.target.children[1].children[0].src = room.keyframe"
            @mouseleave="$event.target.children[1].children[0].src = room.cover_from_user">
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
              <q-btn dense color="green" class="absolute show-go" style="top: -120%; right: 35%; "
                :href="'https://live.bilibili.com/' + room.room_id" target="_blank">浏览器打开</q-btn>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
      <div class="q-pa-xs row items-center justify-center q-gutter-xs flex-container">
        <q-card class="my-card" v-for="room in rooms" :key="room.room_id">
          <q-img :src="room.cover_from_user" @mouseenter="$event.target.children[1].children[0].src = room.keyframe"
            @mouseleave="$event.target.children[1].children[0].src = room.cover_from_user">
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
              <q-btn dense color="green" class="absolute show-go" style="top: -120%; right: 35%; "
                :href="'https://live.bilibili.com/' + room.room_id" target="_blank">浏览器打开</q-btn>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
      <div class="q-pa-xs row items-center justify-center q-gutter-xs flex-container">
        <q-card class="my-card" v-for="room in followrooms" :key="room.roomid">
          <q-img :src="room.pic" @mouseenter="$event.target.children[1].children[0].src = room.cover"
            @mouseleave="$event.target.children[1].children[0].src = room.pic">
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
              <q-btn dense color="green" class="absolute show-go" style="top: -120%; right: 35%; "
                :href="'https://live.bilibili.com/' + room.roomid" target="_blank">浏览器打开</q-btn>
            </q-item-section>
          </q-item>
        </q-card>
      </div>
    </q-scroll-area>
  </Window>
</template>
<style scoped>
.data-card{
  width: 100%;
  max-width: 93px;
}
.data-card:hover {
 transform: scale(1.13);
 transition: all 0.3s;
} 
.my-card{
  width: 100%;
  max-width: 250px;
} 
.my-card:hover{
  background: radial-gradient(circle, #35a2ff 0%, #014a88 100%);
 transform: scale(0.95);
 transition: all 0.3s;
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