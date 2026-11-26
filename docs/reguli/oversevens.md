# Peste și sub șapte (Over and Under Seven) (`oversevens`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play oversevens --seed 1`. Sursa implementării: `polytabletop-games/src/oversevens.rs`.

> Rezumat din `polytabletop list`: Pariorul alege «sub» (2–6), «peste» (8–12), plătite 1:1, sau «sapte» (exact 7), plătit 4:1, cu miza 5/10/20; banca (celălalt jucător) aruncă două zaruri («arunc»). Banca se schimbă la fiecare mână. 20 de mâini, 50 de jetoane; câștigă cine are mai multe. Mutări: sub 5, peste 10, sapte 5, arunc.

## Reguli implementate

Pariorul alege «sub» (2–6), «peste» (8–12), plătite 1:1, sau «sapte» (exact 7), plătit 4:1, cu miza 5/10/20; banca (celălalt jucător) aruncă două zaruri («arunc»). Banca se schimbă la fiecare mână. 20 de mâini, 50 de jetoane; câștigă cine are mai multe. Mutări: sub 5, peste 10, sapte 5, arunc.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
