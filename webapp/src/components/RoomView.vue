<template>
  <v-container fluid>
    <v-snackbar v-model="snackbar" top=true>{{message_text}}</v-snackbar>

    <v-toolbar color="primary" dark flat>
      <v-toolbar-items>
        <v-btn icon class="hidden-xs-only" v-on:click="goHome">
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
      <v-toolbar-title>{{$t("room-name", {msg: id}) }} - {{$tc("people-allowed", peopleAllowed, {count: peopleAllowed})}}</v-toolbar-title>
      
    </v-toolbar>

    <v-calendar
      type="week"
      :events="events"
      :first-interval="day_range.start"
      :interval-count="day_range.count"
      :locale="locale"
      :event-color="getEventColor"
      @change="getEvents"
      @mousedown:event="startDrag"
      @mousedown:time="startTime"
      @mousemove:time="mouseMove"
      @mouseup:time="endDrag"
      @mouseleave.native="cancelDrag"
    >
      <template #event="{ event, timed, eventSummary }">
        <div class="v-event-draggable" v-html="eventSummary()"></div>
        <div v-if="timed" class="v-event-drag-bottom" @mousedown.stop="extendBottom(event)"></div>
      </template>
    </v-calendar>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import { i18n } from "../lang/";
import { store } from "../store";
import moment from "moment-timezone";


export default Vue.extend({
  props: ["id"],
  computed: {
    peopleAllowed: function() {
      return store.state.rooms[this.id].maxOccupancy;
    },
    timezone: function() {
      return store.state.rooms[this.id].timezone;
    }
  },
  data() {
    return {
      locale: i18n.locale,
      events: [],
      snackbar: false,
      message_text: "",
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
          if (this.timezone == null) {
            this.timezone = "UTC";
          }
          for (var o of result) {
            // Transform from UTC to local time
            const start = moment
              .utc(o.start)
              .tz(this.timezone)
              .format("YYYY-MM-DD HH:mm:ss");
            const end = moment
              .utc(o.end)
              .tz(this.timezone)
              .format("YYYY-MM-DD HH:mm:ss");
            events.push({
              name: o.userId,
              start,
              end,
              timed: true,
              occupancy: o
            });
          }
          this.events = events;
        });

      this.events = events;
    },
    goHome() {
      this.$router.push("/");
    },
    getEventColor(event) {
      if (event.occupancy == null) {
        return "grey";
      } else if (event.occupancy.userId === store.state.userId) {
        return "red";
      } else {
        return "blue";
      }
    },
    startDrag({ event, timed }) {
      if (event && timed) {
        this.dragEvent = event;
        this.dragTime = null;
        this.extendOriginal = null;
      }
    },
    startTime(tms) {
      const mouse = this.toTime(tms);

      if (this.dragEvent && this.dragTime === null) {
        const start = this.dragEvent.start;

        this.dragTime = mouse - start;
      } else {
        this.createStart = this.roundTime(mouse);
        this.createEvent = {
          name: `Event #${this.events.length}`,
          start: this.createStart,
          end: this.createStart,
          timed: true
        };

        this.events.push(this.createEvent);
      }
    },
    extendBottom(event) {
      this.createEvent = event;
      this.createStart = event.start;
      this.extendOriginal = event.end;
    },
    mouseMove(tms) {
      const mouse = this.toTime(tms);

      if (this.dragEvent && this.dragTime !== null) {
        const start = this.dragEvent.start;
        const end = this.dragEvent.end;
        const duration = end - start;
        const newStartTime = mouse - this.dragTime;
        const newStart = this.roundTime(newStartTime);
        const newEnd = newStart + duration;

        this.dragEvent.start = newStart;
        this.dragEvent.end = newEnd;
      } else if (this.createEvent && this.createStart !== null) {
        const mouseRounded = this.roundTime(mouse, false);
        const min = Math.min(mouseRounded, this.createStart);
        const max = Math.max(mouseRounded, this.createStart);

        this.createEvent.start = min;
        this.createEvent.end = max;
      }
    },
    endDrag() {
      // submit event to rest API
      store.state.api
        .roomsRoomOccupanciesPut({
          room: this.id,
          timeRange: {
            start: moment.utc(this.createEvent.start).toISOString(),
            end: moment.utc(this.createEvent.end).toISOString()
          }
        })
        .then(
          result => {
            this.message_text = i18n.t("added-entry");
            this.snackbar = true;
            this.getEvents(
              this.day_range.start,
              this.day_range.start + this.day_range.count
            );
          },
          failure => {
            failure.text().then(bodyText => {
              this.message_text = i18n.t("error-adding", [bodyText]);
              this.snackbar = true;
              this.getEvents(
                this.day_range.start,
                this.day_range.start + this.day_range.count
              );
            });
          }
        );

      this.dragTime = null;
      this.dragEvent = null;
      this.createEvent = null;
      this.createStart = null;
      this.extendOriginal = null;
    },
    cancelDrag() {
      if (this.createEvent) {
        if (this.extendOriginal) {
          this.createEvent.end = this.extendOriginal;
        } else {
          const i = this.events.indexOf(this.createEvent);
          if (i !== -1) {
            this.events.splice(i, 1);
          }
        }
      }

      this.createEvent = null;
      this.createStart = null;
      this.dragTime = null;
      this.dragEvent = null;
    },
    roundTime(time, down = true) {
      const roundTo = 60; // minutes
      const roundDownTime = roundTo * 60 * 1000;

      return down
        ? time - (time % roundDownTime)
        : time + (roundDownTime - (time % roundDownTime));
    },
    toTime(tms) {
      return new Date(
        tms.year,
        tms.month - 1,
        tms.day,
        tms.hour,
        tms.minute
      ).getTime();
    }
  }
});
</script>

<style lang="scss" scoped>
</style>