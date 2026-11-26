# Coloane (dame în turnuri) (`coloane`)

Coloane (dame în turnuri): tablă 7×7 pe pătratele închise, 11 piese pe jucător.

## Scop

Lasă adversarul fără mutări.

## Pregătire

Piesele nu dispar la captură, ci se adună în „coloane”; o coloană e condusă de piesa de sus.

## Desfășurarea unei ture

Soldatul merge un pas în diagonală înainte; ajuns pe ultimul rând devine ofițer (merge și ia în toate cele patru diagonale). Captura e obligatorie: sari peste o coloană adversă vecină pe un pătrat liber de dincolo și iei doar piesa ei de sus, care ajunge sub coloana ta.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `c3-d4` — pas
* `c3xe5xg3` — salturi înlănțuite (captură)

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play coloane --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `e3-f4` — pas e3-f4.
2. `d2xf4` — captură d2xf4.
3. `e3xc5` — captură e3xc5.

## Reguli speciale și variante alese

Săriturile se înlănțuie; o coloană nu poate fi sărită de două ori în aceeași tură; promovarea încheie tura.

## Punctaj

Nu există scor (afișajul arată soldații și ofițerii).

## Sfârșitul jocului

Pierde cine nu mai poate muta (nu mai conduce nicio coloană sau e blocat); remiză după 80 de semi-mutări fără captură.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Reguli publice ale jocurilor de dame în coloane; denumirile sunt originale.
