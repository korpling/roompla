<template>
  <v-container fluid>
    <v-snackbar v-model="snackbar" :top="true">{{message_text}}</v-snackbar>

    <v-toolbar color="primary" dark flat>
      <v-toolbar-items>
        <v-btn icon label="Go back to room list" v-on:click="goHome">
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
        <v-btn :title="$t('previous-week')" v-on:click="previousWeek" icon>
          <v-icon>mdi-chevron-left</v-icon>
        </v-btn>
        <v-btn color="primary" v-on:click="focus=''">{{$t("calendar-week", [getCalendarWeek()])}}</v-btn>
        <v-btn :title="$t('next-week')" v-on:click="nextWeek" icon>
          <v-icon>mdi-chevron-right</v-icon>
        </v-btn>
      </v-toolbar-items>
      <v-toolbar-title>{{$t("room-name", {msg: id}) }}</v-toolbar-title>
    </v-toolbar>
    <div class="text-center ma-2">
      <v-chip label color="primary">
        <v-icon>mdi-account-multiple</v-icon>
        {{$tc("people-allowed", peopleAllowed, {count: peopleAllowed})}}
      </v-chip>
      <v-chip label color="secondary">
        <v-icon>mdi-map-clock</v-icon>
        {{timezone}}
      </v-chip>
    </div>

    <v-calendar
      ref="calendar"
      type="week"
      v-model="focus"
      :events="events"
      :first-interval="day_range.start"
      :interval-count="day_range.count"
      :locale="locale"
      :event-color="getEventColor"
      @click:event="showEvent"
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
    <v-menu
      v-model="menuEventEditor"
      :close-on-content-click="false"
      :activator="selectedElement"
      offset-x
    >
      <change-event @event-editor-closed="closeEventEditor" :selectedEvent="getSelectedEvent()"></change-event>
    </v-menu>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import ChangeEvent from "./ChangeEvent.vue";
import { i18n } from "../lang/";
import moment from "moment-timezone";
import {store} from "../store";

export default Vue.extend({
  components: { ChangeEvent },
  props: ["id"],
  computed: {
    peopleAllowed: function() {
      const room = this.store.state.rooms[this.id];
      if (room) {
        return room.maxOccupancy;
      }
    },
    timezone: function() {
      const room = this.store.state.rooms[this.id];
      if (room) {
        return room.timezone;
      }
    }
  },
  data() {
    return {
      store: null,
      locale: i18n.locale,
      events: [],
      snackbar: false,
      message_text: "",
      day_range: { start: 7, count: 13 },
      day_ranges: [
        { text: i18n.t("working-hours"), value: { start: 7, count: 13 } },
        { text: i18n.t("whole-day"), value: { start: 0, count: 24 } }
      ],
      focus: "",
      menuEventEditor: false,
      selectedEvent: null,
      selectedElement: null
    };
  },
  created: function() {
    this.store = store;
  },
  methods: {
    getEvents() {
      const start = this.day_range.start;
      const end = this.day_range.start + this.day_range.count;

      const events = [];
      this.store.state.api.roomsRoomOccupanciesGet({ room: this.id }).then(
        result => {
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
        },
        response => {}
      );

      this.events = events;
    },
    goHome() {
      this.$router.push("/");
    },
    getSelectedEvent() {
      return this.selectedEvent;
    },
    closeEventEditor() {
      this.selectedElement = null;
      this.selectedEvent = null;
      this.menuEventEditor = false;

      this.getEvents();
    },
    getCalendarWeek() {
      if (this.focus && this.focus != "") {
        return moment(this.focus).format("ww");
      } else {
        return moment().format("ww");
      }
    },
    previousWeek() {
      this.$refs.calendar.prev();
    },
    nextWeek() {
      this.$refs.calendar.next();
    },
    getEventColor(event) {
      if (event.occupancy == null) {
        return "grey";
      } else if (event.occupancy.userId === this.store.state.userId) {
        return "red";
      } else {
        return "blue";
      }
    },
    showEvent({ nativeEvent, event }) {
      const open = () => {
        this.selectedEvent = event;
        this.selectedElement = nativeEvent.target;
        setTimeout(() => (this.menuEventEditor = true), 10);
      };

      if (this.menuEventEditor) {
        this.menuEventEditor = false;
        setTimeout(open, 1);
      } else {
        open();
      }
      nativeEvent.stopPropagation();
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
      if (this.createEvent) {
        // submit event to rest API
        this.store.state.api
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
              this.getEvents();
            },
            failure => {
              if (failure.status == 409) {
                this.message_text = i18n.t("room-full");
                this.snackbar = true;
              } else {
                failure.text().then(bodyText => {
                  this.message_text = i18n.t("error-adding", [bodyText]);
                  this.snackbar = true;
                });
              }
              this.getEvents();
            }
          );
      }

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