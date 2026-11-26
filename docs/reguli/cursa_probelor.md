# Cursa probelor (joc de familie cu mini-probe, 2–4 jucători) (`cursa_probelor`)

Cursa probelor: joc de familie propriu (corespondentul „Campionii Distracției” din catalog, în variantă originală), cu o pistă circulară de 30 de câmpuri, zar d6 și mini-probe. Tu ești Jucătorul 1; ceilalți (1–3) sunt roboți.

## Scop

Strângi primul 5 medalii. Dacă nimeni nu ajunge la 5, cursa se încheie când toți au făcut 3 ture (90 de pași) și câștigă cine are cele mai multe medalii.

## Pregătire

La început alegi numărul de jucători: `jucatori 2`, `jucatori 3` sau `jucatori 4`. Pista are 30 de câmpuri amestecate din seed: 7 N (numărare), 6 M (memorie), 5 D (duel), 5 R (risc), 5 L (logică) și 2 O (odihnă); câmpul 0 (startul) este întotdeauna O. Randarea ASCII arată pista ca un cadran, fiecare câmp cu litera și numărul lui, iar sub el cifrele jucătorilor aflați acolo.

## Desfășurarea unei ture

Arunci zarul (`zar`), avansezi și rezolvi proba câmpului:

* **N — numărare**: un șir de 14–20 de simboluri `* # o +`; numeri câte apar dintr-un simbol; trei variante: `raspuns 1`, `raspuns 2` sau `raspuns 3`.
* **M — memorie**: se afișează o secvență de 5 litere A–D; tastezi `gata` și secvența dispare; o reproduci literă cu literă (`A`, `B`, `C`, `D`).
* **D — duel**: alegi adversarul (`duel 2`, `duel 3`...); ambii aruncă un zar, câștigă cel mai mare (egalitate = se aruncă din nou) și primește medalia.
* **R — risc**: `arunc` adaugă un zar (cel mult 3), `opresc` te oprește. Un 1 pierde pariul; dacă te oprești cu cel puțin 9 puncte primești medalia (după al treilea zar fără 1 te oprești automat).
* **L — logică**: întrebare generată procedural (calcul `a + b*c`, `a*b - c`, șir aritmetic sau geometric), trei variante, `raspuns 1|2|3`.
* **O — odihnă**: fără probă; primești „a doua șansă”: dacă ratezi următoarea probă (N, M sau L), primești o probă nouă de același fel.

După o probă tura trece la următorul jucător; roboții își joacă tura automat (mutarea forțată `robot`).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `jucatori 3` — alegi numărul de jucători (la început)
* `zar` — arunci zarul de mișcare
* `raspuns 2` — alegi varianta 2 (numărare, logică)
* `gata` / `A` `B` `C` `D` — memorie
* `duel 2` — duel cu jucătorul 2
* `arunc` / `opresc` — pariu de risc
* `rezolv` — mutare forțată (verdict, zaruri ascunse)
* `robot` — mutare forțată (tura unui robot)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Cu `polytabletop play cursa_probelor --seed 1`: `jucatori 3` (începi cu 3 jucători), apoi `zar` (aruncarea ta; proba depinde de câmpul pe care ajungi), apoi, după caz, `raspuns 1`, `gata`, `duel 2`, `arunc` sau `opresc`.

## Reguli speciale și variante alese

* Întrebările sunt generate procedural din seed; nu există bancă de întrebări.
* Verdictele și zarurile ascunse sunt mutări forțate (`rezolv`, `robot`).
* Peste linia de sosire (90 de pași) un jucător se oprește și nu mai joacă.

## Punctaj

Medaliile (5 câștigă imediat). La final, cele mai multe medalii după 3 ture.

## Sfârșitul jocului

Primul cu 5 medalii, sau, după 3 ture, cele mai multe medalii. Dacă la vârf este egalitate care îl include pe jucătorul uman = REMIZĂ; dacă omul pierde, câștigă un robot („Câștigă Jucătorul 2” = oricare robot).

## Note despre AI și informația ascunsă

Nu există informație privată: întrebările sunt afișate tuturor; răspunsul corect apare doar după verdict, iar secvența de memorie e ascunsă la fel pentru toți. Roboții nu trișează: răspund doar din ce se vede (șirul, întrebarea, variantele), cu zgomot după nivel. La Easy numără simboluri cu multe greșeli, rețin ~70% din litere, calculează greșit des și se opresc devreme la risc; la Expert numără, rețin și calculează aproape perfect, se opresc exact la țintă și la duel îl provoacă pe liderul în medalii (Hard/Expert). Nivelul se alege cu `--level`.

## Diferențe față de regulile tradiționale

Joc original (pistă cu mini-probe în stil „party de familie”); nu reproduce reguli, probe sau texte ale unui joc anume.
