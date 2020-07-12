<template>
  <v-container fluid>
    <v-toolbar color="primary" dark flat>
      <v-toolbar-title>{{room.id}}</v-toolbar-title>
      <v-toolbar-items>
        <v-select
          v-model="day_range"
          :items="day_ranges"
          dense
          outlined
          hide-details
          class="ma-2"
          label="hours"
        ></v-select>
      </v-toolbar-items>
    </v-toolbar>

    <v-calendar
      type="week"
      :events="events"
      :first-interval="day_range.start"
      :interval-count="day_range.count"
      @change="getEvents"
    ></v-calendar>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  props: ["room"],
  data() {
    return {
      events: [],
      day_range: { start: 7, count: 13 },
      day_ranges: [
        { text: "working hours", value: { start: 7, count: 13 } },
        { text: "whole day", value: { start: 0, count: 24 } }
      ]
    };
  },
  methods: {
    getEvents({ start, end }) {
      const events = [];

      events.push({
        name: "Test event",
        start: `${start.date}T09:00:00`,
        end: `${start.date}T14:00:00`,
        timed: true
      });

      this.events = events;
    }
  }
});
</script>

<style lang="scss" scoped>
</style>