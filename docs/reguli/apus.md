# Imperii în Apus (cucerire cu popoare în declin) (`apus`)

Imperii în Apus: cucerire cu popoare care ajung în declin, pe o hartă fixă de 12 ținuturi (4×3), cu trei jucători (tu + 2 roboți) și 8 ture.

## Scop

Ai cele mai multe monede după 8 ture.

## Pregătire

Șase combinații popor + dar stau în rând: prima e gratis, următoarele costă 1, 2, … monede (monedele se pun pe cele sărite și le ia cine alege combinația respectivă).

## Desfășurarea unei ture

Alegi o combinație (`alege 2`). Cucerești ținuturi (`cucereste 5`): un ținut costă 2 jetoane + 1 dacă e Munte + jetoanele ocupantului (un popor în declin are un singur jeton pe ținut); trebuie să fie vecin cu ceva al tău (prima cucerire: orice ținut de pe margine). Cine nu are destule jetoane încearcă ultima cucerire cu zarul de întărire (`zar 7`; 0, 0, 0, 1, 2, 3). `gata` termină cuceririle; `continua` trece mai departe; `declin` pune poporul în declin.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `alege 2` — alegi combinația de pe poziția 2
* `cucereste 5` — cucerești ținutul 5
* `zar 7` — încercare cu zarul de întărire pe ținutul 7
* `gata` — termini cuceririle
* `continua` — continui
* `declin` — poporul intră în declin

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play apus --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `alege 5` — alegi combinația 5 (Pitici de Peșteră Mohorâți, costă 5 monede).
2. `cucereste 9` — cucerești ținutul 9 (pădure de margine).
3. `cucereste 10` — cucerești ținutul 10 (câmpie).

## Reguli speciale și variante alese

La final de cucerire jetoanele rămase se așază automat pe ținuturile tale de la margine. Un popor care nu mai poate cuceri intră în declin (se întoarce pe față „vlăguită”) și alegi alt popor data viitoare.

## Punctaj

Fiecare tură câștigi 1 monedă pe ținut, plus bonusuri.

## Sfârșitul jocului

După 8 ture câștigă cine are mai multe monede.

## Note despre AI și informația ascunsă

Roboții joacă după reguli publice; informație completă (nu există mâini ascunse).

## Diferențe față de regulile tradiționale

Simplificări declarate: fără ținuturi cu comori, fără ordine specială de redistribuire, hartă fixă.
