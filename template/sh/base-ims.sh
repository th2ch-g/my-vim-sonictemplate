#!/bin/sh
#PBS -l select=1:ncpus=64:mpiprocs=32:ompthreads=2
#PBS -l walltime=24:00:00

if [ ! -z "${PBS_O_WORKDIR}"  ]; then
      cd "${PBS_O_WORKDIR}"
fi

set -e

source /etc/profile.d/modules.sh
module purge

{{_cursor_}}

echo "[INFO] job done" >&1
