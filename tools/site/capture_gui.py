#!/usr/bin/env python3 -I
"""Capturi REALE din GUI (egui/eframe) pentru jocurile jucabile, plus clip din ele.

Rulare:  python3 -I tools/site/capture_gui.py [--only id1,id2] [--force] [--bin CALE]
              [--jobs 2] [--timeout 120] [--frames 24] [--max-moves 400] [--sim-secs 45] [--out docs/media]
Cerinte: xvfb-run, Mesa software (LIBGL_ALWAYS_SOFTWARE=1), ffmpeg, PIL.
Binarul: polytabletop-gui cu modul `--capture` (vezi polytabletop-gui/src/capture.rs).
Rezultat: docs/media/<id>/gui_shot{1,2,3}.png, gui_poster.png, gui_clip.mp4 si
docs/media/gui_manifest.json {id: {kind:"gui", frames, moves, complete,
hidden_info_shown, graphical_board}}. Idempotent; --force reface.
graphical_board=false: jocul nu are tabla desenata, GUI-ul arata panoul text.
"""
import argparse, json, os, shutil, subprocess, sys, tempfile, threading, zlib
from concurrent.futures import ThreadPoolExecutor
from PIL import Image, ImageStat

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
LOCK = threading.Lock()
FILES = ("gui_shot1.png", "gui_shot2.png", "gui_shot3.png", "gui_poster.png", "gui_clip.mp4")
FRAME_S, LAST_S = 0.4, 1.5


def ffmpeg():
    return shutil.which("ffmpeg") or "/usr/local/bin/ffmpeg"


def list_games(binp):
    out = subprocess.run([binp, "--capture-list"], capture_output=True, text=True, check=True).stdout
    res = []
    for ln in out.splitlines():
        p = ln.split()
        if len(p) == 2:
            res.append((p[0], p[1] == "1"))
    return res


def run_capture(binp, gid, tmp, args):
    seed = zlib.crc32(gid.encode()) % 100000
    cmd = ["xvfb-run", "-a", "-s", "-screen 0 1280x800x24", "env", "LIBGL_ALWAYS_SOFTWARE=1",
           "WINIT_UNIX_BACKEND=x11", binp, "--capture", gid, "--out", tmp, "--seed", str(seed),
           "--frames", str(args.frames), "--size", "960x600", "--delay-ms", "200", "--max-moves", str(args.max_moves),
           "--sim-secs", str(args.sim_secs)]
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=args.timeout)
        err = r.stderr[-300:] if r.returncode else ""
    except subprocess.TimeoutExpired:
        err = "timeout"
    meta = {}
    try:
        meta = json.load(open(os.path.join(tmp, "meta.json")))
    except Exception:
        pass
    return meta, err


def save_png(img, path):
    q = img.convert("RGB").quantize(colors=96, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE)
    q.save(path, optimize=True)


def varied(img):
    """Imagine nealterata/nevida: multe culori distincte, abatere standard nenula."""
    im = img.convert("RGB").resize((240, 150))
    cols = im.getcolors(1 << 20) or []
    sd = max(ImageStat.Stat(im).stddev)
    return len(cols) >= 12 and sd > 8


def make_clip(frames, dest, tmp):
    lst = os.path.join(tmp, "list.txt")
    with open(lst, "w") as f:
        for p in frames:
            f.write(f"file '{p}'\nduration {FRAME_S}\n")
        f.write(f"file '{frames[-1]}'\n")
    vf = f"scale=960:600:flags=lanczos,tpad=stop_mode=clone:stop_duration={LAST_S},format=yuv420p"
    cmd = [ffmpeg(), "-v", "error", "-y", "-f", "concat", "-safe", "0", "-i", lst, "-vf", vf,
           "-r", "25", "-c:v", "libx264", "-preset", "slow", "-crf", "30", "-pix_fmt", "yuv420p",
           "-movflags", "+faststart", "-an", dest]
    subprocess.run(cmd, check=True, timeout=180)


def probe(path):
    r = subprocess.run(["ffprobe", "-v", "error", "-show_entries", "format=duration",
                        "-of", "default=nw=1:nk=1", path], capture_output=True, text=True)
    return float(r.stdout.strip() or 0)


def process(gid, graphical, args):
    out = os.path.join(args.out, gid)
    tmp = tempfile.mkdtemp(prefix=f"gui_{gid}_")
    try:
        meta, err = run_capture(args.bin, gid, tmp, args)
        frames = sorted(f for f in os.listdir(tmp) if f.startswith("frame_") and f.endswith(".png"))
        if not frames:
            return gid, None, err or "nicio captura"
        paths = [os.path.join(tmp, f) for f in frames]
        n = len(paths)
        pick = [paths[0], paths[n // 2], paths[-1]]
        os.makedirs(out, exist_ok=True)
        for i, p in enumerate(pick, 1):
            save_png(Image.open(p), os.path.join(out, f"gui_shot{i}.png"))
        shutil.copyfile(os.path.join(out, "gui_shot2.png"), os.path.join(out, "gui_poster.png"))
        make_clip(paths, os.path.join(out, "gui_clip.mp4"), tmp)
        ok = all(varied(Image.open(os.path.join(out, f"gui_shot{i}.png"))) for i in (1, 2, 3))
        entry = {"kind": "gui", "frames": n, "moves": meta.get("moves", 0),
                 "complete": bool(meta.get("complete")) and ok,
                 "hidden_info_shown": bool(meta.get("hidden_info_shown")),
                 "graphical_board": graphical}
        if meta.get("outcome"):
            entry["outcome"] = meta["outcome"]
        if not ok:
            err = (err + " imagine suspect (prea uniforma)").strip()
        return gid, entry, err
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


def is_done(gid, man, out):
    return gid in man and all(os.path.exists(os.path.join(out, gid, f)) for f in FILES)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", default="")
    ap.add_argument("--force", action="store_true")
    ap.add_argument("--bin", default=os.environ.get("PTT_GUI_BIN", "/tmp/tgt_g/release/polytabletop-gui"))
    ap.add_argument("--jobs", type=int, default=2)
    ap.add_argument("--timeout", type=int, default=120)
    ap.add_argument("--frames", type=int, default=24)
    ap.add_argument("--max-moves", type=int, default=400)
    ap.add_argument("--sim-secs", type=int, default=45)
    ap.add_argument("--out", default=os.path.join(ROOT, "docs", "media"))
    args = ap.parse_args()
    args.jobs = max(1, min(args.jobs, 2))
    games = list_games(args.bin)
    if args.only:
        want = set(args.only.split(","))
        games = [g for g in games if g[0] in want]
    mpath = os.path.join(args.out, "gui_manifest.json")
    os.makedirs(args.out, exist_ok=True)
    man = json.load(open(mpath)) if os.path.exists(mpath) else {}
    todo = [g for g in games if args.force or not is_done(g[0], man, args.out)]
    print(f"{len(games)} jocuri, {len(todo)} de facut", flush=True)

    def job(g):
        try:
            return process(g[0], g[1], args)
        except Exception as e:  # noqa
            return g[0], None, repr(e)

    with ThreadPoolExecutor(args.jobs) as ex:
        for gid, entry, err in ex.map(job, todo):
            with LOCK:
                if entry:
                    man[gid] = entry
                    json.dump(dict(sorted(man.items())), open(mpath, "w"), indent=1, ensure_ascii=False)
            print(("OK  " if entry and entry["complete"] else "PROB") + f" {gid} {err}", flush=True)


if __name__ == "__main__":
    main()
