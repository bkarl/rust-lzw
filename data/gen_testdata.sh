#!/bin/sh

python3 gen_raw_input_data.py
compress -b 9 -f *.dat