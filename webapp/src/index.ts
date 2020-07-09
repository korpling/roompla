import Vue from 'vue';
import App from './App.vue';
import Vuetify from 'vuetify';
import 'vuetify/dist/vuetify.min.css';

Vue.use(Vuetify)

const vuetify = new Vuetify();

new Vue({ render: createElement => createElement(App), vuetify: vuetify }).$mount('#app');

