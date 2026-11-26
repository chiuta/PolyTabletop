# Texas 42 (`texas42`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play texas42 --seed 1`. Sursa implementării: `polytabletop-games/src/texas42.rs`.

> Rezumat din `polytabletop list`: Levate cu domino pentru doi: licitație 30–42, atu, 42 de puncte; primul la 4 semne. Mutări: licitez 32, pas, atu 5, apoi piesa (ex. 6-4).

## Scop

Vezi rezumatul de mai sus: scopul și condiția de victorie sunt cele descrise acolo.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Reguli speciale și variante alese

* Simplificat: fără joc fără atu, nello, plunge, splash; doi jucători, 7 piese fiecare.

## Note despre AI

Informație ascunsă: vezi doar propriile cărți/secrete, nu și pe ale adversarului. AI-ul nu citește informația privată a adversarului; elementele aleatoare (tragere, zaruri) se rezolvă leneș, după alegerea mutării.

Stil AI în cod: vezi `ai_style` în sursă.

Modul „la masă” (`--you both`): jocul este în `HOTSEAT_SAFE`; informația privată se arată doar celui la mutare.

## Diferențe față de regulile tradiționale

* Simplificat: fără joc fără atu, nello, plunge, splash; doi jucători, 7 piese fiecare.
