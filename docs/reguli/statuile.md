# Statuile (Statues) (`statuile`)

Categorie: Familie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play statuile --seed 1`. Sursa implementării: `polytabletop-games/src/statuile.rs`.

> Rezumat din `polytabletop list`: Joc de înghețat pentru copii, pentru doi: fiecare alergător trebuie să ajungă la linia 12; la rundele impare aleargă J1, la cele pare J2. Alergătorul alege în ascuns 1–3 pași, cel care strigă alege în ascuns când strigă stop (1–3); dacă face mai mulți pași decât a strigat, e prins în mișcare și se întoarce la start. Dezvăluirea e pasul forțat dezvaluie; primul la 12 câștigă. Mutări: 1, 2, 3, dezvaluie.

## Reguli implementate

Joc de înghețat pentru copii, pentru doi: fiecare alergător trebuie să ajungă la linia 12; la rundele impare aleargă J1, la cele pare J2. Alergătorul alege în ascuns 1–3 pași, cel care strigă alege în ascuns când strigă stop (1–3); dacă face mai mulți pași decât a strigat, e prins în mișcare și se întoarce la start. Dezvăluirea e pasul forțat dezvaluie; primul la 12 câștigă. Mutări: 1, 2, 3, dezvaluie.

## Variante alese și limite

Alegere simultană ascunsă: alergătorul alege pașii, cel care strigă alege momentul; rezultatul la pasul forțat `dezvaluie`; AI-ul joacă uniform. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
