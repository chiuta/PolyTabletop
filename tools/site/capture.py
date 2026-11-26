#!/usr/bin/env python3 -I
"""Genereaza media (capturi + clip) pentru paginile de prezentare ale jocurilor.

Rulare:  python3 -I tools/site/capture.py [--only id1,id2] [--force] [--bin CALE]
                                          [--jobs 2] [--timeout 60] [--out docs/media]
Doar stdlib + PIL; ffmpeg din PATH sau /usr/local/bin/ffmpeg.
Sunt capturi ale iesirii REALE din terminal (CLI), nu ale GUI-ului ("kind": "terminal").
Rezultat: docs/media/<id>/{shot1,shot2,shot3,poster}.png, clip.mp4 si docs/media/manifest.json.
Idempotent: sare peste id-urile deja gata (manifest + fisiere), cu --force le reface.
"""
import argparse, json, os, re, shutil, subprocess, sys, tempfile, threading, time
from concurrent.futures import ThreadPoolExecutor
from PIL import Image, ImageDraw, ImageFont

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf"
FONT_B = "/usr/share/fonts/truetype/dejavu/DejaVuSansMono-Bold.ttf"
W, H = 800, 450
TITLE_H, FOOT_H, PAD = 30, 28, 14
BG, FG, DIM = (20, 22, 28), (222, 226, 232), (130, 138, 150)
BAR, ACC, RED = (36, 40, 50), (125, 205, 255), (255, 105, 115)
MAX_CLIP_FRAMES, FRAME_S, LAST_S = 34, 0.3, 1.5
PLAYS = re.compile(r"^Player \d+ \(.*?\) .*?(?:plays|passes|moves)[: ].*$")
PLAYS2 = re.compile(r"^Player \d+ \((?:computer|you|human)\)")
MANIFEST_LOCK = threading.Lock()


def find_ffmpeg():
    return shutil.which("ffmpeg") or "/usr/local/bin/ffmpeg"


def playable_ids(binpath):
    out = subprocess.run([binpath, "list"], capture_output=True, text=True, check=True).stdout
    ids = []
    for ln in out.splitlines():
        if ln and not ln[0].isspace():
            ids.append(ln.split()[0])
    return ids


def hotseat_safe():
    src = open(os.path.join(ROOT, "polytabletop-games/src/lib.rs"), encoding="utf-8").read()
    m = re.search(r"pub const HOTSEAT_SAFE: &\[&str\] = &\[(.*?)\];", src, re.S)
    return set(re.findall(r'"([^"]+)"', m.group(1))) if m else set()


def run_game(binpath, gid, timeout):
    env = dict(os.environ, POLYTABLETOP_LANG="en", POLYTABLETOP_AI_MS="30")
    with tempfile.TemporaryFile() as f:
        p = subprocess.Popen([binpath, "play", gid, "--you", "none", "--seed", "1"],
                             stdout=f, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL,
                             env=env, start_new_session=True)
        complete = True
        try:
            p.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            complete = False
            try:
                os.killpg(p.pid, 9)
            except Exception:
                p.kill()
            p.wait()
        if p.returncode not in (0, None) and complete:
            complete = False
        f.seek(0)
        text = f.read().decode("utf-8", "replace")
    return text, complete, p.returncode


def split_frames(text):
    """-> (frames: list[list[str]], legends: list[str], moves)"""
    lines = text.replace("\r", "").split("\n")
    segs, legends, cur = [], [""], []
    for ln in lines:
        if PLAYS.match(ln) or PLAYS2.match(ln):
            segs.append(cur); cur = []; legends.append(ln.strip())
        else:
            cur.append(ln)
    segs.append(cur)
    # preambul: "(seed N)" + paragraful de reguli, pana la prima linie goala
    s0 = segs[0]
    if s0 and s0[0].startswith("(seed"):
        try:
            i = s0.index("", 1)
            segs[0] = s0[i + 1:]
        except ValueError:
            pass
    frames, leg = [], []
    for s, l in zip(segs, legends):
        while s and not s[0].strip(): s = s[1:]
        while s and not s[-1].strip(): s = s[:-1]
        if s:
            frames.append([x.rstrip() for x in s]); leg.append(l)
    return frames, leg, len(segs) - 1


_notdef = {}
def clean(font, s):
    s = s.replace("\t", "    ")
    out = []
    for ch in s:
        if ord(ch) < 0x2000 and ord(ch) >= 32 or ch == " ":
            out.append(ch); continue
        if ch not in _notdef:
            try:
                nd = font.getmask("￿").tobytes() == font.getmask(ch).tobytes() and ch != "￿"
            except Exception:
                nd = False
            _notdef[ch] = nd
        out.append("·" if _notdef[ch] else ch)
    return "".join(out)


def pick_size(frames):
    body_h = H - TITLE_H - FOOT_H - 2 * PAD // 2
    for size in range(17, 11, -1):
        font = ImageFont.truetype(FONT, size)
        cw = font.getlength("M"); lh = round(size * 1.28)
        cols = int((W - 2 * PAD) / cw); rows = int(body_h / lh)
        ok = sum(1 for fr in frames if len(fr) <= rows and max(len(x) for x in fr) <= cols)
        if ok >= 0.7 * len(frames):
            return size
    return 12


def render(gid, frame, legend, idx, total, size, extra=""):
    img = Image.new("RGB", (W, H), BG)
    d = ImageDraw.Draw(img)
    font = ImageFont.truetype(FONT, size); bold = ImageFont.truetype(FONT_B, 13)
    small = ImageFont.truetype(FONT, 12)
    cw = font.getlength("M"); lh = round(size * 1.28)
    body_h = H - TITLE_H - FOOT_H - PAD
    cols = int((W - 2 * PAD) / cw); rows = int(body_h / lh)
    d.rectangle([0, 0, W, TITLE_H], fill=BAR)
    for i, c in enumerate(((255, 95, 86), (255, 189, 46), (39, 201, 63))):
        d.ellipse([12 + i * 20, 9, 22 + i * 20, 19], fill=c)
    d.text((82, 7), f"polytabletop play {gid}", font=bold, fill=FG)
    tag = "terminal output · seed 1"
    d.text((W - 12 - small.getlength(tag), 9), tag, font=small, fill=DIM)
    lines = [clean(font, x) for x in frame]
    if len(lines) > rows:
        lines = lines[:rows - 3] + ["…"] + lines[-2:]
    y = TITLE_H + PAD // 2 + 2
    for ln in lines:
        if len(ln) > cols:
            ln = ln[:cols - 1] + "…"
        # rulare pe segmente: ♥♦ rosii
        x, buf, red = PAD, "", False
        def flush(buf, red, x):
            if buf:
                d.text((x, y), buf, font=font, fill=RED if red else FG)
            return x + font.getlength(buf)
        for ch in ln:
            r = ch in "♥♦♡♢"
            if r != red:
                x = flush(buf, red, x); buf = ""; red = r
            buf += ch
        flush(buf, red, x)
        y += lh
    d.rectangle([0, H - FOOT_H, W, H], fill=BAR)
    lg = legend if legend else "start of game"
    lg = clean(small, lg)
    if len(lg) > 78: lg = lg[:77] + "…"
    d.text((12, H - FOOT_H + 7), lg, font=small, fill=ACC)
    cnt = f"{idx + 1}/{total}"
    d.text((W - 12 - small.getlength(cnt), H - FOOT_H + 7), cnt, font=small, fill=DIM)
    return img


def save_png(img, path):
    q = img.quantize(colors=64, method=Image.Quantize.MEDIANCUT, dither=Image.Dither.NONE)
    q.save(path, optimize=True)


def sample(n, k):
    if n <= k:
        return list(range(n))
    return sorted({round(i * (n - 1) / (k - 1)) for i in range(k)})


def make_clip(ff, imgs, dest, tmp):
    n = len(imgs)
    seq = 0
    for i, im in enumerate(imgs):
        reps = round(FRAME_S * 10) if i < n - 1 else round(LAST_S * 10)
        for _ in range(reps):
            im.save(os.path.join(tmp, f"f{seq:05d}.png"), compress_level=1); seq += 1
    cmd = [ff, "-v", "error", "-y", "-framerate", "10", "-i", os.path.join(tmp, "f%05d.png"),
           "-an", "-c:v", "libx264", "-preset", "slow", "-crf", "27", "-pix_fmt", "yuv420p",
           "-profile:v", "high", "-g", "100", "-threads", "1", "-movflags", "+faststart",
           "-vf", f"scale={W}:{H}", dest]
    subprocess.run(cmd, check=True)
    return seq / 10.0


def entropy_ok(path):
    im = Image.open(path).convert("L")
    return len(im.getcolors(maxcolors=256) or range(256)) >= 4


def process(gid, args, safe):
    out = os.path.join(args.out, gid)
    os.makedirs(out, exist_ok=True)
    t0 = time.time()
    text, complete, rc = run_game(args.bin, gid, args.timeout)
    frames, legends, moves = split_frames(text)
    if not frames:
        raise RuntimeError(f"niciun cadru (rc={rc}): {text[:200]!r}")
    size = pick_size(frames)
    n = len(frames)
    shot_idx = [0, n // 2, n - 1] if n >= 3 else [0, min(1, n - 1), n - 1]
    if n >= 3 and shot_idx[0] == 0 and n > 4:
        shot_idx[0] = 0
    names = []
    for k, i in enumerate(shot_idx, 1):
        im = render(gid, frames[i], legends[i], i, n, size)
        p = os.path.join(out, f"shot{k}.png"); save_png(im, p); names.append(f"shot{k}.png")
    shutil.copyfile(os.path.join(out, "shot2.png"), os.path.join(out, "poster.png"))
    sel = sample(n, MAX_CLIP_FRAMES)
    imgs = [render(gid, frames[i], legends[i], i, n, size) for i in sel]
    with tempfile.TemporaryDirectory() as tmp:
        secs = make_clip(args.ffmpeg, imgs, os.path.join(out, "clip.mp4"), tmp)
    for f in names + ["poster.png"]:
        if not entropy_ok(os.path.join(out, f)):
            raise RuntimeError("imagine goala: " + f)
    entry = {"kind": "terminal", "frames": n, "shots": names, "poster": "poster.png",
             "clip": "clip.mp4", "clip_seconds": round(secs, 2), "clip_frames": len(sel),
             "moves": moves, "terminal_lines": len(text.splitlines()),
             "complete": complete, "shows_hidden": gid not in safe, "font_px": size,
             "capture": "polytabletop play %s --you none --seed 1 (POLYTABLETOP_LANG=en)" % gid}
    return entry, time.time() - t0


def load_manifest(path):
    try:
        return json.load(open(path, encoding="utf-8"))
    except Exception:
        return {}


def save_manifest(path, man):
    tmp = path + ".tmp"
    with open(tmp, "w", encoding="utf-8") as f:
        json.dump(dict(sorted(man.items())), f, ensure_ascii=False, indent=1)
    os.replace(tmp, path)


def is_done(gid, man, out):
    if gid not in man:
        return False
    return all(os.path.exists(os.path.join(out, gid, f))
               for f in ("shot1.png", "shot2.png", "shot3.png", "poster.png", "clip.mp4"))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", default="")
    ap.add_argument("--force", action="store_true")
    ap.add_argument("--bin", default=os.environ.get("POLYTABLETOP_BIN", "/tmp/tgt_main/release/polytabletop"))
    ap.add_argument("--jobs", type=int, default=2)
    ap.add_argument("--timeout", type=int, default=60)
    ap.add_argument("--out", default=os.path.join(ROOT, "docs", "media"))
    args = ap.parse_args()
    args.ffmpeg = find_ffmpeg()
    os.makedirs(args.out, exist_ok=True)
    mpath = os.path.join(args.out, "manifest.json")
    man = load_manifest(mpath)
    ids = playable_ids(args.bin)
    if args.only:
        want = [x for x in args.only.split(",") if x]
        bad = [x for x in want if x not in ids]
        if bad:
            sys.exit("id necunoscut: " + ",".join(bad))
        ids = want
    safe = hotseat_safe()
    todo = [g for g in ids if args.force or not is_done(g, man, args.out)]
    print(f"{len(ids)} jocuri, {len(todo)} de facut", flush=True)
    errors = {}

    def job(g):
        try:
            entry, dt = process(g, args, safe)
            with MANIFEST_LOCK:
                man[g] = entry; save_manifest(mpath, man)
            print(f"ok   {g:14s} frames={entry['frames']:4d} moves={entry['moves']:4d} "
                  f"complete={entry['complete']} {dt:5.1f}s", flush=True)
        except Exception as e:
            errors[g] = str(e)
            print(f"FAIL {g}: {e}", flush=True)

    with ThreadPoolExecutor(max(1, args.jobs)) as ex:
        list(ex.map(job, todo))
    print(f"gata; erori: {len(errors)}", flush=True)
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
