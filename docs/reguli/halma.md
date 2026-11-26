# Halma (`halma`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play halma --seed 1`. Sursa implementării: `polytabletop-games/src/halma.rs`.

> Rezumat din `polytabletop list`: Halma pe 16×16 pentru doi: 19 piese fiecare în colțuri opuse. Muți un pas (și pe diagonală) sau un lanț de sărituri peste piese vecine; nu se captează nimic. Câștigă primul care aduce toate cele 19 piese în tabăra adversă; după 150 de mutări pe jucător, cine are mai multe acolo. Mutări: b2d4 (de la, la).

## Reguli implementate

Halma pe 16×16 pentru doi: 19 piese fiecare în colțuri opuse. Muți un pas (și pe diagonală) sau un lanț de sărituri peste piese vecine; nu se captează nimic. Câștigă primul care aduce toate cele 19 piese în tabăra adversă; după 150 de mutări pe jucător, cine are mai multe acolo. Mutări: b2d4 (de la, la).

## Variante alese și limite

Varianta pentru doi pe 16×16; după 150 de mutări pe jucător câștigă cine are mai multe piese în tabăra adversă (regulă proprie contra blocajelor). Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Fără informație privată a unui jucător (NO_HIDDEN): tablă/zaruri publice sau rezultate aleatoare hotărâte la pași forțați; AI-ul nu vede rezultatele viitoare.

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
