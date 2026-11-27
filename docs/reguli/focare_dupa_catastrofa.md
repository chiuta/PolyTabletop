# Focare după Catastrofă (`focare_dupa_catastrofa`)

Categorie: Cooperativ. Echipa (Jucătorul 1) contra Jocului (Jucătorul 2, mutări forțate). Pornire: `polytabletop play focare_dupa_catastrofa --seed 1`. Sursa implementării: `polytabletop-games/src/c2_route.rs` (specificația în `c2_specs.rs`).

Joc original, variantă proprie inspirată doar de mecanica generală a genului (clean-room); nu folosește nume, texte, hărți sau cărți ale unui produs comercial.

> Variantă originală cooperativă pe graf: 14 noduri-oraș pe un inel cu legături diametrale; baza este nodul 0, nodurile multiple de 5 sunt surse. Echipa (J1: Medicul și Curierul, câte 3 acțiuni pe rând) mută (mers N), stinge focare (stinge: -1; al doilea personaj curăță tot), ridică piese din surse (ia, maxim 2) și le predă la bază (preda); din bază se poate zbura (zbor N) la un nod cu focare. J2 este Jocul: eveniment (forțat) — 5 (în creștere) focare noi; un nod cu 3 focare care primește altul izbucnește și adaugă focare vecinilor. Câștigă echipa cu 4 piese predate; Jocul câștigă la 3 izbucniri. Mutări: mers N, stinge, ia, preda, zbor N, pas.

Informație ascunsă: doar ordinea pachetelor, care nu aparține vreunui jucător (nu există informație privată).
