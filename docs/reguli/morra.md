# Morra (Mora) (`morra`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play morra --seed 1`. Sursa implementării: `polytabletop-games/src/morra.rs`.

> Rezumat din `polytabletop list`: Joc italian cu degete: ambii arată pe ascuns 1–5 degete și strigă suma presupusă; dacă exact unul ghicește, ia un punct, altfel runda se reia. Alegerea e simultană (ascunsă până la dezvăluire); primul la 7. Mutări: 3/7 (3 degete, suma presupusă 7), dezvaluie.

## Reguli implementate

Joc italian cu degete: ambii arată pe ascuns 1–5 degete și strigă suma presupusă; dacă exact unul ghicește, ia un punct, altfel runda se reia. Alegerea e simultană (ascunsă până la dezvăluire); primul la 7. Mutări: 3/7 (3 degete, suma presupusă 7), dezvaluie.

## Variante alese și limite

Alegerea simultană: J1 alege primul în ascuns, J2 al doilea, rezultatul la pasul forțat `dezvaluie`; AI-ul joacă uniform (evaluare constantă). Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
