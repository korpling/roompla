<template>
  <v-container fluid>
    <div v-for="r in store.state.rooms" :key="r.id">
      <v-card>
        <v-card-title>{{r.id}}</v-card-title>
        <v-card-text>{{$tc("people-allowed", r.maxOccupancy, {count: r.maxOccupancy})}}</v-card-text>
        <v-card-actions>
           <router-link :to="'/room/' + r.id">{{$t("book-time-slot")}}</router-link>
        </v-card-actions>
      </v-card>
    </div>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";
import { Configuration } from "../runtime";
import { RoomplaApi } from "../apis/RoomplaApi";
import {store} from "../store";

export default Vue.extend({
  data() {
    return {store: store};
  }
});
</script>

<style lang="scss" scoped>
</style>