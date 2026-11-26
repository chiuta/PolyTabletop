# Jocul de 15 (cursă) (`joc15`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play joc15 --seed 1`. Sursa implementării: `polytabletop-games/src/joc15.rs`.

> Rezumat din `polytabletop list`: Fiecare jucător are propria tablă 4×4, amestecată identic. Pe rând, mutați o piesă lângă locul gol pe tabla voastră (ex. 7); câștigă primul care aduce piesele în ordinea 1–15. După 300 de mutări câștigă cel mai apropiat de ordine.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Cursă între două table identice; după 300 de mutări decide apropierea de ordine.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Cursă între două table identice; după 300 de mutări decide apropierea de ordine.
