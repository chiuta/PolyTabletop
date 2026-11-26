# TwixT (`twixt`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play twixt --seed 1`. Sursa implementării: `polytabletop-games/src/twixt.rs`.

> Rezumat din `polytabletop list`: Joc de conectare pe tabla 24×24: J1 leagă marginea de sus de cea de jos, J2 pe cea din stânga de cea din dreapta; nu poți pune cuie pe marginile adversarului sau în colțuri. Cuiele aflate la mutare de cal se leagă automat dacă puntea nu taie alta. Câștigă cine închide primul lanțul. Mutări: e12 (coloana a–x, rândul 1–24; rândul 24 e sus).

## Reguli implementate

Joc de conectare pe tabla 24×24: J1 leagă marginea de sus de cea de jos, J2 pe cea din stânga de cea din dreapta; nu poți pune cuie pe marginile adversarului sau în colțuri. Cuiele aflate la mutare de cal se leagă automat dacă puntea nu taie alta. Câștigă cine închide primul lanțul. Mutări: e12 (coloana a–x, rândul 1–24; rândul 24 e sus).

## Variante alese și limite

Legăturile se fac automat la plasarea cuiului (nu se pot retrage); fără regula de schimb (pie rule). Dacă nu mai rămâne niciun câmp liber, remiză. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
