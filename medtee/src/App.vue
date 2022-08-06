<template>
  <div>
    <h1>medTEE</h1>
      <p>Is connected? {{isConnected ? "Yes" : "No"}}</p>
      <button
        @click="connect"
        :disabled="isConnected">
        Bootstrap
      </button>
      <p>Your count is: {{count}}</p>
      <p>Batch ID: 42, data:</p>
      <p>threshold reached?: {{threshold_reached}}</p>
      <p>locations: {{locations}}</p>
      <!-- <button @click="incrementCount">{{loading ? 'Loading...' : 'Increment by 1'}}</button> -->
      <button @click="createBatch">{{loading ? 'creating...' : 'Create default batch'}}</button>
      <button @click="addPatient">{{loading ? 'adding...' : 'Add patient'}}</button>
      <button @click="getCount">Get count</button>
      <button @click="checkBatch">Check batch</button>
  </div>
</template>

<script>
import { counterContract } from './contracts/counter';
import { bootstrap, onAccountAvailable } from '@stakeordie/griptape.js';

export default {
  data: () => ({
    count: '',
    locations: [],
    threshold_reached: false,
    loading: false,
    isConnected: false,
    removeOnAccountAvailable:null
  }),
  mounted(){
    this.removeOnAccountAvailable = onAccountAvailable(()=>{
      this.isConnected= true;
    })
  },
  unmounted(){
    this.removeOnAccountAvailable()
  },
  methods: {
    async checkBatch() {
      console.log("checking batch ...");
      const response = await counterContract.checkBatch();
      console.log("got response: ", response);
      this.locations = response.locations;
      console.log("this.locations: ", this.locations);
      this.threshold_reached = response.threshold_reached;
      console.log("this.threshold_reached: ", this.threshold_reached);
    },
    async getCount() {
      const response = await counterContract.getCount();
      
      this.count = response.count;
    },
    async connect() {
      await bootstrap();
    },

    async createBatch() {
      this.loading = true;
      await counterContract.createBatch();
      this.loading = false;
    },

    async addPatient() {
      this.loading = true;
      await counterContract.addPatient();
      this.loading = false;
    }
    //async incrementCount() {
    //  this.loading = true;
    //  await counterContract.incrementCount();
    //  this.loading = false;
    //}
  }
}
</script>
