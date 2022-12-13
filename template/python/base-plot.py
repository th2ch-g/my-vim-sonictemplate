#!/usr/bin/env python3

import os
import sys
import subprocess as sb
import matplotlib.pyplot as plt
import seaborn as sns


sns.set(style = "darkgrid", palette = "muted", color_codes = True)
fig, ax = plt.subplots()
#fig, ax = plt.subplots(figsize=(19,13))
TITLE =
XLABEL =
YLABEL =
FIGNAME =
ax.set_title(TITLE)
ax.set_ylabel(YLABEL)
ax.set_xlabel(XLABEL)


{{_cursor_}}









plt.xticks(rotation=60)
ax.grid()
plt.grid()
fig.tight_layout()
plt.savefig(FIGNAME)
