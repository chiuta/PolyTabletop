# Sholo Guti (Șaisprezece soldați) (`shologuti`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play shologuti --seed 1`. Sursa implementării: `polytabletop-games/src/shologuti.rs (+ linegraph.rs)`.

> Rezumat din `polytabletop list`: 16 piese pe jucător pe o tablă de 37 de puncte (grilă 5 × 5 cu diagonale plus două triunghiuri), rândul din mijloc gol. Mută un pas pe un punct liber vecin sau sare peste o piesă adversă (captură obligatorie, lanț opțional). Câștigă cine capturează tot sau blochează adversarul. Mutări: c2-c3, c1xc3, opreste.

## Scop

Capturezi toate piesele adversarului sau îl blochezi.

## Pregătire

Tablă de 37 de puncte: grilă 5 × 5 (`a1`–`e5`, linii ortogonale + diagonale prin punctele „pare”) și două triunghiuri de câte 6 puncte (`u1`–`u6` jos, `t1`–`t6` sus). Jucătorul 1: rândurile 1–2 și triunghiul de jos; Jucătorul 2: rândurile 4–5 și triunghiul de sus; rândul 3 e gol.

## Desfășurarea turei

Muți un pas pe un punct vecin liber, sau sari peste o piesă adversă vecină pe punctul liber de după ea (pe aceeași linie), capturând-o. Captura este obligatorie dacă există; după un salt poți continua cu aceeași piesă.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* `c2-c3`: pas.
* `c1xc3`: salt.
* `opreste`: încheie un lanț de sărituri.

## Reguli speciale și variante alese

* Geometria triunghiurilor este reconstituită; nu a putut fi verificată din surse în această sesiune.

## Punctaj

Nu există punctaj.

## Sfârșitul jocului

Câștigă cine capturează toate piesele adversarului sau îl lasă fără mutări; remiză după 100 de mutări fără captură.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 5.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Tabla (37 de puncte, 16 + 16 piese, rând mijlociu gol) este o reconstituire; captura obligatorie este alegerea documentată.
