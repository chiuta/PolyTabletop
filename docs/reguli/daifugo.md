# Daifugo (Daihinmin) (`daifugo`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play daifugo --seed 1`. Sursa implementării: `polytabletop-games/src/daifugo.rs`.

> Rezumat din `polytabletop list`: 54 de cărți (cu 2 jokeri), 27 fiecare, 5 runde. Ordine 3…K, A, 2, joker. Joci un grup de același rang (33, 777), adversarul același număr de cărți mai mari sau pas; după pas conduce cine a jucat ultimul. Joker singur (X) sau wild (5X). 4 cărți = revoluție (ordinea se inversează), orice grup cu 8 oprește jocul (conduci din nou). Câștigi runda când rămâi fără cărți; perdantul îți dă cele mai bune 2 cărți, tu dai 2 înapoi (da 4 5). Mutări: 3, 77, JJJ, 55X, X, pas, da 4 5.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Daifugo / Daihinmin (Japonia) — joc de eliminare a cărților, 2 jucători, 54 de cărți.

Reguli folosite (varianta de bază, cu cele mai răspândite reguli locale):
* 54 de cărți (cu 2 jokeri), 27 fiecăruia — se împart toate, deci mâna adversarului se poate
deduce din ce vezi. Ordinea: 3 < 4 < … < 10 < J < Q < K < A < 2; jokerul singur e cel mai
mare. Culoarea nu contează.
* Cine conduce joacă un grup de cărți de același rang (1–4); celălalt trebuie să dea același
număr de cărți de rang mai mare, ori `pas`. După un `pas` câmpul se curăță și conduce
cel care a jucat ultimul. Cine rămâne primul fără cărți câștigă runda (Daifugō).
* Jokerul poate fi folosit singur (mutare `X`, bate orice singleton) sau ca „wild” într-un
grup cu cel puțin o carte reală (`5X` = pereche de 5, `55X` = trei de 5).
* Revoluție (kakumei): un grup de 4 cărți (cu joker sau nu) inversează ordinea rangurilor
(2 devine cel mai mic, 3 cel mai mare; jokerul rămâne cel mai mare) până la o nouă
revoluție sau sfârșitul rundei. 8-stop (yagiri): orice grup care conține un 8 curăță câmpul
și același jucător conduce din nou.
* Meci de 5 runde. De la runda a doua: Daihinmin (perdantul) dă câștigătorului cele mai
puternice 2 cărți ale lui (la revoluție tot ordinea normală de dinaintea rundei), iar
Daifugō alege 2 cărți oarecare de dat înapoi (`da 4 5` → rangurile date) și conduce.
Câștigă cine are mai multe runde.
* Omise: scara (kaidan), shibari (blocarea pe culoare), 11-back, 3 de pică peste joker,
interdicția de a termina pe 8/joker, ranguri intermediare (Fugō/Hinmin), 4–5 jucători.
* Mutări: `3`, `77`, `JJJ`, `55X`, `X`, `pas`, `da 4 5`.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
