<template>
  <v-container fluid>
    <v-card>
      <v-toolbar color="primary" dark flat>
        <v-toolbar-title >{{room.id}}</v-toolbar-title>
      </v-toolbar>
    </v-card>
  </v-container>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  props: ["room"],
  methods: {}
});
</script>

<style lang="scss" scoped>
</style>