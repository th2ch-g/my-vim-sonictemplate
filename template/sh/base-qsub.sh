#!/bin/bash
#$ -S /bin/zsh
#$ -pe smp 24
#$ -cwd
#$ -V
#$ -p
#$ -1023
#$ -j y
#$ -o log
#$ -N job
set -e


{{_cursor_}}


echo "job done" >&1
