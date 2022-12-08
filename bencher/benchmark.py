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

MAX_X = 10000
MAX_T = 10000

STEP = 500

CPU = 'CPU'
GPU = 'GPU'

RAW = 'raw'
MEAN = 'mean'

N = 20
SOFT_LIMIT = 3
HARD_LIMIT = 10

def test(x, t, mode):
    line = f'{gol_path} {x} {x} {t} {mode}'
    t = time.time()
    subprocess.call(line, stdout=FNULL, stderr=FNULL, shell=False)
    return time.time() - t

def propsPrefix(x, t, mode):
    return f'{x},{t},{mode}'

def test_mean(props, fRaw, fMean):
    m = 0
    low = 200 * HARD_LIMIT
    high = 0
    pref = propsPrefix(*props)

    for i in range(N):
        t = test(*props)
        m += t
        low = min(low, t)
        high = max(high, t)
        fRaw.write(f'{pref},{t}\n')
    m /= N
    
    fMean.write(f'{pref},{m},{low},{high},{N}\n')
    return m


sizes = np.arange(MIN_X, MAX_X, STEP)
times = np.arange(MIN_T, MAX_T, STEP)

if __name__ == '__main__':
    now = datetime.now()
    timestamp = now.strftime("%m-%d-%Y_%H-%M-%S")
    fRaw = open('logs/' + timestamp + '-raw.csv', "w")
    fRaw.write(log_header_raw)
    fMean = open('logs/' + timestamp + '-mean.csv', "w")
    fMean.write(log_header_mean)

    sc = len(sizes)
    tc = len(times)
    limit_x = -1
    for i in trange(sc * tc):
        x = sizes[int(i / tc)]
        t = times[i % tc]
        if (x == limit_x):
            continue
        params = (x, t, GPU)
        mn = test_mean(params, fRaw, fMean)
        if (mn > SOFT_LIMIT):
            limit_x = x

    fRaw.close()
    fMean.close()
