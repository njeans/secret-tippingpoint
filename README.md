# MedTrace
This project was completed for IC3 2022 Camp Hackathon (2nd Place)

## Exaplanation Slides
[slides](https://docs.google.com/presentation/d/1wSdZ-LFQ6cE-TV7JmsYidm7367ONyk6vjcO6R16wLYw/edit?usp=sharing)

## Abstract
We aim to create a private and traceable application that can be used for patients to track and be notified of contamination of medical products such as medicine, vaccines, medical devices, etc. Current methods are not user friendly and are not effective at notifying all users who may be effected. Our solution uses the secure enclave backed blockchain platform [Secret Network](https://scrt.network) to implement a private smart contract that can be used by manufacturers, doctors, pharmacists, patients, regulators, and in neccesary cases the public. We also aim to protect manufacturer interests by only revealing adverse affects that have met the predetermined threshold.

## Architecture
* First our system is initialized with a list of known manufacturers and pharmacists who will be creating and dispensing the perscribed drugs or other products. [init](https://github.com/njeans/secret-tippingpoint/blob/master/src/contract.rs#L25)
* As the products are created through the supply chain the manufacturers will update the given batch with relevant information using [`create_batch`](https://github.com/njeans/secret-tippingpoint/blob/master/src/contract.rs#L58) function 
* Once a medicine is prescribed the pharmacist will provide the patient with a `symptom_token` unique to the patient and batch number. They will also call the [`add_patient`](https://github.com/njeans/secret-tippingpoint/blob/master/src/contract.rs#L96) method which will privately record this information.
* If a patient has a symptom caused by the product they will go to a doctor which will use the unique `symptom_token` to record the adverse affect privately in the blockchain using [`add_symptom`](https://github.com/njeans/secret-tippingpoint/blob/master/src/contract.rs#L96).
* If the predetermined threshold of symptoms is reached this information will become public to regulators and the general population. Regulators will be able to track the potential batch using the manufacturer locations and potentially cross reference with other products that could be contaminated.

<img width="1046" alt="Screen Shot 2022-09-09 at 9 25 22 AM" src="https://user-images.githubusercontent.com/12213974/189360381-4729e8d8-13c4-4d5d-b69c-f3b94a775ada.png">

## Demo 

```
cd test
export PATH=$PATH:${PWD}
cd ..
python3 tests/tests.py
```
