# Skunk (zaruri) (`skunk`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play skunk --seed 1`. Sursa implementării: `polytabletop-games/src/skunk.rs`.

> Rezumat din `polytabletop list`: Joc de zaruri «împinge-ți norocul», 5 runde. În tură arunci două zaruri cât vrei (arunc) sau te oprești (opresc). Fără 1: adaugi suma la tură. Un singur 1 (skunk): pierzi punctele turei. Doi de 1: pierzi tot totalul. Confirmi skunk-ul cu ok. Câștigă totalul cel mai mare după 5 runde.

## Reguli implementate

Joc de zaruri «împinge-ți norocul», 5 runde. În tură arunci două zaruri cât vrei (arunc) sau te oprești (opresc). Fără 1: adaugi suma la tură. Un singur 1 (skunk): pierzi punctele turei. Doi de 1: pierzi tot totalul. Confirmi skunk-ul cu ok. Câștigă totalul cel mai mare după 5 runde.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
