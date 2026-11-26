# Simon zice (`simon`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play simon --seed 1`. Sursa implementării: `polytabletop-games/src/simon.rs`.

> Rezumat din `polytabletop list`: Simon dă o poruncă (simon-zice sari sau ordin sari); celălalt o execută (execut sari) doar dacă a zis „Simon zice”, altfel stă (stau). Răspuns corect = punct pentru cel care ascultă, greșit = punct pentru Simon. 12 runde, rolurile alternează.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare pe ture cu 12 runde; porunca și execuția sunt mutări separate.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Adaptare pe ture cu 12 runde; porunca și execuția sunt mutări separate.
