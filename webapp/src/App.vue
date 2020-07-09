<template>
  <v-app>
    <v-main>
      <div v-if="credentials">
        <room-overview-link v-for="r in rooms" :key="r.id" :room="r.id" />
      </div>
      <div v-else>

        <h1>Login</h1>
        <form>
          <v-text-field v-model="user_id" label="User Name" required></v-text-field>
          <v-text-field v-model="password" label="User Name" required></v-text-field>
        </form>
      </div>
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import RoomOverviewLink from "./components/RoomOverviewLink.vue";
import {Room} from "./models/Room";

export default Vue.extend({
  components: { RoomOverviewLink },
  data() {
    return {
      credentials: null,
      rooms: [
        {
          id: "3.333",
          max_occupancy: 2
        },
        {
          id: "3.408",
          max_occupancy: 1
        }
      ]
    };
  }
});
</script>

<style lang="scss" scoped>
</style>