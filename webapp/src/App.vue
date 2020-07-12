<template>
  <v-app>
    <v-main>
      <v-container fluid v-if="loggedIn">
        <router-view></router-view>
        <v-footer>
          <v-btn v-on:click="logout">{{$t("logout")}}</v-btn>
        </v-footer>
      </v-container>
      <login v-else @logged-in="login_callback" />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import VueRouter from "vue-router";

import Login from "./components/Login.vue";
import RoomView from "./components/RoomView.vue";
import RoomList from "./components/RoomList.vue";
import { Room } from "./models/Room";
import { Configuration } from "./runtime";
import { i18n } from "./lang";

import { store } from "./store";

Vue.use(VueRouter);

const router = new VueRouter({
  routes: [
    { path: "/", component: RoomList },
    { path: "/room/:id", component: RoomView }
  ]
});

export default Vue.extend({
  i18n,
  router,
  components: { Login, RoomList, RoomView },
  data() {
    return {
      loggedIn: false,
    };
  },
  created: function() {},
  methods: {
    login_callback: function(token) {
      store.login(token);
      this.loggedIn = true;
      
    },
    logout: function() {
      store.logout();
      this.loggedIn = false;
    },
  }
});
</script>

<style lang="scss" scoped>
</style>