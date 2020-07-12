import Vue from 'vue';
import App from './App.vue';
import Vuetify from 'vuetify';
import 'vuetify/dist/vuetify.min.css';
import '@mdi/font/css/materialdesignicons.css'

Vue.use(Vuetify)

const vuetify = new Vuetify({
    icons: {
        iconfont: 'mdi', // default - only for display purposes
    },
});

new Vue({ render: createElement => createElement(App), vuetify: vuetify }).$mount('#app');

