# Salina (variantă originală) (`salina`)

Salina: plasare de agenți și construire de pachet, variantă originală pentru doi, în deșertul sărat al planetei Salina.

## Scop

Fii primul la 10 PV (puncte de victorie) sau ai mai multe PV după runda 10.

## Pregătire

Resurse: Taleri (T), Sare (S), Apă (A) și PV. Pachet: 10 cărți de start; mână de 5. Fiecare carte are pictograme-agent (O oraș, D deșert, P palat), valori la revelare (persuasiune, săbii) și un efect mic când e trimisă ca agent. Locații: a Târgul(O), b Hanul(O), c Casa de sare(O; plătești 2 S → 5 T), d Fântâna(O); e Mina de sare(D, +2 S), f Oaza(D, +1 A +1 S), g Pasul(D; plătești 1 T → +2 trupe), h Drumul caravanelor(D; +1 T +1 trupă); i Sala tronului(P; +1 trupă și primul la rând), j Consiliul(P; plătești 4 T → +1 PV +1 carte), k Tezaurul(P; +2 T +1 S), l Arhiva(P; 2 cărți).

## Desfășurarea unei ture

Tura ta, o singură acțiune: `agent 3 e` — joci cartea 3 din mână și trimiți un agent (2 pe rundă) pe locația `e`, a cărei pictogramă o are cartea; la locațiile de deșert (luptă) `agent 3 e 2` desfășoară în plus 2 trupe din garnizoană în conflict; `revela` — îți arunci restul mâinii în față: suma persuasiunii (și a săbiilor) se adună și treci pe cumpărături: `compra 2` (cartea 2 din piață), `apa` (1 Apă = +1 persuasiune), `sare` (1 Sare = +1 sabie), `fin`. După revelare nu mai joci în runda asta. Mutarea forțată `runda` pregătește runda următoare.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `agent 3 e` — trimiți un agent cu cartea 3 pe locația e
* `agent 4 g 4` — la un loc de luptă, desfășori și 4 trupe
* `revela` — îți dezvălui mâna
* `compra 1` — cumperi cartea 1 din piață
* `apa / sare` — convertești Apă în persuasiune / Sare în sabie
* `fin` — termini cumpărăturile
* `runda` — mutare forțată: runda următoare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play salina --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `agent 4 g 4` — cartea 4 (Inel de sigiliu) pe locația g (Pasul), cu 4 trupe desfășurate în conflict.
2. `agent 2 h 0` — cartea 2 (Pumnal) pe locația h (Drumul caravanelor), fără trupe în plus.
3. `revela` — `revela`: treci la cumpărături.

## Reguli speciale și variante alese

Forța în conflict = 2 × trupe desfășurate + săbii (doar cu cel puțin o trupă); cel mai puternic ia premiul I, al doilea premiul II; la egalitate pe primul loc ambii iau premiul II. Trupele desfășurate se pierd.

## Punctaj

PV din premii, locații (Consiliul) etc.; la egalitate, mai multe resurse.

## Sfârșitul jocului

Când cineva are 10 PV (sau după runda 10) câștigă cine are mai multe PV.

## Note despre AI și informația ascunsă

`render` arată doar mâna J1; AI-ul nu citește mâna adversarului: evaluează suma valorii tuturor cărților deținute, deci tragerile nu-i dau informație.

## Diferențe față de regulile tradiționale

Simplificare declarată: fără facțiuni și fără cărți de intrigă.
