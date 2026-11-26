# Tien Gow (Cer și nouă) (`tiengow`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tiengow --seed 1`. Sursa implementării: `polytabletop-games/src/tiengow.rs`.

> Rezumat din `polytabletop list`: Joc cantonez de levate cu 32 de piese de domino, adaptat pentru doi: 8 piese fiecare. Conduci cu o piesă sau o pereche; celălalt bate cu o piesă mai mare de aceeași culoare (civil/militar) sau o pereche mai mare de același tip, ori aruncă cu fața în jos. Ultima levată câștigă mâna; primul la 10 puncte. Mutări: 66, 66+66, d64 (d = aruncare ascunsă).

## Reguli implementate

Joc cantonez de levate cu 32 de piese de domino, adaptat pentru doi: 8 piese fiecare. Conduci cu o piesă sau o pereche; celălalt bate cu o piesă mai mare de aceeași culoare (civil/militar) sau o pereche mai mare de același tip, ori aruncă cu fața în jos. Ultima levată câștigă mâna; primul la 10 puncte. Mutări: 66, 66+66, d64 (d = aruncare ascunsă).

## Variante alese și limite

Adaptare pentru doi: 8 piese fiecare, celelalte 16 rămân ascunse; doar piese singure și perechi (fără triple/cvartete, fără bancher), punctaj simplificat (1 punct, dublat la joc complet sau Gee Joon în ultima levată), primul la 10. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
