# Pare sau impare (`parimpar`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play parimpar --seed 1`. Sursa implementării: `polytabletop-games/src/parimpar.rs`.

> Rezumat din `polytabletop list`: J1 este «impar», J2 este «par». La fiecare rundă fiecare alege în secret 1–5 degete (n1…n5), apoi se arată (arat). Suma impară dă runda lui J1, cea pară lui J2. Primul la 5 runde câștigă. Alegerea ta rămâne ascunsă până la arătare.

## Reguli implementate

J1 este «impar», J2 este «par». La fiecare rundă fiecare alege în secret 1–5 degete (n1…n5), apoi se arată (arat). Suma impară dă runda lui J1, cea pară lui J2. Primul la 5 runde câștigă. Alegerea ta rămâne ascunsă până la arătare.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
