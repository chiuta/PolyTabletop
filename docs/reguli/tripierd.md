# Trei-pierde (hexagonal) (`tripierd`)

Trei-pierde: joc de aliniere pe tablă hexagonală (hexagon cu latura 5, 61 de celule). Reguli publice, implementare originală.

## Scop

Obții patru (sau mai multe) piese în linie; evită să faci exact trei în linie.

## Pregătire

Tablă hexagonală cu rândurile a–i; celulele a1..i5.

## Desfășurarea unei ture

Pe rând pui o piesă pe o celulă goală.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `e5` — litera = rândul de sus în jos (a–i), cifra = poziția în rând

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play tripierd --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `d5` — pui o piesă pe d5.
2. `d4` — pui o piesă pe d4.
3. `c4` — pui o piesă pe c4.

## Reguli speciale și variante alese

Linii drepte pe cele 3 direcții ale hexagonului.

## Punctaj

Nu există scor.

## Sfârșitul jocului

Patru (sau mai multe) în linie dreaptă câștigă; exact trei în linie (fără să fie și patru) pierde; tabla plină fără învingător = remiză.

## Note despre AI și informația ascunsă

Informație completă; alpha-beta cu adâncire iterativă cu buget de timp.

## Diferențe față de regulile tradiționale

Reguli publice; nu se declară abateri.
