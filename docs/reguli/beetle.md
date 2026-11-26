# Gândacul (Beetle) (`beetle`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play beetle --seed 1`. Sursa implementării: `polytabletop-games/src/beetle.rs`.

> Rezumat din `polytabletop list`: Arunci pe rând un zar: 1 corp, 2 cap, 3 picior (6), 4 antenă (2), 5 ochi (2), 6 coadă. Corpul primul; capul, picioarele și coada cer corpul; antenele și ochii cer capul. Primul gândac complet (13 piese) câștigă. Mutare: arunc.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Fără limită de runde: primul gândac complet câștigă.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Fără limită de runde: primul gândac complet câștigă.
