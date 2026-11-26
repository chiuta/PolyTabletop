# Ship, Captain and Crew (`shipcrew`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play shipcrew --seed 1`. Sursa implementării: `polytabletop-games/src/shipcrew.rs`.

> Rezumat din `polytabletop list`: 5 zaruri, 3 aruncări pe tură: ai nevoie, în ordine, de 6 (ship), 5 (captain), 4 (crew); celelalte două zaruri sunt încărcătura (scorul turei). După o aruncare reușită: pastrez sau rearunc. 5 ture pe jucător; câștigă suma cea mai mare. Mutări: arunc, pastrez, rearunc.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Aruncarea de zaruri este mutare forțată (arunc); rearuncarea trece printr-o mutare separată.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Aruncarea de zaruri este mutare forțată (arunc); rearuncarea trece printr-o mutare separată.
