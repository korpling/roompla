<template>
  <v-container fluid>
    <v-card flat>
      <v-toolbar>
        <v-btn icon>
          <v-icon>mdi-content-save</v-icon>
        </v-btn>
        <v-btn icon v-on:click="deleteEvent">
          <v-icon>mdi-delete</v-icon>
        </v-btn>
        <v-spacer></v-spacer>
        <v-btn icon>
          <v-icon v-on:click="close">mdi-close</v-icon>
        </v-btn>
      </v-toolbar>
      <v-card-text></v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import { RoomplaApi } from "../apis/RoomplaApi";
import { store } from "../store";
import moment from "moment-timezone";

export default Vue.extend({
  props: ["selectedEvent"],
  data() {
    return { store };
  },
  methods: {
    deleteEvent() {
      this.store.state.api.roomsRoomOccupanciesIdDelete({
        room: this.selectedEvent.occupancy.room,
        id: this.selectedEvent.occupancy.id
      }).then(response => {
        this.$emit("event-editor-closed");
      }, response => {
          alert(response.statusText);
      });
    },
    close() {
      this.$emit("event-editor-closed");
    }
  }
});
</script>