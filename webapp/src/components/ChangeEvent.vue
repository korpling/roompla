<template>
  <v-container fluid>
    <v-card flat>
      <v-toolbar>
        <div v-if="selectedEvent != null && store.state.userId === selectedEvent.occupancy.userId">
          <v-btn icon v-on:click="saveEvent">
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
          <v-row align="center" justify="center">
            <v-col>
              <v-time-picker format="24hr" :allowed-minutes="['00']" v-model="start"></v-time-picker>
            </v-col>
            <v-col>
              <v-icon x-large>mdi-arrow-right</v-icon>
            </v-col>
            <v-col>
              <v-time-picker format="24hr" v-model="end" :allowed-minutes="['00']"></v-time-picker>
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
import {store} from "../store";
import moment from "moment-timezone";

export default Vue.extend({
  props: ["selectedEvent"],
   data() {
    return {
      store: null,
      start: null,
      end: null,
    };
  },
  created() {
    this.store = store;
  },
  mounted() {
    this.start = moment(this.selectedEvent.start).format("HH:mm");
    this.end = moment(this.selectedEvent.end).format("HH:mm");
    this.store = store;
  },
  methods: {
    saveEvent() {
      const parsed_start = moment(this.start,"HH:mm");
      const parsed_end = moment(this.end,"HH:mm");
      const adjusted_start = moment(this.selectedEvent.start, "YYYY-MM-DD HH:mm:ss").hour(parsed_start.hour()).minute(parsed_start.minute());
      const adjusted_end = moment(this.selectedEvent.end, "YYYY-MM-DD HH:mm:ss").hour(parsed_end.hour()).minute(parsed_end.minute());
      this.store.state.api
        .roomsRoomOccupanciesIdPut({
          room: this.selectedEvent.occupancy.room,
          id: this.selectedEvent.occupancy.id,
          timeRange: {
            start: adjusted_start.toISOString(),
            end: adjusted_end.toISOString(),
          }
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
    deleteEvent() {
      this.store.state.api
        .roomsRoomOccupanciesIdDelete({
          room: this.selectedEvent.occupancy.room,
          id: this.selectedEvent.occupancy.id
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