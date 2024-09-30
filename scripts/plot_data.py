#!/usr/bin/env python3

import sys

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.ticker import FormatStrFormatter


def main():
    # parse data from a file
    try:
        data_file = sys.argv[1]
        time_ms, _, _, _, electrical_signal, _, gross_signal, _, net_signal = (
            np.genfromtxt(
                data_file,
                dtype=np.dtype(
                    [
                        ("f0", np.dtype(int)),  # time [ms]
                        ("f1", np.dtype(str), 16),  # Ok
                        ("f2", np.dtype(str), 16),
                        ("f3", np.dtype(str), 16),
                        ("f4", np.dtype(float)),  # electrical signal [mV/V]
                        ("f5", np.dtype(str), 16),
                        ("f6", np.dtype(float)),  # gross
                        ("f7", np.dtype(str), 16),
                        ("f8", np.dtype(float)),  # net
                    ]
                ),
                delimiter=",",
                unpack=True,
            )
        )
    except (IndexError, FileNotFoundError, ValueError):
        # usage
        print(
            "provide path to a data file as the first argument",
            file=sys.stderr,
        )
        sys.exit(1)
    time_hr = time_ms / 3_600_000

    _figure, axes = plt.subplots()
    plt.plot(time_hr, electrical_signal)
    plt.ylabel("Electrical Signal [mV/V]")
    plt.xlabel("Time Elapsed [hr]")
    # place the x ticks every 12 hours and at optima.
    xticks = np.concatenate(
        [
            np.arange(time_hr[0], time_hr[-1], 12),
            time_hr.take(
                [
                    -1,
                    np.argmax(electrical_signal),
                    np.argmin(electrical_signal),
                ]
            ),
        ],
    )
    xticks.sort()
    # if an optimum is within 5% of an existing tick, don't show that
    # tick, unless the tick is an endpoint, then don't use the optimum.
    xticks = np.concatenate(
        [
            xticks[np.where(np.divide(np.diff(xticks), xticks[1:]) >= 0.05)],
            time_hr.take([0, -1]),
        ]
    )

    plt.xticks(
        xticks,
        rotation=20,
        horizontalalignment="right",
    )
    axes.xaxis.set_major_formatter(FormatStrFormatter("%.1f"))
    # place the y ticks at optima and at the quartiles.
    plt.yticks(
        np.quantile(electrical_signal, [0, 0.25, 0.5, 0.75, 1]),
    )
    axes.yaxis.set_major_formatter(FormatStrFormatter("%.5f"))
    # display!
    plt.show()


if __name__ == "__main__":
    main()
