#!/bin/bash
iverilog -o single_cycle_tb -g2012 rv64i_cpu.v tb_rv64i.v 2>&1
if [ $? -ne 0 ]; then
    echo "COMPILE FAILED"
    exit 1
fi
vvp single_cycle_tb 2>&1
