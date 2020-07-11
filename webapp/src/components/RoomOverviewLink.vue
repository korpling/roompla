<template>
  <v-card>
    <v-card-title>{{room}}</v-card-title>
  </v-card>
</template>

<script lang="ts">
import Vue from "vue";
import { Configuration } from "../runtime";
import {RoomplaApi} from "../apis/RoomplaApi";

export default Vue.extend({
  props: ["room"],
  methods: {
    attempt_login: function(event) {
      let result = this.api.loginPost({ credentials: this.credentials });
      result.then(
        response => {
          this.api_config.accessToken = response;
          let config = new Configuration();
          this.api = new RoomplaApi(config);
      

          this.credentials.password = null;
          this.message.show = false;
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