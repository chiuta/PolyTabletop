# Tabula (`tabula`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tabula --seed 1`. Sursa implementării: `polytabletop-games/src/tabula.rs`.

> Rezumat din `polytabletop list`: Strămoșul roman al tablelor: 15 piese fiecare pornesc din afara tablei, 3 zaruri, iar ambii jucători merg în același sens pe cele 24 de puncte. Intri, muți, apoi scoți exact; câștigă cine scoate primul toate piesele. Mutări: ent-3, 5-9, 20-off.

## Reguli implementate

Strămoșul roman al tablelor: 15 piese fiecare pornesc din afara tablei, 3 zaruri, iar ambii jucători merg în același sens pe cele 24 de puncte. Intri, muți, apoi scoți exact; câștigă cine scoate primul toate piesele. Mutări: ent-3, 5-9, 20-off.

## Variante alese și limite

Motor comun `bgfam`: scoaterea (ca la table) cu zar exact sau mai mare dacă nu mai ai piese în urmă; fără obligația de reintrare (sursele nu o precizează). Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
