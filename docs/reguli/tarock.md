# Tarock (2 jucători) (`tarock`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tarock --seed 1`. Sursa implementării: `polytabletop-games/src/tarock.rs`.

> Rezumat din `polytabletop list`: Adaptare pentru doi, fără licitație. 54 de cărți: 22 de tarocuri (t1 … t21 și Škis) + 4 culori × 8; câte 18, 18 rămân nefolosite. Respecți culoarea, altfel ești obligat să joci un tarok; tarokul bate orice carte de culoare, tarokul mai mare câștigă. Škis (skis) se joacă oricând, nu câștigă niciodată și rămâne al celui care l-a jucat. Puncte: K 5, Q 4, C 3, J 2, restul culorilor 1; tarocurile I, XXI, Škis 5, celelalte 1; ultima levată câștigată cu I dă +10. Primul la 2 mâini. Mutări: t7, skis, Kh, 10s, 1d, imparte.

## Reguli implementate

Adaptare pentru doi, fără licitație. 54 de cărți: 22 de tarocuri (t1 … t21 și Škis) + 4 culori × 8; câte 18, 18 rămân nefolosite. Respecți culoarea, altfel ești obligat să joci un tarok; tarokul bate orice carte de culoare, tarokul mai mare câștigă. Škis (skis) se joacă oricând, nu câștigă niciodată și rămâne al celui care l-a jucat. Puncte: K 5, Q 4, C 3, J 2, restul culorilor 1; tarocurile I, XXI, Škis 5, celelalte 1; ultima levată câștigată cu I dă +10. Primul la 2 mâini. Mutări: t7, skis, Kh, 10s, 1d, imparte.

## Variante alese și limite

Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită, documentată în antetul fișierului sursă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: `set_viewer` este respectat (mod hot-seat sigur).

## Calculatorul

Adversarul AI (niveluri Easy/Normal/Hard/Expert) nu citește informația ascunsă și nu vede aruncările/extragerile viitoare; nu trișează.
