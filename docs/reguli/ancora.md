# Ancora și coroana (`ancora`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ancora --seed 1`. Sursa implementării: `polytabletop-games/src/ancora.rs`.

> Rezumat din `polytabletop list`: Trei zaruri cu simboluri (coroana, ancora, inima, romb, trefla, pica) — joc britanic tradițional «Crown and Anchor». Mizezi 5, 10 sau 20 pe unul sau mai multe simboluri; pentru fiecare zar cu simbolul tău primești miza înapoi plus câștig (1:1, 2:1, 3:1), altfel pierzi miza. 12 runde împotriva băncii, câștigă cine are mai multe jetoane. Mutări: pun ancora 10, gata, arunc.

## Reguli implementate

Trei zaruri cu simboluri (coroana, ancora, inima, romb, trefla, pica) — joc britanic tradițional «Crown and Anchor». Mizezi 5, 10 sau 20 pe unul sau mai multe simboluri; pentru fiecare zar cu simbolul tău primești miza înapoi plus câștig (1:1, 2:1, 3:1), altfel pierzi miza. 12 runde împotriva băncii, câștigă cine are mai multe jetoane. Mutări: pun ancora 10, gata, arunc.

## Variante alese și limite

Titlu propriu pentru jocul tradițional britanic «Crown and Anchor»; plata 1:1, 2:1, 3:1; 12 runde. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație completă (fără cărți ascunse).

## Calculatorul

Adversarul AI caută în arbore (alpha-beta) cu o funcție de evaluare care nu citește informația ascunsă; nu trișează.
