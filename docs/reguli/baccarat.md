# Baccarat (Punto Banco) (`baccarat`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play baccarat --seed 1`. Sursa implementării: `polytabletop-games/src/baccarat.rs`.

> Rezumat din `polytabletop list`: Punto Banco: parieți pe punto, banco (comision 5%) sau egal (8:1) cu 1–3 unități (punto 2); cărțile se trag după tabelul fix. Totalul = suma mod 10; 8–9 = natural. 20 de coupe; cine are mai multe jetoane câștigă. Mutări: punto N, banco N, egal N, impart.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Doar pariurile punto/banco/egal; 20 de coupe.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Doar pariurile punto/banco/egal; 20 de coupe.
