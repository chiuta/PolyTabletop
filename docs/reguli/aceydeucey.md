# Acey-deucey (`aceydeucey`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play aceydeucey --seed 1`. Sursa implementării: `polytabletop-games/src/aceydeucey.rs`.

> Rezumat din `polytabletop list`: Variantă americană de table: toate cele 15 piese pornesc din afara tablei și intră în casa adversarului; zarul 1+2 (acey-deucey) se joacă, apoi alegi orice dublă și mai arunci; dublele dau patru mutări. Scoaterea e exactă când toate sunt acasă. Mutări: ent-3, 5-9, 20-off, dublet 4.

## Reguli implementate

Variantă americană de table: toate cele 15 piese pornesc din afara tablei și intră în casa adversarului; zarul 1+2 (acey-deucey) se joacă, apoi alegi orice dublă și mai arunci; dublele dau patru mutări. Scoaterea e exactă când toate sunt acasă. Mutări: ent-3, 5-9, 20-off, dublet 4.

## Variante alese și limite

Motor comun `bgfam`: fără regula interzicerii „primului” de 6 puncte; reintrarea nu e obligatorie înaintea altor mutări. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
