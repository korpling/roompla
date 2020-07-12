<template>
  <v-container fluid>
    <div v-for="r in rooms" :key="r.id">
      <v-card>
        <v-card-title>{{r.id}}</v-card-title>
        <v-card-text>{{$tc("people-allowed", r.maxOccupancy, {count: r.maxOccupancy})}}</v-card-text>
        <v-card-actions>
           <router-link :to="'/room/' + r.id">{{$t("book-time-slot")}}</router-link>
        </v-card-actions>
      </v-card>
    </div>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import { Configuration } from "../runtime";
import { RoomplaApi } from "../apis/RoomplaApi";
import {store} from "../store";

export default Vue.extend({
  data() {
    return { rooms: [] };
  },
  created() {
    this.fetch_rooms();
  },
  watch: {
    $route: "fetch_rooms"
  },
  methods: {
    fetch_rooms: function() {
      store.state.api.roomsGet().then(
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