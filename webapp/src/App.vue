<template>
  <v-app>
    <v-main>
     <room-description v-for="r in rooms" :key="r.id" :room="r.id" />
    </v-main>
  </v-app>
</template>

<script lang="ts">
import Vue from "vue";
import RoomDescription from "./components/RoomDescription.vue";

export default Vue.extend({
  components: { RoomDescription },
  data() {
    return {
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