# Crazyhouse (șah cu reintroducere) (`crazyhouse`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play crazyhouse --seed 1`. Sursa implementării: `polytabletop-games/src/crazyhouse.rs`.

> Rezumat din `polytabletop list`: Șah în care piesele capturate trec în rezerva ta și pot fi introduse pe orice câmp liber în loc de mutare (pionii nu pe prima/ultima linie); o piesă promovată capturată revine pion. Reguli complete de șah; remiză prin repetiție triplă sau după 200 de mutări. Mutări: e2e4, e7e8q, N@f3.

## Reguli implementate

Șah în care piesele capturate trec în rezerva ta și pot fi introduse pe orice câmp liber în loc de mutare (pionii nu pe prima/ultima linie); o piesă promovată capturată revine pion. Reguli complete de șah; remiză prin repetiție triplă sau după 200 de mutări. Mutări: e2e4, e7e8q, N@f3.

## Variante alese și limite

Copie a motorului de șah cu rezerve și introduceri. Fără regula celor 50 de mutări și fără material insuficient; remiză la repetiție triplă sau după 200 de mutări complete. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
