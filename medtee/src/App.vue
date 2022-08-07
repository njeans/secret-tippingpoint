<template>
  <div>
  <div class="container">
      <div class="level-item has-text-centered">
        <h1 class="title">medTEE</h1>
      </div>
      <hr>
      <br>
      <div class="level-item has-text-centered">
      <div>
          <p class="heading">Is connected?  </p>
          <p class="title">{{isConnected ? "Yes" : "No"}}</p>
      </div>
      </div>
      <br>
      <div class="level-item has-text-centered">

        <button rounded class="button is-medium is-info is-light"
          @click="connect"
          :disabled="isConnected">
          Bootstrap
        </button>
      </div>
      <br>
      <hr>
      <nav class="level is-mobile">
      <div class="level-item has-text-centered">
          <div>
            <p class="heading">Your count is: </p>
            <p class="title">{{count}}</p>
          </div>
          </div>
          <div class="level-item has-text-centered">
          <div>
            <p class="heading">Batch ID:</p>
            <p class="title"> 42</p>
          </div>
          </div>
          <div class="level-item has-text-centered">
          <div>
            <p class="heading">Data:</p>
            <p class="title"> Dummy Data </p>
          </div>
          </div>

          <div class="level-item has-text-centered">
          <div>
            <p class="heading">threshold reached?: </p>
            <p class="title"> {{threshold_reached}}</p>
          </div>
          </div>
          <div class="level-item has-text-centered">
          <div>
            <p class="heading">locations:  </p>
            <p class="title">{{locations}}</p>
          </div>
          </div>
      </nav>
      <!-- <button @click="incrementCount">{{loading ? 'Loading...' : 'Increment by 1'}}</button> -->
      <hr>
      <div class="buttons level-item has-text-centered">
        <button rounded class="button is-medium is-primary is-light" @click="createBatch">{{loading ? 'creating...' : 'Create default batch'}}</button>
        <button rounded class="button is-medium is-primary is-light" @click="addPatient">{{loading ? 'adding...' : 'Add patient'}}</button>
        <button rounded class="button is-medium is-primary is-light" @click="getCount">Get count</button>
        <button rounded class="button is-medium is-primary is-light" @click="checkBatch">Check batch</button>
      </div>
    </div>
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

    //async addPatient() {
    //  this.loading = true;
    //  await counterContract.addPatient();
    //  this.loading = false;
    //}
    async addPatient() {
      this.loading = true;
      setTimeout(async () => {
        await counterContract.addPatient();
        this.loading = false;
      }, 100);
    }
    //async incrementCount() {
    //  this.loading = true;
    //  await counterContract.incrementCount();
    //  this.loading = false;
    //}
  }
}
</script>
