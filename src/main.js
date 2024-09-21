import { createApp } from "vue";
import { createPinia } from "pinia";
import "./index.css"; // Add this line
import App from "./App.vue";

const app = createApp(App);
app.use(createPinia());
app.mount("#app");
