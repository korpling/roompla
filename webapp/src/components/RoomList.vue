<template>
  <v-container>
    <div v-for="r in rooms" :key="r.id">
      <v-card>
        <v-card-title>{{r.id}}</v-card-title>
        <v-card-actions>
          <v-btn v-on:click="open_room($event, r)">Select</v-btn>
        </v-card-actions>
      </v-card>
    </div>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import { Configuration } from "../runtime";
import { RoomplaApi } from "../apis/RoomplaApi";

export default Vue.extend({
  data() {
    return { rooms: [] };
  },
  props: ["api"],
  created() {
    this.fetch_rooms();
  },
  watch: {
    $route: "fetch_rooms"
  },
  methods: {
    fetch_rooms: function() {
      this.api.roomsGet().then(
        response => (this.rooms = response),
        reason => {
          this.message.text = "Could not fetch rooms: " + reason;
          this.message.show = true;
        }
      );
    },
    open_room: function(event, room) {
      this.$emit("room-selected", room);
    }
  }
});
</script>

<style lang="scss" scoped>
</style>