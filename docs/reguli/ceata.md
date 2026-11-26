# Câmpul Ceții (piese ascunse, 10×10) (`ceata`)

Categorie: Strategie. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play ceata --seed 1`. Sursa implementării: `polytabletop-games/src/ceata.rs`.

> Rezumat din `polytabletop list`: Variantă originală: 40 de piese pe parte, ranguri 1–X, mine (B), steag (F), lacuri. Vezi doar piesele tale și pe ale adversarului după bătălie. Mutări: e4-e5, e4xe5 (atac), apoi mutarea forțată bataie. Săpătorul dezamorsează minele, Iscoada bate Mareșalul. 100 de mutări fără bătălie = remiză.

## Scop

Capturezi steagul adversarului sau îl lași fără mutări.

## Pregătire

Tablă 10×10, 40 de piese pe parte. Piese: Steag (F), Mine (B ×6), 1 Iscoadă (×1), 2 Iscoditori (×8), 3 Săpători (×5), 4 Sergenți (×4), 5 Locotenenți (×4), 6 Căpitani (×3), 7 Maiori (×3), 8 Coloneli (×2), 9 Generali (×2), X = 10 Mareșal (×1). Poziția de start este amestecată automat (steagul în rândul din spate, câteva mine lângă el). J1 e jos (rândurile 1–4), J2 sus (7–10). Lacurile (~) pe coloanele c, d, g, h la rândurile 5–6 nu se trec.

## Desfășurarea turei

Un pas ortogonal pe pătrat liber sau atac pe o piesă adversă. Atacul se rezolvă printr-o mutare forțată separată (`bataie`).

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Pas: `e4-e5`; atac: `e4xe5`.
* `bataie` (forțată).

### Exemple verificate (`--seed 1`)

Rulate efectiv în CLI; calculatorul joacă Jucătorul 2.

1. Mutări legale (10): `b4-b5`, `e4-e5`, `e4-e6`, `e4xe7`, `f4-f5`, `f4-f6`, `f4xf7`, `j4-j5`, `j4-j6`, `j4xj7`. Introduci `b4-b5`; răspunsul calculatorului (în ordine): `b7xb5`; `bataie`.
2. Mutări legale (14): `b3-b4`, `c4-b4`, `e4-e5`, `e4-e6`, `e4xe7`, `f4-f5`, `f4-f6`, `f4xf7`, `j4-j5`, `j4-j6` …. Introduci `f4xf7`; răspunsul calculatorului (în ordine): (calculatorul nu a mutat: rămâi la mutare).
3. Mutări legale (1): `bataie`. Introduci `bataie`; răspunsul calculatorului (în ordine): `a7xa4`; `bataie`.

## Reguli speciale și variante alese

* Iscoditorul merge oricât în linie dreaptă (se arată că e Iscoditor când face mai mult de un pas). Steagul și minele nu se mișcă.
* Bătălia: rang mai mare câștigă; egal = dispar amândouă; Iscoada atacă Mareșalul și îl învinge; Săpătorul dezamorsează o mină, orice altă piesă care atacă o mină dispare; steagul capturat = victorie. Ambele piese devin vizibile după bătălie.

## Punctaj

Fără punctaj.

## Sfârșitul jocului

Steag capturat = victorie. Pierde cine nu mai are mutări. 100 de mutări fără bătălie = remiză.

## Note despre AI

Informație ascunsă: adversarul vede doar locul pieselor, nu rangul. `evaluate` citește rangul pieselor proprii și doar al pieselor adverse devenite vizibile; pentru restul folosește distribuția pieselor încă necunoscute. `bataie` e forțată ca AI-ul să nu vadă piesa ascunsă înainte de a ataca.

Stil AI în cod: alpha-beta cu adâncime fixă 1.

Modul „la masă” (`--you both`): jocul este în lista `HOTSEAT_SAFE`: afișajul respectă convenția „la masă” (informația privată e arătată doar celui la mutare, cu ecran de predare în GUI), deci `--you both` și `hint` funcționează.

## Diferențe față de regulile tradiționale

* Fără regula celor două pătrate / mutare repetată; în schimb, 100 de mutări fără bătălie = remiză.
* Variantă originală cu ierarhie de piese ascunse.
