# Fanorona (Madagascar) (`fanorona`)

Fanorona (Madagascar): joc de capturi pe tablă 5×9 de puncte, 22 piese pe jucător.

## Scop

Scoate toate piesele adverse.

## Pregătire

Tablă de 5 rânduri × 9 coloane de puncte; liniile ortogonale leagă toate punctele, diagonalele leagă punctele „pare” (rând + coloană par). Tabla de pornire (rândul de mijloc X O X O . X O X O) e cea uzuală. X începe.

## Desfășurarea unei ture

O piesă se mută pe un punct vecin liber. Dacă mutarea continuă în aceeași direcție spre o piesă adversă, o capturează pe ea și toată linia continuă de piese adverse (ABORDARE, sufix `a`); dacă piesa se îndepărtează de o piesă adversă din spatele ei, o capturează la fel (RETRAGERE, sufix `w`). Captura e obligatorie dacă există; după o captură aceeași piesă poate continua (opțional) cu alte capturi.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `e2-e3` — un pas simplu (fără captură)
* `c3-d3a` — pas spre d3 cu captură prin abordare
* `c3-d3w` — pas spre d3 cu captură prin retragere
* `c3-d3a-d4w-e5a` — lanț de capturi; tot ce apare în lista `?` este o tură completă

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play fanorona --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `f2-e3w` — primul pas: f2 spre e3 cu captură prin retragere.
2. `g2-g3a` — pas g2-g3 cu captură prin abordare.
3. `f3-f2w` — pas f3-f2 cu captură prin retragere.

## Reguli speciale și variante alese

Piesa care continuă captura nu revine pe un punct deja ocupat în tura respectivă și nu repetă direcția pasului precedent.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Câștigă cine scoate toate piesele adverse. Remiză după 40 de semi-mutări fără captură (regulă proprie, simplificată).

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Regulă proprie, simplificată: remiza după 40 de semi-mutări fără captură. Regula tradițională a „alegerii între abordare și retragere” se aplică prin listarea ambelor mutări în lista `?`.
