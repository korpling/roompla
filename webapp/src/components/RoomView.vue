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

    <v-calendar type="week" :first-interval="day_range.start" :interval-count="day_range.count" ></v-calendar>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  props: ["room"],
  data() {
    return {
      day_range:  { start: 7, count: 13 },
      day_ranges: [
        { text: "working hours", value: { start: 7, count: 13 } },
        { text: "whole day", value: { start: 0, count: 24 } }
      ],
      day_start: 8,
      day_end: 20
    };
  },
  methods: {}
});
</script>

<style lang="scss" scoped>
</style>