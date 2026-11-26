# Schocken (`schocken`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play schocken --seed 1`. Sursa implementării: `polytabletop-games/src/schocken.rs`.

> Rezumat din `polytabletop list`: Joc german cu trei zaruri și 13 jetoane: până la trei aruncări pe rundă (păstrezi zarurile alese); clasament Schock Out (1-1-1) > Schock (1-1-x) > General > Straße > obișnuită (cel mai mare zar întâi). Cine pierde runda ia jetoane după valoarea mâinii câștigătoare; o repriză se termină când cineva ține toate cele 13 și o pierde; două reprize pierdute pierd meciul. Mutări: arunc, stop, re 13 (re-arunci zarurile 1 și 3), decontare.

## Reguli implementate

Joc german cu trei zaruri și 13 jetoane: până la trei aruncări pe rundă (păstrezi zarurile alese); clasament Schock Out (1-1-1) > Schock (1-1-x) > General > Straße > obișnuită (cel mai mare zar întâi). Cine pierde runda ia jetoane după valoarea mâinii câștigătoare; o repriză se termină când cineva ține toate cele 13 și o pierde; două reprize pierdute pierd meciul. Mutări: arunc, stop, re 13 (re-arunci zarurile 1 și 3), decontare.

## Variante alese și limite

Doar cele două reprize (fără finala `Finale`), zarurile sunt vizibile (fără aruncări oarbe); doi jucători. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
