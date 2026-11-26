# Fotbal de masă pe ture (Table football) (`fotbalmasa`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play fotbalmasa --seed 1`. Sursa implementării: `polytabletop-games/src/fotbalmasa.rs`.

> Rezumat din `polytabletop list`: 8 bare, fiecare acoperă 2 din 5 culoare. O tură = muți o bară (atac 3) sau, cu mingea, trimiți pe un culoar acoperit de bara ta (trimit 2): mingea se oprește la prima bară care acoperă culoarul (pasă sau interceptare); gol dacă trece pe culoarele 2–4. Primul la 5.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare proprie pe ture a jocului de masă, nu o simulare în timp real.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Adaptare proprie pe ture a jocului de masă, nu o simulare în timp real.
