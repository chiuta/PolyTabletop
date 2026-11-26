# Camelot (`camelot`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play camelot --seed 1`. Sursa implementării: `polytabletop-games/src/camelot.rs`.

> Rezumat din `polytabletop list`: Tablă 160 de pătrate cu castele. 10 pioni + 4 cavaleri. Pas în 8 direcții, canter (sari peste o piesă proprie), salt (capturezi, obligatoriu); cavalerul poate încărca (canter + salturi). Câștigi cu două piese în castelul adversarului sau prin capturare/blocare. Mutări: d6-d7, e7xe9, f6-f8xf10.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Fără excepția intrării în castel a regulilor tradiționale; încărcarea cavalerului simplificată.

## Note despre AI

Joc fără informație privată (sau cu zaruri publice); aleatorul se rezolvă prin mutări forțate, ca AI-ul să nu îl poată prevedea.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul nu are informație privată de ascuns.

## Diferențe față de regulile tradiționale

* Fără excepția intrării în castel a regulilor tradiționale; încărcarea cavalerului simplificată.
