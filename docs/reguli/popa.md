# Popa prostul (`popa`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play popa --seed 1`. Sursa implementării: `polytabletop-games/src/popa.rs`.

> Rezumat din `polytabletop list`: Old Maid românesc: 51 de cărți (fără Dama de treflă), perechile se aruncă automat. Tragi pe rând o carte din mâna adversarului, la alegere după poziție; cine rămâne cu popa (Dama nepereche) pierde. Informație ascunsă: vezi doar mâna ta. Mutări: trage 3, sfarsit.

## Scop

Nu rămâi cu popa (Dama nepereche).

## Pregătire

Pachet de 52 de cărți fără Dama de treflă. Se împart toate cărțile; fiecare aruncă perechile de aceeași valoare (automat, la început și după fiecare carte trasă).

## Desfășurarea turei

Tragi o carte din mâna adversarului, la alegere după poziție; dacă face pereche cu o carte a ta, perechea se aruncă. Cel care trage își amestecă apoi mâna. Încheierea tragerii este o mutare formală (`sfarsit`).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `trage 3` (cartea de pe poziția 3 din mâna adversarului).
* `sfarsit` (formală).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (9): `trage 1`, `trage 2`, `trage 3`, `trage 4`, `trage 5`, `trage 6`, `trage 7`, `trage 8`, `trage 9`. Introduci `trage 1`; răspunsul calculatorului (în ordine): `trage 3`.
2. Mutări legale (7): `trage 1`, `trage 2`, `trage 3`, `trage 4`, `trage 5`, `trage 6`, `trage 7`. Introduci `trage 4`; răspunsul calculatorului (în ordine): `trage 2`.
3. Mutări legale (5): `trage 1`, `trage 2`, `trage 3`, `trage 4`, `trage 5`. Introduci `trage 5`; răspunsul calculatorului (în ordine): `trage 2`.

## Reguli speciale și variante alese

* `sfarsit` e formală ca AI-ul să nu poată deduce din rezultat ce carte a tras; AI-ul alege deci poziția la întâmplare.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Cine rămâne cu ultima carte (o Damă nepereche) pierde.

## Note despre AI

Informație ascunsă: vezi doar mâna ta. Poziția aleasă nu dezvăluie nimic, deci AI-ul (adâncime 1) alege la întâmplare.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Old Maid românesc, joc tradițional din domeniul public.
