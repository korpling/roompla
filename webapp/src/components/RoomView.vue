<template>
  <v-container fluid>
    <v-toolbar color="primary" dark flat>
      <v-toolbar-title>{{$t("room-name", {msg: id}) }}</v-toolbar-title>
      <v-toolbar-items>
        <v-btn icon class="hidden-xs-only">
          <v-icon>mdi-arrow-left</v-icon>
        </v-btn>
        <v-select
          v-model="day_range"
          :items="day_ranges"
          dense
          outlined
          hide-details
          class="ma-2"
          :label="$t('hours-selection')"
        ></v-select>
      </v-toolbar-items>
    </v-toolbar>

    <v-calendar
      type="week"
      :events="events"
      :first-interval="day_range.start"
      :interval-count="day_range.count"
      :locale="locale"
      @change="getEvents"
    ></v-calendar>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import { i18n } from "../lang/";
import { store } from "../store";
import moment from "moment-timezone";

export default Vue.extend({
  props: ["id", "timezone"],
  data() {
    return {
      locale: i18n.locale,
      events: [],
      day_range: { start: 7, count: 13 },
      day_ranges: [
        { text: i18n.t("working-hours"), value: { start: 7, count: 13 } },
        { text: i18n.t("whole-day"), value: { start: 0, count: 24 } }
      ]
    };
  },
  methods: {
    getEvents({ start, end }) {
      const events = [];
      store.state.api
        .roomsRoomOccupanciesGet({ room: this.id })
        .then(result => {
          if(this.timezone == null) {
            this.timezone = "UTC";
          }
          for (var o of result) {
            // Transform from UTC to local time
            const start = moment.utc(o.start).tz(this.timezone).format('YYYY-MM-DD HH:mm:ss');;
            const end = moment.utc(o.end).tz(this.timezone).format('YYYY-MM-DD HH:mm:ss');;
            events.push({
              name: o.userId,
              start,
              end,
              timed: true
            });
          }
          this.events = events;
        });

      this.events = events;
    }
  }
});
</script>

<style lang="scss" scoped>
</style>