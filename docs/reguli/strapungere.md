# Străpungere (8×8) (`strapungere`)

Străpungere (8×8): cursă de piese cu pași înainte și capturi în diagonală; fiecare are 16 piese.

## Scop

Ajungi primul pe rândul de capăt al adversarului sau captează toate piesele adverse.

## Pregătire

Fiecare jucător are 16 piese pe primele două rânduri; J1 (X) urcă, J2 (O) coboară.

## Desfășurarea unei ture

O piesă înaintează un pas drept pe un pătrat gol, sau un pas în diagonală pe un pătrat gol ori ocupat de adversar (captură); înainte nu se capturează.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `b2-b3` — pas drept
* `b2-c3` — pas diagonal pe pătrat gol
* `b2xc3` — pas diagonal cu captură

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play strapungere --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `c2-d3` — pas diagonal c2-d3.
2. `f2-g3` — pas diagonal f2-g3.
3. `g3-g4` — pas drept g3-g4.

## Reguli speciale și variante alese

Fără capturi înainte.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Câștigă cine ajunge primul pe rândul de capăt al adversarului sau cine capturează toate piesele adverse; cine nu mai are nicio mutare pierde.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Joc abstract în stil domeniu public; nu se declară abateri.
