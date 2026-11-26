# Pub quiz (`pubquiz`)

Categorie: Party. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play pubquiz --seed 1`. Sursa implementării: `polytabletop-games/src/pubquiz.rs`.

> Rezumat din `polytabletop list`: Seară de întrebări pentru două echipe: 4 runde de câte 5 întrebări cu 4 variante, o altă categorie în fiecare rundă; un răspuns corect = 1 punct. Înaintea fiecărei runde o echipă poate juca jokerul (o dată pe meci; dublează punctele rundei) sau refuza. Răspunsurile rămân secrete până la verific; scor egal = remiză. Mutări: joker, fara joker, raspund a…raspund d, verific.

## Reguli implementate

Seară de întrebări pentru două echipe: 4 runde de câte 5 întrebări cu 4 variante, o altă categorie în fiecare rundă; un răspuns corect = 1 punct. Înaintea fiecărei runde o echipă poate juca jokerul (o dată pe meci; dublează punctele rundei) sau refuza. Răspunsurile rămân secrete până la verific; scor egal = remiză. Mutări: joker, fara joker, raspund a…raspund d, verific.

## Variante alese și limite

Întrebări din banca jocului de cunoaștere; răspunsurile fiecărei echipe sunt secrete până la `verific`; AI-ul alege la întâmplare (nu are „cunoaștere” simulată). Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
