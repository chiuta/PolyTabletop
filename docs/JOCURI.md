# Jocurile jucabile

Generat de `tools/gen_docs.py` din `polytabletop list` și `polytabletop play ID` (141 jocuri). Nu edita manual; regenerează cu `python3 -I tools/gen_docs.py`.

Sintaxa generală: scrii mutarea exact cum apare în lista `?` (fără diferență între majuscule/minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Cuprins

- [Șah](#chess) (`chess`)
- [Dame (8×8)](#draughts) (`draughts`)
- [Reversi](#reversi) (`reversi`)
- [Patru în linie](#connect4) (`connect4`)
- [Gomoku (15×15)](#gomoku) (`gomoku`)
- [Moara (Țintar)](#morris) (`morris`)
- [Mancala (Kalah)](#mancala) (`mancala`)
- [Go (9×9)](#go) (`go`)
- [Xiangqi (șah chinezesc)](#xiangqi) (`xiangqi`)
- [Shogi (șah japonez)](#shogi) (`shogi`)
- [Dara (Nigeria)](#dara) (`dara`)
- [Morabaraba (Africa de Sud)](#morabaraba) (`morabaraba`)
- [Sungka (Filipine)](#sungka) (`sungka`)
- [Makruk (șah thailandez)](#makruk) (`makruk`)
- [Janggi (șah coreean)](#janggi) (`janggi`)
- [Septica](#septica) (`septica`)
- [Macao](#macao) (`macao`)
- [Domino (dublu-6)](#domino) (`domino`)
- [Table (Backgammon)](#backgammon) (`backgammon`)
- [Nu te supăra, frate (Ludo)](#ludo) (`ludo`)
- [Șerpi și scări](#snakes) (`snakes`)
- [Barbut (duel simplificat)](#barbut) (`barbut`)
- [Șase-zeci și șase (Santase)](#66) (`66`)
- [Blackjack (21)](#blackjack) (`blackjack`)
- [Remi (Rummy, 13 cărți)](#rummy) (`rummy`)
- [Poker (Texas hold'em, heads-up)](#poker) (`poker`)
- [Cărți colecționabile (Ținutul Cenușiu)](#tcg) (`tcg`)
- [Miniaturi hexagonale (Trecătoarea Cenușie)](#miniaturi) (`miniaturi`)
- [Mahjong (Hong Kong, 2 jucători)](#mahjong) (`mahjong`)
- [Hanafuda (Koi-Koi)](#hanafuda) (`hanafuda`)
- [Hwatu (Go-Stop)](#hwatu) (`hwatu`)
- [Karuta (recitare vs AI, proverbe)](#karuta) (`karuta`)
- [Dou dizhu (2 jucători)](#doudizhu) (`doudizhu`)
- [Pai Gow (domino chinezesc)](#paigow) (`paigow`)
- [Teen Patti (heads-up)](#teenpatti) (`teenpatti`)
- [Ganjifa (levate)](#ganjifa) (`ganjifa`)
- [Yut (Yunnori)](#yut) (`yut`)
- [Sugoroku (cursă cu casete)](#sugoroku) (`sugoroku`)
- [Chaupar (cauri, 2 jucători)](#chaupar) (`chaupar`)
- [Culori (potrivit cărți colorate)](#culori) (`culori`)
- [Moșii (afaceri imobiliare cu zaruri)](#mosii) (`mosii`)
- [Magistrale (rute cu cărți de vagon)](#magistrale) (`magistrale`)
- [Misterul din Conac (deducție)](#conac) (`conac`)
- [Insula (colonizare cu resurse)](#insula) (`insula`)
- [Mozaic (plăcuțe cu drumuri, cetăți, mănăstiri)](#mozaic) (`mozaic`)
- [Litere (cuvinte pe tablă)](#litere) (`litere`)
- [Faianța (variantă originală)](#faianta) (`faianta`)
- [Bresla (variantă originală)](#bresla) (`bresla`)
- [Cristale (variantă originală)](#cristale) (`cristale`)
- [Curieri (variantă originală)](#curieri) (`curieri`)
- [Fitil (variantă originală)](#fitil) (`fitil`)
- [Bufet (variantă originală)](#bufet) (`bufet`)
- [Expediții](#expeditii) (`expeditii`)
- [Artificii (cooperativ, partener AI)](#artificii) (`artificii`)
- [Orășel (variantă originală)](#orasel) (`orasel`)
- [Capcana (variantă originală)](#capcana) (`capcana`)
- [Congkak (Malaezia)](#congkak) (`congkak`)
- [Pallanguzhi (Tamil Nadu)](#pallanguzhi) (`pallanguzhi`)
- [Ayo (Ayoayo, Yoruba)](#ayo) (`ayo`)
- [Bătălia navală (10×10)](#battleship) (`battleship`)
- [Mastermind (cu schimb de roluri)](#mastermind) (`mastermind`)
- [Memory (perechi)](#memory) (`memory`)
- [Bingo (75 de bile)](#bingo) (`bingo`)
- [Lotería (imagini)](#loteria) (`loteria`)
- [Pachisi (cursă cu cauri)](#pachisi) (`pachisi`)
- [Carrom (discret, fără fizică)](#carrom) (`carrom`)
- [Arhipelagul în Flăcări (cucerire cu zaruri)](#arhipelag) (`arhipelag`)
- [Roata Destinului (parcurs de viață)](#roata) (`roata`)
- [Cinci Zaruri (13 categorii)](#zaruri5) (`zaruri5`)
- [Plăcuțe (seturi și serii)](#placute) (`placute`)
- [Turnul Șovăitor (turn de blocuri)](#turnul) (`turnul`)
- [Cupa Cunoașterii (întrebări, 6 felii)](#cunoastere) (`cunoastere`)
- [Câmpul Ceții (piese ascunse, 10×10)](#ceata) (`ceata`)
- [Umbra Urbei (urmărire cu bilete)](#umbra) (`umbra`)
- [Zidurile Mișcătoare (labirint cu plăci culisante)](#zidurile) (`zidurile`)
- [Drumul Prăjiturilor (cursă de culori)](#prajituri) (`prajituri`)
- [Portretul Misterios (ghicește personajul)](#portret) (`portret`)
- [Briscola (Italia)](#briscola) (`briscola`)
- [Scopa (Italia)](#scopa) (`scopa`)
- [Durak (Rusia)](#durak) (`durak`)
- [Belote (Franța, 2 jucători)](#belote) (`belote`)
- [Skat (2 jucători)](#skat) (`skat`)
- [Tarot francez (78 de cărți, 2 jucători)](#tarot) (`tarot`)
- [Cribbage](#cribbage) (`cribbage`)
- [Euchre (2 jucători)](#euchre) (`euchre`)
- [Tarneeb (2 jucători)](#tarneeb) (`tarneeb`)
- [Baloot (2 jucători)](#baloot) (`baloot`)
- [Tablanet](#tablanet) (`tablanet`)
- [Truco (Argentina)](#truco) (`truco`)
- [Trix (2 jucători)](#trix) (`trix`)
- [Bridge duplicat (2 jucători)](#bridge) (`bridge`)
- [Canasta (2 jucători, o mână)](#canasta) (`canasta`)
- [Escadrile din Rift (manevre secrete)](#nave) (`nave`)
- [Patrula (skirmish cu echipă mică)](#patrula) (`patrula`)
- [Ordine și reacții (skirmish cu reacții)](#reactii) (`reactii`)
- [Ordine de bătaie (infanterie, zaruri de ordine)](#ordine) (`ordine`)
- [Gridiron (sport fantastic pe ture)](#gridiron) (`gridiron`)
- [Focare (cooperativ, boli pe hartă, partener AI)](#focare) (`focare`)
- [Străjile Codrului (cooperativ de apărare, cărți de putere)](#strajile) (`strajile`)
- [Farul Fantomei (deducție cooperativă cu indicii vizuale)](#farul) (`farul`)
- [Noapte Cenușie (cooperativ de supraviețuire pe grilă)](#cenusii) (`cenusii`)
- [Cripta Clopotelor (aventură tactică cu cărți de acțiune)](#cripta) (`cripta`)
- [Lupii din Hârtop (roluri ascunse, masa = roboți)](#lupii) (`lupii`)
- [Străjerii Farului (misiuni cu echipe, roluri ascunse)](#santinela) (`santinela`)
- [Sfatul Negustorilor (alegeri și legi, roluri ascunse)](#consiliul) (`consiliul`)
- [Valea Uscată (duel western, roluri ascunse)](#valea) (`valea`)
- [Hoinari prin Hrubă (monștri și comori, masa = roboți)](#hruba) (`hruba`)
- [Imperii în Apus (cucerire cu popoare în declin)](#apus) (`apus`)
- [Fulger (simbolul comun, cronometru simulat)](#fulger) (`fulger`)
- [Roade de Toamnă (legume și schimburi, masa = roboți)](#roade) (`roade`)
- [Cărți colecționabile: evoluție și energie (Făpturi de Câmp)](#tcg_evo) (`tcg_evo`)
- [Cărți colecționabile: monștri în zone și capcane (Hotarele Ceții)](#tcg_zone) (`tcg_zone`)
- [Cărți colecționabile: pitch, lanț și echipament (Lanțuri de Oțel)](#tcg_lant) (`tcg_lant`)
- [Cărți colecționabile: cerneală, misiuni și provocări (Povestiri din Cerneală)](#tcg_cerneala) (`tcg_cerneala`)
- [Spionii Cuvintelor (grilă 5×5)](#spionii) (`spionii`)
- [Povestitorul (cărți-simbol, 2 jucători)](#povestitor) (`povestitor`)
- [Gura Satului (completează fraza)](#gura) (`gura`)
- [Trei Căi (spune, mimează, desenează)](#treicai) (`treicai`)
- [Cuvinte Oprite (descrie fără cuvinte interzise)](#oprite) (`oprite`)
- [Schița (desen simbolic)](#schita) (`schita`)
- [Dicționar Viu (explică în rafală)](#dictionar) (`dictionar`)
- [Impostorul din Grilă (4 locuri)](#impostor) (`impostor`)
- [Cuvânt Unic (indicii care se anulează)](#cuvantunic) (`cuvantunic`)
- [Moșia (roluri, plantații și clădiri)](#mosia) (`mosia`)
- [Gospodăria (plasare de muncitori și fermă)](#gospodaria) (`gospodaria`)
- [Cetățuia (muncitori pe drum de clădiri, castel)](#cetatuia) (`cetatuia`)
- [Tărâmuri (transformarea terenului și piste de ritual)](#taramuri) (`taramuri`)
- [Furnale (rețea de legături și industrii)](#furnale) (`furnale`)
- [Reverdire (variantă originală)](#reverdire) (`reverdire`)
- [Salina (variantă originală)](#salina) (`salina`)
- [Cătun (variantă originală)](#catun) (`catun`)
- [Solzi (variantă originală)](#solzi) (`solzi`)
- [Poieni (variantă originală)](#poieni) (`poieni`)
- [Epoci (variantă originală)](#epoci) (`epoci`)
- [Ținuturi (variantă originală)](#tinuturi) (`tinuturi`)
- [Rezervația (variantă originală)](#rezervatia) (`rezervatia`)
- [Hotarul (variantă originală)](#hotar) (`hotar`)
- [Codrul Nesupus (război asimetric, variantă originală)](#codru) (`codru`)
- [Coroana Siderală (4X compact, variantă originală)](#sideral) (`sideral`)
- [Ceasornicul din Valea Ceții (deducție cu povestitor, masa = roboți)](#ceasornic) (`ceasornic`)
- [Drumul spre Coroană (aventură pe inele, om contra roboți)](#coroana) (`coroana`)

<a id="chess"></a>
## Șah

- **Id:** `chess`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chess` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Reguli complete: rocadă (mută regele două pătrate: e1g1 sau e1c1), en passant, promovare (obligatorie; e7e8q, sufixul q/r/b/n alege piesa), remiză prin pat, repetiție de trei ori, 50 de mutări fără captură sau pion, material insuficient.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e2e4, e7e8q.
- Exemple legale la început: `a2a3`, `a2a4`
- La început sunt 20 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="draughts"></a>
## Dame (8×8)

- **Id:** `draughts`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play draughts` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Dame engleze 8×8: piesele simple mută și capturează doar înainte, regele (piesa care ajunge pe ultimul rând; mutarea se oprește acolo) un pas în toate cele patru diagonale.
- Captura e obligatorie și salturile continuă cât se poate.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c3-d4, c3xe5xg7.
- Exemple legale la început: `a3-b4`, `c3-b4`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="reversi"></a>
## Reversi

- **Id:** `reversi`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play reversi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pui o piesă care încadrează în linie dreaptă piese ale adversarului și le întoarce; negrul (J1) începe; câștigă cine are mai multe piese la final.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d3 (pătratul ales), pass (doar când nu ai nicio mutare).
- Exemple legale la început: `d3`, `c4`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="connect4"></a>
## Patru în linie

- **Id:** `connect4`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play connect4` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Lasă discuri în coloane; primul cu patru în linie (orizontal, vertical sau diagonal) câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–7 (numărul coloanei).
- Exemple legale la început: `4`, `3`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="gomoku"></a>
## Gomoku (15×15)

- **Id:** `gomoku`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play gomoku` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci sau mai multe în linie câștigă; negrul începe în centru.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: h8 (coloane a–o, rânduri 1–15).
- Exemple legale la început: `h8`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="morris"></a>
## Moara (Țintar)

- **Id:** `morris`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play morris` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 9 piese: plasare, apoi mutare pe linii; o moară (trei pe o linie) scoate o piesă adversă; cu 3 piese poți „zbura”.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d1 (plasare), a1-a4 (mutare); cu moară se adaugă piesa scoasă: g1xb2, a1-a4xb2.
- Exemple legale la început: `a1`, `d1`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mancala"></a>
## Mancala (Kalah)

- **Id:** `mancala`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play mancala` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 6 gropi și un depozit; semănatul ajunge în depozit = încă o mutare; groapă goală proprie = captură.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–6 (groapa proprie aleasă).
- Exemple legale la început: `1`, `2`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="go"></a>
## Go (9×9)

- **Id:** `go`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play go` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Scor pe arie (pietre + teritoriu), komi 7,5 pentru alb, ko simplu, fără sinucidere; două pass-uri consecutive încheie jocul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e5 (coloane a–h și j, fără i; rânduri 1–9), pass.
- Exemple legale la început: `a1`, `b1`
- La început sunt 82 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="xiangqi"></a>
## Xiangqi (șah chinezesc)

- **Id:** `xiangqi`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play xiangqi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 9×10, palat, râu, tun cu ecran, cal blocabil, generalii nu se pot privi.
- Mat sau pat = pierdere.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: h2e2 (fișiere a–i, rânduri 0–9).
- Exemple legale la început: `b2b9`, `h2h9`
- La început sunt 44 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="shogi"></a>
## Shogi (șah japonez)

- **Id:** `shogi`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play shogi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 9×9; piesele capturate se pun înapoi (drop).
- Promovare în zona adversă (obligatorie dacă piesa n-ar mai avea mutări).
- Drop interzis: nifu (doi pioni pe fișier), pion/lance/cal fără mutări, pion care dă mat.
- Mat = pierdere; a 4-a repetiție sau 320 de semi-mutări = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7g7f, 2b8h+ (promovare), P*5e (drop).
- Exemple legale la început: `9g9f`, `8g8f`
- La început sunt 30 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="dara"></a>
## Dara (Nigeria)

- **Id:** `dara`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play dara` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 5×6, 12 piese pe jucător.
- Faza 1: plasare pe rând (fără capturi).
- Faza 2: mutare ortogonală cu un pas; o moară de exact 3 (rând sau coloană, nu 4) scoate o piesă adversă oarecare.
- Pierzi cu sub 3 piese sau blocat; remiză după 100 de mutări fără captură.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b3, b3-b4, b3-b4xd2 (fișiere a–f, rânduri 1–5).
- Exemple legale la început: `a1`, `b1`
- La început sunt 30 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="morabaraba"></a>
## Morabaraba (Africa de Sud)

- **Id:** `morabaraba`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play morabaraba` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă ca la Moară plus diagonalele prin colțuri (20 de „vaci”), 12 piese.
- Plasare, apoi mutare pe linii (și diagonale); vaca scoate o piesă adversă (nu din vacă, dacă are alta); cu 3 piese zbori; sub 3 piese sau blocat = pierdere; tablă plină după plasare = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d1, a1-b2, c3xg7, a1-d5xg7.
- Exemple legale la început: `a1`, `d1`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sungka"></a>
## Sungka (Filipine)

- **Id:** `sungka`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sungka` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 7 gropi și un depozit pe jucător, 7 semințe în fiecare groapă.
- Semănat în sens trigonometric prin depozitul propriu, sărind depozitul adversarului.
- Ultima sămânță în depozit = încă o mutare; în groapă ocupată = ridici și continui; în groapă goală proprie = capturezi groapa opusă; în groapă goală adversă = tura se termină.
- O singură rundă, fără gropi „arse”, mutări pe rând; câștigă cine are peste 49 în depozit sau mai mult la final.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–7 (groapa proprie, de la stânga la dreapta).
- Exemple legale la început: `1`, `2`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="makruk"></a>
## Makruk (șah thailandez)

- **Id:** `makruk`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play makruk` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 8×8: Khun (rege), Met (un pas diagonal), Khon (un pas diagonal sau înainte), cal, turn; pionii mută și capturează cu un pas și se promovează în Met pe rândul 6.
- Fără rocadă.
- Mat = victorie; pat = remiză; fără pioni, numărare limitată (8–64 de mutări) = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e3e4.
- Exemple legale la început: `a1a2`, `b1d2`
- La început sunt 23 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="janggi"></a>
## Janggi (șah coreean)

- **Id:** `janggi`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play janggi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 9×10, palate cu diagonale, Cho (jos) începe.
- Car pe diagonalele palatului, tun care sare exact o piesă (nu tun), cal blocabil, elefant (1 ortogonal + 2 diagonale) blocabil, soldați înainte/lateral.
- Pass permis când nu ești în șah; mat = pierdere; repetiție, 2 pass-uri sau 150 de semi-mutări fără captură = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b2e2, pass.
- Exemple legale la început: `a0a1`, `a0a2`
- La început sunt 32 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="septica"></a>
## Septica

- **Id:** `septica`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play septica` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet de 32 de cărți, 4 în mână.
- Preiei masa cu aceeași valoare ca prima carte sau cu 7 / 8; poți tăia la rândul tău; cine dă pas pierde masa. 10 și A = 1 punct (8 în total), peste 4 câștigi.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 9h, 7s, pas.
- Exemple legale la început: `9d`, `7d`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="macao"></a>
## Macao

- **Id:** `macao`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play macao` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 5 în mână: culoare sau valoare. 2 / 3 se adună (taie cu alt 2 / 3, altfel tragi); 4 sare peste rând; 7 merge peste orice și alege culoarea (7h:s);
- As cere culoarea (Ah:d).
- Fără carte bună tragi una.
- Câștigă cine rămâne fără cărți (nu cu carte specială).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 9h, trage, sar, pas.
- Exemple legale la început: `7d:h`, `7d:d`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="domino"></a>
## Domino (dublu-6)

- **Id:** `domino`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play domino` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 28 de piese, 7 fiecare, restul stoc.
- Începe cel mai mare dublu.
- Așază la stânga (S) sau dreapta (D); fără piesă pescuiești (pesc), cu stocul gol dai pas.
- Câștigă cine termină piesele (ia punctele adversarului) sau, la blocaj, cine are mai puține puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3-4:S, 3-4:D, pesc, pas.
- Exemple legale la început: `3-3`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="backgammon"></a>
## Table (Backgammon)

- **Id:** `backgammon`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu (informație completă; zarurile/amestecul nu se consideră ascunse)
- **Pornire:** `polytabletop play backgammon` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Două zaruri aruncate automat; 15 piese, bară, bear-off, dubla se joacă de 4 ori, fără cub de dublare.
- Un pas = un zar; trebuie jucate cât mai multe zaruri.
- Numerotarea e a jucătorului la mutare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 13/7, bar/22, 3/off.
- Exemple legale la început: `6/3`, `8/5`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ludo"></a>
## Nu te supăra, frate (Ludo)

- **Id:** `ludo`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu (informație completă; zarurile/amestecul nu se consideră ascunse)
- **Pornire:** `polytabletop play ludo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 4 piese, un zar.
- Cu 6 scoți o piesă din bază; 6 = încă o aruncare; aterizarea pe adversar îl trimite în bază; ținta se atinge exact.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: intră, sau progresul piesei (ex. 12).
- Exemple legale la început: `intră`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="snakes"></a>
## Șerpi și scări

- **Id:** `snakes`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play snakes` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 1–100; zarul e aruncat automat și afișat; scările urcă, șerpii coboară; ținta se atinge exact.
- Singura mutare: arunc.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="barbut"></a>
## Barbut (duel simplificat)

- **Id:** `barbut`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play barbut` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 10 jetoane fiecare.
- Pariu 1–3; prima aruncare: 7/11 câștigă, 2/3/12 pierde, altfel punct; adversarul plătește (dublează) sau lasă; apoi punctul bate 7.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pariez 1, arunc, plătesc, las.
- Exemple legale la început: `pariez`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="66"></a>
## Șase-zeci și șase (Santase)

- **Id:** `66`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play 66` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 jucători, 24 de cărți, atu, anunțuri 20/40, schimbul nouarului de atu, închiderea jocului; 66 de puncte încheie mâna, meci până la 7 puncte de joc.
- Informație ascunsă: vezi doar mâna ta (J1) și numărul de cărți ale adversarului.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 9h, Qs, 10d, Ac, Qh+ (anunț), schimb, inchide, urmatoarea.
- Exemple legale la început: `Jh`, `9c`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="blackjack"></a>
## Blackjack (21)

- **Id:** `blackjack`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play blackjack` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tu (J1) contra dealerului (J2, trage până la 17): carte / stai / dublu; blackjack plătește 3:2.
- Set de 21 de mâini cu pariu de 10 din 100 de jetoane: câștigi dacă termini peste 100.
- Pașii dealerului sunt mutări forțate: dealer-trage, dealer-sta, decontare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `carte`, `stai`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="rummy"></a>
## Remi (Rummy, 13 cărți)

- **Id:** `rummy`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play rummy` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 jucători, 13 cărți, un pachet: tragi (stoc sau talon) și arunci; închizi când cele 13 cărți rămase formează secvențe și seturi (cel puțin o secvență; aranjare automată).
- Penalizare = punctele libere; meci până la 101.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: stoc, talon, Ah, 10c, inchide-Ah, urmatoarea.
- Exemple legale la început: `stoc`, `talon`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="poker"></a>
## Poker (Texas hold'em, heads-up)

- **Id:** `poker`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play poker` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 20 de mâini, blinduri fixe 10/20, 1000 de jetoane, raise fix (20 / 40 de la turn), maxim 3 raise-uri pe rundă.
- Câștigă cine are mai multe jetoane.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: fold, call/check, raise; pașii de împărțire sunt forțați: flop, turn, river, showdown, decontare.
- Exemple legale la început: `fold`, `call`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tcg"></a>
## Cărți colecționabile (Ținutul Cenușiu)

- **Id:** `tcg`
- **Categorie:** TCG
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tcg` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Motor generic de cărți colecționabile cu 60 de cărți originale (resurse, creaturi, vrăji); pachete pregătite de 30 de cărți (J1 Cetatea, J2 Furtuna), 20 de viață, 5 cărți în mână.
- Tură: pescuire (forțată), faza principală, fin.
- O resursă pe tură (+1 mană).
- Creaturile noi nu atacă (Grabă = da); cuvinte-cheie: Zbor, Grabă, Armură, Venin, Primul, Străjer.
- Atac: ataca 1,3; apărătorul alege pe rând bloc N sau fara.
- Pachet gol la pescuit = pierdere.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pesca, joaca 2, joaca 3 e1 (țintă: e=inamic, m=al meu, fata), ataca 1,2, bloc 1, fara, fin.
- Exemple legale la început: `pesca`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="miniaturi"></a>
## Miniaturi hexagonale (Trecătoarea Cenușie)

- **Id:** `miniaturi`
- **Categorie:** Miniaturi
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play miniaturi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Motor generic de miniaturi pe hartă hexagonală: 7 unități originale pe parte (Lăncieri, Arcași, Călăreți, Colos, Catapulte, Comandant), teren (stâncă blochează vederea, pădure = acoperire), zaruri de lovire și salvare, moral (șovăit, apoi fugă). 6 runde cu activări alternante; 3 obiective (cine are mai multe figuri la ≤1 hex ia 1 punct pe rundă); eliminarea armatei adverse câștigă.
- Informație completă; zarurile se aruncă la mutarea forțată arunca.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: A pas, A>d4, A>d4 xK (mută și atacă), A xK, arunca.
- Exemple legale la început: `A`, `pas`
- La început sunt 265 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mahjong"></a>
## Mahjong (Hong Kong, 2 jucători)

- **Id:** `mahjong`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mahjong` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 136 de piese, 13 în mână, zid viu de 96 și zid mort de 14 (ascuns; vezi doar câte au rămas).
- Tragi și arunci; adversarul poate lua aruncarea cu pung / kong / chi (de la oricare aruncare) sau pe câștig.
- Mână câștigătoare = 4 seturi + pereche și minimum 1 fan (tragere proprie, mână închisă, toate secvențele, toate pungurile, o culoare, dragoni, vânturi).
- Fără șapte perechi / mâini speciale.
- Zid epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, d5m, d Dr, mahjong, pung, kong, chi2m3m, kong5m, pas, trage-moarta.
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hanafuda"></a>
## Hanafuda (Koi-Koi)

- **Id:** `hanafuda`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play hanafuda` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 48 de cărți de flori (12 luni × 4), 8 în mână și 8 pe masă; potrivești luna: obligatoriu capturezi dacă ai pereche (doi candidați = alegi: 3a:3c, trei = iei toate).
- Apoi întorci o carte din stoc (intoarce) și o joci la fel.
- Yaku nou (luminoase, mistreț-cerb-fluturi, panglici, animale, simple, hanami, tsukimi): koi-koi (continui) sau shobu (încasezi; ×2 la 7+ și ×2 dacă adversarul a spus koi-koi). 3 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3b, 3a:3c, intoarce, ia 3c, koi-koi, shobu.
- Exemple legale la început: `1a`, `1d`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hwatu"></a>
## Hwatu (Go-Stop)

- **Id:** `hwatu`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play hwatu` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Aceleași 48 de cărți de flori; 10 în mână, 8 pe masă, 20 în stoc.
- Captură ca la Hanafuda (3c, 3a:3c, intoarce, ia 3c).
- Scor: gwang, godori, hongdan/cheongdan/chodan, tti, yeolkkeut, pi.
- De la 7 puncte alegi stop (încasezi) sau go (continui: +1 la primele două, apoi ×2; ×2 gwang-bak, pi-bak și go-bak).
- Măturarea mesei ia o simplă de la adversar. 3 runde.
- Fără ppeok/jjok/ttadak.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `1d`, `3d:3a`
- La început sunt 11 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="karuta"></a>
## Karuta (recitare vs AI, proverbe)

- **Id:** `karuta`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play karuta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Karuta cu 40 de proverbe românești: 16 cărți pe masă (8 pe parte, cu sfârșitul proverbului), 24 de citiri (8 false).
- Începutul se dezvăluie cuvânt cu cuvânt; pe rând: asteapta sau ia N (atingi cartea).
- Corect: cartea dispare (ținta de pe partea adversarului: îi trimiți una de-a ta).
- Greșit: adversarul îți trimite o carte.
- Câștigă cine rămâne primul fără cărți (sau are mai puține după toate citirile).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: asteapta, ia 12.
- Exemple legale la început: `asteapta`, `ia`
- La început sunt 33 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="doudizhu"></a>
## Dou dizhu (2 jucători)

- **Id:** `doudizhu`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play doudizhu` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 54 de cărți (3…K, A, 2, jokeri X/Y); 17 fiecare, 20 ascunse nefolosite.
- Combinații: single, pereche, trio (+1 sau +pereche), scară de 5+ (34567), scară de perechi 3+ (334455), bombă (5555), rachetă (XY).
- Răspunzi cu același tip și lungime, mai mare, sau bombă/rachetă, ori pas.
- Cine rămâne fără cărți câștigă runda; cel mai bun din 3.
- Fără avioane și careu+2.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 5, 55, 555+3, 34567, 5555, XY, pas.
- Exemple legale la început: `4`, `44`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="paigow"></a>
## Pai Gow (domino chinezesc)

- **Id:** `paigow`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play paigow` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 32 de piese; 4 piese fiecare, împărțite în față (mâna mică) și spate (mâna mare); față vs față și spate vs spate contra dealerului (rolul alternează; copiile și 0–0 merg la dealer).
- Perechi (Gee Joon > Teen > Day > … > Chop-Ng) > Wong > Gong > puncte modulo 10.
- Ambele câștigate = +10, una și una = remiză. 12 runde, 100 de jetoane.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: aranj 1/2/3, arata, urmatoarea.
- Exemple legale la început: `aranj`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="teenpatti"></a>
## Teen Patti (heads-up)

- **Id:** `teenpatti`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play teenpatti` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 3 cărți fiecare, 12 mâini, 500 de jetoane, boot 10.
- Ierarhie: trio > scară de culoare > scară (AKQ, A23, KQJ…) > culoare > pereche > carte mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pack (abandon), chaal (plătești miza), raise (dublezi, max 80), show (după cel puțin o acțiune; la egalitate pierde cel care cere show); arata și urmatoarea sunt forțate. Maxim 6 acțiuni pe mână.
- Exemple legale la început: `pack`, `chaal`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ganjifa"></a>
## Ganjifa (levate)

- **Id:** `ganjifa`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play ganjifa` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 48 de cărți (4 culori: Taj T, Shamsher S, Surkh R, Chang C;
- Mir M, Vazir V, 1–10).
- La T și S 10 e cel mai mare număr, la R și C ordinea e inversată (1 cel mai mare). 12 cărți fiecare; cartea de sus din stoc arată atu. 12 levate, culoare obligatorie, atu bate.
- Mai multe levate câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: TM, S10, R1, CV.
- Exemple legale la început: `T2`, `TV`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="yut"></a>
## Yut (Yunnori)

- **Id:** `yut`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play yut` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 4 bețe-zar (arunc): 1 plat = do, 2 = gae, 3 = geol, 4 = yut (încă o aruncare), 0 = mo (5, încă o aruncare). 4 piese pe jucător; rezultatele se folosesc în orice ordine; grupurile de pe același nod merg împreună; capturarea trimite la start și dă o aruncare.
- Scurtături din colțurile 5 și 10 (centrul 22).
- Câștigă cine scoate toate piesele.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, do:start, gae:7, mo:22.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sugoroku"></a>
## Sugoroku (cursă cu casete)

- **Id:** `sugoroku`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sugoroku` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Drum 0–40, două zaruri (arunc), alegi un zar sau suma (avansez N).
- Atingi exact 40 ca să câștigi (surplusul te întoarce).
- Casete: 4 +3, 9 −3, 12 și 28 pierzi tura, 15 și 36 încă o aruncare, 18 +5, 21 schimbi locul cu adversarul, 25 −6, 30 +4, 33 −8, 38 −10 (fără lanț).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, avansez 7.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chaupar"></a>
## Chaupar (cauri, 2 jucători)

- **Id:** `chaupar`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chaupar` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 4 piese, circuit de 48 de pătrate (start 0 / 24), 4 cauri: 1–4 = pași, 0 = 8; 4 și 8 dau încă o aruncare; scoți din curte cu 1 sau 8 (intra).
- Castelele (multipli de 6) sunt sigure; captura trimite în curte și dă o aruncare.
- Coloană de acasă și centru exact.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, intra, mut:12, mut:casa2, pas.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="culori"></a>
## Culori (potrivit cărți colorate)

- **Id:** `culori`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play culori` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original: 108 cărți (4 culori cu cifre 0–9, sari, intoarce, plus2, plus joker și joker4).
- Joci o carte de aceeași culoare sau cu aceeași cifră/simbol; în doi, sari/intoarce = adversarul pierde rândul, plus2 = trage 2, joker4 = trage 4 (jokerii aleg culoarea: joker:r).
- Fără carte jucabilă tragi (trage), apoi o joci sau dai pas.
- Câștigă cine rămâne primul fără cărți.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: r5, gsari, vplus2, joker:a, trage, pas.
- Exemple legale la început: `joker4:r`, `joker4:v`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mosii"></a>
## Moșii (afaceri imobiliare cu zaruri)

- **Id:** `mosii`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play mosii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original, 32 de câmpuri: arunci două zaruri (automat), cumperi terenuri și gări (cumpar / las), încasezi chirii (grup complet = chirie dublă, case: construiesc 3), taxe, evenimente Noroc și închisoare (platesc 50 sau dublă); dublă = încă o aruncare.
- Faliment = pierdere; după 1500 de mutări câștigă averea mai mare.
- Fără licitații și ipoteci.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="magistrale"></a>
## Magistrale (rute cu cărți de vagon)

- **Id:** `magistrale`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play magistrale` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original: hartă cu 19 orașe și 42 de rute; strângi cărți de vagon (r a v g m, j = locomotivă) și ocupi rute (ocupa 12 r), tragi 2 cărți (trage / trage r) sau bilete de destinație secrete (bilete, pastrez 1,3).
- La final bilet îndeplinit +valoarea, neîndeplinit −valoarea, drum lung +10.
- În doi, rutele duble se ocupă o singură dată.
- Informație ascunsă: vezi doar cărțile și biletele tale.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pastrez`, `1,2`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="conac"></a>
## Misterul din Conac (deducție)

- **Id:** `conac`
- **Categorie:** Deducție
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play conac` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original: o crimă cu soluție secretă (suspect s1–s6, armă a1–a6, cameră c1–c9); 18 cărți împărțite 9–9.
- Sugerezi (sugerez s2 a3 c5), adversarul îți arată în secret o carte pe care o are (arat s2) sau spune nu-am; deduci și acuzi (acuz s2 a3 c5, apoi verific): corect câștigi, greșit pierzi.
- Fără tablă de mers.
- Informație ascunsă: vezi doar mâna ta și cărțile arătate ție.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `sugerez`, `s1`
- La început sunt 2592 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="insula"></a>
## Insula (colonizare cu resurse)

- **Id:** `insula`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play insula` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original pe 19 hexagoane (pădure l, argilă a, pășune p, lan g, munte m) cu numere 2–12: pui colonii și drumuri (colonie v12, drum e7), arunci zarurile (arunc) ca să primești resurse, la 7 mută tâlharul (bandit h5), construiești colonii/orașe/drumuri, schimbi 4:1 cu banca (schimb l g) și închei (sfarsit).
- Colonie 1 punct, oraș 2, cel mai lung drum (≥5) 2; primul la 10 câștigă.
- Fără cărți de dezvoltare, porturi sau schimb între jucători.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `colonie`, `v0`
- La început sunt 108 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mozaic"></a>
## Mozaic (plăcuțe cu drumuri, cetăți, mănăstiri)

- **Id:** `mozaic`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play mozaic` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original, 65 de plăcuțe: tragi o plăcuță și o lipești cu muchiile potrivite (F câmp, R drum, C cetate; pune(x,y)r1), apoi poți pune un pion pe un drum, o cetate sau o mănăstire (drum:N, oras:E, manastire, sau fara).
- Cetate închisă 2/plăcuță (+2 per scut), drum 1/plăcuță, mănăstire 9; majoritatea ia punctele, la final se numără și ce-i neterminat.
- Fără câmpuri/fermieri.
- Dacă plăcuța nu se potrivește: arunca.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pune(0,-1)r2`, `pune(0,1)r1`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="litere"></a>
## Litere (cuvinte pe tablă)

- **Id:** `litere`
- **Categorie:** Cuvinte
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play litere` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original pe tablă 15×15 cu 97 de plăcuțe (2 blank): formezi cuvinte legate de cele existente, cu pătrate premiate (# cuvânt ×3, + cuvânt ×2, x literă ×3, - literă ×2); +50 cu toate cele 7 plăcuțe.
- Dicționar mic românesc încorporat (~4000 de cuvinte uzuale, fără diacritice; cuvintele absente sunt respinse).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: casa@h8> (orizontal), casa@h8v (vertical), ca(s)a@h8> (blank între paranteze), schimb (toată mâna), pas. Primul cuvânt acoperă h8. Informație ascunsă: vezi doar mâna ta.
- Exemple legale la început: `al@g8>`, `as@g8>`
- La început sunt 80 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="faianta"></a>
## Faianța (variantă originală)

- **Id:** `faianta`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play faianta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Draft de plăcuțe pe modele, 2 jucători: iei toate plăcuțele de o culoare dintr-o manufactură (restul trec în centru) sau din centru (primul ia jetonul de start, −1) și le pui pe o linie de model (1–5, o singură culoare, nu dacă peretele are deja culoarea) sau pe podea (−1…−3).
- La sfârșitul rundei (mutarea forțată decontare) liniile pline urcă pe perete 5×5 și punctează adiacența; jocul se oprește la un rând complet; bonus rând +2, coloană +7, culoare +10.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: f3 T 2, c O p, decontare.
- Exemple legale la început: `f1`, `T`
- La început sunt 342 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bresla"></a>
## Bresla (variantă originală)

- **Id:** `bresla`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play bresla` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Construire de pachet pentru doi: 7 Bani + 3 Parcele, mână de 5.
- Tură: acțiune (joc sat), cumparaturi (comorile se numără automat), cumpar <carte>, gata. 14 cărți: bani/argint/aur, parcelă/moșie/domeniu (1/3/6 p), acțiuni Sat, Meșteșug, Târg, Sărbătoare, Tăietor, Laborator, Atelier (iau <carte>), Cămătar (adversarul aruncă la 3 cărți: dau <carte>).
- Sfârșit: Domeniile epuizate sau 3 rezerve goale; câștigă cine are mai multe puncte.
- Informație ascunsă: ordinea pachetului și mâna adversarului.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `cumparaturi`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cristale"></a>
## Cristale (variantă originală)

- **Id:** `cristale`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play cristale` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Jetoane de minerale și dezvoltări, 2 jucători: o acțiune pe tură — iau G A T (3 culori diferite), iau2 G (2 la fel, dacă sunt ≥4 în bancă), rezerv a1 (+ chihlimbar-joker), cumpar a1 / cumpar rez1.
- Cărțile cumpărate dau reduceri permanente și puncte; peste 10 jetoane arunci (arunc G).
- Patronii (3 p) vizitează automat.
- La 15 puncte se mai joacă până la capătul rundei.
- Cărțile sunt generate de noi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `iau`, `G`
- La început sunt 74 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="curieri"></a>
## Curieri (variantă originală)

- **Id:** `curieri`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play curieri` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Deducție cu 16 cărți: tragi, joci una din două. 1 Iscoadă ghicește (iscoada:5), 2 Crainic se uită, 3 Judecător compară, 4 Străjer protejează, 5 Mag (mag:eu/mag:el) schimbă cartea, 6 Cămăraș schimbă mâinile, 7 Contesă (obligatorie cu 5/6), 8 Prințesă (aruncată = ieși).
- Ultimul rămas sau cartea cea mai mare câștigă runda; primul cu 3 runde câștigă.
- Informație ascunsă: vezi doar cartea ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `iscoada:2`, `iscoada:3`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fitil"></a>
## Fitil (variantă originală)

- **Id:** `fitil`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play fitil` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- O Bombă în pachet: joci orice cărți, apoi tragi (trag).
- Fără Dezamorsare, bomba te scoate; cu ea o repui unde vrei (dezamorsez 0..N).
- Cărți: atac (adversarul trage de 2 ori), sari, amesteca, privire (primele 3), cere (adversarul îți dă o carte: dau X), pereche A|B|C (furi o carte la întâmplare).
- Informație ascunsă: vezi doar mâna ta, privirea ta și locul bombei puse de tine.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `trag`, `juc`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bufet"></a>
## Bufet (variantă originală)

- **Id:** `bufet`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play bufet` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Draft rapid de 10 cărți, 3 runde: alegi o carte (alege mici), mâinile se schimbă după revelare (revelare).
- Mici (perechi 5), salată (triplete 10), colțunași (1/3/6/10/15), cârnați 1-2-3 (muștarul îi triplează), pateuri (majoritate 6/3), papanași (la final cei mai mulți +6, cei mai puțini −6).
- Alegerea adversarului rămâne ascunsă până la revelare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `mici`
- La început sunt 14 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="expeditii"></a>
## Expediții

- **Id:** `expeditii`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play expeditii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci masive (B F R D C), cărți 2–10 și 3 angajamente (x) pe culoare; mână de 8.
- Tură: joci o carte pe expediția ta (valori crescătoare) sau o arunci (juc B5, arunc B5), apoi tragi din pachet sau iei de pe o grămadă (trag, ia F).
- Scor pe expediție: (suma − 20) × (1 + angajamente), +20 cu ≥8 cărți.
- Se termină la ultima carte din pachet.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `juc`, `B3`
- La început sunt 32 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="artificii"></a>
## Artificii (cooperativ, partener AI)

- **Id:** `artificii`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play artificii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tu și AI-ul construiți 5 șiruri de artificii (R G V A W) de la 1 la 5; vezi mâna partenerului, nu pe a ta.
- Tură: juc N, arunc N (+1 indiciu) sau indiciu R / indiciu 3 (cost 1 din 8 jetoane). 3 fitile arse = pierdeți.
- Obiectiv: 20 de puncte (echipa reușește = „Câștigă Jucătorul 1”; altfel remiză).
- Încheierea e o mutare forțată (sfârșit).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `juc`, `1`
- La început sunt 22 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="orasel"></a>
## Orășel (variantă originală)

- **Id:** `orasel`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play orasel` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Zaruri și construit orașe: arunci 1 sau 2 zaruri (după Gară), clădirile albastre plătesc în orice tură, verzi doar în a ta, roșii (cafenea, grătar) iau de la cel care aruncă; cumperi o clădire (cumpar lan) sau un reper (construiesc gara/mall/parc/turn).
- Câștigă cine are primul cele 4 repere.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc 1, reiau, pastrez, nimic.
- Exemple legale la început: `arunc`, `1`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="capcana"></a>
## Capcana (variantă originală)

- **Id:** `capcana`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play capcana` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Bluf și licitație cu discuri ascunse: 3 flori + 1 capcană.
- Pui discuri (pun floare / pun capcana), apoi licitezi (licitez N) câte discuri poți întoarce fără capcană; celălalt ridică sau dă pas.
- Provocatorul întoarce întâi ale lui, apoi ale adversarului (intorc). 2 reușite câștigă; capcana te costă un disc.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pun`, `floare`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="congkak"></a>
## Congkak (Malaezia)

- **Id:** `congkak`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play congkak` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 × 7 case, 7 semințe în fiecare, depozit pe jucător; semănat trigonometric, depozitul adversarului se sare.
- Ultima în depozit = încă o mutare; în casă ocupată = ridici și continui; în casă goală proprie = capturezi casa opusă plus sămânța ultimă (opusa goală: doar sămânța ultimă); în casă goală adversă = tura se termină.
- Runda se încheie când jucătorul la mutare nu mai are semințe; apoi casele se reumplu din depozite (7 fiecare), cele nerefăcute sunt arse.
- Cine nu mai poate umple nicio casă pierde.
- Mutări pe rând, nu simultan.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–7.
- Exemple legale la început: `1`, `2`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pallanguzhi"></a>
## Pallanguzhi (Tamil Nadu)

- **Id:** `pallanguzhi`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pallanguzhi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 × 7 cupe, 5 semințe în fiecare.
- Ridici o cupă proprie și semeni trigonometric; apoi: cupa următoare ocupată = o ridici și continui; goală = capturezi cupa de după ea și tura se termină.
- Captură suplimentară: orice cupă care ajunge la exact 4 semințe e capturată.
- Fără semințe la mutare = jocul se termină, fiecare ia ce are pe partea lui; câștigă cine are peste 35.
- O singură rundă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–7.
- Exemple legale la început: `1`, `2`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ayo"></a>
## Ayo (Ayoayo, Yoruba)

- **Id:** `ayo`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ayo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 × 6 gropi, 4 semințe în fiecare; semănat trigonometric cu semănat în lanț (ultima într-o groapă ocupată = continui) și groapa de start sărită.
- Captură: ultima sămânță într-o groapă goală proprie ia groapa opusă (fără sămânța ultimă).
- Dacă adversarul nu mai are semințe, ești obligat să-l hrănești; dacă nu poți, jocul se termină.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–6.
- Exemple legale la început: `1`, `2`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="battleship"></a>
## Bătălia navală (10×10)

- **Id:** `battleship`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play battleship` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Flotă de 5 nave (5, 4, 3, 3, 2).
- Îți așezi navele (a1h = orizontal, a1v = vertical, sau aleator); flota AI-ului se așază printr-o mutare forțată (plasez) și rămâne ascunsă.
- Tragi pe rând (e5): apă, lovit sau scufundat.
- AI-ul vânează cu hartă de densitate și țintește, fără să citească flota ta.
- Final cu mutarea forțată recunosc.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1h, aleator, plasez, e5, recunosc.
- Exemple legale la început: `a1h`, `a1v`
- La început sunt 121 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mastermind"></a>
## Mastermind (cu schimb de roluri)

- **Id:** `mastermind`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mastermind` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cod de 4 cifre 1–6 (repetiții permise), cel mult 10 încercări; răspuns N = poziție exactă, A = culoare corectă pe altă poziție.
- Runda 1: spargi codul ascuns al AI-ului.
- Runda 2: îți alegi un cod (cod:1234) și AI-ul îl sparge.
- Câștigă cine a avut nevoie de mai puține încercări (nespart = 11).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1234, cod:1234, confirm.
- Exemple legale la început: `1111`, `1112`
- La început sunt 1296 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="memory"></a>
## Memory (perechi)

- **Id:** `memory`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play memory` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 24 de cărți (12 perechi A–L), 6 coloane × 4 rânduri.
- O tură = două întoarceri (mutări separate): pereche = punct și mută din nou, altfel cărțile se întorc și mută adversarul.
- AI-ul are memorie limitată (memorează 70% din cărțile văzute, uită cu timpul).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1–f4.
- Exemple legale la început: `a1`, `b1`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bingo"></a>
## Bingo (75 de bile)

- **Id:** `bingo`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play bingo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Fișă 5 × 5 (B 1–15, I 16–30, N 31–45 cu centru liber, G 46–60, O 61–75).
- Tragerea e o mutare (trag), marcarea o alta (marchez); cu o combinație (rând, coloană, diagonală) strigi sau continui.
- Cine strigă singur câștigă; dacă strigă amândoi = remiză.
- Fișa AI-ului e ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trag, marchez, strig, continui.
- Exemple legale la început: `trag`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="loteria"></a>
## Lotería (imagini)

- **Id:** `loteria`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play loteria` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 4 × 4 cu 16 din 54 de imagini (set propriu, nu cel comercial).
- Tragi o imagine (trag), o marchezi dacă o ai (marchez); combinații: rând, coloană, diagonală, cele patru colțuri sau pătratul din centru; strigi sau continui.
- Fișa AI-ului e ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trag, marchez, strig, continui.
- Exemple legale la început: `trag`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pachisi"></a>
## Pachisi (cursă cu cauri)

- **Id:** `pachisi`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu (informație completă; zarurile/amestecul nu se consideră ascunse)
- **Pornire:** `polytabletop play pachisi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 6 cauri: 1 deschis = 10, 2 = 2, 3 = 3, 4 = 4, 5 = 25, 6 = 6, niciunul = 12; 6, 10, 12 și 25 sunt grațioase (încă o aruncare și poți scoate o piesă din bază).
- Pistă de 48 de pătrate, castele sigure din 6 în 6, două piese proprii = bloc, aterizarea pe o piesă singură o captură (+ aruncare).
- Ținta exactă de pe pistă; din culoarul de acasă orice aruncare ≥ distanța.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: intră, sau progresul piesei (ex. 12).
- Exemple legale la început: `intră`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="carrom"></a>
## Carrom (discret, fără fizică)

- **Id:** `carrom`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play carrom` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 9 × 9, 12 monede pe jucător, damă, patru buzunare în colțuri.
- Lovitura = coloană + direcție (L/S/R) + forță 1–3, traseu deterministic cu ricoșeu; monedă proprie băgată = +1 și mai tragi; moneda adversarului = punct pentru el; dama se acoperă la lovitura următoare (+3) sau revine; strikerul în buzunar = greșeală.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: eS3, aL1.
- Exemple legale la început: `bL1`, `bL2`
- La început sunt 63 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="arhipelag"></a>
## Arhipelagul în Flăcări (cucerire cu zaruri)

- **Id:** `arhipelag`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play arhipelag` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 14 insule în 4 regiuni cu bonus, 7 insule și 3 armate fiecare la start.
- Tura: întărire (pune Fiord / pune Fiord*), atac (ataca Fiord>Prag, apoi mutarea forțată arunc: 3 zaruri contra 2, egalitatea o câștigă apărătorul; gata atac), o mutare de armate (muta A>B n) sau gata.
- Câștigă cine rămâne singur; după 70 de ture, cine are mai multe insule.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pune`, `Fiord`
- La început sunt 28 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="roata"></a>
## Roata Destinului (parcurs de viață)

- **Id:** `roata`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play roata` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: roata (1–6) e afișată; mutarea avansez alege ramura la bifurcații (facultate/ucenicie, afacere/salariu) și, pe pătratul Casă, cumpar/nu.
- Plată la fiecare trecere, copii și casă valorează la final.
- Scor = bani + 30 casă + 8 per copil.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: avansez, avansez stanga, avansez cumpar.
- Exemple legale la început: `avansez`, `facultate`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="zaruri5"></a>
## Cinci Zaruri (13 categorii)

- **Id:** `zaruri5`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play zaruri5` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 13 runde, 5 zaruri, până la 3 aruncări.
- Aruncarea e mutare forțată (arunc); între aruncări ții zaruri (tine 3 3 5, tine -) sau scrii o categorie (scor casa).
- Bonus 35 la 63 sus; fără regula joker.
- Câștigă totalul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="placute"></a>
## Plăcuțe (seturi și serii)

- **Id:** `placute`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play placute` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 106 plăci (4 culori ×1–13 ×2 + 2 jokeri), 14 în mână.
- Pune serii/grupuri (pune r3 r4 r5), adaugă la masă (adauga r6 la 2), schimbă un joker (schimb J din 2 cu r4); prima ieșire ≥ 30.
- Terminare: gata; altfel trag / pas; anuleaza.
- Fără reorganizarea liberă a mesei.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pune`, `g4`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="turnul"></a>
## Turnul Șovăitor (turn de blocuri)

- **Id:** `turnul`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play turnul` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Simulare discretă, deterministă: 12 straturi de 3 blocuri; iei un bloc de sub stratul complet de sus și îl pui deasupra.
- Turnul stă dacă centrul de masă de deasupra fiecărui strat cade în învelișul blocurilor lui rămase; cine îl prăbușește pierde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ia 4b pune a.
- Exemple legale la început: `ia`, `1a`
- La început sunt 396 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cunoastere"></a>
## Cupa Cunoașterii (întrebări, 6 felii)

- **Id:** `cunoastere`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cunoastere` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Banca proprie de 126 de întrebări în 6 categorii; alege o categorie (alege istorie), răspunde a–d (raspund b), apoi verific.
- Corect = felia și încă o întrebare; greșit = trece tura.
- Prima 6/6 câștigă.
- AI-ul nu are cunoștințe (ghicește); pentru meci echilibrat jucați în doi oameni.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `geografie`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ceata"></a>
## Câmpul Ceții (piese ascunse, 10×10)

- **Id:** `ceata`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play ceata` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 40 de piese pe parte, ranguri 1–X, mine (B), steag (F), lacuri.
- Vezi doar piesele tale și pe ale adversarului după bătălie.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e4-e5, e4xe5 (atac), apoi mutarea forțată bataie. Săpătorul dezamorsează minele, Iscoada bate Mareșalul. 100 de mutări fără bătălie = remiză.
- Exemple legale la început: `a4-a5`, `b4-b5`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="umbra"></a>
## Umbra Urbei (urmărire cu bilete)

- **Id:** `umbra`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play umbra` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală, J1 = 4 detectivi, J2 = Umbra (ascunsă; apare după mutările 4, 9, 14, 19).
- Hartă de 30 de stații pe jos/tram/metro cu bilete limitate;
- Umbra are 3 bilete negre.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d2 17 tram, x 12 jos. Detectivii câștigă prinzând sau blocând Umbra; Umbra, rezistând 22 de mutări. Se afișează stațiile posibile.
- Exemple legale la început: `d1`, `2`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="zidurile"></a>
## Zidurile Mișcătoare (labirint cu plăci culisante)

- **Id:** `zidurile`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play zidurile` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tablă 7×7 cu plăci mobile; introduci placa de rezervă (mete N2 r1) apoi muți pionul pe un drum deschis (mergi c3) ca să iei comorile în ordine, apoi acasă.
- Nu poți anula inserția adversarului.
- După 100 de ture, cine are mai multe comori.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `mete`, `N2`
- La început sunt 144 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="prajituri"></a>
## Drumul Prăjiturilor (cursă de culori)

- **Id:** `prajituri`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play prajituri` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 58 de pătrate colorate, pachet de cărți (simple, duble, locuri), scurtături și pătrate lipicioase.
- Cartea e afișată; singura mutare este trag.
- Primul în Palatul de zahăr câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `trag`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="portret"></a>
## Portretul Misterios (ghicește personajul)

- **Id:** `portret`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play portret` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 24 de personaje inventate cu 14 trăsături.
- Întrebi (intreb ochelari), adversarul răspunde corect prin mutare forțată (raspund da/nu); ghicești (ghicesc Matei) și apoi verdict corect/gresit.
- Ghicit greșit = pierzi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `intreb`, `ochelari`
- La început sunt 76 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="briscola"></a>
## Briscola (Italia)

- **Id:** `briscola`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play briscola` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet italian de 40 de cărți (culori d, c, s, b; rang 2 4 5 6 7 F C R 3 A), 3 în mână, atuul întors rămâne jos.
- Nu e obligatoriu să răspunzi la culoare; levata o ia cartea mai mare de aceeași culoare sau un atu; câștigătorul trage primul.
- Puncte: A 11, 3 10, R 4, C 3, F 2; peste 60 câștigi, 60–60 remiză.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: Ad, 3s, Rb, 7c.
- Exemple legale la început: `5c`, `Cc`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="scopa"></a>
## Scopa (Italia)

- **Id:** `scopa`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play scopa` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet italian de 40 de cărți (d c s b; rang 1–7 F C R, valori 1–10), 3 în mână, 4 pe masă.
- Joci o carte: dacă pe masă e aceeași valoare, iei acea carte; altfel iei un grup cu aceeași sumă; altfel o lași.
- Golești masa = scopa (1 punct, nu la ultima carte).
- Punctaj: carte, denari, settebello (7d), primiera, scope; meci până la 11.
- Informație ascunsă: vezi doar mâna Jucătorului 1; împărțirea următoare e mutare forțată.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7d, 7d:3c+4b, 7d:7s, imparte, urmatoarea.
- Exemple legale la început: `2b:2c`, `1s`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="durak"></a>
## Durak (Rusia)

- **Id:** `durak`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play durak` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 36 de cărți (6..A, culori c d h s), 6 în mână, atu întors jos.
- Atacatorul pune o carte; apărătorul o bate (mai mare de aceeași culoare sau atu) ori ia.
- Atacatorul adaugă cărți cu valori deja pe masă (cel mult 6, cel mult câte are apărătorul; 5 la început); gata = rundă încheiată, apărătorul devine atacator.
- După fiecare rundă se trage până la 6 (mutare forțată umple).
- Fără pachet, cine rămâne primul fără cărți câștigă, celălalt e durak; ambii = remiză.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 6h, 9h>6h (bate 6h cu 9h), iau, gata, umple.
- Exemple legale la început: `10d`, `Kc`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="belote"></a>
## Belote (Franța, 2 jucători)

- **Id:** `belote`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play belote` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 32 de cărți (7..A; c d h s), 5 în mână + o carte întoarsă.
- Licitație în două tururi: prends (atu = culoarea cărții întoarse) / pas, apoi prends-c|d|h|s; toți pas = redistribuire.
- Cine ia primește cartea și se împarte până la 8 (imparte); 16 cărți rămân nejucate.
- Obligatoriu culoarea, altfel tai cu atu, la atu urci.
- Atu: J 20, 9 14, A 11, 10 10, K 4, Q 3; altfel A 11, 10 10, K 4, Q 3, J 2.
- Belote 20, dix de der 10, capot 90; contractul cade dacă ai mai puțin decât adversarul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: prends, pas, prends-h, imparte, Jh, 10s, urmatoarea.
- Exemple legale la început: `prends`, `pas`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="skat"></a>
## Skat (2 jucători)

- **Id:** `skat`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play skat` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 32 de cărți (7..A; c ♣, s ♠, h ♥, d ♦), 10 în mână, 2 în skat, 10 ascunse nejucate.
- Licitația: cine a împărțit spune 18 20 22 23 24 27 … (licit N), celălalt ține (tin) sau pas; declarantul ia skat-ul (skat) și pune 2 cărți (pune 10h Jc) sau joacă din mână (hand), apoi alege jocul: trefla, pica, cupa, caro (atu = juniorii + culoarea), grand (doar juniorii), null (declarantul nu ia nicio levată).
- Valoare = bază × (matadori + 1 + mână + schneider + schwarz); câștig +valoarea, pierdere −2×, supralicitare = pierdere. 61 din 120 de puncte.
- Meci de 6 mâini.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: licit 18, tin, pas, skat, trefla, grand, Jh, reimparte, urmatoarea.
- Exemple legale la început: `licit`, `18`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tarot"></a>
## Tarot francez (78 de cărți, 2 jucători)

- **Id:** `tarot`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tarot` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 78 de cărți (c d h s: 1–10 V C D R; atuuri T1–T21;
- Scuza Ex), 36 în mână și 6 în chien.
- Licitație într-un singur tur: pas, petite, garde, garde-sans, garde-contre (multiplicatori 1/2/4/6); cine ia primește chien-ul (chien) și pune jos 6 cărți (ecart Vh).
- Culoare obligatorie, altfel taie cu atu, la atu urci;
- Scuza nu câștigă. 91 de puncte; cerut 56/51/41/36 cu 0/1/2/3 oudlers; scor (25 + diferența + 10 petit au bout) × multiplicator, zero-sum; meci de 3 mâini.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: petite, pas, chien, ecart 2h, T21, Rs, Ex, reimparte, urmatoarea.
- Exemple legale la început: `pas`, `petite`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cribbage"></a>
## Cribbage

- **Id:** `cribbage`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play cribbage` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 jucători, 6 cărți: dai 2 în crib (al dealerului), se taie startul (valet = 2 pentru dealer), pegging până la 31 (15, 31, perechi, serii, go) și numărarea mâinilor și a cribului (15-uri, perechi, serii, culoare, valet).
- Tabla de colac de 121 de găuri e desenată în text.
- Primul la 121 câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d 5h 6s, taie, 5h, go, numara, imparte.
- Exemple legale la început: `d`, `9d`
- La început sunt 45 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="euchre"></a>
## Euchre (2 jucători)

- **Id:** `euchre`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play euchre` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 24 de cărți (9–A), 5 fiecare, o carte întoarsă.
- Runda 1: pas / up (dealerul ia cartea și dă una); runda 2: numești altă culoare (dealerul e obligat).
- Valetul de atu (bower dreapta) și cel de aceeași culoare (stânga, tot atu) sunt cele mai mari.
- Maker: 3–4 levate = 1 punct, 5 = 2, euchre = 2 pentru adversar; până la 10.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pas, up, nume h, dis 9h, Jh, imparte.
- Exemple legale la început: `pas`, `up`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tarneeb"></a>
## Tarneeb (2 jucători)

- **Id:** `tarneeb`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tarneeb` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 13 cărți fiecare din 52 (celelalte 26 ascunse).
- Licitezi levate 7–13 (urci cu 1 sau 2), câștigătorul numește atuul și conduce; făcut = punctele pe levatele luate (16 la 13), pierdut = minus cât ai licitat; adversarul ia levatele lui (+3 dacă ai căzut); până la 31.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pas, 7…13, atu h, Ah, imparte.
- Exemple legale la început: `pas`, `7`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="baloot"></a>
## Baloot (2 jucători)

- **Id:** `baloot`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play baloot` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 32 de cărți, 8 fiecare, o carte întoarsă.
- Licitație în două runde: hokm (atu), sun (fără atu) sau pas.
- Atu: J 20, 9 14, A 11, 10 10, K 4, Q 3; ultima levată +10; la hokm tai obligatoriu cu atu și urci peste atu.
- Cumpărătorul trebuie să aibă mai multe puncte decât adversarul; baloot (K+Q de atu) = +2; până la 50.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pas, hokm, hokm h, sun, Jh, imparte.
- Exemple legale la început: `pas`, `hokm`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tablanet"></a>
## Tablanet

- **Id:** `tablanet`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tablanet` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți: 4 pe masă, câte 6 fiecare, de patru ori.
- Capturezi cărți egale sau grupuri care se adună la valoarea ta (A = 1 sau 11;
- J, Q, K doar egale); captura e obligatorie și maximă.
- Punctaj: cele mai multe cărți 3, cele mai multe treflă 1, 10♦ 2, 2♣ 1, fiecare As 1, fiecare tablă 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7h, 7h x 3c 4d, imparte.
- Exemple legale la început: `4h`, `Qd`
- La început sunt 14 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="truco"></a>
## Truco (Argentina)

- **Id:** `truco`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play truco` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet spaniol de 40, 3 cărți fiecare, ierarhie specială (1 de spadă cea mai mare), 3 runde, până la 15.
- Cânți truco / retruco / vale4 (cel provocat: quiero, noquiero sau urcă) și envido / real / falta în prima rundă; mazo = te retragi.
- Fără flor.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7e, truco, envido, quiero, noquiero, cuenta, mazo, reparte.
- Exemple legale la început: `1e`, `12b`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="trix"></a>
## Trix (2 jucători)

- **Id:** `trix`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play trix` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 32 de cărți, 16 fiecare, 10 mâini (5 „împărăția” fiecăruia).
- Proprietarul alege contractul: king (K♥ −75), queens (−25 fiecare), diamonds (−10 fiecare), slaps (−15 pe levată) sau trex (aranjament pornit din J, primul care termină +200).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: king, queens, diamonds, slaps, trex, Jh, pas, imparte.
- Exemple legale la început: `king`, `queens`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bridge"></a>
## Bridge duplicat (2 jucători)

- **Id:** `bridge`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play bridge` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Două tabele cu aceeași donă: J1 conduce N–S la tabla 1 și E–V la tabla 2 (J2 invers); fiecare vede ambele mâini ale cuplului lui.
- Licitație (1c…7n, x, xx, pass), declarant Sud / Vest, mâna moartă vizibilă după prima carte; scor duplicat (nimeni vulnerabil).
- Câștigă scorul combinat mai bun.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1h, 3n, x, xx, pass, Ah.
- Exemple legale la început: `pass`, `1c`
- La început sunt 36 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="canasta"></a>
## Canasta (2 jucători, o mână)

- **Id:** `canasta`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play canasta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 108 de cărți, 15 fiecare; 2 și jokerii sunt wild, 3 roșii = bonus, 3 negre doar se aruncă.
- Tura: roba (2 cărți) sau ia aruncarea, combinații de 3+ cărți de același rang (cu cel puțin 2 naturale), arunci o carte.
- Canastă (7 cărți) = 500 naturală / 300 mixtă; ieși doar cu canastă.
- O mână, fără minim de coborâre; câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: roba, ia, m 7, mw 7, w K, d 9c.
- Exemple legale la început: `roba`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="nave"></a>
## Escadrile din Rift (manevre secrete)

- **Id:** `nave`
- **Categorie:** Miniaturi
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play nave` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Manevre planificate simultan: 3 nave pe parte (Bivol, Șoim, Vrabie) pe o hartă 12×12 cu asteroizi.
- Fiecare navă primește în secret o manevră (cadran): drept dN, deviere blN/brN, viraj tlN/trN, întoarcere u, oprire s; roșii (viraj 3, u, s) dau stres, iar o navă stresată nu poate alege roșu.
- Apoi revelarea mută forțat, în ordinea îndemânării pilotului; ciocnire = oprire și fără foc; asteroid = 1 daună.
- Foc în arcul din față (Bivolul: turelă) la distanță 1–3, zaruri de atac contra zaruri de apărare, scut apoi coca. 10 runde.
- Informație ascunsă (AI-ul nu vede planurile tale).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: A d2, A bl1, A tr2, A u, A s, revela, A>X, A pas, arunca.
- Exemple legale la început: `A`, `d1`
- La început sunt 28 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="patrula"></a>
## Patrula (skirmish cu echipă mică)

- **Id:** `patrula`
- **Categorie:** Miniaturi
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play patrula` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 5 miniaturi individuale pe parte (Lider, 2 Pușcași, Greu, Asalt) pe o hartă 10×10 cu ziduri, acoperiri și 4 obiective; fiecare are răni proprii, armă de foc și armă de luptă.
- Părțile alternează activări (fiecare miniatură o dată pe rundă) cu ordine: Avans (mută și poate trage), Țintă (pe loc, +1 la lovire; lipit = luptă), Asalt, Retragere (doar lipit), Gardă. 6 runde; obiectivele dau puncte la finalul rundei.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: A>d4, A>d4 xV, A xV, A!d4 xV, A<d4, A garda, arunca.
- Exemple legale la început: `A`, `garda`
- La început sunt 605 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="reactii"></a>
## Ordine și reacții (skirmish cu reacții)

- **Id:** `reactii`
- **Categorie:** Miniaturi
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play reactii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 4 soldați pe parte (Lider, Pușcaș, Lunetist, Asalt), hartă 10×10, zaruri d20.
- Partea activă are un ordin per soldat (+1 pentru Lider); fiecare ordin mută și/sau trage.
- În tura adversarului, inamicii care văd soldatul activ reacționează pe rând: pas, tira (reacție) sau esiva (doar ținta declarată); față în față câștigă cel mai mare rezultat reușit, egalul la critic. 8 runde; obiective.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: A>d4 xV, A xV, A>d4, fin, V pas, V tira, V esiva, arunca.
- Exemple legale la început: `fin`, `A`
- La început sunt 433 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ordine"></a>
## Ordine de bătaie (infanterie, zaruri de ordine)

- **Id:** `ordine`
- **Categorie:** Miniaturi
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ordine` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Bătălie de infanterie pe hartă 12×8: Ofițer, 2 Grupe, Mitralieră și Mortier pe parte.
- Un zar de ordine pe unitate într-un săculeț; mutarea forțată trage extrage un zar la întâmplare și partea respectivă dă un ordin: fire, adv, run (asalt), amb, rally, down.
- Suprimare: marcajele de suprimare scad moralul și trebuie testate pentru a executa ordinul. 6 ture; obiective.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, B fire xW, B adv d4 xW, B run d4, B amb, B rally, B down, arunca.
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="gridiron"></a>
## Gridiron (sport fantastic pe ture)

- **Id:** `gridiron`
- **Categorie:** Miniaturi
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play gridiron` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Echipe de 5 pe un teren 12×7; 6 ture pe repriză, 2 reprize.
- O acțiune per jucător: mutare (escapare, ridicare minge, mutări în plus cu zar), tăvăleală cu zaruri de forță, asalt (mutare + tăvăleală), pasă, predare.
- O cădere sau o minge scăpată pierde tura.
- Purtătorul mingii în zona adversă = punct.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: A>d4, A>d4 xV, A xV, A pass B, A hand B, A sus, fin, lovitura, arunca.
- Exemple legale la început: `lovitura`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="focare"></a>
## Focare (cooperativ, boli pe hartă, partener AI)

- **Id:** `focare`
- **Categorie:** Cooperativ
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play focare` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: voi doi, agronomi, opriți 4 boli ale culturilor pe o hartă cu 24 de așezări.
- Tura: 4 acțiuni (mers X, zbor X, charter X, naveta X, statie, tratez r, dau/iau, vindec r, pas), apoi tragi 2 cărți (trage; 3 epidemii în pachet; max 7 cărți: descarta X) și se infectează (infecteaza).
- Al 4-lea spor dintr-o boală produce focar în lanț.
- Vindecare: la stație, 3 cărți de aceeași culoare.
- Victorie (toate 4 vindecate) = „Câștigă Jucătorul 1”; înfrângere (6 focare, spori epuizați, pachet golit) = remiză.
- Încheierea e mutare forțată (sfârșit).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `mers`, `bora`
- La început sunt 60 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="strajile"></a>
## Străjile Codrului (cooperativ de apărare, cărți de putere)

- **Id:** `strajile`
- **Categorie:** Cooperativ
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play strajile` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: voi, două duhuri (J1 Rădăcinilor, J2 Furtunii, AI), apărați o vale cu 8 ținuturi de invadatori.
- Tura: creștere (creste N / aduna / reia), apoi până la 2 cărți de putere (joc spini 3; plătești energia, ținut în raza cărții) și gata.
- Apoi runda invadatorilor, mutări forțate: devastare, constructie, explorare.
- Colonie ucisă = frică.
- Victorie: frică 28 sau valea curată după ultima carte de invadatori (= „Câștigă Jucătorul 1”); 10 blesteme sau invadatori rămași = remiză.
- Încheierea e mutare forțată (sfârșit).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `creste`, `2`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="farul"></a>
## Farul Fantomei (deducție cooperativă cu indicii vizuale)

- **Id:** `farul`
- **Categorie:** Cooperativ
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play farul` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tu ești mediumul (J1), AI-ul este fantoma (J2) care îți trimite „viziuni” de câte 3 simboluri (viziune N).
- Patru etape: portretul, camera, obiectul (6 candidați cu câte 5 simboluri, ghic N sau asteapt; max 10 viziuni) și scena finală (3 candidați, o singură ghicire).
- Ghicirea se verifică într-o mutare forțată (verific).
- Victorie = „Câștigă Jucătorul 1”; eșec = remiză.
- Informație ascunsă: nu vezi soluția, nici mâna fantomei.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ghic`, `1`
- La început sunt 13 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cenusii"></a>
## Noapte Cenușie (cooperativ de supraviețuire pe grilă)

- **Id:** `cenusii`
- **Categorie:** Cooperativ
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cenusii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: Ilinca (J1) și Radu (AI) strâng 3 baterii (B) și ajung amândoi la ieșire (E) pe o grilă 7×6, ținând piept umbrelor. 3 acțiuni pe tură: mers n|s|e|v, atac 1 c3 (zaruri într-o mutare forțată: zaruri), cauta (extrage), ia, pansez, gata; apoi umbre și aparitie (forțate).
- Rătăciți, alergători și colosi (necesită armă cu forță 2). 4 răni = mort.
- Victorie = „Câștigă Jucătorul 1”; moartea, roi sau 40 de ture = remiză.
- Încheierea e mutare forțată (sfârșit).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `mers`, `n`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cripta"></a>
## Cripta Clopotelor (aventură tactică cu cărți de acțiune)

- **Id:** `cripta`
- **Categorie:** Cooperativ
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cripta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: Paznicul de Far (J1) și Vânătoarea de Umbre (AI) coboară într-o criptă; 3 scenarii (omorâți toți monștrii / ajungeți la ieșire / omorâți Regele Clopot).
- Rundă: fiecare alege 2 cărți (joc 3 5) sau odihna; revela; apoi în ordinea inițiativei jucați sus 3 jos 5: mers c4, atac m2 / atac toti, aplic, sari; atacul trage modificatorul într-o mutare forțată (trage).
- Monștrii (monstri) trag cărți de comportament.
- Victorie = „Câștigă Jucătorul 1”; ambii epuizați sau 25 de runde = remiză.
- Încheierea e mutare forțată (sfârșit).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `joc`, `1`
- La început sunt 168 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="lupii"></a>
## Lupii din Hârtop (roluri ascunse, masa = roboți)

- **Id:** `lupii`
- **Categorie:** Party
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play lupii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tu ești locul 1 la o masă de 8–9; ceilalți sunt roboți care văd doar informația publică.
- Noaptea acționează lupii și rolurile speciale (văzător, paznic/doctor, vrăjitoare, vânător); ziua toți votează cine pleacă, rolul celui eliminat se arată.
- Satul câștigă fără lupi; lupii când sunt cât restul.
- Alegi presetarea la început: preset mafie (clasic), preset sat (stil francez), preset stepa (stil chinezesc).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: preset sat, ucide 3, cerceteaza 3, pazeste 3, salveaza, otraveste 3, dormi, vot 3, impusca 3, masa.
- Exemple legale la început: `preset`, `mafie`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="santinela"></a>
## Străjerii Farului (misiuni cu echipe, roluri ascunse)

- **Id:** `santinela`
- **Categorie:** Party
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play santinela` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci locuri (tu + 4 roboți): trei paznici (unul e Ochiul, care știe sabotorii) și doi sabotori (unul e Vulpea).
- Cinci misiuni cu echipe de 2, 3, 2, 3, 3: conducătorul propune, toți votează, cei din echipă aleg în secret succes sau sabotaj (doar sabotorii pot sabota). 3 reușite = paznicii, dar Vulpea poate ghici Ochiul; 3 ratate sau 5 respingeri = sabotorii.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: echipa 1 3, da, nu, succes, sabotaj, tinta 4, masa.
- Exemple legale la început: `da`, `nu`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="consiliul"></a>
## Sfatul Negustorilor (alegeri și legi, roluri ascunse)

- **Id:** `consiliul`
- **Categorie:** Party
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play consiliul` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci locuri (tu + 4 roboți): trei negustori, un complice și Cămătarul (se cunosc între ei).
- Președintele nominalizează un cancelar, toți votează; guvernul ales trage 3 legi, una e aruncată, una promulgată (6 P Prosperitate, 11 M Monopol în pachet). 5 P sau Cămătarul executat = negustorii; 6 M sau Cămătarul cancelar după 3 M = conspiratorii.
- A 3-a lege M: vezi pachetul; a 4-a și a 5-a: execuție.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cancelar 3, da, nu, arunca M, promulga P, executa 4, masa.
- Exemple legale la început: `da`, `nu`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="valea"></a>
## Valea Uscată (duel western, roluri ascunse)

- **Id:** `valea`
- **Categorie:** Cărți
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play valea` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci pistolari (tu + 4 roboți): Șerif (public), Adjunct, doi Haiduci, Trădător.
- Legea câștigă dacă mor haiducii și trădătorul, haiducii dacă moare Șeriful, trădătorul rămânând singur.
- Pe tură tragi 2 cărți: foc (o dată, distanță 1; 2 cu pușcă), ocolire, leac, salon, mitralieră, pușcă, panică; arunci la câte vieți ai.
- Secetă după runda 30.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: foc 3, leac, salon, mitraliera, pusca, panica 2, incheie, scarta foc, ocolire, accept, masa.
- Exemple legale la început: `accept`, `ocolire`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hruba"></a>
## Hoinari prin Hrubă (monștri și comori, masa = roboți)

- **Id:** `hruba`
- **Categorie:** Cărți
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play hruba` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Trei hoinari (tu + 2 roboți); primul la nivelul 10 omorând un monstru câștigă.
- Deschizi ușa: monstru (lupți: forța ta trebuie să DEPĂȘEASCĂ nivelul lui; poți folosi poțiuni +3; sau fugi cu zar 5+), blestem sau carte în mână; fără monstru poți căuta sau vâna un monstru din mână.
- Roboții pot îngreuna lupta.
- Max 5 cărți la final de tură; limită 90 de ture.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: deschide, cauta, echipeaza arma3, vaneaza monstru4, crestere, forta, lupta, fugi, scapa blestem, masa.
- Exemple legale la început: `deschide`, `echipeaza`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="apus"></a>
## Imperii în Apus (cucerire cu popoare în declin)

- **Id:** `apus`
- **Categorie:** Strategie
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play apus` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Hartă de 12 ținuturi, trei jucători (tu + 2 roboți), 8 ture.
- Alegi o combinație popor + dar din șase (cea din față e gratis, celelalte costă monede lăsate pe cele sărite).
- Un ținut costă 2 + 1 (munte) + jetoanele ocupantului; trebuie să fie vecin cu ale tale; ultima cucerire cu zarul de întărire.
- Câștigi 1 monedă pe ținut + bonusuri; poporul vlăguit intră în declin și alegi altul.
- Mai multe monede câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: alege 2, continua, cucereste 5, zar 7, gata, declin.
- Exemple legale la început: `alege`, `0`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fulger"></a>
## Fulger (simbolul comun, cronometru simulat)

- **Id:** `fulger`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play fulger` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 31 de cărți cu 6 simboluri; oricare două au exact un simbol comun.
- Numești simbolul comun dintre cartea ta și cea din mijloc; masa (robot) are un timp de reacție tras la zar și greșește uneori.
- Cine e corect și mai rapid ia cartea din mijloc (+1 punct).
- Simbol greșit = runda masei.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: numele simbolului (ex. luna, stea), cronometru.
- Exemple legale la început: `clopot`, `dop`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="roade"></a>
## Roade de Toamnă (legume și schimburi, masa = roboți)

- **Id:** `roade`
- **Categorie:** Cărți
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play roade` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Trei fermieri (tu + 2 roboți), 8 legume, 2 ogoare (al treilea 3 monede), mână cu ordine fixă: plantezi mereu prima carte.
- Tura: plantezi 1–2 cărți, întorci 2 cărți (le plantezi sau le oferi mesei în schimbul unei cărți orbe), iei 3 cărți.
- Poți recolta oricând (cărți → monede după tabel).
- Roboții îți pot oferi cărți.
- Final la a doua golire a pachetului.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: planteaza 1, recolta 2, compra, treci, ofera, accepta ceapa, refuza.
- Exemple legale la început: `planteaza`, `1`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tcg_evo"></a>
## Cărți colecționabile: evoluție și energie (Făpturi de Câmp)

- **Id:** `tcg_evo`
- **Categorie:** TCG
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tcg_evo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Set original de 40+ cărți (făpturi evolutive pe elemente, energii, ajutoare); pachete pregătite de 30 (J1 Pădurea, J2 Vulcanul).
- Pregătire: activ N, banca N, gata; 3 cărți-premiu.
- Tură: pesca (forțat), apoi banca N, evolueaza N T (nu pe făptură nouă, nu în prima ta tură), energie N T (una pe tură), joaca N [T] (un singur «sprijin»), retragere bK, apoi atac 1|2 (încheie tura) sau fin.
- Țintă: a (activa), b1..b5 (banca mea), e1..e5 (a adversarului).
- Slăbiciunea dublează daunele; efecte: vindecare, daune la bancă, recul, aruncare de energie, amețire.
- Făptura activă căzută = adversarul ia un premiu, tu alegi promova bK.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `activ`, `3`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tcg_zone"></a>
## Cărți colecționabile: monștri în zone și capcane (Hotarele Ceții)

- **Id:** `tcg_zone`
- **Categorie:** TCG
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tcg_zone` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Set original de 44 de cărți (monștri, vrăji, capcane); pachete pregătite de 30 (J1 Hotarul, J2 Mlaștina), 8000 PV.
- Tură: pesca (forțat), apoi o invocare pe tură (nivel 1-4 liber, 5-6 un tribut, 7+ două): invoca 2, seteaza 2 (întors, în apărare), invoca 2 tribut 1,3; intoarce I, schimba I, joaca N [e2|m1|g1] (vrăji), seteaza N (capcană), ataca 1 e2 / ataca 1 direct, fin.
- ATK vs ATK / ATK vs apărare (Pătrunde, Vânător).
- După fiecare atac apărătorul primește continua sau activa T (capcane puse într-o tură anterioară).
- J1 nu atacă în tura 1; pachet gol = pierdere.
- Informație ascunsă: cărțile întoarse ale adversarului nu se văd.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pesca`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tcg_lant"></a>
## Cărți colecționabile: pitch, lanț și echipament (Lanțuri de Oțel)

- **Id:** `tcg_lant`
- **Categorie:** TCG
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tcg_lant` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Set original de 40+ cărți; pachete pregătite de 30 (J1 Gardiana Zorilor, J2 Vânătorul de Furtuni), 30 viață, mână de 4, patru piese de echipament și o armă.
- Resursele vin din pitch: atac 2 pitch 1,3 (cărțile aruncate în pitch merg la fundul pachetului la sfârșitul turei).
- Un punct de acțiune pe tură: atac H, arma, joaca H (acțiuni, nu costă punctul), fin; «continuă» îl returnează.
- Lanț: apărătorul alege bloc 1,3 echip 2 (sau bloc fara), apoi reacții de atac / de apărare (reactie H pitch ..., pasa).
- Daune = putere - apărare.
- După 25 de ture pe jucător câștigă cine are mai multă viață.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pesca`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tcg_cerneala"></a>
## Cărți colecționabile: cerneală, misiuni și provocări (Povestiri din Cerneală)

- **Id:** `tcg_cerneala`
- **Categorie:** TCG
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tcg_cerneala` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Set original de 42 de cărți (personaje, acțiuni); pachete pregătite de 30 (J1 Pădurea Cântătoare, J2 Muntele de Fier), 7 cărți în mână; primul la 15 faimă câștigă.
- Tură: pesca (forțat;
- J1 nu în prima tură), cerneala N (o carte cu cerneală pe tură în călimară = 1 cerneală pe tură), joaca N [e2|m1] (personajele se usucă o tură), misiune P (aduce faimă), provocare P e Q (doar contra personajelor ÎNCLINATE), fin.
- Cuvinte-cheie: Elan, Gardian, Zburător, Provocator, Rezistență.
- Daunele rămân până la eliminare.
- Informație ascunsă: vezi doar mâna J1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pesca`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="spionii"></a>
## Spionii Cuvintelor (grilă 5×5)

- **Id:** `spionii`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play spionii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc de spionaj cu cuvinte: grilă de 25 de cuvinte, 9 agenți roșii (J1), 8 albaștri (J2), 7 neutre, 1 asasin.
- Maestrul-spion al fiecărei echipe este calculatorul: dă ca indiciu numele unei teme din tabela proprie (MARE, CIRC…) și un număr.
- Tu ești ghicitorul: ghici cuvânt (fără diacritice), apoi dezvaluie întoarce cartea; cel mult număr+1 ghiciri, stop încheie tura.
- Asasinul pierde pe loc.
- AI-ul ghicește după aceeași tabelă de teme, fără să vadă cheia.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ghici`, `cont`
- La început sunt 50 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="povestitor"></a>
## Povestitorul (cărți-simbol, 2 jucători)

- **Id:** `povestitor`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play povestitor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc de povestit cu cărți: 231 de descrieri-simbol.
- Povestitorul alege o carte și unul din indiciile ei (spune 3a); celălalt pune o momeală (momeala 2); amesteca adaugă 3 cărți-fantomă; ghicitorul votează (vot N), Publicul votează după potrivirea cuvintelor-cheie, apoi dezvaluie punctează (3/3 dacă exact un votant găsește cartea; altfel 0 și 2; +1 pentru momeală votată de Public).
- Până la 12 puncte sau 14 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `spune`, `1a`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="gura"></a>
## Gura Satului (completează fraza)

- **Id:** `gura`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play gura` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc de completat fraze: 205 fraze cu gol și 231 de răspunsuri originale.
- Judecătorul alternează; celălalt joacă un răspuns (joaca N), amesteca adaugă 2 răspunsuri ale casei, judecătorul alege cea mai nostimă completare (alege N), numara dă punctul: jucătorului dacă a ales cartea lui, judecătorului dacă a ales una a casei.
- Primul la 6 puncte sau după 12 runde.
- AI-ul potrivește etichetele frazei cu ale răspunsului.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="treicai"></a>
## Trei Căi (spune, mimează, desenează)

- **Id:** `treicai`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play treicai` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc de explicat și ghicit, în terminal: 218 acțiuni și meserii cu aspecte predefinite redate pe rând vorbit, mimă sau schiță.
- Descriptorul alege indicii (arata N), adversarul poate fura ghicind (ghici N, pas), apoi robot lasă colegul-robot să ghicească dintre 6 candidați.
- Cu cât mai puține indicii, cu atât mai multe puncte (4…1); ghicire greșită a adversarului −1. 12 runde, rolurile alternează.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arata`, `1`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="oprite"></a>
## Cuvinte Oprite (descrie fără cuvinte interzise)

- **Id:** `oprite`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play oprite` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc cu cuvinte interzise: 233 de cărți cu cuvânt-țintă, 4 interdicții și indicii-propoziție predefinite; indiciile care calcă o interdicție sunt fauluri (−1, detectate automat).
- Descriptorul alege indicii (arata N), adversarul poate fura ghicind (ghici N, pas), colegul-robot ghicește la pasul robot. 10 runde, cu cât mai puține indicii cu atât mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arata`, `1`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="schita"></a>
## Schița (desen simbolic)

- **Id:** `schita`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play schita` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc de desenat și ghicit: 224 de cuvinte, fiecare cu 6 tușe textuale (cerc, coadă, urechi ascuțite…).
- Desenatorul arată tușele una câte una (arata N), adversarul poate fura ghicind (ghici N, pas), apoi colegul-robot ghicește dintre 6 candidați (robot).
- Cu cât mai puține tușe, cu atât mai multe puncte (5…1). 10 runde, rolurile alternează.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arata`, `1`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="dictionar"></a>
## Dicționar Viu (explică în rafală)

- **Id:** `dictionar`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play dictionar` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc de explicat cuvinte: 215 cuvinte cu trei explicații predefinite.
- Fiecare tură are 9 acțiuni (arata N sau treci); fiecare cuvânt ghicit de colegul-robot (robot) sau furat de adversar (ghici N) valorează 1 punct; rădăcina cuvântului în explicație este fault (−1). 6 ture, rolurile alternează.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arata`, `1`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="impostor"></a>
## Impostorul din Grilă (4 locuri)

- **Id:** `impostor`
- **Categorie:** Party
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play impostor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc cu impostor: 14 subiecte × 16 cuvinte; un cuvânt secret pe care îl știu toți mai puțin impostorul (J1, J2 sau unul din cei doi roboți).
- Fiecare dă un indiciu dintr-un cuvânt (indiciu cuvânt), apoi J1 și J2 votează (vot N), roboții votează la numara.
- Impostorul prins ghicește cuvântul (ghici cuvant, verifica).
- Puncte: vot corect +1, impostor neprins +2, prins fără să ghicească: nevinovații +2, prins care ghicește +1. 8 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `indiciu`, `tropical`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cuvantunic"></a>
## Cuvânt Unic (indicii care se anulează)

- **Id:** `cuvantunic`
- **Categorie:** Party
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play cuvantunic` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc cu indicii de un cuvânt, în duel: 224 de cuvinte cu lexicon de indicii.
- Cel care dă indiciul (indiciu N) are doi martori robotici; indiciile identice se anulează (dezvaluie).
- Ghicitorul alege dintre 6 candidați (ghici N) sau pas; verifica: ghicit +1 ghicitorului, greșit +1 celui care a dat indiciul. 12 runde, rolurile alternează.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ghici`, `1`
- La început sunt 13 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mosia"></a>
## Moșia (roluri, plantații și clădiri)

- **Id:** `mosia`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play mosia` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original: 7 roluri (rol cultivator/constructor/producator/negustor/capitan/vistier/zidar); fiecare alege 3 pe rând, toți execută acțiunea, cine a ales are un privilegiu.
- Cultivator: planta 2 sau cariera; constructor: construieste targ (costul scade cu carierele); negustor: vinde z; capitan: expediaza z 1 (nave 4 și 6, un fel de marfă pe navă).
- Culturile p i z t c (prețuri 0–4).
- Câștigă cine are mai multe puncte (expediere + clădiri + monede/4).
- Fără colonii, ci doar plantații care produc direct.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `rol`, `cultivator`
- La început sunt 14 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="gospodaria"></a>
## Gospodăria (plasare de muncitori și fermă)

- **Id:** `gospodaria`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play gospodaria` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original, 6 runde, recoltă după rundele 2, 4, 6: pui pe rând un muncitor pe o acțiune liberă (pune lemn / lut / peste / oi / cereale / legume / zilier / arat / semanat 2 / camera / tarc / familie / inceput); fiecare acțiune ține un singur muncitor pe rundă, lemn, lut, peste și oi se acumulează dacă nu sunt luate.
- La recoltă câmpurile dau cereale, se hrănește familia (2 pe membru, altfel cerșetorii −3) și oile se înmulțesc.
- Punctaj pe categorii (câmpuri, țarcuri, cereale, oi, legume), camere, familie.
- Ferma e doar contoare, fără grilă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pune`, `lemn`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cetatuia"></a>
## Cetățuia (muncitori pe drum de clădiri, castel)

- **Id:** `cetatuia`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cetatuia` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original, 9 runde: 5 muncitori pe jucător, plasați pe rând pe locurile unui drum (pune ferma / padure / cariera / piata / atelier / arhitect / castel / pod), primele 3 plasări costă 1 ban, apoi 2; pas dă 1 ban primului care pasează.
- La pod muncitorul mută jalonerul (pune pod +2): se activează doar locurile până la jaloner.
- Atelierul și arhitectul construiesc clădiri proprii (construieste gater) care cresc drumul și dau puncte când le folosește altcineva; la castel livrezi piese (castel l2 p1).
- Final: cetate completă sau runda 9; majoritatea pieselor +4.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pune`, `ferma`
- La început sunt 31 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="taramuri"></a>
## Tărâmuri (transformarea terenului și piste de ritual)

- **Id:** `taramuri`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play taramuri` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original pe hartă 7×6 cu 7 tipuri de teren pe o roată: J1 e de tip 0, J2 de tip 3.
- Pe rând o acțiune: colonie c3 (transformi terenul vecin în al tău, plătind lopeți, și pui o locuință), oras c3, templu c3 (îmbunătățiri), preot foc|apa|pamant|aer (+2 trepte pe pista de cult; la 5 privilegiu), pas.
- Început: inicial c3. 6 runde cu venit și un bonus de rundă; la final puncte pe piste, cel mai mare grup de clădiri și monede.
- Fără mană, fără facțiuni distincte, vecinătate pe 4 direcții.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `inicial`, `a3`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="furnale"></a>
## Furnale (rețea de legături și industrii)

- **Id:** `furnale`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play furnale` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original, 8 runde, 2 acțiuni pe tură: pe o hartă cu 8 orașe construiești industrii (fabrica 3 t; c cărbune, f fier, t textil, p port) și legături (link 2-4) lipite de rețeaua ta; cărbunele se ia din cea mai apropiată mină legată sau din piață (2 bani), fierul din orice fierărie sau piață.
- Plăcuța se întoarce când se golește sau se vinde (vinde 4, prin legături până la un port) și dă venit și puncte; la final legăturile punctează după plăcuțele întoarse din orașele lor. imprumut = +8 bani, −2 puncte.
- Fără cărți, o singură eră.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `fabrica`, `0`
- La început sunt 71 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="reverdire"></a>
## Reverdire (variantă originală)

- **Id:** `reverdire`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play reverdire` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Motor de resurse și proiecte de cărți: ridici Umiditatea, Aerul și Lacurile (RR).
- O generație: Cercetare (compra N, 3 L; gata), acțiuni una pe tură (joc N, vinde N, pompa, irigare, plantare, asezare, conv sem, conv caldura, pas; apoi pune c3), producție.
- Cărămizile plătesc proiectele B, Cristalele pe cele S.
- Scor: RR + PV + arbori + orașe.
- Informație ascunsă: mâna, oferta și pachetul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `generatie`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="salina"></a>
## Salina (variantă originală)

- **Id:** `salina`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play salina` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Plasare de agenți și construire de pachet, 10 runde sau 10 PV. agent 3 e (carte + locație a–l cu pictograma potrivită; la locațiile de luptă e–h: agent 3 e 2 = desfășori 2 trupe), revela apoi compra N / apa (1 Apă = +1 persuasiune) / sare (1 Sare = +1 sabie) / fin.
- Conflictul rundei: forța = 2 × trupe + săbii; premiul I/II; runda pregătește runda următoare.
- Informație ascunsă: mâna adversarului și pachetele.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `agent`, `1`
- La început sunt 235 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="catun"></a>
## Cătun (variantă originală)

- **Id:** `catun`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play catun` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Plasare de muncitori și cătun de cărți, 4 anotimpuri (2,3,4,6 muncitori). lucru a (locație a–h; lucru g L la Han), joc N / piata N (carte din mână / piață, max 15 în cătun), sezon (pregătire: muncitorii se întorc, producătoarele lucrează; în Toamnă ieși din joc).
- Scor: puncte de pe cărți, jetoane, prosperitate.
- Informație ascunsă: mâna adversarului și pachetul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `lucru`, `a`
- La început sunt 27 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="solzi"></a>
## Solzi (variantă originală)

- **Id:** `solzi`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play solzi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Motor de cărți-reptile și amfibieni cu habitate, hrană și ouă, 4 runde (8,7,6,5 acțiuni). joc N p|s|b (vietate în Pădure/Stâncă/Baltă, plătești hrană și ouă), padure I V (hrană), stanca p1 s2 (ouă), balta (cărți).
- Puterile cărților se declanșează doar în habitatul acțiunii.
- Obiective de rundă; scor: puncte de pe cărți + ouă + obiective.
- Informație ascunsă: mâna adversarului și pachetul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `joc`, `4`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="poieni"></a>
## Poieni (variantă originală)

- **Id:** `poieni`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play poieni` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Plăcuțe hexagonale de habitat și jetoane de animale.
- Tura: ia N (pereche din piață), pune q,r rN (plăcuța lipită de ținutul tău, rotire 0–5), animal q,r (jetonul pe o plăcuță liberă care îl acceptă) sau arunca.
- Jeton-natură: nat N M (orice plăcuță cu orice jeton), nat reset. 20 de ture.
- Scor: animale Urs/Vidră/Zimbru/Gaiță/Vulpe cu reguli proprii, cel mai mare coridor al fiecărui habitat (+3 pentru cel mai mare), jetoane-natură.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ia`, `1`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="epoci"></a>
## Epoci (variantă originală)

- **Id:** `epoci`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play epoci` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Draft de cărți în 3 epoci cu comerț între vecini, pentru J1, J2 și Orașul Oarb (automat, al treilea oraș din inel, vecin cu amândoi).
- Fiecare epocă: 7 cărți pe oraș, 6 ture; alegi o carte și pasezi mâna. joc N (construiești, cumpărând resursele lipsă de la vecini cu 2 monede, 1 cu reducere), etapa N (cartea ține loc de etapă a minunii), vinde N (+3 monede).
- Scor: civice, minune, victorii militare (1/3/5, înfrângere −1), 1 p la 3 monede, știință, bresle.
- Împărțirea epocii următoare: imparte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `joc`, `1`
- La început sunt 26 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tinuturi"></a>
## Ținuturi (variantă originală)

- **Id:** `tinuturi`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play tinuturi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Domino de teritorii cu coroane, 2 jucători: regat 9×9 cu castel în e5, încape în 7×7. 24 de piese-domino (generate de noi) în rânduri de câte 4: proprietarul piesei cu numărul mic joacă primul.
- Tura: pune e6-e7 (prima jumătate pe e6) lipit de castel sau de același teritoriu, apoi alege 2 din rândul următor; descarta doar dacă nu ai nicio așezare.
- Primul rând: alege N (J1, J2, J2, J1); revelare dezvăluie rândul.
- Scor: regiune = pătrate × coroane, +5 fără aruncări, +10 castel central.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="rezervatia"></a>
## Rezervația (variantă originală)

- **Id:** `rezervatia`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play rezervatia` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Rezervație naturală cu cărți-animale și sponsori.
- Cinci acțiuni (Faună, Amenajare, Sponsori, Cărți, Conservare) într-un rând: puterea = locul (1–5); cea jucată trece pe locul 1. animal 2 e1 (la putere ≥ 4 încă un animal sau gata), construieste padure 3 / chiosc / pavilion, sponsor 3 / finantare, carti / carti v2, proiect 1 / donatie.
- Pauză la ritm 9: venit.
- Scor = apel + 2×conservare; la 50 jocul se încheie la sfârșitul rundei.
- Informație ascunsă: mâna adversarului și pachetul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `animal`, `5`
- La început sunt 52 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hotar"></a>
## Hotarul (variantă originală)

- **Id:** `hotar`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hotar` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Motor de resurse și teritorii pe hartă hexagonală (a1–g4), facțiuni cu avantaje diferite (alege luncenii/munteanii/padurenii/campenii).
- Tura: o secțiune a tablei tale (nu cea de data trecută): acțiunea de sus (s1 mars h:g1>f1 / bani, s2 putere / carti, s3 produce a1 b2 / muncitor, s4 comert grane / faima, sau sari), apoi cea de jos (mobilizeaza f1, imbunatateste mars, construieste moara f1, inrolare 2, sau gata).
- Luptă: lupta P C (putere și carte angajate pe ascuns), apoi rezolva; egal câștigă apărătorul. 5 stele sau 32 de ture încheie jocul; scor = monede + stele + hexuri + resurse (după popularitate).
- Informație ascunsă: cărțile de luptă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `luncenii`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="codru"></a>
## Codrul Nesupus (război asimetric, variantă originală)

- **Id:** `codru`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play codru` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pădure cu 8 poieni și drumuri;
- J1/J2 primesc la sorți Regența (gărzi, Cherestea/Atelier/Cazarmă, lemn) sau Frunzișul (luptători, simpatie, suporteri, baze).
- Regența: trag, 3 acțiuni (mars A>B n, lupta P, recruteaza, construieste c|a|k P — în poiană stăpânită, 1 lemn), seara fabrica P, gata.
- Frunzișul: trag, 3 + baze acțiuni (mobilizeaza fel, propaga P, revolta P, recruteaza, mars, lupta).
- Lupta: ambuscada/fara, contra/lasa, apoi zar (gherila: apărând, Frunzișul lovește cu zarul mare).
- Mâna max 5 (arunca fel). 18 puncte sau 30 de ture.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `trag`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sideral"></a>
## Coroana Siderală (4X compact, variantă originală)

- **Id:** `sideral`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sideral` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 9 sisteme (Citadela în centru, inel, două capete), planete cu resurse/influență, nave, 5 tehnologii.
- Runda: alegi pe rând 2 din 6 cărți de strategie (alege Comert), apoi acțiuni în ordinea inițiativei: taktic A>B n (activezi B, muți n nave; adauga C>B n; gata), carte Nume (efect principal; adversarul poate lua secundarul: sec / nu), pas.
- Lupta: zar până rămâne o singură parte; ocupi planeta cu ≥ r nave (r+1 dacă e a adversarului; capitalele nu se cuceresc).
- În sub-faze: nava Sistem, tech Nume, gata.
- Status: obiectiv N. 8 puncte sau 12 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `Conducere`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ceasornic"></a>
## Ceasornicul din Valea Ceții (deducție cu povestitor, masa = roboți)

- **Id:** `ceasornic`
- **Categorie:** Party
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** da (stare aleatoare); în „la masă” afișajul poate arăta informația Jucătorului 1
- **Pornire:** `polytabletop play ceasornic` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 12 locuri (tu + 11 roboți), 14 roluri în scenariu (7 localnici din 9, Bețivul, Sfântul, Otrăvitorul, Moștenitoarea, Strigoiul).
- Povestitorul e automat;
- Bețivul și cei otrăviți primesc informații false.
- Noaptea rolurile cu informație (Ghicitoarea, Cronicarul, Veghea, Cercetașul, Groparul) află ceva, Călugărul ocrotește, Strigoiul ucide din noaptea 2.
- Ziua roboții își spun rolul (cer N îi întreabă), tu declari adevărul (declar) sau minți (mint rol), nominalizezi și votezi; cine strânge jumătate din vii și cel mai mult e executat.
- Morții au un vot fantomă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ghicesc 3 7, otravesc 3, ocrotesc 3, ucid 3, declar, mint veghea, cer 4, vanez 5, treci, nominez 6, pas, da, nu, masa.
- Exemple legale la început: `mint`, `ghicitoarea`
- La început sunt 29 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="coroana"></a>
## Drumul spre Coroană (aventură pe inele, om contra roboți)

- **Id:** `coroana`
- **Categorie:** Strategie
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play coroana` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tu și doi roboți, șase personaje cu avantaje (alege haiducul, vrajitoarea, negustorul, padurarul, pelerinul, calauza).
- Pornești din Câmpie (16 câmpuri), treci Podul Gri (plată de 2 Aur, Cheia sau luptă), în Țara Cenușie câștigi Pecetea în Turn (asalt forta/iscusinta), urci Scara Neagră în Inelul de Jar și intri pe Ușa Coroanei.
- Tura: zar, apoi dreapta sau stanga; cărți de aventură (monștri, obiecte, descântece, întâmplări), han, capelă, târg (cumpar sabie, plec).
- Lupta: Forța + d6 trebuie să depășească forța monstrului (lupt, lupt fulger, fug, ceata).
- Cine rămâne fără Viață cade la Han.
- Cine e în camera Coroanei dă Porunca: ceilalți pierd 1 Viață pe tură (la 0 sunt scoși).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: zar, bea, leac, urc, urc plata, urc cheie, urc lupta, intru, mantin, masa.
- Exemple legale la început: `alege`, `haiducul`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

