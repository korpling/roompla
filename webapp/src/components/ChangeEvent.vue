<template>
  <v-container fluid>
    <v-card flat>
      <v-toolbar>
        <v-btn icon>
          <v-icon>mdi-content-save</v-icon>
        </v-btn>
        <v-btn icon>
          <v-icon>mdi-delete</v-icon>
        </v-btn>
        <v-spacer></v-spacer>
        <v-btn icon>
          <v-icon v-on:click="close">mdi-close</v-icon>
        </v-btn>
      </v-toolbar>
      <v-card-title>{{$t("edit-event")}}</v-card-title>
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
  props: ["event"],
  data() {
    return { store: store };
  },
  methods: {
    close() {
        this.$emit('event-editor-closed', event);
    }
  }
});
</script>