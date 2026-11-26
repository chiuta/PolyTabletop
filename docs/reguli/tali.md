# Knucklebones (Tali) (`tali`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tali --seed 1`. Sursa implementării: `polytabletop-games/src/tali.rs`.

> Rezumat din `polytabletop list`: Oscioarele antice: patru astragale cu fețele 1, 3, 4, 6; Venus (toate diferite) bate orice, Canis (patru de 1) pierde în fața oricărei alte aruncări, altfel contează suma. 10 runde, 20 de jetoane fiecare. Mutări: miza 2, arunc.

## Scop

Ai mai multe jetoane după 10 runde.

## Pregătire

20 de jetoane fiecare; începătorul alternează.

## Desfășurarea turei

Începătorul spune `miza 1/2/3` și aruncă (`arunc`), celălalt aruncă; aruncarea mai bună ia miza (egal = nimic).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `miza 2`
* `arunc`

## Simplificări și note

Reguli reconstituite (sursele antice nu dau o singură formă). Aruncările sunt mutări separate; AI-ul nu le vede în avans.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
