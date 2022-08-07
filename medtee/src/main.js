import { createApp, use } from 'vue';
import App from './App.vue';
import Buefy from 'buefy'
import 'buefy/dist/buefy.css'
import Vue from "vue";


import {
  gripApp,
  getKeplrAccountProvider
} from '@stakeordie/griptape.js';

// See https://github.com/scrtlabs/api-registry for endpoint URLs
// const restUrl = 'https://api.pulsar.griptapejs.com';
const restUrl = 'https://api.pulsar.scrttestnet.com';
const provider = getKeplrAccountProvider();
function runApp() {
  createApp(App)
    .use(Buefy.Default)
    .mount('#app');
}

gripApp(restUrl, provider, runApp);
