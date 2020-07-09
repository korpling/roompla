<template>
    <v-card>
        <v-card-title>{{room}}</v-card-title>
    </v-card>
</template>

<script lang="ts">
import Vue from "vue";

export default Vue.extend({
  props: ["room"]
});
</script>

<style lang="scss" scoped>

</style>