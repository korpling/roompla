<template>
  <v-app>
    <v-main>
      <v-container fluid v-if="userId">
        <router-view></router-view>
        <v-footer>
          <v-btn v-on:click="logout">{{$t("logout", {msg: userId})}}</v-btn>
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
    {
      path: "/room/:id",
      component: RoomView,
      props: route => ({
        id: route.params.id,
        timezone: route.query.tz,
        peopleAllowed: route.query.p
      })
    }
  ]
});

export default Vue.extend({
  i18n,
  router,
  components: { Login, RoomList, RoomView },
  data() {
    return {
      userId: null
    };
  },
  created: function() {},
  methods: {
    login_callback: function(token, userId) {
      store.login(token, userId);
      this.userId = userId;
    },
    logout: function() {
      store.logout();
      this.userId = null;
    }
  }
});
</script>

<style lang="scss" scoped>
</style>