import {
  createContractClient
} from '@stakeordie/griptape.js';

const medteeDef = {
  messages: {
    createBatch() {
      const handleMsg = {
        create_batch: {
          'batch_id': 108,
          'locations': [
            "Champaign, IL, USA: 08/01/2022 to 08/07/2022",
            "Bora Bora, Tahiti: 07/01/2022 to 07/08/2022",
          ],
          'threshold': 3,
        }
      };
      return { handleMsg };
    },
    addPatient() {
      const handleMsg = {
        add_patient: {
          'symptom_token': 27,
          'batch_id': 42,
        }
      };
      return { handleMsg };
    }
  },

  queries: {
    checkBatch() {
      return { check_batch: {'batch_id': 42} };
    },
    getCount() {
      return { get_count: {} };
    }
  }
};
export const counterContract = createContractClient({
  id: 'counter',
  //at: 'secret1vk6j69amm37zkhgqgtvjkymjeee4yhxvmmyxja',
  //at: 'secret1ua50nx20ehllr44yf87cdlt55jzucuwcu2k4tg',
  //at: 'secret1msejfwhtkxt6fwjme5lhkgvwag06a7cx7dkwy5',
  //at: 'secret13ev4jq4u6h6p6dj2wrv7ag7hyk6wj55n97lvrv',
  //at: 'secret1fw8v40cx8h5g50nvt9uzhnylkafmjgeahk8agy',
  //at: 'secret1adrj6hpac4zue405xvg9epwzqqw3t2xhj9c0fd',
  
  // without authorization constraints
  at: 'secret1r5plmxz9ncdqwjap6rmmjjtn0nzwg6083rav74',

  //at: 'secret1k2y6m2n6shcx7h3chpzmz8fn52h7un4esfymxu',
  definition: medteeDef
});
