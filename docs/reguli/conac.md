# Misterul din Conac (deducție) (`conac`)

Categorie: Deducție. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play conac --seed 1`. Sursa implementării: `polytabletop-games/src/conac.rs`.

> Rezumat din `polytabletop list`: Joc original: o crimă cu soluție secretă (suspect s1–s6, armă a1–a6, cameră c1–c9); 18 cărți împărțite 9–9. Sugerezi (sugerez s2 a3 c5), adversarul îți arată în secret o carte pe care o are (arat s2) sau spune nu-am; deduci și acuzi (acuz s2 a3 c5, apoi verific): corect câștigi, greșit pierzi. Fără tablă de mers. Informație ascunsă: vezi doar mâna ta și cărțile arătate ție.

## Scop

Deduci soluția secretă (suspect, armă, cameră) și o acuzi corect.

## Pregătire

21 de cărți: 6 suspecți (s1–s6), 6 arme (a1–a6), 9 camere (c1–c9). Una din fiecare categorie e pusă deoparte (soluția); celelalte 18 se împart câte 9. Nu există tablă de mers: poți sugera oricând orice cameră.

## Desfășurarea turei

Fie sugerezi (`sugerez s2 a3 c5`): adversarul îți arată, numai ție, o carte din cele trei pe care le are (`arat s2`) sau spune `nu-am`; dacă are de ales, alege el. Fie acuzi (`acuz s2 a3 c5`), apoi `verific`: acuzația corectă câștigă, cea greșită pierde pe loc.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `sugerez s2 a3 c5`.
* Răspunsul adversarului: `arat s2` sau `nu-am`.
* `acuz s2 a3 c5`, apoi `verific`.

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (648): `sugerez s1 a1 c1`, `sugerez s1 a1 c2`, `sugerez s1 a1 c3`, `sugerez s1 a1 c4`, `sugerez s1 a1 c5`, `sugerez s1 a1 c6`, `sugerez s1 a1 c7`, `sugerez s1 a1 c8`, `sugerez s1 a1 c9`, `sugerez s1 a2 c1` …. Introduci `sugerez s1 a1 c1`; răspunsul calculatorului (în ordine): `arat c1`; `sugerez s2 a5 c2`.
2. Mutări legale (1): `arat s2`. Introduci `arat s2`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (648): `sugerez s1 a1 c1`, `sugerez s1 a1 c2`, `sugerez s1 a1 c3`, `sugerez s1 a1 c4`, `sugerez s1 a1 c5`, `sugerez s1 a1 c6`, `sugerez s1 a1 c7`, `sugerez s1 a1 c8`, `sugerez s1 a1 c9`, `sugerez s1 a2 c1` …. Introduci `acuz s6 a6 c9`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).

## Reguli speciale și variante alese

* Verificarea e o mutare separată, ca AI-ul să nu „vadă” soluția.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Acuzație corectă = victorie, greșită = înfrângere. După 150 de sugestii: remiză.

## Note despre AI

Informație ascunsă: afișajul arată mâna ta și ce ți s-a arătat. AI-ul deduce din propria mână, cărțile arătate lui și sugestiile fără răspuns; nu citește mâna adversarului sau soluția.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul NU este în `HOTSEAT_SAFE`: `--you both` afișează avertismentul „nu ascunde informația privată în modul la masă” (afișajul poate arăta mâna/informația Jucătorului 1), iar `hint` este dezactivat.

## Diferențe față de regulile tradiționale

* Joc original de deducție, fără tablă.
