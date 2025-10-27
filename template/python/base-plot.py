#!/usr/bin/env python3

import os
import sys
import re
import subprocess as sb
import matplotlib.pyplot as plt
import seaborn as sns
import numpy as np
from matplotlib.ticker import ScalarFormatter


# seaborn theme
sns.set(style = "darkgrid", palette = "muted", color_codes = True)

# figure size
fig, ax = plt.subplots()
#fig, ax = plt.subplots(figsize=(19,13))

# setting
TITLE =
XLABEL =
YLABEL =
FIGNAME =
ax.set_title(TITLE)
ax.set_ylabel(YLABEL)
ax.set_xlabel(XLABEL)


{{_cursor_}}






# # make x-axis 10^n
# ax.set_xscale('linear')
# ax.xaxis.set_major_formatter(ScalarFormatter(useMathText=True))
# ax.xaxis.set_minor_formatter(ScalarFormatter(useMathText=True))

# # x-axis label rotate
# plt.xticks(rotation=60)

# label left upper
ax.legend(loc='upper left', bbox_to_anchor=(1, 1))

plt.grid(True)
fig.tight_layout()
plt.savefig(FIGNAME)
