# Sittuyin (șahul birman) (`sittuyin`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play sittuyin --seed 1`. Sursa implementării: `polytabletop-games/src/sittuyin.rs`.

> Rezumat din `polytabletop list`: Tablă 8×8; cei 8 pioni sunt deja pe a3 b3 c3 d4 e4 f3 g3 h3 / a6 b6 c6 d5 e5 f6 g6 h6. Întâi fiecare își așază regele, generalul, 2 elefanți, 2 cai și 2 bărci în jumătatea proprie (K@e1). Apoi se mută: general = un pas în diagonală, elefant = un pas în diagonală sau înainte, cal ca în șah, barcă = turn; un pion aflat pe o diagonală mare în jumătatea adversă poate deveni general (d5=G) dacă nu mai ai. Mat câștigă; patul e interzis. Mutări: K@e1, e3-e4, e4xd5, d5=G.

## Reguli implementate

Tablă 8×8; cei 8 pioni sunt deja pe a3 b3 c3 d4 e4 f3 g3 h3 / a6 b6 c6 d5 e5 f6 g6 h6. Întâi fiecare își așază regele, generalul, 2 elefanți, 2 cai și 2 bărci în jumătatea proprie (K@e1). Apoi se mută: general = un pas în diagonală, elefant = un pas în diagonală sau înainte, cal ca în șah, barcă = turn; un pion aflat pe o diagonală mare în jumătatea adversă poate deveni general (d5=G) dacă nu mai ai. Mat câștigă; patul e interzis. Mutări: K@e1, e3-e4, e4xd5, d5=G.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
