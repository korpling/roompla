<template>
  <v-container class="fill-height" fluid>
    <v-snackbar v-model="message.show" top=true>
      {{ message.text }}
      <v-btn text @click="message.show = false">Close</v-btn>
    </v-snackbar>

    <v-row align="center" justify="center">
      <v-col cols="12" sm="8" md="4">
        <v-card class="elevation-12">
          <v-toolbar color="primary" dark flat>
            <v-toolbar-title>{{$t('login')}}</v-toolbar-title>
          </v-toolbar>
          <v-card-text>
            <v-form>
              <v-text-field
                :label="$t('user-name')"
                name="login"
                prepend-icon="mdi-account"
                type="text"
                v-model="credentials.userId"
              ></v-text-field>

              <v-text-field
                id="password"
                :label="$t('password')"
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
            <v-btn v-on:click="attempt_login" color="primary">{{$t('login')}}</v-btn>
          </v-card-actions>
        </v-card>
      </v-col>
    </v-row>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import { Configuration } from "../runtime";
import { RoomplaApi } from "../apis";

export default Vue.extend({
  data() {
    return {
      credentials: {
        userId: null,
        password: null
      },
      message: {
        show: false,
        text: ""
      }
    };
  },
  methods: {
    attempt_login: function(event) {
      let api = new RoomplaApi();
      let result = api.loginPost({
        credentials: this.credentials
      });
      result.then(
        response => {
          this.message.show = false;

          this.$emit("logged-in", response, this.credentials.userId);
        },
        reason => {
          this.message.text = "Login failed: " + reason.statusText;
          this.message.show = true;
          this.credentials.password = null;
        }
      );
    }
  }
});
</script>

<style lang="scss" scoped>
</style>