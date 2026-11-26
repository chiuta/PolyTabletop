# Yatzy (Scandinavia) (`yatzy`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play yatzy --seed 1`. Sursa implementării: `polytabletop-games/src/yatzy.rs`.

> Rezumat din `polytabletop list`: 5 zaruri, 3 aruncări pe tură, 15 categorii: unu…sase (bonus 50 dacă suma ≥ 63), pereche, două perechi, trei/patru la fel, scara mică 1-5 (15), scara mare 2-6 (20), casă, șansă, yatzy (50). Aruncarea e mutare separată; ții zaruri (tine 3 3 5) sau scrii într-o categorie. Câștigă totalul cel mai mare. Mutări: arunc, tine 3 3 5, scor pereche.

## Reguli implementate

5 zaruri, 3 aruncări pe tură, 15 categorii: unu…sase (bonus 50 dacă suma ≥ 63), pereche, două perechi, trei/patru la fel, scara mică 1-5 (15), scara mare 2-6 (20), casă, șansă, yatzy (50). Aruncarea e mutare separată; ții zaruri (tine 3 3 5) sau scrii într-o categorie. Câștigă totalul cel mai mare. Mutări: arunc, tine 3 3 5, scor pereche.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
