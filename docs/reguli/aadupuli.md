# Aadu Puli Attam (Oi și tigri) (`aadupuli`)

Categorie: Abstract. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play aadupuli --seed 1`. Sursa implementării: `polytabletop-games/src/aadupuli.rs (+ linegraph.rs)`.

> Rezumat din `polytabletop list`: 3 tigri (Jucătorul 2) contra a 15 oi (Jucătorul 1) pe o tablă de 23 de puncte în formă de triunghi. Oile se plasează pe rând, apoi se mută un pas; tigrii se mută sau sar peste o oaie (captură, lanț opțional). Tigrii câștigă cu 5 oi capturate; oile câștigă imobilizându-i. Mutări: b3, b3-b4, vxb2, opreste.

## Scop

Tigrii: 5 oi capturate. Oile: imobilizarea tuturor tigrilor.

## Pregătire

Tablă de 23 de puncte în formă de triunghi: vârful `v`, șapte trepte de câte trei puncte (`a1 b1 c1` … `a7 b7 c7` de sus în jos) și un punct de bază `z` pe axă. Jucătorul 1 = 15 oi (în mână), Jucătorul 2 = 3 tigri pe `v`, `a1`, `c1`.

## Desfășurarea turei

Oile (Jucătorul 1, mută primul) se plasează pe rând câte una pe un punct liber; abia după ce au fost plasate toate, se mută un pas pe un punct vecin liber. Tigrii se mută un pas sau sar peste o oaie vecină pe punctul liber de după ea (pe aceeași linie), capturând-o; săritura se poate lanțui.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

* Plasare: `b3`.
* Pas: `b3-b4`.
* Salt: `vxb2`.
* `opreste`: încheie un lanț de sărituri.

## Reguli speciale și variante alese

* Captura nu e obligatorie.

## Punctaj

Oi capturate.

## Sfârșitul jocului

Tigrii câștigă cu 5 oi capturate; oile câștigă dacă toți tigrii sunt imobilizați; remiză după 120 de semi-mutări fără captură în faza de mutare.

## Note despre AI

Informație completă. Alpha-beta cu adâncime fixă 4.

Modul „la masă” (`--you both`): jocul nu este în `HOTSEAT_SAFE`, dar nu are informație ascunsă.

## Diferențe față de regulile tradiționale

* Geometria tablei, numărul de oi necesar tigrilor (5) și punctul `z` sunt reconstituiri care nu au putut fi verificate din surse în această sesiune.
