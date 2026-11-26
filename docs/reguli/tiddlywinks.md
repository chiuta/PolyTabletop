# Tiddlywinks (pe ture) (`tiddlywinks`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tiddlywinks --seed 1`. Sursa implementării: `polytabletop-games/src/tiddlywinks.rs`.

> Rezumat din `polytabletop list`: Masă 11×7, 4 winks pe jucător, găleata în f4. Muți un wink liber cu până la 3 câmpuri (c4-d4); aterizarea pe un wink îl acoperă (îl blochează), în găleată îl scoate (3 puncte). Scor: 3 × potted + winks adverse acoperite. Se termină când cineva nu mai are winks libere sau după 80 de mutări.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Adaptare proprie pe ture: mutarea e deterministă (fără săritura cu squidger).

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Adaptare proprie pe ture: mutarea e deterministă (fără săritura cu squidger).
