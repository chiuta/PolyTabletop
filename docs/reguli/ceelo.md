# Cee-lo (zaruri de stradă) (`ceelo`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ceelo --seed 1`. Sursa implementării: `polytabletop-games/src/ceelo.rs`.

> Rezumat din `polytabletop list`: Trei zaruri. 4-5-6 sau trei egale câștigă pe loc; 1-2-3 pierde pe loc; o pereche plus un zar diferit dă acel zar ca punct; orice altceva se rearuncă. Pariorul alege miza (5/10/20); bancherul aruncă până iese un rezultat, apoi pariorul; punctul mai mare câștigă, egal = remiză. Banca trece când bancherul pierde. 16 mâini, 50 de jetoane. Mutări: pariez 10, arunc.

## Reguli implementate

Trei zaruri. 4-5-6 sau trei egale câștigă pe loc; 1-2-3 pierde pe loc; o pereche plus un zar diferit dă acel zar ca punct; orice altceva se rearuncă. Pariorul alege miza (5/10/20); bancherul aruncă până iese un rezultat, apoi pariorul; punctul mai mare câștigă, egal = remiză. Banca trece când bancherul pierde. 16 mâini, 50 de jetoane. Mutări: pariez 10, arunc.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
