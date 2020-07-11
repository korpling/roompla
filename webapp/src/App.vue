<template>
  <v-app>
    <v-main>
      <v-snackbar v-model="message.show" :top="true">
        {{ message.text }}
        <v-btn color="pink" text @click="message.show = false">Close</v-btn>
      </v-snackbar>
      <div v-if="api.configuration.accessToken">
        <room-overview-link v-for="r in rooms" :key="r.id" :room="r.id" />
      </div>
      <v-container v-else class="fill-height" fluid>
        <v-row align="center" justify="center">
          <v-col cols="12" sm="8" md="4">
            <v-card class="elevation-12">
              <v-toolbar color="primary" dark flat>
                <v-toolbar-title>Login</v-toolbar-title>
                <v-spacer></v-spacer>
                <v-tooltip bottom>
                  <template v-slot:="{ on }">
                    <v-btn :href="source" icon large target="_blank" v-on="on">
                      <v-icon>mdi-code-tags</v-icon>
                    </v-btn>
                  </template>
                  <span>Source</span>
                </v-tooltip>
              </v-toolbar>
              <v-card-text>
                <v-form>
                  <v-text-field
                    label="User Name"
                    name="login"
                    prepend-icon="mdi-account"
                    type="text"
                    v-model="credentials.userId"
                  ></v-text-field>

                  <v-text-field
                    id="password"
                    label="Password"
                    name="password"
                    prepend-icon="mdi-lock"
                    type="password"
                    v-model="credentials.password"
                    v-on:keyup.enter="attempt_login"
                  ></v-text-field>
                </v-form>
              </v-card-text>
              <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn v-on:click="attempt_login" color="primary">Login</v-btn>
              </v-card-actions>
            </v-card>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import RoomOverviewLink from "./components/RoomOverviewLink.vue";
import { Room } from "./models/Room";
import { RoomplaApi, LoginPostRequest } from "./apis/RoomplaApi";

export default Vue.extend({
  components: { RoomOverviewLink },
  data() {
    return {
      api: new RoomplaApi(),
      credentials: {
        userId: "",
        password: null
      },
      message: {
        show: false,
        text: ""
      },
      rooms: [
        {
          id: "3.333",
          max_occupancy: 2
        },
        {
          id: "3.408",
          max_occupancy: 1
        }
      ]
    };
  },
  methods: {
    attempt_login: function(event) {
      let api = new RoomplaApi();
      let result = api.loginPost({ credentials: this.credentials });
      result
        .then(response => {
          this.api.configuration = {accessToken: response};
          this.credentials.password = null;
          this.message.show = false;
          this.message.text = "Logged in"
        }, reason => {
          this.message.text = "Login failed: " + reason.statusText;
          this.message.show = true;
          this.credentials.password = null;
        });
    }
  }
});
</script>

<style lang="scss" scoped>
</style>