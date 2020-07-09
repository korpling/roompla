<template>
<v-app>
    <v-main>
        <v-container>
            <v-col>
                <v-row>
                    <room-description room="3.333"></room-description>
                </v-row>
                 <v-row>
                    <room-description room="3.408"></room-description>
                </v-row>
            </v-col>
        </v-container>
    </v-main>
</v-app>
</template>

<script lang="ts">
import Vue from "vue";
import RoomDescription from './components/RoomDescription.vue';

export default Vue.extend({
  components: {
    RoomDescription
  }
});
</script>

<style lang="scss" scoped>

</style>