# Arhipelagul în Flăcări (cucerire cu zaruri) (`arhipelag`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play arhipelag --seed 1`. Sursa implementării: `polytabletop-games/src/arhipelag.rs`.

> Rezumat din `polytabletop list`: Variantă originală: 14 insule în 4 regiuni cu bonus, 7 insule și 3 armate fiecare la start. Tura: întărire (pune Fiord / pune Fiord*), atac (ataca Fiord>Prag, apoi mutarea forțată arunc: 3 zaruri contra 2, egalitatea o câștigă apărătorul; gata atac), o mutare de armate (muta A>B n) sau gata. Câștigă cine rămâne singur; după 70 de ture, cine are mai multe insule.

## Scop

Rămâi singurul jucător cu insule; după 70 de ture câștigă cine are mai multe insule.

## Pregătire

Hartă cu 14 insule în 4 regiuni (Nord, Est, Vest, Sud) cu bonusuri. Insulele se împart la sorți câte 7, cu câte 3 armate pe insulă. Începe J1.

## Desfășurarea turei

Întărire (`pune Fiord` = 1 armată, `pune Fiord*` = toate cele rămase) → Atac (`ataca Fiord>Prag`, apoi mutarea forțată `arunc` rezolvă o rundă de zaruri; `gata atac`) → Mutare (`muta Fiord>Prag 3`, o singură dată, sau `gata`).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `pune Fiord`, `pune Fiord*`.
* `ataca Fiord>Prag`, `arunc`, `gata atac`.
* `muta Fiord>Prag 3`, `gata`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (14): `pune Fiord`, `pune Fiord*`, `pune Corn`, `pune Corn*`, `pune Prag`, `pune Prag*`, `pune Dune`, `pune Dune*`, `pune Tamar`, `pune Tamar*` …. Introduci `pune Fiord`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (14): `pune Fiord`, `pune Fiord*`, `pune Corn`, `pune Corn*`, `pune Prag`, `pune Prag*`, `pune Dune`, `pune Dune*`, `pune Tamar`, `pune Tamar*` …. Introduci `pune Dune*`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (13): `ataca Fiord>Gheata`, `ataca Corn>Gheata`, `ataca Prag>Delta`, `ataca Prag>Recif`, `ataca Dune>Stanca`, `ataca Tamar>Pesteri`, `ataca Laguna>Recif`, `ataca Laguna>Pesteri`, `ataca Laguna>Stanca`, `ataca Laguna>Vulcan` …. Introduci `gata atac`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Întăriri: max(3, insule/3) + bonusul regiunilor deținute întregi (Nord 2, Est 3, Vest 2, Sud 3).
* Zaruri: atacatorul dă min(3, armate−1) zaruri, apărătorul min(2, armate); se compară cele mai mari perechi, egalitatea o câștigă apărătorul. Insula cucerită primește atâtea armate câte zaruri a dat atacatorul în runda câștigătoare.

## Punctaj

Numărul de insule și armate (pentru decizia la limită).

## Sfârșitul jocului

Când un jucător rămâne singur; după 70 de ture câștigă cine are mai multe insule (apoi armate); egal = remiză.

## Note despre AI

Rezolvarea zarurilor e o mutare separată (`arunc`), deci AI-ul nu vede zarurile viitoare. Informație rest completă. Împărțirea insulelor este aleatoare (reproductibilă cu `--seed`).

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Variantă originală de cucerire a teritoriilor, cu hartă proprie.
