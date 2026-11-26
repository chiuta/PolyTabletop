# Mexico (Mäxchen, Mia) (`mexico`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mexico --seed 1`. Sursa implementării: `polytabletop-games/src/mexico.rs`.

> Rezumat din `polytabletop list`: Joc de bluf cu două zaruri și cupă: arunci ascuns și anunți o valoare strict mai mare decât ultimul anunț (31 cea mai mică … dublele … 21 «Mexico» cea mai mare). Următorul crede (și aruncă) sau contestă: mincinosul pierde o viață, contestatarul greșit tot una (la Mexico două). Câte 5 vieți. Mutări: arunc, anunt 65, cred, contest, dezvaluie.

## Reguli implementate

Joc de bluf cu două zaruri și cupă: arunci ascuns și anunți o valoare strict mai mare decât ultimul anunț (31 cea mai mică … dublele … 21 «Mexico» cea mai mare). Următorul crede (și aruncă) sau contestă: mincinosul pierde o viață, contestatarul greșit tot una (la Mexico două). Câte 5 vieți. Mutări: arunc, anunt 65, cred, contest, dezvaluie.

## Variante alese și limite

Zarurile aruncate sunt ascunse (cupa); contestarea ridică cupa; 5 vieți fiecare; AI-ul nu vede zarurile adversarului. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
