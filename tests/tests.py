#!/bin/python
# Call this from the project root directory, not from inside tests
#

walletAddress1 = "secret1vuslf45vatgly9p5px7g276jx4y5hzmgja360m"
walletSeed1 = "salon tower stereo fun you immense wrist raven ten armed scene pond \n" #Test net seed, ok to leak
walletName1 = 'USER1'

walletAddress2 = "secret1vw3ex6qnknfp0w283tuy83w36sec8yac70c4y8"
walletSeed2 = "spoil nature marble hen purpose next slim nuclear fit fix hour divorce \n" #Test net seed, ok to leak
walletName2 = 'USER2'

walletAddress3 = "secret1k0zep9739y8j5fat3du9zvmln9msdfmdfvxjch"
walletSeed3 = "aspect pepper bridge armed bulk loyal husband mushroom rival rural mammal sure \n" #Test net seed, ok to leak
walletName3 = 'USER3'

walletAddress4 = "secret14dygjy2em5xfy8gnx6cdgvkcndvh4e79lpvjjm"
walletSeed4 = "finish science choose night develop jump keen verify ethics network drift cheap \n" #Test net seed, ok to leak
walletName4 = 'USER4'

walletAddress5 = "secret1hhexexxhvdgnw8ts7tas08luxf30v2zmftrd4f"
walletSeed5 = "sibling museum rural industry equal senior salmon evolve science parrot receive stay \n" #Test net seed, ok to leak
walletName5 = 'USER5'

NODE_ADDR = "https://rpc.pulsar.scrttestnet.com" #tcp://52.152.146.115:26657
PROJECT_PATH = "./"

import subprocess
import sys
import os
import string, random, json


def runcmd(cmd, canFail = False):
  fail = False
  try:
    cmd = f"{cmd}"
    print(cmd)
    result = subprocess.run(cmd, stdout=subprocess.PIPE, shell=True)
    if result.returncode != 0:
      print(f"UNKNOWN ERROR {result.returncode}")
      raise BaseException("Failed")
  except subprocess.CalledProcessError as e:
    result = e
    fail = True
  except BaseException as e:
    fail = True
  finally:
    print(result.stdout.decode("utf8"))
    if fail and not canFail:
      os._exit(-1)
  return fail, result.stdout.decode("utf8")


def randomHexStr(len=10):
  return "".join([random.choice(string.ascii_letters + string.digits) for n in range(len)])

def Setup():
  runcmd("secretcli config chain-id pulsar-2")
  runcmd("secretcli config node " + NODE_ADDR)
  runcmd("secretcli config output json")
  runcmd("secretcli config keyring-backend test")
  runcmd("secretcli config broadcast-mode block")
  CreateWallet(walletSeed1, walletName1)
  CreateWallet(walletSeed2, walletName2)
  CreateWallet(walletSeed3, walletName3)
  CreateWallet(walletSeed4, walletName4)
  CreateWallet(walletSeed5, walletName5)

def CreateWallet(seed, name):
  runcmd(f"secretcli keys delete {name} -y", True)
  _, data = runcmd(f"echo '{seed}' | secretcli keys add {name} --recover || exit 1")
  address = json.loads(data)['address'].strip()
  return address

def publishAndInitContract(name, /, *, params='{}', path=PROJECT_PATH, walletName=walletName1):
  os.chdir(PROJECT_PATH)
  runcmd("make build")
  _, codeId = runcmd(f"secretcli tx compute store contract.wasm.gz --from {walletName} --gas 2000000 -y")
  codeId = json.loads(codeId.strip())['logs'][0]['events'][0]['attributes'][3]['value']
  print(f"Contract stored successfully! Code ID: {codeId}")
  _, contractAddress = runcmd(f"secretcli tx compute instantiate {codeId} '{params}' --label '{name}' --from {walletName} -y")
  contractAddress = json.loads(contractAddress.strip())['logs'][0]['events'][0]['attributes'][4]['value']
  return codeId, contractAddress

def queryContract(contractAddress, functionName, arg={}):
  err, rv = runcmd(f"secretcli query compute query {contractAddress} '{{\"{functionName}\":{json.dumps(arg)}}}'", True)
  if not err:
    return json.loads(rv)
  else:
    return rv

def executeContract(contractAddress, functionName, arg={}, /, *, caller=walletName1):
  err, rv = runcmd(f"secretcli tx compute execute {contractAddress} '{{\"{functionName}\":{json.dumps(arg)}}}' --from {caller} -y", True)
  if not err:
    return json.loads(rv)
  else:
    return rv

#

# CreateBatch { batch_id: BatchId, locations: Vec<String>, threshold: u64},
# AddPatient { symptom_token: SymptomToken, batch_id: BatchId},
# AddSymptom { symptom_token: SymptomToken, batch_id: BatchId}

def testCreation():
  name = randomHexStr()
  id, addr = publishAndInitContract(name, params=f'{{"pharmacists": ["{walletAddress2}"], "manufacturers": ["{walletAddress3}"]}}')
  rv = queryContract(addr, 'check_batch', {'batch_id': 0})
  assert rv == ""
  rv = queryContract(addr, 'check_batch', {'batch_id': 1})
  assert rv == ""
  rv = executeContract(addr, 'create_batch', {'batch_id': 1, 'locations': [], 'threshold': 2}, caller=walletName3)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 0})
  assert rv == ""
  rv = queryContract(addr, 'check_batch', {'batch_id': 1})
  assert rv['threshold_reached'] == False

  rv = executeContract(addr, 'create_batch', {'batch_id': 0, 'locations': [], 'threshold': 2}, caller=walletName3)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 0})
  assert rv['threshold_reached'] == False
  rv = queryContract(addr, 'check_batch', {'batch_id': 1})
  assert rv['threshold_reached'] == False


  # Double creation:
  #
  rv = executeContract(addr, 'create_batch', {'batch_id': 0, 'locations': [], 'threshold': 2}, caller=walletName3)
  assert rv['code'] == 3

  # Non leader creation:
  #
  rv = executeContract(addr, 'create_batch', {'batch_id': 4, 'locations': [], 'threshold': 2}, caller=walletName2)
  assert rv['code'] == 3

  rv = executeContract(addr, 'create_batch', {'batch_id': 4, 'locations': [], 'threshold': 2}, caller=walletName3)
  assert rv['code'] == 0


def testToken():
  name = randomHexStr()
  id, addr = publishAndInitContract(name, params=f'{{"pharmacists": ["{walletAddress2}"], "manufacturers": ["{walletAddress3}"]}}')
  rv = executeContract(addr, 'create_batch', {'batch_id': 42, 'locations': ["Ithaca, NY, USA: 08/01/2022 to 08/07/2022"], 'threshold': 2}, caller=walletName3)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == False

  # Adding a symptom without approval token should fail:
  #
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 1, 'batch_id': 42}, caller=walletName1)
  assert rv['code'] == 3
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 1, 'batch_id': 42}, caller=walletName2)
  assert rv['code'] == 3
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 1, 'batch_id': 42}, caller=walletName3)
  assert rv['code'] == 3
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == False

  # Adding tokens and symptoms should work:
  #
  rv = executeContract(addr, 'add_patient', {'symptom_token': 1, 'batch_id': 42}, caller=walletName2)
  assert rv['code'] == 0
  rv = executeContract(addr, 'add_patient', {'symptom_token': 2, 'batch_id': 42}, caller=walletName2)
  assert rv['code'] == 0
  rv = executeContract(addr, 'add_patient', {'symptom_token': 3, 'batch_id': 42}, caller=walletName2)
  assert rv['code'] == 0
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 1, 'batch_id': 42}, caller=walletName1)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == False
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 2, 'batch_id': 42}, caller=walletName2)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == True
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 3, 'batch_id': 42}, caller=walletName3)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == True
  assert rv['locations'] == ["Ithaca, NY, USA: 08/01/2022 to 08/07/2022"]

  # OnlyPharmacyCanCreateTokens:
  #
  rv = executeContract(addr, 'add_patient', {'symptom_token': 500, 'batch_id': 42}, caller=walletName3)
  assert rv['code'] == 3
  rv = executeContract(addr, 'add_patient', {'symptom_token': 500, 'batch_id': 42}, caller=walletName2)
  assert rv['code'] == 0

  # Can't create tokens for non existing batches:
  #
  # UNDONE(1): Make sure batch exist:
  #
  # rv = executeContract(addr, 'add_patient', {'symptom_token': 404, 'batch_id': 4919}, caller=walletName2)
  # assert rv['code'] == 3

  # Tokens get consumed after being used:
  #
  rv = executeContract(addr, 'create_batch', {'batch_id': 4919, 'locations': [], 'threshold': 2}, caller=walletName3)
  assert rv['code'] == 0
  rv = executeContract(addr, 'add_patient', {'symptom_token': 408, 'batch_id': 4919}, caller=walletName2)
  assert rv['code'] == 0
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 408, 'batch_id': 4919}, caller=walletName1)
  assert rv['code'] == 0
  ## UNDONE: this is not failing
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 408, 'batch_id': 4919}, caller=walletName2)
  print("rv['code']:",rv['code'])
  assert rv['code'] == 3
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 408, 'batch_id': 4919}, caller=walletName3)
  assert rv['code'] == 3
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == True


def runDemo():
  name = randomHexStr()
  UI_AR = "secret1vp7lfaq2zzpy88d7q8m5sfed9qg0hf6rj0a5g0"
  id, addr = publishAndInitContract(name, params=f'{{"pharmacists": ["{walletAddress2}","{UI_AR}"], "manufacturers": ["{walletAddress3}","{UI_AR}"]}}')
  rv = executeContract(addr, 'create_batch', {'batch_id': 42, 'locations': ["Ithaca, NY, USA: 08/01/2022 to 08/07/2022"], 'threshold': 2}, caller=walletName3)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == False

  rv = executeContract(addr, 'add_patient', {'symptom_token': 1, 'batch_id': 42}, caller=walletName2)
  # print(json.dumps(rv, indent=2))
  assert rv['code'] == 0
  rv = executeContract(addr, 'add_patient', {'symptom_token': 2, 'batch_id': 42}, caller=walletName2)
  assert rv['code'] == 0
  rv = executeContract(addr, 'add_patient', {'symptom_token': 3, 'batch_id': 42}, caller=walletName2)
  assert rv['code'] == 0
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 1, 'batch_id': 42}, caller=walletName3)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == False
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 2, 'batch_id': 42}, caller=walletName3)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == True
  rv = executeContract(addr, 'add_symptom', {'symptom_token': 3, 'batch_id': 42}, caller=walletName3)
  assert rv['code'] == 0
  rv = queryContract(addr, 'check_batch', {'batch_id': 42})
  assert rv['threshold_reached'] == True
  assert rv['locations'] == ["Ithaca, NY, USA: 08/01/2022 to 08/07/2022"]

def testIncrement():
  name = randomHexStr()
  id, addr = publishAndInitContract(name, params='{}')
  assert executeContract(addr, 'create', {'id': 1})['code'] == 0
  assert executeContract(addr, 'create', {'id': 0})['code'] == 0
  assert queryContract(addr, 'get_count', {'id': 0})['count'] == 0
  assert queryContract(addr, 'get_count', {'id': 1})['count'] == 0
  rv = executeContract(addr, 'increment', {'id': 1})
  assert rv['code'] == 0
  assert queryContract(addr, 'get_count', {'id': 0})['count'] == 0
  assert queryContract(addr, 'get_count', {'id': 1})['count'] == 1

  rv = executeContract(addr, 'increment', {'id': 0})
  assert rv['code'] == 0
  assert queryContract(addr, 'get_count', {'id': 0})['count'] == 1
  assert queryContract(addr, 'get_count', {'id': 1})['count'] == 1

  assert executeContract(addr, 'create', {'id': 2})['code'] == 0
  assert executeContract(addr, 'create', {'id': 3})['code'] == 0
  assert queryContract(addr, 'get_count', {'id': 2})['count'] == 0
  assert queryContract(addr, 'get_count', {'id': 3})['count'] == 0
  rv = executeContract(addr, 'increment', {'id': 2})
  assert rv['code'] == 0
  rv = executeContract(addr, 'increment', {'id': 3})
  assert rv['code'] == 0

  assert queryContract(addr, 'get_count', {'id': 0})['count'] == 1
  assert queryContract(addr, 'get_count', {'id': 1})['count'] == 1
  assert queryContract(addr, 'get_count', {'id': 2})['count'] == 1
  assert queryContract(addr, 'get_count', {'id': 3})['count'] == 1


  # test multiple users:
  #
  rv = executeContract(addr, 'increment', {'id': 2}, caller=walletName2)
  assert rv['code'] == 0
  rv = executeContract(addr, 'increment', {'id': 2}, caller=walletName3)
  assert rv['code'] == 0
  assert queryContract(addr, 'get_count', {'id': 0})['count'] == 1
  assert queryContract(addr, 'get_count', {'id': 1})['count'] == 1
  assert queryContract(addr, 'get_count', {'id': 2})['count'] == 3
  assert queryContract(addr, 'get_count', {'id': 3})['count'] == 1

  rv = executeContract(addr, 'increment', {'id': 0}, caller=walletName3)
  assert rv['code'] == 0
  rv = executeContract(addr, 'increment', {'id': 3}, caller=walletName3)
  assert rv['code'] == 0
  assert queryContract(addr, 'get_count', {'id': 0})['count'] == 2
  assert queryContract(addr, 'get_count', {'id': 1})['count'] == 1
  assert queryContract(addr, 'get_count', {'id': 2})['count'] == 3
  assert queryContract(addr, 'get_count', {'id': 3})['count'] == 2

  # UNDONE(): make sure same user/token can't increment twice:
  #

  # UNDONE(): make sure token/user is valid:
  #



def testCountBehavior():
  # UNDONE(): test the behavior of getCount for when the user is the contract owner and when it is not.
  #
  pass

if __name__ == "__main__":
  Setup()
  # testCreation()
  # testToken()
  runDemo()

