# Biliard francez (carambol) (`biliard`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play biliard --seed 1`. Sursa implementării: `polytabletop-games/src/biliard.rs`.

> Rezumat din `polytabletop list`: 3 bile, masă fără buzunare. Lovești bila ta (albă J1, galbenă J2) cu unghi și forță (u45f2 = 45°, forța 2); dacă atinge ambele bile marchezi și mai joci. Primul la 10 puncte; fizică deterministă fără efect.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Fizică deterministă proprie, fără efect; 540 de mutări (unghi și forță).

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Fizică deterministă proprie, fără efect; 540 de mutări (unghi și forță).
