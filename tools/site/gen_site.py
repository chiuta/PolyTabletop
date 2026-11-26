#!/usr/bin/env python3 -I
"""Rulează în ordine generatoarele site-ului: gen_games.py (index.html), gen_pages.py (jocuri/*, sitemap, robots, sw.js).
capture.py (capturile, alt agent) se rulează ÎNAINTE. Extragerea topurilor (extract_rankings.py, cere openpyxl) e separată."""
import subprocess, sys, os
here = os.path.dirname(os.path.abspath(__file__))
for s in ('gen_games.py', 'gen_pages.py'):
    extra = sys.argv[1:] if s == 'gen_pages.py' else []
    subprocess.run([sys.executable, '-I', os.path.join(here, s)] + extra, check=True)
