import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "./router";
import { Quasar,Notify,Dialog } from 'quasar'
import quasarLang from 'quasar/lang/zh-CN'

// Import icon libraries
import '@quasar/extras/material-icons/material-icons.css'
import '@quasar/extras/material-icons-outlined/material-icons-outlined.css'

// Import Quasar css
// import 'quasar/src/css/index.sass'
import 'quasar/dist/quasar.css'

const app = createApp(App);

app.use(Quasar, {
    plugins: {Notify,Dialog}, // import Quasar plugins and add here
    lang: quasarLang,
  })
// app.use(createPinia());
app.use(router);

app.mount("#app");
