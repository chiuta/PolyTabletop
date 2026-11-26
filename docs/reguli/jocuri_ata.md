# Jocuri cu ață (leagănul pisicii) (`jocuri_ata`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play jocuri_ata --seed 1`. Sursa implementării: `polytabletop-games/src/jocuri_ata.rs`.

> Rezumat din `polytabletop list`: Doi jucători își pasează figuri de ață în ordinea clasică: leagăn → patul soldatului → lumânări → ieslea → diamante → ochi de pisică → pește în farfurie → ceas → leagăn. Alegi «atent» (85 %, 1 punct) sau «rapid» (60 %, 2 puncte), apoi «preiau». Ața încurcată costă 1 punct și revine la leagăn; închiderea ciclului dă 3 puncte bonus. Primul la 12 câștigă.

## Reguli implementate

Doi jucători își pasează figuri de ață în ordinea clasică: leagăn → patul soldatului → lumânări → ieslea → diamante → ochi de pisică → pește în farfurie → ceas → leagăn. Alegi «atent» (85 %, 1 punct) sau «rapid» (60 %, 2 puncte), apoi «preiau». Ața încurcată costă 1 punct și revine la leagăn; închiderea ciclului dă 3 puncte bonus. Primul la 12 câștigă.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
