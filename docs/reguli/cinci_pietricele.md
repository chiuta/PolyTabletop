# Cinci pietricele (`cinci_pietricele`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play cinci_pietricele --seed 1`. Sursa implementării: `polytabletop-games/src/cinci_pietricele.rs`.

> Rezumat din `polytabletop list`: Jocul copiilor cu cinci pietricele, cu norocul în locul îndemânării. 5 etape (uni, doi, trei, patru, tot). În tură alegi «calm» (70 %, +1 etapă) sau «iute» (45 %, +2 etape), apoi «incerc». O reușită te lasă să continui; o greșeală dă rândul. Primul care termină etapa 5 câștigă.

## Reguli implementate

Jocul copiilor cu cinci pietricele, cu norocul în locul îndemânării. 5 etape (uni, doi, trei, patru, tot). În tură alegi «calm» (70 %, +1 etapă) sau «iute» (45 %, +2 etape), apoi «incerc». O reușită te lasă să continui; o greșeală dă rândul. Primul care termină etapa 5 câștigă.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
