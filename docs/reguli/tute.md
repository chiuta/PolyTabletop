# Tute (`tute`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play tute --seed 1`. Sursa implementării: `polytabletop-games/src/tute.rs`.

> Rezumat din `polytabletop list`: Joc spaniol de levate cu 40 de cărți pentru doi (o mână): 6 cărți fiecare, atu întors. Cât timp mai e pachet joci orice carte; apoi trebuie să răspunzi la culoare și să bați dacă poți. Câștigătorul poate anunța canto (rege+cal: 20, la atu 40), cambio (schimbi 7-ul de atu) sau tute (patru regi sau patru cai: câștigi pe loc). Ultima levată 10. Mutări: 3o, 12b, canto e, cambio, tute, trag.

## Reguli implementate

Joc spaniol de levate cu 40 de cărți pentru doi (o mână): 6 cărți fiecare, atu întors. Cât timp mai e pachet joci orice carte; apoi trebuie să răspunzi la culoare și să bați dacă poți. Câștigătorul poate anunța canto (rege+cal: 20, la atu 40), cambio (schimbi 7-ul de atu) sau tute (patru regi sau patru cai: câștigi pe loc). Ultima levată 10. Mutări: 3o, 12b, canto e, cambio, tute, trag.

## Variante alese și limite

O singură mână pentru doi: 6 cărți fiecare, cantes, cambio, tute, arrastre; AI-ul citește doar mâna lui și cărțile jucate. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
