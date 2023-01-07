import numpy as np
import matplotlib.pyplot as plt
import pandas as pd

# file = 'data/small-CPU-raw.csv'
file = 'data/meanGPU.csv'
file2 = 'data/mean.csv'

means_types = {
    'SIZE': np.int64,
    'DURATION': np.int64,
    'MODE': str,
    'TIME': np.float64,
    'MIN': np.float64,
    'MAX': np.float64,
    'N': np.int64
}

def plotBoundary():
    data = []
    colors = []
    with open(file, 'r') as f:
        for line in f.readlines()[1:]:
            parts = line.split(',')
            data.append([float(parts[0]), float(parts[1]), float(parts[3])])
            colors.append('blue' if float(parts[3]) < 3 else 'red')
    data = np.array(data)

    fig, axs = plt.subplots(1, 2)

    ax = axs[0]
    ax.scatter(data[:, 0], data[:, 1], c=colors, marker='x')
    ax.set_xlabel('size (side length L)')
    ax.set_ylabel('duration')
    

    ax = axs[1]
    ax.scatter(data[:, 0] * data[:, 0] * data[:, 1], data[:, 2], c=colors, marker='o')
    ax.set_xlabel('size * duration (L^2 * T)')
    ax.set_ylabel('running time')

    plt.show()


def plotError():
    # data = []
    # colors = []
    df = pd.read_csv(file2, dtype=means_types)

    fig, axs = plt.subplots(1, 2)
    # fig.suptitle('CPU simulations of GOL (mean of 10) on AMD Radeon RX 6600 XT')
    fig.suptitle('CPU simulations of GOL (mean of 10) on AMD Ryzen 5 3600')

    ax = axs[0]
    m = ax.scatter(df['SIZE'], df['DURATION'], marker='x', vmin=0, vmax=1, c=df['TIME'], cmap='jet')
    ax.set_xlabel('size (side length L)')
    ax.set_ylabel('duration')
    fig.colorbar(m, cmap='jet')
    

    ax = axs[1]
    ax.set_xlabel('duration')
    ax.set_ylabel('running time (s)')

    def plotErrorAx(n):
        dfn = df[df['SIZE'] == n]
        err = [
            dfn['TIME'] - dfn['MIN'],
            dfn['MAX'] - dfn['TIME']
        ]
        ax.errorbar(dfn['DURATION'], dfn['TIME'], yerr=err, label=f'size = {n}')
    
    # plotErrorAx(100)
    # plotErrorAx(500)
    # plotErrorAx(1000)
    # plotErrorAx(1500)
    # plotErrorAx(2000)
    plotErrorAx(10)
    plotErrorAx(40)
    plotErrorAx(50)
    plotErrorAx(60)
    plotErrorAx(100)
    plotErrorAx(30)
    plotErrorAx(80)
    
    ax.legend()

    plt.show()

def plotComp():
    df = pd.read_csv(file, dtype=means_types)
    df2 = pd.read_csv(file2, dtype=means_types)

    fig, axs = plt.subplots(1, 2)
    # fig.suptitle('CPU simulations of GOL (mean of 10) on AMD Radeon RX 6600 XT')
    fig.suptitle('Simulations of GOL CPU vs GPU (mean of 10) on AMD Ryzen 5 3600 and AMD Radeon RX 6600 XT')

    ax = axs[0]
    m = ax.scatter(df['SIZE'], df['DURATION'], marker='+', vmin=0, vmax=1, c=df['TIME'], cmap='jet', label="GPU")
    m = ax.scatter(df2['SIZE'], df2['DURATION'], marker='x', vmin=0, vmax=1, c=df2['TIME'], cmap='jet', label="CPU")
    ax.set_xlabel('size (side length L)')
    ax.set_ylabel('duration')
    ax.legend();
    fig.colorbar(m, cmap='jet')
    

    ax = axs[1]
    ax.set_xlabel('duration')
    ax.set_ylabel('running time (s)')

    def plotErrorAx(mode, n):
        dfn = None
        if (mode == 'CPU'):
            dfn = df2[df2['SIZE'] == n]
        else:
            dfn = df[df['SIZE'] == n]
        err = [
            dfn['TIME'] - dfn['MIN'],
            dfn['MAX'] - dfn['TIME']
        ]
        ax.errorbar(dfn['DURATION'], dfn['TIME'], yerr=err, label=f'size = {n} ({mode})')
    
    plotErrorAx('GPU', 100)
    # plotErrorAx('GPU', 500)
    plotErrorAx('GPU', 1000)
    # plotErrorAx('GPU', 1500)
    plotErrorAx('GPU', 2000)
    # plotErrorAx(10)
    plotErrorAx('CPU', 40)
    # plotErrorAx('CPU', 60)
    # plotErrorAx('CPU', 80)
    plotErrorAx('CPU', 100)
    # plotErrorAx(50)
    # plotErrorAx(30)
    
    ax.legend()

    plt.show()

if __name__ == '__main__':
    # plotBoundary()
    # plotError()
    plotComp()
