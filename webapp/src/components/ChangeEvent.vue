<template>
  <v-container fluid>
    <v-card flat>
      <v-toolbar>
        <div v-if="selectedEvent != null && store.state.userId === selectedEvent.occupancy.userId">
          <v-btn icon>
            <v-icon>mdi-content-save</v-icon>
          </v-btn>
          <v-btn icon v-on:click="deleteEvent">
            <v-icon>mdi-delete</v-icon>
          </v-btn>
        </div>
        <v-spacer></v-spacer>
        <v-btn icon>
          <v-icon v-on:click="close">mdi-close</v-icon>
        </v-btn>
      </v-toolbar>
      <v-card-text>
        <v-container fill-height fluid>
          <v-row  align="center" justify="center">
            <v-col>
              <v-time-picker format="24hr" v-model="start"></v-time-picker>
            </v-col>
            <v-col><v-icon x-large>mdi-arrow-right</v-icon></v-col>
            <v-col>
              <v-time-picker format="24hr" v-model="end"></v-time-picker>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import { RoomplaApi } from "../apis/RoomplaApi";
import moment from "moment-timezone";

export default Vue.extend({
  props: ["selectedEvent", "store"],
  data() {
    return {
      start: moment(this.selectedEvent.start).format("HH:mm:ss"),
      end: moment(this.selectedEvent.end).format("HH:mm:ss")
    };
  },
  methods: {
    deleteEvent() {
      this.store.state.api
        .roomsRoomOccupanciesIdDelete({
          room: this.updatedEvent.occupancy.room,
          id: this.updatedEvent.occupancy.id
        })
        .then(
          response => {
            this.$emit("event-editor-closed");
          },
          response => {
            alert(response.statusText);
          }
        );
    },
    close() {
      this.$emit("event-editor-closed");
    }
  }
});
</script>