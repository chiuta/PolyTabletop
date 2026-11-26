# Twenty-Nine (29) (`twentynine`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play twentynine --seed 1`. Sursa implementării: `polytabletop-games/src/twentynine.rs`.

> Rezumat din `polytabletop list`: Joc sud-asiatic de levate cu licitație, adaptat pentru doi: 32 de cărți, 8 fiecare; valet 3, nouă 2, as 1, zece 1 puncte. Licitezi 10–20 de puncte, câștigătorul alege atuul și celălalt conduce; răspunzi la culoare dacă poți. Contractul făcut = 1 punct de meci; primul la 5. Mâini ascunse. Mutări: licitez 12, pas, atu s, Jh.

## Reguli implementate

Joc sud-asiatic de levate cu licitație, adaptat pentru doi: 32 de cărți, 8 fiecare; valet 3, nouă 2, as 1, zece 1 puncte. Licitezi 10–20 de puncte, câștigătorul alege atuul și celălalt conduce; răspunzi la culoare dacă poți. Contractul făcut = 1 punct de meci; primul la 5. Mâini ascunse. Mutări: licitez 12, pas, atu s, Jh.

## Variante alese și limite

Adaptare pentru doi: 8 cărți fiecare (celelalte 16 ascunse și nejucate), licitație 10–20, fără atu ascuns, fără marriage și fără dublare; contract făcut = 1 punct de meci, primul la 5 (max. 10 mâini). Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
