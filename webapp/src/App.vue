<template>
  <v-app>
    <v-main>
      <v-snackbar
        v-model="message.show"
        top = true
      >
        {{ message.text }}
        <v-btn color="pink" flat @click="message.show = false">Close</v-btn>
      </v-snackbar>
      <div v-if="user.jwt_token">
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
                    v-model="user.id"
                  ></v-text-field>

                  <v-text-field
                    id="password"
                    label="Password"
                    name="password"
                    prepend-icon="mdi-lock"
                    type="password"
                    v-model="user.provided_password"
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
import {Room} from "./models/Room";
import {RoomplaApi, LoginPostRequest} from "./apis/RoomplaApi";

export default Vue.extend({
  components: { RoomOverviewLink },
  data() {
    return {
      user: {
        id: null,
        provided_password: null,
        jwt_token: null,
      },
      message: {
        show: false,
        text: "",
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
    attempt_login: function (event) {
      let api = new RoomplaApi();
      let request = {
        credentials: {
          userId: this.user.id,
          password: this.user.provided_password,
        }
      };
      let result = api.loginPost(request);
      result.then(
        (response) => {
          this.user.jwt_token = response; 
        }
      ).catch(
        (response) => {
          this.message.text = "Login failed: " + response.statusText;
          this.message.show = true;
          }
      );
    }
  }
});
</script>

<style lang="scss" scoped>
</style>