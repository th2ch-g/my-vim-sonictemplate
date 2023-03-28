#!/bin/sh
#$ -cwd
#$ -l f_node=1
#$ -l h_rt=00:10:0
#$ -o log
#$ -e err
#$ -m abe
#$ -M {{_input_:email_address_prefix}}@yahoo.co.jp
#$ -N hoge
set -e
module purge


{{_cursor_}}


echo "hoge done" >&1
