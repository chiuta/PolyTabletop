# Konane (dame hawaiiene) (`konane`)

Konane (dame hawaiiană): 8×8, piese pe toată tabla în șah, capturi prin salt ortogonal. Reguli publice, implementare originală.

## Scop

Fă ultima mutare: cine nu mai poate muta pierde.

## Pregătire

Negrul (J1) ocupă pătratele cu (fișier+rând) par (a1, c1, …), albul (J2) pe cele impare. Deschidere: J1 ridică una dintre piesele sale d4, e5, a1 sau h8; J2 ridică apoi o piesă proprie vecină ortogonal cu locul rămas gol.

## Desfășurarea unei ture

O piesă sare ortogonal peste o piesă adversă vecină într-un pătrat gol (scoțând-o); poate continua în aceeași direcție, peste 1–n piese.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `remove d4` — deschidere: ridici piesa de pe d4
* `c3-e3` — un salt
* `c3-g3` — salt dublu

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play konane --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `remove e5` — deschiderea: ridici piesa de pe e5.
2. `e7-e5` — saltul e7-e5.
3. `e5-e3` — saltul e5-e3.

## Reguli speciale și variante alese

Fără remize.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Cine nu mai poate muta pierde.

## Note despre AI și informația ascunsă

Informație completă; alpha-beta cu adâncire iterativă cu buget de timp.

## Diferențe față de regulile tradiționale

Reguli publice; nu se declară abateri.
