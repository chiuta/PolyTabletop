# Snooker (`snooker`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play snooker --seed 1`. Sursa implementării: `polytabletop-games/src/snooker.rs`.

> Rezumat din `polytabletop list`: Snooker simplificat, o singură frame, cu fizică 2D a bilelor: bagi o roșie, apoi o culoare, alternativ; faulturile dau adversarului minimum 4. Lovitură: bila:buzunar:forța, de ex. r4:ne:2 (buzunare sv, nv, se, ne, ms, mn; forța 1–3).

## Scop

La sfârșitul frame-ului ai mai multe puncte.

## Pregătire

Masă 356,9×177,8, bila albă, 15 roșii și 6 culori la locurile lor (galben 2, verde 3, maro 4, albastru 5, roz 6, negru 7).

## Desfășurarea turei

O lovitură `bila:buzunar:forța` (ex. `r4:ne:2`); bila albă ține spre punctul „bilă-fantomă”. Ordinea: roșie, culoare la alegere, …, apoi culorile 2–7. Faulturile: prima atingere greșită, nicio atingere, albă băgată, bilă greșită băgată.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `r4:ne:2`

## Simplificări și note

Simplificări: fără snooker de evitat/bilă liberă, fără „D” la alegere, fără respot al negrului. Frame-ul se termină când masa e goală sau după 100 de lovituri. AI: simulează fiecare lovitură candidată (fizică deterministă).

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
