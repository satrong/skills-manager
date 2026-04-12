import { createApp } from "vue";
import "./assets/theme.css";
import App from "./App.vue";
import router from "./router";

createApp(App).use(router).mount("#app");
