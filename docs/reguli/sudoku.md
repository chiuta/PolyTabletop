# Sudoku duel (`sudoku`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sudoku --seed 1`. Sursa implementării: `polytabletop-games/src/sudoku.rs`.

> Rezumat din `polytabletop list`: Grilă cu soluție unică. Pe rând alegeți celulă și cifră fără conflict (r3c4=5): corect = +1 (+2 pentru fiecare rând/coloană/pătrat completat), greșit = −1 și cifra se elimină din acea celulă. Câștigă cine are mai multe puncte când grila e plină.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Duel pe ture (variantă proprie); soluția rămâne ascunsă și se rezolvă leneș, ca AI-ul să nu o citească.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Duel pe ture (variantă proprie); soluția rămâne ascunsă și se rezolvă leneș, ca AI-ul să nu o citească.
