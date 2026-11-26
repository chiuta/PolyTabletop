# Șerpi și scări (`snakes`)

Categorie: Zaruri. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play snakes --seed 1`. Sursa implementării: `polytabletop-games/src/snakes.rs`.

> Rezumat din `polytabletop list`: Tablă 1–100; zarul e aruncat automat și afișat; scările urcă, șerpii coboară; ținta se atinge exact. Singura mutare: arunc.

## Scop

Ajungi primul pe pătratul 100.

## Pregătire

Tablă de 1–100 cu scări și șerpi. Pionii pornesc în afara tablei.

## Desfășurarea turei

Zarul este aruncat automat imediat după fiecare mutare și este vizibil în afișaj. Singura mutare este `arunc`, care avansează pionul cu valoarea zarului curent.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `arunc`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): `arunc`.
2. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): `arunc`.
3. Mutări legale (1): `arunc`. Introduci `arunc`; răspunsul calculatorului (în ordine): `arunc`.

## Reguli speciale și variante alese

* Capătul unei scări urcă pionul, capul unui șarpe îl coboară.
* Ținta (100) se atinge exact: dacă zarul depășește 100, pionul rămâne pe loc.
* Fără aruncare suplimentară la 6.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Când un pion ajunge exact pe 100.

## Note despre AI

Zarul este informație publică. AI-ul nu are alegeri reale (o singură mutare). Întregul joc depinde doar de seed.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`; are elemente aleatoare, dar acestea se consideră publice pentru categoria Zaruri, deci `--you both` afișează avertisment, dar `hint` rămâne disponibil.

## Diferențe față de regulile tradiționale

Sursa nu documentează simplificări; regulile urmează varianta standard descrisă mai sus.
