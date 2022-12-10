import json
import os
import time
import subprocess
from datetime import datetime
import numpy as np
from tqdm import trange

log_header_raw = 'SIZE,DURATION,MODE,TIME\n'
log_header_mean = 'SIZE,DURATION,MODE,TIME,MIN,MAX,N\n'

gol_path = '../target/release/golmaker.exe'
FNULL = open(os.devnull, 'w')

MIN_X = 10
MIN_T = 10

MAX_X = MIN_X + 10000
MAX_T = MIN_T + 10000

STEP_X = 500
STEP_T = 500

CPU = 'CPU'
GPU = 'GPU'

RAW = 'raw'
MEAN = 'mean'

N = 10              # nr of samples 
SOFT_LIMIT = 3      # those params are big enough, don't go higher
HARD_LIMIT = 5      # that's too long, don't bother for now

# Overrides ---------

MIN_X = 15
STEP_X = 5
MAX_X = MIN_X + 45
SOFT_LIMIT = 2
HARD_LIMIT = 4
STEP_T = 200
MIN_T = 6000

# -------------------



def test(x, t, mode):
    line = f'{gol_path} {x} {x} {t} {mode}'
    try:
        t = time.time()
        subprocess.call(line, stdout=FNULL, stderr=FNULL, shell=False, timeout=HARD_LIMIT)
        return time.time() - t
    except subprocess.TimeoutExpired:
        return HARD_LIMIT
        

def propsPrefix(x, t, mode):
    return f'{x},{t},{mode}'

def test_mean(props, fRaw, fMean):
    m = 0
    low = 200 * HARD_LIMIT
    high = 0
    pref = propsPrefix(*props)

    for i in range(N):
        t = test(*props)
        if (t >= HARD_LIMIT): 
            return t
        m += t
        low = min(low, t)
        high = max(high, t)
        fRaw.write(f'{pref},{t}\n')
    m /= N
    
    fMean.write(f'{pref},{m},{low},{high},{N}\n')
    return m


sizes = np.arange(MIN_X, MAX_X, STEP_X)
times = np.arange(MIN_T, MAX_T, STEP_T)

if __name__ == '__main__':
    now = datetime.now()
    timestamp = now.strftime("%m-%d-%Y_%H-%M-%S")
    os.mkdir('logs/' + timestamp)
    fRaw = open('logs/' + timestamp + '/raw.csv', "w")
    fRaw.write(log_header_raw)
    fMean = open('logs/' + timestamp + '/mean.csv', "w")
    fMean.write(log_header_mean)

    with open('logs/' + timestamp + '/env.json', "w") as fJson:
        json.dump({
            'MIN_X': MIN_X,
            'MIN_T': MIN_T,

            'MAX_X': MAX_X,
            'MAX_T': MAX_T,

            'STEP_X': STEP_X,
            'STEP_T': STEP_T,

            'N': N,
            'SOFT_LIMIT': SOFT_LIMIT,
            'HARD_LIMIT': HARD_LIMIT 
        }, fJson, indent=4)

    sc = len(sizes)
    tc = len(times)
    limit_x = -1
    for i in trange(sc * tc):
        x = sizes[int(i / tc)]
        t = times[i % tc]
        if (x == limit_x):
            continue
        params = (x, t, CPU)
        mn = test_mean(params, fRaw, fMean)
        if (mn > SOFT_LIMIT):
            limit_x = x
            if (i % tc == 0): # first in row was too long
                break

    fRaw.close()
    fMean.close()
