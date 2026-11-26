# Povestitorul (cărți-simbol, 2 jucători) (`povestitor`)

Povestitorul: joc de povestit cu cărți-simbol pentru două persoane, cu 231 de descrieri-simbol.

## Scop

Ajungi primul la 12 puncte (sau mai multe puncte după 14 runde).

## Pregătire

Fiecare are 6 cărți-simbol (descrieri), fiecare cu indicii predefinite. Povestitorul alternează (J1/J2).

## Desfășurarea unei ture

Povestitorul alege o carte și unul dintre indiciile ei: `spune 3a`. Celălalt pune o momeală din mâna lui (`momeala 2`), apoi pasul forțat `amesteca` adaugă 3 cărți-fantomă din pachet și amestecă cele 5 cărți. Ghicitorul votează (`vot N`) cartea pe care crede că a spus-o povestitorul (nu pe a lui). Un al doilea votant, Publicul, votează după potrivirea cuvintelor-cheie cu indiciul (cu o doză de întâmplare), în pasul forțat `dezvaluie`, care și punctează.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `spune 3a` — povestitorul alege cartea 3 cu indiciul a (sau b)
* `momeala 2` — pui cartea 2 ca momeală
* `amesteca` — pas forțat
* `vot 3` — votezi cartea 3
* `dezvaluie` — pas forțat: dezvăluie și punctează

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play povestitor --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `spune 6a` — alegi cartea 6 cu indiciul a.
2. `momeala 3` — (când ești celălalt jucător) pui cartea 3 ca momeală.
3. `vot 3` — votezi cartea 3.

## Reguli speciale și variante alese

Cartea de vot nu poate fi cea proprie.

## Punctaj

Dacă exact unul dintre cei doi votanți găsește cartea povestitorului, povestitorul ia 3 și ghicitorul (dacă a găsit-o) 3; dacă au găsit-o amândoi sau niciunul, povestitorul ia 0 și ghicitorul 2. Ghicitorul mai ia 1 dacă Publicul a votat momeala lui.

## Sfârșitul jocului

Până la 12 puncte sau 14 runde.

## Note despre AI și informația ascunsă

AI-ul vede doar indiciul public, cărțile de pe masă și propria mână. Cărțile-fantomă și Publicul introduc elemente aleatoare.

## Diferențe față de regulile tradiționale

Variantă originală a mecanicii genului; cărțile sunt ale noastre.
