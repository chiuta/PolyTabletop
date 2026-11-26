# Sprouts (Conway–Paterson) (`sprouts`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sprouts --seed 1`. Sursa implementării: `polytabletop-games/src/sprouts.rs`.

> Rezumat din `polytabletop list`: Joc topologic, 4 puncte la start. La rândul tău unești două puncte (sau un punct cu el însuși, printr-o buclă) cu o curbă care nu taie nimic, apoi pui un punct nou pe curbă. Din fiecare punct pleacă cel mult 3 curbe. Cine nu mai poate muta pierde. Poziția e dată combinatoriu (regiuni și frontiere); mutări: a-b (puncte), a-a pentru buclă, cu .1/.2 când există mai multe variante.

## Reguli implementate

Joc topologic, 4 puncte la start. La rândul tău unești două puncte (sau un punct cu el însuși, printr-o buclă) cu o curbă care nu taie nimic, apoi pui un punct nou pe curbă. Din fiecare punct pleacă cel mult 3 curbe. Cine nu mai poate muta pierde. Poziția e dată combinatoriu (regiuni și frontiere); mutări: a-b (puncte), a-a pentru buclă, cu .1/.2 când există mai multe variante.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
