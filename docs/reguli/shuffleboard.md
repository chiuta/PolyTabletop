# Shuffleboard de masă (`shuffleboard`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play shuffleboard --seed 1`. Sursa implementării: `polytabletop-games/src/shuffleboard.rs`.

> Rezumat din `polytabletop list`: Alunecă pucuri pe un traseu de 96 de unități: zonele 60 (1 p), 70 (2), 80 (3), 90–95 (4, agățat); de la 96 cad. Câte 4 pucuri pe rundă, pe rând. Alegi forța (tir 60), apoi «lansez»: pucul ajunge la ±5 de țintă; pucurile lovite transmit mișcarea. Scorează doar jucătorul cu pucul cel mai departe, cu pucurile lui aflate dincolo de cel mai bun al adversarului. Primul la 15. Mutări: tir 30…100, lansez.

## Reguli implementate

Alunecă pucuri pe un traseu de 96 de unități: zonele 60 (1 p), 70 (2), 80 (3), 90–95 (4, agățat); de la 96 cad. Câte 4 pucuri pe rundă, pe rând. Alegi forța (tir 60), apoi «lansez»: pucul ajunge la ±5 de țintă; pucurile lovite transmit mișcarea. Scorează doar jucătorul cu pucul cel mai departe, cu pucurile lui aflate dincolo de cel mai bun al adversarului. Primul la 15. Mutări: tir 30…100, lansez.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
