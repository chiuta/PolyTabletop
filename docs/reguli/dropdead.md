# Drop Dead (`dropdead`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play dropdead --seed 1`. Sursa implementării: `polytabletop-games/src/dropdead.rs`.

> Rezumat din `polytabletop list`: Joc cu cinci zaruri și risc pentru doi, fără decizii: la tura ta arunci toate zarurile rămase; dacă nu apare niciun 2 sau 5, adaugi totalul la scor; altfel nu punctezi, iar zarurile cu 2 și 5 «cad». Tura se termină când au căzut toate cele cinci zaruri. 5 ture fiecare, câștigă scorul mai mare. Mutare: arunc.

## Reguli implementate

Joc cu cinci zaruri și risc pentru doi, fără decizii: la tura ta arunci toate zarurile rămase; dacă nu apare niciun 2 sau 5, adaugi totalul la scor; altfel nu punctezi, iar zarurile cu 2 și 5 «cad». Tura se termină când au căzut toate cele cinci zaruri. 5 ture fiecare, câștigă scorul mai mare. Mutare: arunc.

## Variante alese și limite

Fără decizii; numărul de ture (5) e o regulă a implementării, sursele nu fixează sfârșitul jocului. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
