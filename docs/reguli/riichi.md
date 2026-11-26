# Riichi Mahjong (2 jucători) (`riichi`)

Categorie: Clasic. Doi jucători (Jucătorul 1 contra Jucătorului 2 sau calculatorului). Pornire: `polytabletop play riichi --seed 1`. Sursa implementării: `polytabletop-games/src/riichi.rs`.

> Rezumat din `polytabletop list`: Mahjong japonez pentru doi: 136 de piese (1m–9m, 1p–9p, 1s–9s, 1z–7z = vânturi și dragoni), 13 piese, 4 mâini (Est 1–2, Sud 1–2), 25 000 de puncte. Tura: trage, apoi arunci (5m), riichi 5m, tsumo, ankan 5m, kakan 5m (după careu: trage-moarta); pe aruncarea adversarului: ron, pon, kan, chi 4m 6m sau pas. Mână = 4 seturi + pereche (sau 7 perechi / 13 orfani) cu cel puțin un yaku; furiten, dora/ura-dora, fu și han până la yakuman. Zidul e ascuns. Fără piese roșii.

## Reguli implementate, variantă aleasă și omisiuni

Se alege varianta cea mai răspândită; ce lipsește este menționat explicit mai jos (rânduri cu „Omis”).

Riichi Mahjong (Japonia) — mahjong japonez standard, adaptat pentru 2 jucători.

Reguli folosite (regulile standard de club, fără piese roșii „aka”):
* 136 de piese (3 culori × 9 × 4 + 4 vânturi + 3 dragoni), 13 piese fiecăruia. Zidul mort are
14 piese (indicatori de dora, ura-dora, trageri după careu); zidul viu se trage pe rând.
**Zidul e ascuns** — draw-ul (`trage`) e o mutare forțată, ca AI-ul să nu vadă piesa următoare.
Notația pieselor: `1m`–`9m` (caractere), `1p`–`9p` (cercuri), `1s`–`9s` (bambus),
`1z`–`7z` = Est, Sud, Vest, Nord, alb, verde, roșu.
* Meci: 4 mâini (Est 1, Est 2, Sud 1, Sud 2), dealerul alternează între J1 și J2; ambii încep cu
25 000 de puncte. Dealerul = vântul Est, celălalt = Sud; vântul rundei: Est pentru primele două
mâini, Sud pentru ultimele. Dealerul care câștigă sau are mână gata (tenpai) la remiză
rămâne dealer (renchan, +1 honba = +300 puncte la fiecare câștig). Meciul se termină după
a patra mână (dacă dealerul câștigă în ultima mână și e în frunte, se oprește imediat) sau când
cineva coboară sub 0. Câștigă cine are mai multe puncte.
* Tura: `trage`, apoi arunci (`5m`), declari `riichi 5m` (mână închisă gata, 1000 de puncte
pe masă; după riichi mâna e blocată: doar tsumogiri, `tsumo` sau careu închis fără a schimba
așteptarea), `tsumo` (câștig din tragere), `ankan 5m` (careu închis), `kakan 5m` (careu din
pung expus). După careu se trage din zidul mort (`trage-moarta`). Pe aruncarea adversarului:
`ron`, `pon`, `kan`, `chi 4m 6m` (cu cele două piese din mână; în 2 jucători se poate de la
orice aruncare) sau `pas`.
* Mână câștigătoare: 4 seturi + pereche, șapte perechi (chiitoitsu) sau treisprezece orfani
(kokushi), cu cel puțin un yaku. **Furiten**: nu poți da ron dacă una din piesele pe care
le aștepți e în aruncările tale; nici după ce ai ratat un ron (până la următoarea ta aruncare,
iar după riichi, pentru restul mâinii).
* Yaku implementate: riichi, riichi dublu, ippatsu, tsumo închis, pinfu, tanyao, iipeikou,
yakuhai (dragoni, vântul tău, vântul rundei), haitei, houtei, rinshan kaihou, sanshoku
doujun, ittsu, chanta, toitoi, sanankou, sanshoku doukou, sankantsu, shousangen, honroutou,
chiitoitsu, honitsu, junchan, ryanpeikou, chinitsu; yakuman: kokushi, suuankou, daisangen,
shousuushii, daisuushii, tsuuiisou, chinroutou, ryuuiisou, chuuren poutou, suukantsu,
tenhou, chiihou (fără yakuman dublu; ≥13 han = kazoe yakuman). Dora și ura-dora.
* Puncte: fu (20 + deschideri/așteptări/seturi, rotunjit la 10; 25 la chiitoitsu) și han →
puncte de bază `fu·2^(han+2)`, cu limite mangan 2000 (5 han sau ≥ 2000), haneman 3000
(6–7), baiman 4000 (8–10), sanbaiman 6000 (11–12), yakuman 8000. Ron: dealer ×6, celălalt ×4;
tsumo: se plătește suma celor trei plătitori din jocul în patru (dealer: 3 × 2b; nedealer:
2b + 2 × b), rotunjit la 100, pe lângă +300/honba și bățul de riichi. Remiză (zidul epuizat):
cine nu e gata plătește 1500 celui gata.
* Omise: piese roșii (aka dora), kuikae complet verificat doar pentru aceeași piesă / capătul
scării, chankan, nagashi mangan, remize abortive (kyuushu, suukaikan etc.), pao, plata dublă
la yakuman, regula riichi cu zid sub 4 piese, 3–4 jucători, kazoe pe yaku-uri peste 13 han
doar ca yakuman simplu.
* AI: eficiență (shanten + piese utile) din propria mână și piesele vizibile; la Normal+ evită
piesele periculoase față de riichi (genbutsu, suji), la Hard/Expert apără mai ferm și
cheamă doar cu yaku. Nu vede mâna adversarului, zidul sau zidul mort.

## Sintaxa mutărilor

Scrii mutarea exact ca în lista afișată de `?` (fără diferență între majuscule și minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese. Evenimentele de șansă (zaruri, împărțire, tragere) sunt mutări forțate.
