<template>
  <v-app>
    <v-main>
      <v-snackbar v-model="message.show" :top="true">
        {{ message.text }}
        <v-btn text @click="message.show = false">Close</v-btn>
      </v-snackbar>
      <v-container>
        <div v-if="api.configuration.accessToken">
          <div>
            <room-overview-link v-for="r in rooms" :key="r.id" :room="r.id" />
          </div>
          <div>
            <v-btn v-on:click="logout">Logout</v-btn>
          </div>
        </div>
        <login v-else @logged-in="login_callback" />
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import RoomOverviewLink from "./components/RoomOverviewLink.vue";
import Login from "./components/Login.vue";
import { Room } from "./models/Room";
import { RoomplaApi, LoginPostRequest } from "./apis/RoomplaApi";
import { Configuration } from "./runtime";


export default Vue.extend({
  components: { RoomOverviewLink, Login },
  data() {
    return {
      api: new RoomplaApi(),
      userId: null,
      message: {
        show: false,
        text: ""
      },
      rooms: []
    };
  },
  created: function() {},
  mounted: function() {
    if (this.api.configuration.accessToken) {
      this.fetch_rooms();
    }
  },
  methods: {
    login_callback: function(token) {
      // Recreate the internal API
      if (token) {
        this.api = new RoomplaApi(new Configuration({ accessToken: token }));
        // Since we are now authentifcated, we can fetch the rooms
        this.fetch_rooms();
      } else {
        this.logout();
      }
    },
    logout: function() {
      this.api = new RoomplaApi();
      this.rooms = [];
    },
    fetch_rooms: function() {
      this.api.roomsGet().then(
        response => (this.rooms = response),
        reason => {
          this.message.text = "Could not fetch rooms: " + reason;
          this.message.show = true;
        }
      );
    }
  }
});
</script>

<style lang="scss" scoped>
</style>