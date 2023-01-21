<template>
    <div data-tauri-drag-region class="subtitles-renderer" style="color:white;">{{ subtitles }}
      <q-menu touch-position auto-close context-menu>
                                    <q-list>
                                        <q-item v-close-popup clickable @click="()=>invoke('window_mouse_penetration')">
                                            <q-item-section>锁定</q-item-section>
                                        </q-item>
                                    </q-list>
                                </q-menu>
    </div>
</template>
<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref,reactive,watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useSpeechSynthesis,useSpeechRecognition } from '@vueuse/core'
const subtitles = ref('')
invoke("window_mouse_penetration").then(r=>console.log(`鼠标穿透设置结果:${r}`))
const speech = useSpeechRecognition({
    lang: 'zh-CN',
    interimResults: true,
    continuous: true,
  })
const zimu_max_length = 50
let pos = zimu_max_length/2
let keyword:string
async function startSpeechRecognition() {
  speech.result.value = ''
  speech.start()

  watch(speech.result, () => {
    if (speech.result.value.length >= zimu_max_length) {
      // pos 截取的问题是 返回的内容不是一个字一个字的增加
      //改为用固定结尾的字取。
      if (!keyword) {
        keyword = speech.result.value.slice(-pos - 5, -pos)
      }
      pos = speech.result.value.lastIndexOf(keyword)
      if (pos == -1) {
        console.log(speech.result.value)
        console.log('keyword', keyword)
        console.log('%c见鬼,识别完整句后keyword中间被插入标点了,重新算keyword...', 'color: red')
        pos = zimu_max_length / 2
        keyword = speech.result.value.slice(-pos - 5, -pos)
        pos = speech.result.value.lastIndexOf(keyword)
        console.log('修正新keyword', keyword)
      }
      subtitles.value = speech.result.value.slice(pos)
      if (subtitles.value.length >= zimu_max_length + 5) {
        console.log('%c字幕满了,开始清一行', 'color: blue')
        pos = zimu_max_length / 2
        keyword = speech.result.value.slice(-pos - 5, -pos)
        pos = speech.result.value.lastIndexOf(keyword)
        console.log('新keyword', keyword)
      }
    } else {
        subtitles.value = speech.result.value
    }
  })
  watch(speech.isListening , () => {
    console.log('isListening',speech.isListening.value);
    if (!speech.isListening.value){
      console.log('isListening',speech.isListening.value);
      speech.start()
    }
  })
}
startSpeechRecognition()
</script>
<style scoped>
.subtitles-renderer {
    text-shadow: -2px -2px #000000, -2px -1px #000000, -2px 0px #000000, -2px 1px #000000, -2px 2px #000000, -1px -2px #000000, -1px -1px #000000, -1px 0px #000000, -1px 1px #000000, -1px 2px #000000, 0px -2px #000000, 0px -1px #000000, 0px 0px #000000, 0px 1px #000000, 0px 2px #000000, 1px -2px #000000, 1px -1px #000000, 1px 0px #000000, 1px 1px #000000, 1px 2px #000000, 2px -2px #000000, 2px -1px #000000, 2px 0px #000000, 2px 1px #000000, 2px 2px #000000;
    font-family: "Imprima", "Helvetica Neue", Helvetica, "PingFang SC", "Hiragino Sans GB", "Microsoft YaHei", "\5FAE \8F6F \96C5 \9ED1 ", SimHei, Arial, sans-serif;
    font-size: 36px !important;
    line-height: 36px !important;
}
</style>