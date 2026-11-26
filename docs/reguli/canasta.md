# Canasta (2 jucători, o mână) (`canasta`)

Canasta pentru doi jucători, o singură mână, cu 108 de cărți.

## Scop

Obții scorul mai mare la sfârșitul mâinii: cărți așezate plus bonusuri minus cărțile rămase.

## Pregătire

108 de cărți (două pachete + 4 jokeri); câte 15 fiecăruia. Jokerii și 2-urile sunt wild. 3-urile roșii se pun deoparte (100 fiecare, 800 toate patru) și se înlocuiesc; 3-urile negre nu intră în combinații (doar se aruncă).

## Desfășurarea unei ture

(1) `roba` — iei 2 cărți din stoc — sau `ia` — iei toată aruncarea; (2) așezi combinații; (3) arunci o carte (`d 7h`). Cine rămâne fără cărți (cu cel puțin o canastă) a ieșit.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `roba` — iei 2 cărți din stoc
* `ia` — iei aruncarea
* `m 7` — așezi toate naturalele de 7 (minim 3) sau le adaugi la combinație
* `mw 7` — așezi 2 naturale + 1 wild
* `w K` — adaugi un wild la combinația de K
* `d 9c` — arunci o carte
* `pas` — doar când nu ai altă mutare

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play canasta --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `roba` — iei 2 cărți din stoc (singura mutare la început).
2. `mw Q` — așezi o combinație de dame cu 2 naturale și un wild.
3. `mw 9` — așezi o combinație de 9 cu 2 naturale și un wild.

## Reguli speciale și variante alese

Combinație = cel puțin 3 cărți de același rang, cu cel puțin 2 naturale, mai multe naturale decât wild-uri și cel mult 3 wild-uri. Canastă = 7+ cărți: naturală 500, mixtă 300. `ia`: cartea de sus trebuie să fie naturală (nu 3, nu wild) și se folosește imediat; aruncarea e înghețată dacă are un wild în ea sau dacă n-ai nicio combinație.

## Punctaj

Cărțile așezate + bonusuri (canaste, 3 roșii, ieșire 100) − cărțile rămase în mână.

## Sfârșitul jocului

Mâna se termină când cineva iese sau stocul e gol și nu se poate lua aruncarea. Câștigă scorul mai mare.

## Note despre AI și informația ascunsă

Informație ascunsă: stocul și mâna adversarului; AI-ul citește doar mâna proprie și informația publică (combinații, aruncare).

## Diferențe față de regulile tradiționale

Simplificări: fără minim de coborâre, o singură mână (nu până la 5000), fără 3 negre în combinații.
