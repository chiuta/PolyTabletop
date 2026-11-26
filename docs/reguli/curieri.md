# Curieri (variantă originală) (`curieri`)

Categorie: Cărți. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play curieri --seed 1`. Sursa implementării: `polytabletop-games/src/curieri.rs`.

> Rezumat din `polytabletop list`: Deducție cu 16 cărți: tragi, joci una din două. 1 Iscoadă ghicește (iscoada:5), 2 Crainic se uită, 3 Judecător compară, 4 Străjer protejează, 5 Mag (mag:eu/mag:el) schimbă cartea, 6 Cămăraș schimbă mâinile, 7 Contesă (obligatorie cu 5/6), 8 Prințesă (aruncată = ieși). Ultimul rămas sau cartea cea mai mare câștigă runda; primul cu 3 runde câștigă. Informație ascunsă: vezi doar cartea ta.

## Scop

Câștigi runde: ultimul rămas sau cartea cea mai mare; primul cu 3 runde câștigă.

## Pregătire

16 cărți: 1 Iscoadă ×5, 2 Crainic ×2, 3 Judecător ×2, 4 Străjer ×2, 5 Mag ×2, 6 Cămăraș ×1, 7 Contesă ×1, 8 Prințesă ×1. Se scoate 1 carte la întâmplare (ascunsă) și 3 descoperite. Fiecare ține o carte în mână.

## Desfășurarea turei

Tragi o carte, apoi joci una din cele două și aplici efectul.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Cartea jucată: `iscoada:5` (Iscoadă ghicind cartea 5), `crainic`, `judecator`, `strajer`, `mag:eu` / `mag:el`, `camaras`, `contesa`, `printesa`.
* `decontare` (mutare forțată care încheie runda).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (2): `judecator`, `strajer`. Introduci `judecator`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
2. Mutări legale (1): `decontare`. Introduci `decontare`; răspunsul calculatorului (în ordine): `crainic`.
3. Mutări legale (8): `iscoada:2`, `iscoada:3`, `iscoada:4`, `iscoada:5`, `iscoada:6`, `iscoada:7`, `iscoada:8`, `judecator`. Introduci `judecator`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Iscoadă: ghicești cartea adversarului (nu 1); dacă ai ghicit, iese din rundă.
* Crainic: te uiți la cartea adversarului.
* Judecător: compari cărțile; cea mai mică iese.
* Străjer: ești ferit până la tura ta următoare.
* Mag: un jucător (tu sau el) aruncă cartea din mână și trage alta; dacă aruncă 8, iese.
* Cămăraș: schimbi mâinile.
* Contesă: trebuie jucată dacă ții 5 sau 6.
* Prințesă: dacă o arunci, ieși.

## Punctaj

Câte o rundă câștigată.

## Sfârșitul jocului

Runda: ultimul rămas sau, la golirea pachetului, cartea cea mai mare (egal: suma aruncatelor). Meciul: primul cu 3 runde.

## Note despre AI

Informație ascunsă: vezi doar cartea ta. AI-ul nu citește mâna adversarului; alege după o euristică din ce știe (cartea lui, aruncatele publice, ce a aflat de la Crainic/Cămăraș), nu după rezultatul real al mutării.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Joc original de deducție cu 16 cărți și mecanică „o carte în mână”.
