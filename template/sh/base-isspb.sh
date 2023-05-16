#!/bin/bash
#SBATCH -N 6
#SBATCH -n 384
#SBATCH -c 2
#SBATCH -p L16cpu
#SBATCH -o job.log
#SBATCH -e job.err
#SBATCH -J job
#SBATCH --time=24:00:00
#SBATCH --mail-type=END
#SBATCH --mail-user=MAIL
set -e
module purge

{{_cursor_}}

echo "[INFO] job done" >&1
