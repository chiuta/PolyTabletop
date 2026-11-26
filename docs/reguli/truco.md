# Truco (Argentina) (`truco`)

Truco (Argentina / Uruguay) pentru doi jucători, cu pachet spaniol de 40 de cărți, ierarhie specială, cântări (truco, envido) și meci până la 15.

## Scop

Ajungi primul la 15 puncte câștigând mâini, prin levate, truco și envido.

## Pregătire

Pachet spaniol de 40 (1–7, 10, 11, 12; culori spadă `e`, bastos `b`, aur `o`, cupă `c`). Fiecare primește 3 cărți; „mano” (non-dealerul) conduce. Ierarhia: 1e, 1b, 7e, 7o, toți 3, toți 2, 1o și 1c, 12, 11, 10, 7b și 7c, 6, 5, 4. `reparte` împarte o mână nouă.

## Desfășurarea unei ture

Mâna se joacă în 3 runde (levate); cine câștigă 2 ia mâna. La egalitate („parda”) câștigă cine a luat prima rundă; dacă toate sunt egale, câștigă mano. Cine nu vrea să continue spune `mazo`. Truco: `truco` (2), `retruco` (3), `vale4` (4); cel provocat spune `quiero`, `noquiero` sau urcă. Envido (doar în prima rundă, înainte de orice truco): `envido` (2), `real` (3), `falta`.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `7e` — joci cartea 7 de spadă
* `mazo` — te retragi (adversarul ia miza)
* `truco / retruco / vale4` — urci miza mâinii
* `envido / real / falta` — cânți envido
* `quiero / noquiero` — accepți / refuzi
* `cuenta` — arată tantos-urile după un `quiero`
* `reparte` — împarte următoarea mână

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play truco --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `3c` — joci cartea 3 de cupă.
2. `retruco` — adversarul a cântat truco; tu urci cu `retruco` (miza 3).
3. `quiero` — accepți (`quiero`).

## Reguli speciale și variante alese

Tantos (envido): două cărți de aceeași culoare = 20 + valorile lor (figurile 10/11/12 valorează 0); altfel cea mai mare carte; câștigă cel mai mare tanto (egal: mano). Pentru `noquiero` caller-ul ia miza de dinainte ultimei urcări (minim 1). Nu avem flor. Simplificare: după un truco nu se mai poate cânta envido.

## Punctaj

Miza mâinii afișată la „Miza mâinii”; începe de la 1 punct și crește cu cântările. `falta` = punctele rămase liderului până la 15.

## Sfârșitul jocului

Primul la 15 puncte câștigă.

## Note despre AI și informația ascunsă

AI-ul (alpha-beta adâncime 1) citește doar propria mână și informațiile publice; tantosul tău e afișat doar ție. `reparte` este mutare forțată.

## Diferențe față de regulile tradiționale

Fără flor; după truco nu se mai poate cânta envido.
