# Codrul Nesupus (război asimetric, variantă originală) (`codru`)

Codrul Nesupus: război asimetric într-o pădure cu poieni și drumuri, variantă originală pentru doi, cu două facțiuni cu reguli complet diferite.

## Scop

Fii primul la 18 puncte; după 30 de ture câștigă cine are mai multe puncte.

## Pregătire

Două facțiuni atribuite la sorți (J1 = om): **Regența** (ocupantul: gărzi, clădiri Cherestea/Atelier/Cazarmă, lemn) și **Frunzișul** (insurgența: luptători, simpatie, suporteri — cărți ascunse — și baze, câte una pe fel de poiană). Harta: 8 poieni (Stejar, Ferigă, Ciupercă; 2 locuri de clădit fiecare) legate prin drumuri. Cărți: 24 (3 feluri × 7 + 3 Păsări joker). Mâna maximă la final de tură: 5 (`arunca fel`).

## Desfășurarea unei ture

**Regența**: `trag`; 3 acțiuni (`mars A>B n`, `lupta P`, `recruteaza`, `construieste c|a|k P` — costă 1 lemn, lemnul/tură = numărul de Cherestele; poiana trebuie *stăpânită*: mai multe piese ale Regenței decât ale Frunzișului); seara `fabrica P` (carte de felul poienii sau Pasăre pe fiecare Atelier: +1 p); `gata`. **Frunzișul**: `trag`; 3 + (numărul de baze) acțiuni: `mobilizeaza fel`, `propaga P`, `revolta P`, `recruteaza`, `mars`, `lupta`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `trag` — tragi o carte (forțat)
* `mars Stejari>Tufis 2` — muți 2 piese din Stejari în Tufiș
* `lupta P` — începi lupta în poiana P
* `recruteaza` — recrutezi
* `construieste c Ferigi` — construiești o Cherestea în Ferigi
* `fabrica P` — seara, Atelier
* `gata` — termini tura
* `mobilizeaza fel / propaga P / revolta P` — acțiunile Frunzișului
* `ambuscada / fara, contra / lasa` — în luptă
* `zar` — mutare forțată: zaruri
* `arunca fel` — arunci o carte

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play codru --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `trag` — tragi o carte (mutare forțată).
2. `construieste c Ferigi` — construiești o Cherestea în Ferigi (1 lemn).
3. `recruteaza` — recrutezi o gardă.

## Reguli speciale și variante alese

Construirea a n-a clădire de un fel dă [0,1,2,3] p. `propaga P`: 1 suporter de felul poienii, 2 dacă acolo sunt ≥ 3 gărzi; adiacentă simpatiei existente. `revolta P`: 2 suporteri, poiană cu simpatie și fără bază de acel fel: bază + luptători, distruge clădirile Regenței. Simpatia dă 1 p (2 p după a patra). Luptă: apărătorul poate juca o ambuscadă (carte de felul poienii sau Pasăre; atacatorul pierde 2 luptători; atacatorul o poate anula cu `contra`), apoi mutarea forțată `zar`: două zaruri, înalt/jos. Luptătorii Frunzișului apărând („gherilă”) lovesc cu zarul înalt. Apărătorul fără luptători: +1 lovitură pentru atacator. Loviturile scot întâi războinici, apoi clădiri/simpatii/baze (+1 p).

## Punctaj

Puncte din clădiri construite, simpatii, baze și lovituri; afișate `Puncte: J1 x, J2 y (la 18)`.

## Sfârșitul jocului

Primul la 18 puncte; după 30 de ture câștigă cine are mai multe puncte.

## Note despre AI și informația ascunsă

Câte un `evaluate` separat pe facțiune. Se citesc doar piesele de pe masă, propria mână și numărul de cărți al adversarului. Tragerea și zarurile sunt mutări forțate separate; AI-ul caută 1 ply. Informație ascunsă: mâna și suporterii adversarului.

## Diferențe față de regulile tradiționale

Joc original; nu se declară abateri.
