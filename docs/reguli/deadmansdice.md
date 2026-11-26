# Dead Man's Dice (zarurile piratului) (`deadmansdice`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play deadmansdice --seed 1`. Sursa implementării: `polytabletop-games/src/deadmansdice.rs`.

> Rezumat din `polytabletop list`: Fiecare are 2 zaruri și zarul Jolly Roger. Aruncate, zarurile se așază într-o grămadă dacă sunt egale cu cel de sus, cu 1 mai mari sau dau 7 împreună; jefuiești zarul de sus al adversarului dacă se potrivește. Runda: suma grămezii (JR deasupra dublează, JR neasezat −7). Primul la 100. Mutări: arunc, arunc-jr, jefuiesc.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Variantă simplificată a zarurilor piratului, cu zarul Jolly Roger.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Variantă simplificată a zarurilor piratului, cu zarul Jolly Roger.
