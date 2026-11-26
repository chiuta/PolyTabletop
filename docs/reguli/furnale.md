# Furnale (rețea de legături și industrii) (`furnale`)

Furnale: joc de rețea de legături și industrii pentru doi jucători, 8 runde, informație completă.

## Scop

Aduni mai multe puncte din plăcuțe întoarse, legături și bani.

## Pregătire

8 orașe (0–7) și 12 legături posibile; fiecare oraș are două locuri pentru industrii (`c` cărbune, `f` fier, `t` textil, `p` port). Pornești cu 12 bani.

## Desfășurarea unei ture

În fiecare rundă fiecare joacă 2 acțiuni (începe alternativ). La sfârșitul rundei primești 3 bani + venitul plăcuțelor întoarse.

## Sintaxa mutărilor

Se scrie exact textul din lista `?` (majusculele și spațiile nu contează; vezi `docs/JOCURI.md`). Mutări folosite de această implementare:

* `fabrica 3 t` — construiești o industrie de textil în orașul 3
* `link 2-4` — construiești o legătură (3 bani + 1 cărbune), lipită de rețeaua ta
* `vinde 4` — vinzi textilul din orașul 4 dacă e legat de un port
* `imprumut` — +8 bani, −2 puncte la final (cel mult 3 împrumuturi)
* `pas` — nu mai faci nimic

Tastezi `?` pentru lista mutărilor legale, `hint` pentru sugestia AI-ului, `q` pentru ieșire.

### Exemple verificate

Pornind cu `polytabletop play furnale --seed 1` (tu ești Jucătorul 1), mutările de mai jos sunt acceptate la rând, în această ordine:

1. `fabrica 5 p` — construiești un port în orașul 5.
2. `imprumut` — iei un împrumut (+8 bani).
3. `link 5-7` — construiești legătura 5-7.

## Reguli speciale și variante alese

Costuri: cărbune 3 (2 cuburi pe plăcuță), fier 4 + 1 cărbune (2 cuburi), textil 5 + 1 cărbune + 1 fier, port 3. Cărbunele vine din cea mai apropiată mină legată prin legături (indiferent de proprietar), altfel din piață (2 bani); fierul din orice fierărie cu cuburi, altfel din piață (2 bani). Prima ta acțiune e liberă. Plăcuță întoarsă: venit (cărbune +2, fier +1, textil +2, port +2) și puncte (1, 2, 4, 3).

## Punctaj

Puncte din plăcuțe întoarse + pentru fiecare legătură a ta câte un punct pe plăcuță întoarsă din cele două orașe ale ei + bani/5 − 2 pe împrumut.

## Sfârșitul jocului

După runda 8.

## Note despre AI și informația ascunsă

Informație completă, fără zaruri; AI-ul (alpha-beta adâncime 6) vede toată starea.

## Diferențe față de regulile tradiționale

Simplificări declarate: o singură eră, fără cărți (acțiunile sunt libere), fără dezvoltare de plăcuțe, piață de resurse cu preț fix (2 bani), o singură treaptă pe industrie.
