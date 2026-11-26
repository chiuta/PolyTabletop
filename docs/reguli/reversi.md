# Reversi (`reversi`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play reversi --seed 1`. Sursa implementării: `polytabletop-games/src/reversi.rs`.

> Rezumat din `polytabletop list`: Pui o piesă care încadrează în linie dreaptă piese ale adversarului și le întoarce; negrul (J1) începe; câștigă cine are mai multe piese la final. Mutări: d3 (pătratul ales), pass (doar când nu ai nicio mutare).

## Scop

Ai la final mai multe piese de culoarea ta decât adversarul.

## Pregătire

Tablă 8×8; patru piese în centru (două negre, două albe, în diagonală). Negrul (Jucătorul 1) începe.

## Desfășurarea turei

Pui o piesă pe un pătrat gol, astfel încât să încadreze în linie dreaptă (orizontal, vertical sau diagonal) una sau mai multe piese ale adversarului între piesa nouă și o piesă a ta; piesele încadrate se întorc.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Pătratul ales: `d3` (coloana a–h, rândul 1–8).
* `pass`: permis doar când nu ai nicio mutare legală.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (4): `d3`, `c4`, `f5`, `e6`. Introduci `d3`; răspunsul calculatorului (în ordine): `e3`.
2. Mutări legale (5): `f2`, `f3`, `f4`, `f5`, `f6`. Introduci `f4`; răspunsul calculatorului (în ordine): `c3`.
3. Mutări legale (7): `c2`, `d2`, `e2`, `c4`, `f5`, `d6`, `e6`. Introduci `e6`; răspunsul calculatorului (în ordine): `g5`.

Notă: AI-ul acestui joc folosește un buget de timp sau simulări; răspunsurile calculatorului pot diferi de la o rulare la alta, chiar cu același `--seed`. Mutările legale și efectul mutărilor tale sunt deterministe.

## Reguli speciale și variante alese

* Mutarea trebuie să întoarcă cel puțin o piesă; dacă nu poți, treci (`pass`).

## Punctaj

Numărul de piese de pe tablă la final.

## Sfârșitul jocului

Când niciun jucător nu mai poate pune o piesă (tabla plină sau blocaj). Câștigă cine are mai multe piese; egalitate = remiză.

## Note despre AI

Informație completă. Alpha-beta cu buget de timp (500 ms, adâncime maximă 14).

Stil AI în cod: alpha-beta cu adâncire iterativă și buget de timp (500 ms, adâncime maximă 14).

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar starea inițială nu este aleatoare și CLI-ul nu afișează avertisment la `--you both`.

## Diferențe față de regulile tradiționale

* Jocul se numește în comerț cu o marcă înregistrată; regulile sunt cele clasice.
