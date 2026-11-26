# Vulpea și gâștele (Fox and Geese) (`foxgeese`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play foxgeese --seed 1`. Sursa implementării: `polytabletop-games/src/foxgeese.rs`.

> Rezumat din `polytabletop list`: Tablă în cruce cu 33 de găuri. J1 = vulpea (d4), J2 = 13 gâște. Vulpea mută ortogonal sau sare peste o gâscă (d4xd6xf6); gâștele mută doar înainte/lateral. Gâștele câștigă blocând vulpea; vulpea câștigă dacă rămân sub 6 gâște. Mutări: d4-d5, d4xd6.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Gâștele nu au regula de captură obligatorie a vulpii; vulpea nu e obligată să sară.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Gâștele nu au regula de captură obligatorie a vulpii; vulpea nu e obligată să sară.
