#!/bin/sh
#$ -cwd
#$ -l f_node=1
#$ -l h_rt=00:10:0
#$ -o job.log
#$ -e job.err
#$ -m abe
#$ -M MAIL
#$ -N job
set -e
. /etc/profile.d/modules.sh
module purge


{{_cursor_}}


echo "job done" >&1
