# Jocul regal din Ur (`ur`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ur --seed 1`. Sursa implementării: `polytabletop-games/src/ur.rs`.

> Rezumat din `polytabletop list`: Cursă mesopotamiană, 7 piese pe jucător pe un drum de 14 pătrate; patru zaruri binare (0–4), ieșire cu număr exact, pe pătratele comune capturezi, rozeta centrală e sigură, rozetele dau o nouă aruncare. Mutări: arunc, 0-3 (de-la), pas.

## Scop

Primul care scoate toate cele 7 piese.

## Pregătire

Fiecare jucător are 7 piese în afara tablei; drum de 14 pătrate (4 private, 8 comune, 2 private), rozete pe 4, 8 și 14.

## Desfășurarea turei

Arunci (`arunc`, mutare separată: 4 zaruri tetraedrice 0/1 = 0–4), apoi muți o piesă exact atâtea pătrate; nu poți ateriza pe propria piesă; pe pătrat comun capturezi piesa adversă; pe rozetă mai arunci o dată.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `arunc`
* `0-3`
* `pas`

## Simplificări și note

Regulile reconstituite de Irving Finkel. Aruncarea e separată, deci AI-ul nu vede aruncările viitoare.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
