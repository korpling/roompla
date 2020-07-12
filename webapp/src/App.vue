<template>
  <v-app>
    <v-main>
      <v-container fluid v-if="api.configuration.accessToken">
        <v-row align="center" justify="center">
          <v-col cols="2">
            <v-btn v-if="room" v-on:click="room = null">Room list</v-btn>
          </v-col>
          <v-col cols="2">
            <v-btn v-on:click="logout">Logout</v-btn>
          </v-col>
        </v-row>
        <div v-if="room">
          <v-row>
            <room-view :room="room"></room-view>
          </v-row>
        </div>
        <v-row v-else>
          <room-list :api="api" @room-selected="room_selected_callback"></room-list>
        </v-row>
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
import { RoomplaApi, LoginPostRequest } from "./apis/RoomplaApi";
import { Configuration } from "./runtime";

Vue.use(VueRouter);

export default Vue.extend({
  components: { Login, RoomList, RoomView },
  data() {
    return {
      api: new RoomplaApi(),
      userId: null,
      message: {
        show: false,
        text: ""
      },
      room: null
    };
  },
  created: function() {},
  methods: {
    login_callback: function(token) {
      // Recreate the internal API
      if (token) {
        this.api = new RoomplaApi(new Configuration({ accessToken: token }));
      } else {
        this.api = new RoomplaApi();
      }
    },
    logout: function() {
      this.api = new RoomplaApi();
      this.rooms = [];
    },
    room_selected_callback: function(room) {
      this.room = room;
    }
  }
});
</script>

<style lang="scss" scoped>
</style>