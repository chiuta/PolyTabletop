# Farul Fantomei (deducție cooperativă cu indicii vizuale) (`farul`)

Farul Fantomei: deducție cooperativă cu indicii vizuale. Tu ești mediumul (J1), AI-ul este fantoma (J2) care îți trimite viziuni de câte 3 simboluri.

## Scop

Ghicești corect, pe patru etape (portretul, camera, obiectul, scena finală), candidatul pe care îl indică viziunile.

## Pregătire

Fantoma are în mână 7 viziuni (3 simboluri); candidații sunt cărți cu câte 5 simboluri din banca de 24. Etapele 1–3 au 6 candidați; scena finală are 3 candidați, cu o singură viziune și o singură ghicire.

## Desfășurarea unei ture

Fantoma alege o viziune; tu vezi simbolurile și le potrivești cu candidații: `ghic N` (ghicești candidatul N) sau `asteapt` (aștepți următoarea viziune). O ghicire greșită scoate candidatul din joc; una corectă trece la etapa următoare. Cel mult 10 viziuni în primele trei etape. Verificarea ghicirii este mutarea forțată `verific`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `ghic 4` — ghicești candidatul 4
* `asteapt` — aștepți următoarea viziune
* `viziune 3` — (partea fantomei) alege viziunea 3
* `verific` — mutare forțată de verificare
* `sfârșit` — mutare forțată de încheiere

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play farul --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `ghic 4` — ghicești Croitoreasa (candidatul 4) după prima viziune.
2. `asteapt` — aștepți următoarea viziune.
3. `ghic 6` — ghicești candidatul 6.

## Reguli speciale și variante alese

Candidații greșiți apar marcați cu `x` în afișaj.

## Punctaj

Nu există scor numeric.

## Sfârșitul jocului

Scena finală ghicită corect = echipa câștigă („Câștigă Jucătorul 1”); orice eșec (timp epuizat, ghicire finală greșită) = REMIZĂ.

## Note despre AI și informația ascunsă

`render` arată doar ce vede mediumul (candidați, viziuni date), nu soluția. Fantoma își cunoaște legitim ținta și alege viziunea care o evidențiază cel mai bine; AI-ul medium (când joacă pentru J1) alege din viziuni și din candidați, fără soluție.

## Diferențe față de regulile tradiționale

Joc original (mecanică generică); fără abateri declarate.
