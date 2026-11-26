# Sărit coarda (Jump rope) (`coarda`)

Categorie: Copii. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play coarda --seed 1`. Sursa implementării: `polytabletop-games/src/coarda.rs`.

> Rezumat din `polytabletop list`: 8 runde, jucătorii alternează sărind și învârtind. Cel care învârte alege viteza (lent, mediu, rapid); săritorul alege trucuri (simplu 1, incrucisat 3, dublu 5, triplu 9 puncte) apoi sar, sau opresc ca să bage șirul; o greșeală îl pierde. Mutări: mediu, dublu, sar, opresc.

## Scop

Aduni mai multe puncte după 8 runde.

## Pregătire

8 runde; J1 sare în rundele pare (prima), J2 în cele impare; celălalt învârte coarda.

## Desfășurarea turei

Cel care învârte alege viteza (lent: +8% șanse, ×1; mediu ×2; rapid: −12% șanse, ×3). Săritorul alege un truc, apoi `sar` (mutare separată), sau `opresc` ca să bage șirul; cel mult 8 sărituri pe rundă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Exemple de mutări:

* `mediu`
* `dublu`
* `sar`
* `opresc`

## Simplificări și note

AI: calculează strategia optimă de oprire din probabilitățile publice; cel care învârte alege viteza care dă săritorului valoarea cea mai mică.

## Informație ascunsă

Nicio informație privată a unui jucător (sau informația ascunsă este simetrică, rezolvată prin mutări separate), deci nu e nevoie de ecran separat în hot-seat.
