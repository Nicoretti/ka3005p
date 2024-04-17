#!/usr/bin/env -S python3 -u
# If your env doesn't support the -S option, you can run this script directly:
# `python3 -u ramp.py`

import sys
import argparse
import time


def ramp(opts):
    t0 = time.time()
    while True:
        t = time.time()
        d = t - t0
        p = (d % opts['period']) / opts['period']
        v = ((opts['to'] - opts['from']) * p) + opts['from']
        if d < opts['period'] or opts['loop']:
            yield 'voltage {:.2f}'.format(v)
        else:
            break


def make_parser():
    parser = argparse.ArgumentParser(
        description='Generates commands for a voltage ramp')
    parser.add_argument('-f', '--from', type=float, required=True,
                        help='voltage to start from')
    parser.add_argument('-t', '--to', type=float, required=True,
                        help='voltage to ramp to')
    parser.add_argument('-p', '--period', type=float,
                        required=True, help='duration of a period in seconds')
    parser.add_argument('-l', '--loop', action='store_true',
                        help='run again and again and ...')
    return parser


def main(args=[]):
    parser = make_parser()

    if not args:
        parser.print_usage()
        sys.exit(1)

    opts = vars(parser.parse_args(args))

    try:
        for cmd in ramp(opts):
            print(f'{cmd}')
            time.sleep(0.05)
    except KeyboardInterrupt:
        pass


if __name__ == '__main__':
    main(sys.argv[1:])
