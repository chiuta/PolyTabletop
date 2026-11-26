# Chaturanga (India) (`chaturanga`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play chaturanga --seed 1`. Sursa implementării: `polytabletop-games/src/chaturanga.rs`.

> Rezumat din `polytabletop list`: Strămoșul șahului: raja, mantri (pas diagonal), gaja (sare 2 pe diagonală), ashva (cal), ratha (car), padati (pion, promovează în mantri). Fără rocadă și pas dublu. Mat = pierdere, pat = remiză, raja rămas singur = pierdere. Mutări: e2e3, c1e3.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Fără mat prin „pat = victorie” a regulilor vechi (pat = remiză) și cu regula raja-singur ca pierdere; fără rocadă.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Fără mat prin „pat = victorie” a regulilor vechi (pat = remiză) și cu regula raja-singur ca pierdere; fără rocadă.
