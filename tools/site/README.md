# Site de prezentare (docs/)

Ordinea de generare:

1. `capture.py` (alt agent): capturi și clipuri în `docs/media/<id>/` (+ `manifest.json`, `gui_manifest.json`).
1b. `python3 -I tools/data/build_cat100.py` (idempotent) îmbină cercetarea din `tools/data/cat100/*.json` (+ `extra.json`, înlocuitori pentru locurile eliberate de dublurile dintre categorii) în `tools/data/data_cat100.py`; apoi `python3 -I tools/gen_catalog.py` + `cargo fmt -p polytabletop-games` regenerează `catalog.rs` (12 categorii × 100 de intrări). URL-urile `sources` din date rămân doar în fișierele de date, nu apar pe site.
2. `python3 -I tools/site/build_continent_sheets.py [fișier.xlsx]` (opțional) scrie în tabel foile Africa, Asia, America de Nord, America de Sud, Oceania din `tools/data/data_cont.py`, apoi `python3 -I tools/site/build_category_sheets.py [fișier.xlsx]` (foile „Top <categorie>” din catalog + foaia Note), apoi `python3 -I tools/site/extract_rankings.py [fișier.xlsx]` — doar când se schimbă tabelul de topuri (9 topuri a câte 100 pe regiuni + 12 topuri pe categorii, câte unul pe categorie, de exact 100 de intrări distincte, în ordinea din `data_cat100.py`; cere `openpyxl`); scrie `data/rankings.json` și mapează fiecare intrare la catalogul PolyTabletop (status: `play`, `var`, `kit`, `brand`, `plan`, + id-ul jocului).
3. `python3 -I tools/site/gen_games.py` — `docs/index.html` din `template.html` (+ `rank.js`, `data/rankings.json`, `data/origins.json`, `ui_i18n.py`).
4. `python3 -I tools/site/gen_pages.py` — `docs/jocuri/<id>.html` (171), `docs/jocuri/index.html`, `sitemap.xml`, `robots.txt`, `sw.js`, din `page_template.html`. Secțiunea media a fiecărei pagini apare doar dacă fișierele există la rulare (`gui_shot1-3.png`, `gui_clip.mp4`, `gui_poster.png`, `shot1-3.png`, `clip.mp4`, `poster.png`): regenerați după ce apar capturile. `--docs DIR` folosește alt director (teste).

`python3 -I tools/site/gen_site.py` rulează pașii 3 și 4. Verificări: `python3 -I tools/site/check_site.py` și `node tools/site/test_i18n.js`.

Traducerile de interfață sunt în `ui_i18n.py` (8 limbi), iar originile și observațiile din clasamente în `data/origins.json`; nu au fost revizuite de vorbitori nativi.
