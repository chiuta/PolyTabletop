# Ombre (L'Hombre) (`ombre`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ombre --seed 1`. Sursa implementării: `polytabletop-games/src/ombre.rs`.

> Rezumat din `polytabletop list`: Ombre în doi cu pachetul spaniol de 40 de cărți: câte 9 cărți. Licitezi (entrada) sau treci; Ombre alege atuul, poate arunca și trage cărți, apoi se joacă 9 levate. Matadorii (atuurile cele mai mari) bat restul. Ombre câștigă dacă ia mai multe levate decât adversarul. Primul la 8 puncte. Mutări: entrada, pas, atu <culoare>, scap <cărți>, gata, cartea, imparte. Mâini ascunse.

## Reguli implementate

Ombre în doi cu pachetul spaniol de 40 de cărți: câte 9 cărți. Licitezi (entrada) sau treci; Ombre alege atuul, poate arunca și trage cărți, apoi se joacă 9 levate. Matadorii (atuurile cele mai mari) bat restul. Ombre câștigă dacă ia mai multe levate decât adversarul. Primul la 8 puncte. Mutări: entrada, pas, atu <culoare>, scap <cărți>, gata, cartea, imparte. Mâini ascunse.

## Variante alese și limite

Adaptare pentru doi jucători (jocul clasic e în trei); ordinea atuurilor verificată după Wikipedia.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
