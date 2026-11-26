# Mehen (Egiptul antic) (`mehen`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play mehen --seed 1`. Sursa implementării: `polytabletop-games/src/mehen.rs`.

> Rezumat din `polytabletop list`: Reconstrucție. Un traseu-șarpe de 18 pătrate (coada 0, capul 17), 3 piese fiecare. Se aruncă patru bețe (0–4 fețe luminoase, 0 = 6; aruncările 1, 4 și 6 dau încă o aruncare). O piesă intră pe coadă doar cu 1, merge spre cap și se întoarce pe același traseu (35 de pași), apoi iese acasă. Aterizezi pe adversar = îl trimiți afară; pe piesa ta nu poți ateriza. Câștigă primul cu toate 3 piesele acasă. Mutări: arunc, intra, m1/m2/m3, pas.

## Reguli implementate

Reconstrucție. Un traseu-șarpe de 18 pătrate (coada 0, capul 17), 3 piese fiecare. Se aruncă patru bețe (0–4 fețe luminoase, 0 = 6; aruncările 1, 4 și 6 dau încă o aruncare). O piesă intră pe coadă doar cu 1, merge spre cap și se întoarce pe același traseu (35 de pași), apoi iese acasă. Aterizezi pe adversar = îl trimiți afară; pe piesa ta nu poți ateriza. Câștigă primul cu toate 3 piesele acasă. Mutări: arunc, intra, m1/m2/m3, pas.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
