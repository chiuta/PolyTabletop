# Portretul Misterios (ghicește personajul) (`portret`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play portret --seed 1`. Sursa implementării: `polytabletop-games/src/portret.rs`.

> Rezumat din `polytabletop list`: Variantă originală: 24 de personaje inventate cu 14 trăsături. Întrebi (intreb ochelari), adversarul răspunde corect prin mutare forțată (raspund da/nu); ghicești (ghicesc Matei), iar adversarul dă verdictul printr-o mutare forțată (verdict corect / verdict gresit). Ghicit greșit = pierzi.

## Scop

Ghicești primul personajul secret al adversarului.

## Pregătire

24 de personaje inventate cu 14 trăsături. Fiecare jucător are un personaj secret, tras la sorți.

## Desfășurarea turei

Poți întreba o trăsătură (`intreb ochelari`) sau ghici (`ghicesc Matei`). Cel întrebat răspunde printr-o mutare forțată (`raspund da` / `raspund nu`). La ghicire, un verdict forțat arată dacă ai ghicit. Se pot pune doar întrebări care încă împart lista ta de candidați.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `intreb ochelari`.
* `raspund da`, `raspund nu`.
* `ghicesc Matei` (numele fără diacritice).
* `verdict corect` / `verdict gresit` (forțate).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (38): `intreb ochelari`, `intreb palarie`, `intreb barba`, `intreb mustata`, `intreb par-negru`, `intreb par-blond`, `intreb par-roscat`, `intreb par-alb`, `intreb barbat`, `intreb cercei` …. Introduci `intreb ochelari`; răspunsul calculatorului (în ordine): `raspund nu`; `intreb barbat`.
2. Mutări legale (1): `raspund nu`. Introduci `raspund nu`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (28): `intreb palarie`, `intreb barba`, `intreb mustata`, `intreb par-negru`, `intreb par-blond`, `intreb par-roscat`, `intreb par-alb`, `intreb barbat`, `intreb cercei`, `intreb pistrui` …. Introduci `ghicesc Mirela`; răspunsul calculatorului (în ordine): `verdict gresit`.

## Reguli speciale și variante alese

Nicio regulă specială dincolo de cele descrise mai sus.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Ghicit corect = victorie; ghicit greșit = pierzi pe loc.

## Note despre AI

Informație ascunsă: afișajul arată personajul tău, candidații rămași pentru al adversarului și ultimele întrebări. Răspunsul e mutare separată, ca AI-ul să nu-l vadă înainte de întrebare. AI-ul alege trăsătura care împarte cel mai echilibrat candidații lui și ghicește când rămâne unul.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Joc original de deducție, cu personaje inventate.
