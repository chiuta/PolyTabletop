# Patru atribute (piesa dată) (`patru`)

Patru atribute: 16 piese distincte cu patru însușiri binare, pe o tablă 4×4 (variantă de selecție comună).

## Scop

Completează un rând, o coloană sau o diagonală cu patru piese cu o însușire comună.

## Pregătire

Piesele au numele hexazecimal 0–9, a–f; bitul 0 = culoare (alb/negru), bitul 1 = înălțime (jos/înalt), bitul 2 = formă (rotund/pătrat), bitul 3 = vârf (plin/gol).

## Desfășurarea unei ture

O tură = pui pe tablă piesa pe care ți-a dat-o adversarul, apoi alegi dintre piesele rămase una pe care i-o dai lui (prima tură are doar „dă o piesă”).

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `>5` — prima tură: dai piesa 5
* `c3>7` — pui piesa primită pe c3 și îi dai adversarului piesa 7
* `c3` — mutare câștigătoare sau ultima: pui piesa pe c3, fără piesă de dat

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play patru --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `>d` — prima tură: îi dai adversarului piesa d.
2. `c3>0` — pui piesa primită pe c3 și îi dai piesa 0.
3. `b4>c` — pui piesa primită pe b4 și îi dai piesa c.

## Reguli speciale și variante alese

Cine așază o piesă care completează o linie nu mai dă piesă.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Câștigă cine completează o linie; tabla plină fără câștigător = remiză.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul folosește alpha-beta cu adâncire iterativă cu buget de timp (`AlphaBetaTime`).

## Diferențe față de regulile tradiționale

Reguli publice; implementare proprie.
