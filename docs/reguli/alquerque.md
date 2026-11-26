# Alquerque (`alquerque`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play alquerque --seed 1`. Sursa implementării: `polytabletop-games/src/alquerque.rs`.

> Rezumat din `polytabletop list`: Tablă 5×5 cu diagonale prin punctele pare, 12 piese pe jucător. Pas pe o linie; captura (salt peste o piesă adversă pe loc liber) e obligatorie și se înlănțuie. Câștigă cine captează toate piesele sau blochează adversarul. Mutări: b3-c3, a3xc3xe3.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Captura obligatorie fără regula „suflării” (piesa care nu a capturat nu se elimină).

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Captura obligatorie fără regula „suflării” (piesa care nu a capturat nu se elimină).
