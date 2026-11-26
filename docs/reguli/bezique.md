# Bézique (`bezique`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play bezique --seed 1`. Sursa implementării: `polytabletop-games/src/bezique.rs`.

> Rezumat din `polytabletop list`: Joc francez pentru doi cu levate și figuri, pachet de 64 de cărți (7–A dublu), 8 cărți fiecare, atu întors. Cât timp mai e pachet, câștigătorul levatei poate declara o figură (căsătorie 20/40, secvență 250, careu 40–100, bézique 40, dublu bézique 500; dix = schimbi 7 de atu pentru 10), apoi ambii trag; după aceea răspunzi la culoare și bați. Brisques (ași și zeci) și ultima levată dau 10. Primul la 1000 (cel mult 4 mâini). Mutări: Qs, mariaj s, secventa, patru K, bezic, dublubezic, dix, fara, trag.

## Reguli implementate

Joc francez pentru doi cu levate și figuri, pachet de 64 de cărți (7–A dublu), 8 cărți fiecare, atu întors. Cât timp mai e pachet, câștigătorul levatei poate declara o figură (căsătorie 20/40, secvență 250, careu 40–100, bézique 40, dublu bézique 500; dix = schimbi 7 de atu pentru 10), apoi ambii trag; după aceea răspunzi la culoare și bați. Brisques (ași și zeci) și ultima levată dau 10. Primul la 1000 (cel mult 4 mâini). Mutări: Qs, mariaj s, secventa, patru K, bezic, dublubezic, dix, fara, trag.

## Variante alese și limite

Pachet de 64 de cărți, 2 jucători, cel mult 4 mâini, primul la 1000; mâna ascunsă, cărțile se trag la pasul `trag`. Regulile au fost reconstituite din surse publice (clean-room); unde sursele diferă s-a ales varianta cea mai răspândită și s-a documentat aici.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Informație ascunsă: da (HOTSEAT_SAFE). La hot-seat se afișează doar informația jucătorului la mutare; AI-ul citește doar mâna lui și informațiile publice, iar tragerile/zarurile/dezvăluirile sunt pași forțați (nu poate trișa).

## Calculatorul

Adversarul AI folosește o funcție de evaluare care nu citește informația ascunsă și nu vede rezultatele aleatoare viitoare; nu trișează.
