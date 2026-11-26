# Bar billiards (`barbilliards`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play barbilliards --seed 1`. Sursa implementării: `polytabletop-games/src/barbilliards.rs`.

> Rezumat din `polytabletop list`: Masa de crâșmă cu găuri și popici, simulată pe lovituri, pentru doi: găurile valorează 10, 20, 30, 50, 100, 200; bila roșie dublează punctele. O vizită continuă cât timp loviturile punctează; ratarea o încheie (punctele rămân); un fault (popică doborâtă) o încheie și anulează punctele vizitei; popica neagră anulează tot scorul. 8 vizite fiecare; lovitura se hotărăște la pasul forțat lovitura. Mutări: 10 alba, 100 rosie, lovitura, stop.

## Reguli implementate

Masa de crâșmă cu găuri și popici, simulată pe lovituri, pentru doi: găurile valorează 10, 20, 30, 50, 100, 200; bila roșie dublează punctele. O vizită continuă cât timp loviturile punctează; ratarea o încheie (punctele rămân); un fault (popică doborâtă) o încheie și anulează punctele vizitei; popica neagră anulează tot scorul. 8 vizite fiecare; lovitura se hotărăște la pasul forțat lovitura. Mutări: 10 alba, 100 rosie, lovitura, stop.

## Variante alese și limite

Simulare pe lovituri cu șanse fixe de reușită/fault pe țintă; 8 vizite pe jucător. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
