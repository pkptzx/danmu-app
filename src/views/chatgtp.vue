<template>
  <Window title="问ChatGTP" hide-minimize hide-maximize hide-top>
    <textarea type="text" style="width:60vw" v-model="input"/>
    <button @click="ask">问ChatGTP</button>
    <div style="white-space: pre-wrap;overflow-y: auto;height:calc(100vh - 100px);" class="selectable" v-text="resout"></div>

  </Window>
</template>
 
<script setup lang="ts"> 
//@ts-nocheck
import { reactive, onMounted, ref } from "vue";
import Window from '../components/Window.vue';
import CryptoJS from "crypto-js";
import { fetch,Body } from '@tauri-apps/api/http';

const resout = ref();
const input = ref('std算法具体是什么意思');
async function ask(){
  resout.value = "正在思考，请等待......";
const body = input.value;
const key = CryptoJS['enc']['Latin1']['parse']('L#$@XowPu!uZ&c%u')
	const iv = CryptoJS['enc']['Latin1']['parse']('2auvLZzxz7bo#^84')
	const encrypted = CryptoJS['AES']['encrypt'](body, key, {
		'iv': iv,
		'mode': CryptoJS['mode']['CBC'],
		'padding': CryptoJS['pad']['ZeroPadding']
	})['toString']()

	const data = JSON['stringify']({
		'prompt': encrypted
	});
  console.log(data); 
  const url = "https://cc-api.sbaliyun.com/v1/completions"
      fetch(`${url}`,{
      method: 'POST',
      headers:{ 
        "referer":"https://chatgpt.sbaliyun.com/",
          "origin":"https://chatgpt.sbaliyun.com",
      },
      timeout: 60000,
      body: Body.json({
        'prompt': encrypted
      })
    }).then((resp)=>{
      console.log(resp);
      resout.value = resp.data.choices[0].text
    });
}

</script>
