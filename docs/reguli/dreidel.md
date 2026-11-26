# Dreidel (`dreidel`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play dreidel --seed 1`. Sursa implementării: `polytabletop-games/src/dreidel.rs`.

> Rezumat din `polytabletop list`: Joc de noroc cu titirezul. Fiecare începe cu 10 jetoane și pune 1 în pot (din nou când potul are 0–1 jetoane). Pe rând se învârte («invart»): nun = nimic, gimel = iei tot potul, hei = iei jumătate, shin = pui 1. Cine nu mai poate plăti pierde; după 300 de învârtiri câștigă cine are mai multe jetoane.

## Reguli implementate

Joc de noroc cu titirezul. Fiecare începe cu 10 jetoane și pune 1 în pot (din nou când potul are 0–1 jetoane). Pe rând se învârte («invart»): nun = nimic, gimel = iei tot potul, hei = iei jumătate, shin = pui 1. Cine nu mai poate plăti pierde; după 300 de învârtiri câștigă cine are mai multe jetoane.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
