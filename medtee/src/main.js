import { createApp } from 'vue';
import App from './App.vue';
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
    .mount('#app');
}

gripApp(restUrl, provider, runApp);
