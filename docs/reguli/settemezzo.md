# Sette e mezzo (șapte și jumătate) (`settemezzo`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play settemezzo --seed 1`. Sursa implementării: `polytabletop-games/src/settemezzo.rs`.

> Rezumat din `polytabletop list`: Pachet italian de 40 de cărți: figurile (8, 9, 10) valorează ½, restul valoarea lor. Te apropii de 7½ fără să treci. Prima carte e ascunsă; carta = tragi, stau = rămâi. Ko pariază 1–3, banca câștigă la egalitate; 7½ natural (două cărți) plătește dublu. 12 runde.

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Pachet de 40 de cărți; prima carte a jucătorului e ascunsă adversarului.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Pachet de 40 de cărți; prima carte a jucătorului e ascunsă adversarului.
