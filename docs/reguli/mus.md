# Mus (`mus`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mus --seed 1`. Sursa implementării: `polytabletop-games/src/mus.rs`.

> Rezumat din `polytabletop list`: Joc spaniol cu 40 de cărți, adaptat pentru doi: 4 cărți fiecare; 3 valorează ca regii, 2 ca asul. Mus opțional (arunci și tragi), apoi patru licitații: grande, chica, pares, juego; paso sau envido (miza 2), celălalt răspunde quiero / no quiero. Primul la 20. Mutări: mus, corto, descarta 0101, reparte, declara, paso, envido, quiero, no quiero, dezvaluie.

## Reguli implementate

Joc spaniol cu 40 de cărți, adaptat pentru doi: 4 cărți fiecare; 3 valorează ca regii, 2 ca asul. Mus opțional (arunci și tragi), apoi patru licitații: grande, chica, pares, juego; paso sau envido (miza 2), celălalt răspunde quiero / no quiero. Primul la 20. Mutări: mus, corto, descarta 0101, reparte, declara, paso, envido, quiero, no quiero, dezvaluie.

## Variante alese și limite

Adaptare pentru doi: 4 cărți fiecare, un singur tur de mus (aruncare + repartizare), patru lance (grande, chica, pares, juego/punto) cu un singur envite de 2 (fără órdago și re-envite); bonusurile pares/juego se adaugă doar la dezvăluire; primul la 20 de puncte (max. 30 de mâini). Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
