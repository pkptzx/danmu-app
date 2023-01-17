//人气榜: https://api.live.bilibili.com/xlive/web-interface/v1/index/getHotRankList
//大航海: https://api.live.bilibili.com/room/v2/Index/getNewRankTop?type=guard

import { fetch, Body } from '@tauri-apps/api/http';
import { invoke } from "@tauri-apps/api/tauri";

// let CACHE_COOKIES = JSON.parse(invoke("get_cookies"))
// let CACHE_COOKIES_STRING = toCookiesPlainText(CACHE_COOKIES)

//生成deviceId
const deviceid = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, (function (name) {
  let randomInt = 16 * Math.random() | 0;
  return ("x" === name ? randomInt : 3 & randomInt | 8).toString(16).toUpperCase()
}));
//人气榜
async function getHotRankList() {
  return fetch(`https://api.live.bilibili.com/xlive/web-interface/v1/index/getHotRankList`, {
    method: 'GET'
  })
}
//大航海榜
async function getGuardRankList() {
  return fetch(`https://api.live.bilibili.com/room/v2/Index/getNewRankTop?type=guard`, {
    method: 'GET'
  })
}
//获取cookies
async function getCookies() {
  const cookies = await invoke("get_cookies");
  const cookies_json = JSON.parse(cookies);
  return cookies_json;
}
//获取cookies文本
function toCookiesPlainText(cookies_json) {
  let cookies_raw_string = "";
  for (let k in cookies_json) {
    cookies_raw_string += (k + "=" + cookies_json[k] + '; ');
  }
  return cookies_raw_string;
}
//发送弹幕 返回f屏蔽词 k直播间屏蔽词
async function send_danmu(roomid, msg) {
  if (msg.trim().length == 0) {
    return;
  }
  const cookies = await getCookies();
  const bili_jct = cookies.bili_jct;
  let cookies_raw_string = toCookiesPlainText(cookies)
  const body = Body.form({
    'bubble': '0',
    'msg': msg,
    'color': '16777215',
    'mode': '1',
    'fontsize': '25',
    'rnd': String(Math.round(Date.now() / 1000)),
    'roomid': String(roomid),
    'csrf': bili_jct,
    'csrf_token': bili_jct
  });
  return fetch(`https://api.live.bilibili.com/msg/send`, {
    method: 'POST',
    body: body,
    headers: {
      "cookie": cookies_raw_string,
      "referer": "https://www.bilibili.com/",
      "origin": "https://live.bilibili.com",
      "sec-fetch-mode": "cors",
      "sec-fetch-site": "same-site"
    }
  })
}
//发送私信
async function send_msg(receiver_id, msg) {
  const cookies = await getCookies();
  const bili_jct = cookies.bili_jct;
  const uid = cookies.DedeUserID;
  const cookies_raw_string = toCookiesPlainText(cookies);
  var formData = new FormData();
  formData.append('msg[sender_uid]', uid);
  formData.append('msg[receiver_id]', receiver_id);
  formData.append('msg[receiver_type]', 1);
  formData.append('msg[msg_type]', 1);
  formData.append('msg[msg_status]', 0);
  formData.append('msg[dev_id]', deviceid);
  formData.append('msg[timestamp]', Math.round(Date.now() / 1000));
  formData.append('msg[content]', JSON.stringify({ 'content': msg }));
  formData.append('csrf', bili_jct);
  const body = Body.form(formData);
  return fetch(`http://api.vc.bilibili.com/web_im/v1/web_im/send_msg`, {
    method: 'POST',
    body: body,
    headers: {
      "cookie": cookies_raw_string,
      "referer": "https://message.bilibili.com/",
      "origin": "https://message.bilibili.com",
      "sec-fetch-mode": "cors",
      "sec-fetch-site": "same-site"
    }
  })
}
async function getAccInfo(mid) {
  const cookies = await getCookies();
  const cookies_raw_string = toCookiesPlainText(cookies);
  const url = "https://api.bilibili.com/x/space/wbi/acc/info"
  return fetch(`${url}?mid=${mid}`, {
    method: 'GET',
    headers: {
      "cookie": cookies_raw_string,
      "referer": "https://message.bilibili.com/",
      "origin": "https://message.bilibili.com",
      "sec-fetch-mode": "cors",
      "sec-fetch-site": "same-site"
    }
  })
}

async function getLiveRoomInfo(room_id) {
  const url = "https://api.live.bilibili.com/room/v1/Room/get_info"
  return fetch(`${url}?room_id=${room_id}`, {
    method: 'GET'
  })
}
//获取主播信息,可以获取头像,直播间ID等信息.
async function getUpInfo(uid) {
  const url = "https://api.live.bilibili.com/live_user/v1/Master/info"
  return fetch(`${url}?uid=${uid}`, {
    method: 'GET'
  })
}
//已关注中当前开播的人
async function getLiveUps() {
  const cookies = await getCookies();
  const cookies_raw_string = toCookiesPlainText(cookies);
  const url = "https://api.live.bilibili.com/xlive/web-ucenter/v1/xfetter/FeedList"
  return fetch(`${url}?page=1&pagesize=100`, {
    method: 'GET',
    headers: {
      "cookie": cookies_raw_string,
      "referer": "https://live.bilibili.com/",
      "origin": "https://live.bilibili.com",
      "sec-fetch-mode": "cors",
      "sec-fetch-site": "same-site"
    }
  })
}
//批量获取直播间状态信息
async function getLivesInfo(uids) {
  const body = Body.json({ 'uids': uids });
  const url = "https://api.live.bilibili.com/room/v1/Room/get_status_info_by_uids"
  return fetch(`${url}`, {
    method: 'POST',
    body: body
  })
}
//关系
// https://api.bilibili.com/x/space/wbi/acc/relation?mid=9365715
//自己的关注列表,直播优先显示,这个是点关注列表的接口
// https://api.live.bilibili.com/xlive/web-ucenter/user/following?page=1&page_size=20
// 关注里的直播的人,这个是首页-动态-直播动态的接口
// https://api.live.bilibili.com/xlive/web-ucenter/v1/xfetter/FeedList?page=1&pagesize=10

//主播信息,获取头像等
// https://api.live.bilibili.com/live_user/v1/Master/info?uid=uid

// 获取主播核心数据 range_type=今日1 7日2 近30日3
// https://api.live.bilibili.com/xlive/app-blink/v1/date/CoreData?platform=web&mobi_app=web&build=1&range_type=1
async function getCoreData(range_type) {
  const cookies = await getCookies();
  const cookies_raw_string = toCookiesPlainText(cookies);
  const url = "https://api.live.bilibili.com/xlive/app-blink/v1/date/CoreData?platform=web&mobi_app=web&build=1&range_type="
  return fetch(`${url}${range_type}`, {
    method: 'GET',
    headers: {
      "cookie": cookies_raw_string,
      "referer": "https://live.bilibili.com/",
      "origin": "https://live.bilibili.com",
      "sec-fetch-mode": "cors",
      "sec-fetch-site": "same-site"
    }
  })
}
//获取粉丝数量/关注数
async function getStat(uid) {
  const url = "https://api.bilibili.com/x/relation/stat"
  return fetch(`${url}?vmid=${uid}`)
}
export {
  getHotRankList,
  getGuardRankList,
  getCookies,
  send_danmu,
  send_msg,
  toCookiesPlainText,
  getAccInfo,
  getLiveRoomInfo,
  getUpInfo,
  getLiveUps,
  getLivesInfo,
  getCoreData,
  getStat

};

// export default {
//     getHotRankList
//   };