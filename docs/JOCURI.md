# Jocurile jucabile

Generat de `tools/gen_docs.py` din `polytabletop list` și `polytabletop play ID` (490 jocuri). Nu edita manual; regenerează cu `python3 -I tools/gen_docs.py`.

Sintaxa generală: scrii mutarea exact cum apare în lista `?` (fără diferență între majuscule/minuscule, spațiile se ignoră); `?` listează mutările legale, `hint` cere sugestia AI-ului, `q` iese.

## Cuprins

- [Șah](#chess) (`chess`)
- [Dame (8×8)](#draughts) (`draughts`)
- [Reversi](#reversi) (`reversi`)
- [Patru în linie](#connect4) (`connect4`)
- [Gomoku (15×15)](#gomoku) (`gomoku`)
- [Hex (11×11)](#hex) (`hex`)
- [Amazoane (10×10)](#amazoane) (`amazoane`)
- [Linii de acțiune (8×8)](#linii) (`linii`)
- [Nim cu grămezi](#nim) (`nim`)
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
- [Popa prostul](#popa) (`popa`)
- [Pescuit (Go Fish)](#pescuit) (`pescuit`)
- [Treizeci și unu (31)](#treizeci1) (`treizeci1`)
- [Cassino (capturi)](#casino) (`casino`)
- [Poker de zaruri](#zarpoker) (`zarpoker`)
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
- [Riichi Mahjong (2 jucători)](#riichi) (`riichi`)
- [Sic Bo](#sicbo) (`sicbo`)
- [Ashta Chamma](#ashtachamma) (`ashtachamma`)
- [Daifugo (Daihinmin)](#daifugo) (`daifugo`)
- [Tiến lên](#tienlen) (`tienlen`)
- [Big Two (Pusoy dos)](#bigtwo) (`bigtwo`)
- [Tong-its](#tongits) (`tongits`)
- [Pasur](#pasur) (`pasur`)
- [Hokm](#hokm) (`hokm`)
- [Piști](#pisti) (`pisti`)
- [Court Piece (Rang, Mendikot)](#courtpiece) (`courtpiece`)
- [Call Break](#callbreak) (`callbreak`)
- [Ganjifa (levate)](#ganjifa) (`ganjifa`)
- [Yut (Yunnori)](#yut) (`yut`)
- [Sugoroku (cursă cu casete)](#sugoroku) (`sugoroku`)
- [Chaupar (cauri, 2 jucători)](#chaupar) (`chaupar`)
- [Culori (potrivit cărți colorate)](#culori) (`culori`)
- [Moșii (afaceri imobiliare cu zaruri)](#mosii) (`mosii`)
- [Itinerar (turism pe tablă, rezervi destinații)](#itinerar) (`itinerar`)
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
- [Cursa probelor (joc de familie cu mini-probe, 2–4 jucători)](#cursa_probelor) (`cursa_probelor`)
- [Capcana (variantă originală)](#capcana) (`capcana`)
- [Congkak (Malaezia)](#congkak) (`congkak`)
- [Pallanguzhi (Tamil Nadu)](#pallanguzhi) (`pallanguzhi`)
- [Ayo (Ayoayo, Yoruba)](#ayo) (`ayo`)
- [Oware (Ghana)](#oware) (`oware`)
- [Bao (Bao la Kiswahili)](#bao) (`bao`)
- [Seega](#seega) (`seega`)
- [Yote](#yote) (`yote`)
- [Hus (Namibia)](#hus) (`hus`)
- [Tâb (Egipt)](#tab) (`tab`)
- [Senet](#senet) (`senet`)
- [Whot (cărți nigeriene)](#whot) (`whot`)
- [Chkobba (Tunisia)](#chkobba) (`chkobba`)
- [Ronda (Maroc / Spania)](#ronda) (`ronda`)
- [Sholo Guti (Șaisprezece soldați)](#shologuti) (`shologuti`)
- [Aadu Puli Attam (Oi și tigri)](#aadupuli) (`aadupuli`)
- [Mangala (mancala turcească)](#mangala) (`mangala`)
- [Dama (dame turcești)](#dama) (`dama`)
- [Okey (2 jucători)](#okey) (`okey`)
- [Bătălia navală (10×10)](#battleship) (`battleship`)
- [Spargerea codului (cu schimb de roluri)](#mastermind) (`mastermind`)
- [Perechi (joc de memorie)](#memory) (`memory`)
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
- [Inimi (evitarea punctelor, 2 jucători)](#inimi) (`inimi`)
- [Pică (licitație și atu pică, 2 jucători)](#pica) (`pica`)
- [Prinde (licitație exactă, 2 jucători)](#prinde) (`prinde`)
- [Whist (2 jucători)](#whist) (`whist`)
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
- [Turnul Zorilor Frânte (aventură cooperativă, fantasy eroic)](#turnul_zorilor) (`turnul_zorilor`)
- [Arhiva Abisală (aventură cooperativă, groază cosmică de investigator)](#arhiva_abisala) (`arhiva_abisala`)
- [Ruinele Cartografilor (aventură cooperativă, expediție în junglă)](#ruinele_cartografilor) (`ruinele_cartografilor`)
- [Noapte de Catifea (aventură cooperativă, urban fantasy cu vampiri)](#noapte_de_catifea) (`noapte_de_catifea`)
- [Pădurea Ochilor de Sticlă (aventură cooperativă, fantasy de basm sumbru)](#padurea_sticlei) (`padurea_sticlei`)
- [Neon și Rune (aventură cooperativă, cyber-urban cu magie)](#neon_si_rune) (`neon_si_rune`)
- [Târgul Ciumat (aventură cooperativă, fantasy sumbru)](#targul_ciumat) (`targul_ciumat`)
- [Insula celor Două Luni (aventură cooperativă, fantasy clasic)](#insula_celor_doua_luni) (`insula_celor_doua_luni`)
- [Hoții din Ceață (aventură cooperativă, jaf într-un oraș cu ceață)](#hotii_din_ceata) (`hotii_din_ceata`)
- [Crucea de Cenușă (aventură cooperativă, apocalipsă sumbră)](#crucea_de_cenusa) (`crucea_de_cenusa`)
- [Strada de Oțel (aventură cooperativă, cyberpunk de cartier)](#strada_de_otel) (`strada_de_otel`)
- [Ținutul Furtunilor Vii (aventură cooperativă, fantasy colorat)](#tinutul_furtunilor) (`tinutul_furtunilor`)
- [Epava din Inelul Șapte (aventură cooperativă, operă spațială)](#epava_din_inelul_sapte) (`epava_din_inelul_sapte`)
- [Fiordul Gheții Negre (aventură cooperativă, fantasy nordic)](#fiordul_gheatii_negre) (`fiordul_gheatii_negre`)
- [Mlaștina Strigoilor (aventură cooperativă, vânătoare de monștri)](#mlastina_strigoilor) (`mlastina_strigoilor`)
- [Satul Lanternelor Stinse (aventură cooperativă, groază de sat cu spirite)](#satul_lanternelor) (`satul_lanternelor`)
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
- [Coridor (cursă de pioni cu ziduri)](#coridor) (`coridor`)
- [Drum de pietre (5×5, stive)](#drum) (`drum`)
- [Trei-pierde (hexagonal)](#tripierd) (`tripierd`)
- [Konane (dame hawaiiene)](#konane) (`konane`)
- [Fanorona (Madagascar)](#fanorona) (`fanorona`)
- [Tigri și capre (Nepal)](#tigri) (`tigri`)
- [Surakarta (Java)](#surakarta) (`surakarta`)
- [Hasami Shogi (Japonia)](#hasami) (`hasami`)
- [Puncte și căsuțe (5×5)](#puncte) (`puncte`)
- [Străpungere (8×8)](#strapungere) (`strapungere`)
- [Izolare (7×7)](#izolare) (`izolare`)
- [Rotire în cinci (6×6)](#rotire) (`rotire`)
- [Coloane (dame în turnuri)](#coloane) (`coloane`)
- [Bile pe hexagoane (5 pe latură)](#bile) (`bile`)
- [Patru atribute (piesa dată)](#patru) (`patru`)
- [Cupe încuibate (4×4)](#cupe) (`cupe`)
- [Poliedre (duo, 14×14)](#poliedre) (`poliedre`)
- [Împletitura (echilibru pe covor cu cercuri colorate)](#impletitura) (`impletitura`)
- [Pinochle](#pinochle) (`pinochle`)
- [Gin Rummy](#gin_rummy) (`gin_rummy`)
- [Crazy Eights (Optarii)](#crazy_eights) (`crazy_eights`)
- [Pitch (Setback)](#pitch) (`pitch`)
- [Bid Whist (2 jucători)](#bid_whist) (`bid_whist`)
- [All Fours](#all_fours) (`all_fours`)
- [Conquián](#conquian) (`conquian`)
- [Chicken Foot (domino)](#chicken_foot) (`chicken_foot`)
- [Cele șase ținte (zaruri)](#bunco) (`bunco`)
- [Zece mii de puncte (zaruri)](#farkle) (`farkle`)
- [Zarurile mincinosului](#liars_dice) (`liars_dice`)
- [Crokinole](#crokinole) (`crokinole`)
- [Chinchón (Spania)](#chinchon) (`chinchon`)
- [Burako (Buraco)](#burako) (`burako`)
- [Escoba (Escoba de 15)](#escoba) (`escoba`)
- [Cuarenta (Ecuador)](#cuarenta) (`cuarenta`)
- [Generala (zaruri)](#generala) (`generala`)
- [Parqués (Colombia)](#parques) (`parques`)
- [Five Hundred (500, doi jucători)](#fivehundred) (`fivehundred`)
- [Pontoon](#pontoon) (`pontoon`)
- [Two-up (Australia)](#twoup) (`twoup`)
- [Ancora și coroana](#ancora) (`ancora`)
- [Mu Torere (Noua Zeelandă)](#mutorere) (`mutorere`)
- [X și 0 (Tic-tac-toe)](#tictactoe) (`tictactoe`)
- [Jocul regal din Ur](#ur) (`ur`)
- [Crazyhouse (șah cu reintroducere)](#crazyhouse) (`crazyhouse`)
- [Omweso](#omweso) (`omweso`)
- [TwixT](#twixt) (`twixt`)
- [Halma](#halma) (`halma`)
- [Mahjong Hong Kong (minimum 3 fan)](#mahjong_hk) (`mahjong_hk`)
- [Fia-med-knuff](#fia) (`fia`)
- [Tien Gow (Cer și nouă)](#tiengow) (`tiengow`)
- [Acey-deucey](#aceydeucey) (`aceydeucey`)
- [Bar billiards](#barbilliards) (`barbilliards`)
- [Tabula](#tabula) (`tabula`)
- [Jeu des petits chevaux](#petitschevaux) (`petitschevaux`)
- [Cờ gánh](#coganh) (`coganh`)
- [Petteia](#petteia) (`petteia`)
- [Morra (Mora)](#morra) (`morra`)
- [Sanma (mahjong în trei, adaptat la doi)](#sanma) (`sanma`)
- [Flip Cup](#flipcup) (`flipcup`)
- [Mad Gab](#madgab) (`madgab`)
- [Pub quiz](#pubquiz) (`pubquiz`)
- [Old Maid (Bătrâna fată)](#oldmaid) (`oldmaid`)
- [Mus](#mus) (`mus`)
- [Bézique](#bezique) (`bezique`)
- [Twenty-Nine (29)](#twentynine) (`twentynine`)
- [Tute](#tute) (`tute`)
- [Șotron (Hopscotch)](#sotron) (`sotron`)
- [Duck Duck Goose](#duckduckgoose) (`duckduckgoose`)
- [Statuile (Statues)](#statuile) (`statuile`)
- [Conkers](#conkers) (`conkers`)
- [Craps](#craps) (`craps`)
- [Mexico (Mäxchen, Mia)](#mexico) (`mexico`)
- [Schocken](#schocken) (`schocken`)
- [Passe-dix](#passedix) (`passedix`)
- [Dice Baseball](#dicebaseball) (`dicebaseball`)
- [Drop Dead](#dropdead) (`dropdead`)
- [Dame internaționale (10×10)](#dame10) (`dame10`)
- [Toguz Kumalak (Kazahstan)](#toguz) (`toguz`)
- [Sittuyin (șahul birman)](#sittuyin) (`sittuyin`)
- [Gebeta (Etiopia)](#gebeta) (`gebeta`)
- [Sprouts (Conway–Paterson)](#sprouts) (`sprouts`)
- [Peg solitaire (duel)](#pegsol) (`pegsol`)
- [Shuffleboard de masă](#shuffleboard) (`shuffleboard`)
- [Yatzy (Scandinavia)](#yatzy) (`yatzy`)
- [Cee-lo (zaruri de stradă)](#ceelo) (`ceelo`)
- [Peste și sub șapte (Over and Under Seven)](#oversevens) (`oversevens`)
- [Skunk (zaruri)](#skunk) (`skunk`)
- [Hazard (strămoșul craps-ului)](#hazard) (`hazard`)
- [Crichet cu zaruri (Howzat!)](#howzat) (`howzat`)
- [Dreidel](#dreidel) (`dreidel`)
- [Pare sau impare](#parimpar) (`parimpar`)
- [Mehen (Egiptul antic)](#mehen) (`mehen`)
- [Cinci pietricele](#cinci_pietricele) (`cinci_pietricele`)
- [Jocuri cu ață (leagănul pisicii)](#jocuri_ata) (`jocuri_ata`)
- [Table skittles (popice de masă)](#skittles) (`skittles`)
- [Doppelkopf (2 jucători)](#doppelkopf) (`doppelkopf`)
- [Tarock (2 jucători)](#tarock) (`tarock`)
- [Șapte (Sevens)](#sevens) (`sevens`)
- [Skitgubbe](#skitgubbe) (`skitgubbe`)
- [Ombre (L'Hombre)](#ombre) (`ombre`)
- [Dominó cubano (doble nueve)](#domino9) (`domino9`)
- [Narde scurtă (Short Nardy)](#narde) (`narde`)
- [Dog (curse cu cărți)](#dog) (`dog`)
- [Toc (jeu)](#toc) (`toc`)
- [Trictrac (simplificat)](#trictrac) (`trictrac`)
- [Tangram (duel pe grilă)](#tangram) (`tangram`)
- [Zip Zap Zop](#zipzapzop) (`zipzapzop`)
- [Ai prefera…](#wouldyourather) (`wouldyourather`)
- [Telefonul fără fir](#chinesewhispers) (`chinesewhispers`)
- [Ghicește melodia](#namethattune) (`namethattune`)
- [Baba-oarba](#babaoarba) (`babaoarba`)
- [Capture the Flag](#capturetheflag) (`capturetheflag`)
- [Închide cutia (Shut the Box)](#shut_box) (`shut_box`)
- [Zarul norocos (Chuck-a-luck)](#chuck) (`chuck`)
- [Zaruri de bar (Bar dice)](#bar_dice) (`bar_dice`)
- [Zaruri cosmice (după Cosmic Wimpout)](#cosmic) (`cosmic`)
- [Tombola (Italia)](#tombola) (`tombola`)
- [Pachinko (Japonia)](#pachinko) (`pachinko`)
- [Fan-Tan (China)](#fan_tan) (`fan_tan`)
- [Turnurile din Hanoi (Tower of Hanoi)](#hanoi) (`hanoi`)
- [Tenis de masă (Ping-pong)](#pingpong) (`pingpong`)
- [Hounds and Jackals (58 de găuri)](#hounds) (`hounds`)
- [Ataxx (7×7)](#ataxx) (`ataxx`)
- [Shatranj (Persia)](#shatranj) (`shatranj`)
- [Dame rusești (Shashki)](#shashki) (`shashki`)
- [Ô ăn quan (Vietnam)](#o_an_quan) (`o_an_quan`)
- [Hypergammon](#hypergammon) (`hypergammon`)
- [Jacquet (Franța)](#jacquet) (`jacquet`)
- [Moultezim (Grecia / Turcia)](#moultezim) (`moultezim`)
- [Chu Shogi (12×12)](#chu_shogi) (`chu_shogi`)
- [Chowka Bara](#chowka_bara) (`chowka_bara`)
- [Uckers](#uckers) (`uckers`)
- [Mau-Mau](#mau_mau) (`mau_mau`)
- [Muggins (All Fives)](#muggins) (`muggins`)
- [Schafkopf (2 jucători)](#schafkopf) (`schafkopf`)
- [Preferans (2 jucători)](#preferans) (`preferans`)
- [Ulti (2 jucători)](#ulti) (`ulti`)
- [Three Card Brag](#brag) (`brag`)
- [Cuvinte încrucișate](#crossword) (`crossword`)
- [Perquackey](#perquackey) (`perquackey`)
- [Ligretto](#ligretto) (`ligretto`)
- [Truth or Dare (Adevăr sau provocare)](#truth_dare) (`truth_dare`)
- [Sticla (Spin the bottle)](#sticla) (`sticla`)
- [Sardines](#sardines) (`sardines`)
- [Scaunele muzicale](#scaune_muzicale) (`scaune_muzicale`)
- [Mikado (Pick-up sticks)](#mikado) (`mikado`)
- [Kick the Can](#kick_can) (`kick_can`)
- [Mingea (Dodgeball)](#mingea) (`mingea`)
- [Kalah](#kalah) (`kalah`)
- [Vulpea și gâștele (Fox and Geese)](#foxgeese) (`foxgeese`)
- [Alquerque](#alquerque) (`alquerque`)
- [Jungle (Dou Shou Qi)](#jungle) (`jungle`)
- [Chaturanga (India)](#chaturanga) (`chaturanga`)
- [Camelot](#camelot) (`camelot`)
- [Fotbal de masă pe ture (Table football)](#fotbalmasa) (`fotbalmasa`)
- [Tiddlywinks (pe ture)](#tiddlywinks) (`tiddlywinks`)
- [Jocul de 15 (cursă)](#joc15) (`joc15`)
- [Sudoku duel](#sudoku) (`sudoku`)
- [Biliard francez (carambol)](#biliard) (`biliard`)
- [Tavli (Portes, Plakoto, Fevga)](#tavli) (`tavli`)
- [Gul bara (Golful Persic)](#gulbara) (`gulbara`)
- [Chinchirorin (zaruri în bol)](#chinchirorin) (`chinchirorin`)
- [Ship, Captain and Crew](#shipcrew) (`shipcrew`)
- [Gândacul (Beetle)](#beetle) (`beetle`)
- [Zaruri colorate (Kismet)](#kismet) (`kismet`)
- [Dead Man's Dice (zarurile piratului)](#deadmansdice) (`deadmansdice`)
- [Snap](#snap) (`snap`)
- [Simon zice](#simon) (`simon`)
- [Sette e mezzo (șapte și jumătate)](#settemezzo) (`settemezzo`)
- [Baccarat (Punto Banco)](#baccarat) (`baccarat`)
- [Spânzurătoarea (Hangman)](#hangman) (`hangman`)
- [Douăzeci de întrebări](#douazeci) (`douazeci`)
- [Ghicește ce văd (I Spy)](#ispy) (`ispy`)
- [Vânătoarea de comori](#comori) (`comori`)
- [Mima (Charades)](#mima) (`mima`)
- [Crima cu ochiade (Wink Murder)](#crima) (`crima`)
- [Sheepshead](#sheepshead) (`sheepshead`)
- [Tysiąc](#tysiac) (`tysiac`)
- [Mariáš](#mariash) (`mariash`)
- [Texas 42](#texas42) (`texas42`)
- [Pit](#pit) (`pit`)
- [American Mah Jongg](#mahjong_am) (`mahjong_am`)
- [Dame braziliene (Damas)](#damas) (`damas`)
- [Renju](#renju) (`renju`)
- [Tablut](#tablut) (`tablut`)
- [Lasca](#lasca) (`lasca`)
- [Ruletă (Roulette)](#ruleta) (`ruleta`)
- [Bile (par sau impar)](#marbles) (`marbles`)
- [Mahjong taiwanez](#mahjong_tw) (`mahjong_tw`)
- [Matador (domino)](#matador) (`matador`)
- [Bagatelle](#bagatelle) (`bagatelle`)
- [Latrunculi](#latrunculi) (`latrunculi`)
- [Salta](#salta) (`salta`)
- [Chaturaji](#chaturaji) (`chaturaji`)
- [Sjoelen](#sjoelen) (`sjoelen`)
- [Nonogram (Picross), duel](#nonogram) (`nonogram`)
- [Snooker](#snooker) (`snooker`)
- [Scattergories](#scattergories) (`scattergories`)
- [Eu niciodată](#nhie) (`nhie`)
- [Spoons (lingurile)](#spoons) (`spoons`)
- [Treci pachetul (Pass the Parcel)](#parcel) (`parcel`)
- [Solitaire (Klondike), duel](#klondike) (`klondike`)
- [Jass (Schieber)](#jass) (`jass`)
- [Manille](#manille) (`manille`)
- [Klaverjas](#klaverjas) (`klaverjas`)
- [Sueca](#sueca) (`sueca`)
- [Piatră, hârtie, foarfece](#rps) (`rps`)
- [Sărit coarda (Jump rope)](#coarda) (`coarda`)
- [Capra (Leapfrog)](#capra) (`capra`)
- [Jocul lui Kim (Kim's Game)](#kim) (`kim`)
- [Slapjack](#slapjack) (`slapjack`)
- [Chō-han (Chō-han bakuchi)](#chohan) (`chohan`)
- [Balut](#balut) (`balut`)
- [Knucklebones (Tali)](#tali) (`tali`)
- [Crag](#crag) (`crag`)
- [Bo Bing (Pua Tiong Chiu)](#bobing) (`bobing`)
- [Ambuteiaj (duel de mașini pe tablă 6×6)](#ambuteiaj) (`ambuteiaj`)
- [Banchiza (pinguini pe sloiuri hexagonale)](#banchiza) (`banchiza`)
- [Balansul Pietrelor (greutăți pe o grindă)](#balans) (`balans`)
- [Piese de Mozaic (puzzle cu forme în doi)](#pietre_puzzle) (`pietre_puzzle`)
- [Firul Luminii (plăci cu drumuri pe tablă 6×6)](#firul) (`firul`)
- [Mozaicul Priceput (domino hexagonal în șase culori)](#genial) (`genial`)
- [Drumul Pelerinului (călătorie cu popasuri pe o potecă)](#pelerin) (`pelerin`)
- [Cursa Morcovilor (cursă cu resursă de energie)](#morcovi) (`morcovi`)
- [Rapidele (canoe pe râu spre cascadă)](#rapide) (`rapide`)
- [Vaporul de pe Râu (cursă cu viteză și cărbune)](#vapor) (`vapor`)
- [Lumina Pădurii (copaci cu umbre pe tablă hexagonală)](#lumina) (`lumina`)
- [Cartierul Înalt (majorități de zgârie-nori)](#zgarie_nori) (`zgarie_nori`)
- [Turnuri de Piatră (cavaleri pe castele în creștere)](#turnuri_piatra) (`turnuri_piatra`)
- [Templele din Junglă (explorare și temple pe grilă 5×5)](#templele) (`templele`)
- [Caravana de Mirodenii (negoț între doi neguțători)](#caravana) (`caravana`)
- [Poteca Pietrelor (cinci poteci de culori)](#poteca) (`poteca`)
- [Evadarea din Temniță (pirați care fug printr-un tunel)](#evadare) (`evadare`)
- [Intrigile Cetății (alegere de roluri și construcție de cartiere)](#cetate_intrigi) (`cetate_intrigi`)
- [Menajeria (cărucioare cu animale și țarcuri)](#menajerie) (`menajerie`)
- [Ținutul Zânelor (călătorie cu cărți de transport)](#tinut_zane) (`tinut_zane`)
- [Poștalionii (rute de livrare între orașe)](#postalioni) (`postalioni`)
- [Cazanul Vrăjit (pariu cu pungi de ingrediente)](#cazanul) (`cazanul`)
- [Șantierul Piramidei (pietre pe corăbii spre șantiere)](#santier) (`santier`)
- [Marele Bazar (neguțători pe o grilă de locații)](#marele_bazar) (`marele_bazar`)
- [Curierii Vrăjitoarelor (roluri curajoase sau fricoase)](#curierii_vraji) (`curierii_vraji`)
- [Vameșul din Poartă (bluf cu geanta de negustor)](#vamesul) (`vamesul`)
- [Agenții Tăcuți (agenți pe o stradă cu valori secrete)](#agenti) (`agenti`)
- [Schița Ascunsă (indicii dintr-un tablou)](#tablou) (`tablou`)
- [Orașul Detectivilor (urmărirea suspecților pe hartă)](#detectiv) (`detectiv`)
- [Ținuturi Negociate (prețul tău, alegerea mea)](#tinuturi_negociate) (`tinuturi_negociate`)
- [Peisaj Liniștit (plăcuțe hexagonale cu margini de teren)](#peisaj) (`peisaj`)
- [Cariera Visată (formula succesului și zarul)](#cariera) (`cariera`)
- [Ziua de Salariu (cursă de buget pe trei luni)](#salariu) (`salariu`)
- [Plapuma (petice, timp și nasturi)](#plapuma) (`plapuma`)
- [Pisicile din Barcă (pisici-poliomino pe barcă)](#pisici) (`pisici`)
- [Triunghiurile Numerotate (domino triunghiular)](#triunghiuri) (`triunghiuri`)
- [Banane-Cuvinte (cursă de cuvinte cu literele din mână)](#banane) (`banane`)
- [Turnuri de Litere (cuvinte încrucișate cu litere suprapuse)](#turnuri_litere) (`turnuri_litere`)
- [Lanțuri Hoteliere (fuziuni și acțiuni pe o tablă mică)](#lanturi) (`lanturi`)
- [Bambuzeria (grădinar, panda și obiective de bambus)](#bambuzeria) (`bambuzeria`)
- [Hnefatafl](#hnefatafl) (`hnefatafl`)
- [Brandubh](#brandubh) (`brandubh`)
- [Pente](#pente) (`pente`)
- [Havannah](#havannah) (`havannah`)
- [Table chinezească (Chinese Checkers)](#chinese_checkers) (`chinese_checkers`)
- [Jocul gâștei (Game of the Goose)](#goose) (`goose`)
- [Keno](#keno) (`keno`)
- [Mahjong Sichuan](#sichuan_mahjong) (`sichuan_mahjong`)
- [Long Nard (Narde)](#long_nard) (`long_nard`)
- [Shove ha'penny](#shove_ha_penny) (`shove_ha_penny`)
- [Rithmomachia](#rithmomachia) (`rithmomachia`)
- [Pong hau k'i](#pong_hau_kiki) (`pong_hau_kiki`)
- [Patolli](#patolli) (`patolli`)
- [Duodecim scripta](#duodecim_scripta) (`duodecim_scripta`)
- [Puzzle cu piese (Jigsaw)](#jigsaw) (`jigsaw`)
- [Mahjong oficial chinez (Guobiao)](#guobiao) (`guobiao`)
- [Boggle](#boggle) (`boggle`)
- [Kings (Ring of Fire)](#kings) (`kings`)
- [Celebrity (Fishbowl)](#celebrity) (`celebrity`)
- [Prinde coada măgarului](#pin_the_tail) (`pin_the_tail`)
- [Război (War)](#war) (`war`)
- [Tressette (Italia)](#tressette) (`tressette`)
- [Piquet (Franța)](#piquet) (`piquet`)
- [Batak (Turcia)](#batak) (`batak`)
- [Yaniv (Israel)](#yaniv) (`yaniv`)
- [Ascunselea (Hide-and-seek)](#ascunselea) (`ascunselea`)
- [De-a prinselea (Tag)](#de_a_prinselea) (`de_a_prinselea`)
- [Elasticul](#elasticul) (`elasticul`)
- [Oina](#oina) (`oina`)
- [Loto cu imagini (Picture lotto)](#loto_imagini) (`loto_imagini`)
- [Bầu cua (dovleac, crab, pește)](#bau_cua) (`bau_cua`)
- [Yacht](#yacht) (`yacht`)
- [Chicago (zaruri)](#chicago) (`chicago`)
- [Zara (joc)](#zara) (`zara`)
- [Daldøs](#daldos) (`daldos`)

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

<a id="hex"></a>
## Hex (11×11)

- **Id:** `hex`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hex` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- X unește sus și jos, O unește stânga și dreapta printr-un lanț de hexagoane vecine; nu există remiză, X începe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: f6 (coloane a–k, rânduri 1–11).
- Exemple legale la început: `a1`, `b1`
- La început sunt 121 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="amazoane"></a>
## Amazoane (10×10)

- **Id:** `amazoane`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play amazoane` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Patru amazoane pe parte; muți una ca o regină de șah, apoi tragi o săgeată (tot ca o regină) care blochează pătratul; pierde cine nu mai poate muta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d1-d5/d8 (de la–la/săgeată).
- Exemple legale la început: `d1-c1/b1`, `d1-c1/a1`
- La început sunt 2176 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="linii"></a>
## Linii de acțiune (8×8)

- **Id:** `linii`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play linii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Muți o piesă pe linie atâtea pătrate câte piese sunt pe acea linie; poți sări piese proprii, nu ale adversarului, și captezi aterizând pe una; câștigă cine își unește toate piesele într-un grup (remiză după 300 de mutări).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b1-b3, b1xd3.
- Exemple legale la început: `b1-h1`, `b1-b3`
- La început sunt 36 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="nim"></a>
## Nim cu grămezi

- **Id:** `nim`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play nim` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Patru grămezi (3, 4, 5, 6 bețe); iei oricâte bețe dintr-o singură grămadă; cine ia ultimul băț câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b3 (litera grămezii + număr).
- Exemple legale la început: `a1`, `a2`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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

<a id="popa"></a>
## Popa prostul

- **Id:** `popa`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play popa` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Old Maid românesc: 51 de cărți (fără Dama de treflă), perechile se aruncă automat.
- Tragi pe rând o carte din mâna adversarului, la alegere după poziție; cine rămâne cu popa (Dama nepereche) pierde.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage 3, sfarsit.
- Exemple legale la început: `trage`, `1`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pescuit"></a>
## Pescuit (Go Fish)

- **Id:** `pescuit`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pescuit` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 7 în mână.
- Ceri o valoare pe care o ai (cer 7); adversarul răspunde forțat da (îți dă toate cărțile) sau pescuieste (tragi din stoc; dacă prinzi valoarea cerută mai ceri).
- Patru la fel formează un set; cele mai multe seturi câștigă.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cer K, da, pescuieste.
- Exemple legale la început: `cer`, `2`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="treizeci1"></a>
## Treizeci și unu (31)

- **Id:** `treizeci1`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play treizeci1` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 3 cărți în mână, A=11, figuri=10; valoarea = suma celei mai bune culori (trei la fel = 30).
- Tragi (trage) sau iei din aruncate (ia), apoi arunci (arunca 5h), sau bați (bat): adversarul mai are o tură, apoi cea mai mică mână pierde o viață (bătătorul singur cel mai mic: două). 31 = adversarul pierde o viață. 3 vieți.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, ia, bat, arunca Kd, sfarsit.
- Exemple legale la început: `trage`, `ia`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="casino"></a>
## Cassino (capturi)

- **Id:** `casino`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play casino` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 4 pe masă, 4 în mână.
- Lași o carte (5h) sau capturezi (5h x 2c 3d): grupuri de cărți identice sau cu suma egală cu a ta (A=1, J/Q/K doar identice).
- Puncte pe împărțire: cărți 3, pici 1, 10♦ 2, 2♠ 1, ași 1, mături 1; se joacă până la 11.
- Fără construiri.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `6c`, `6c`
- La început sunt 11 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="zarpoker"></a>
## Poker de zaruri

- **Id:** `zarpoker`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play zarpoker` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 7 runde, 5 zaruri, până la 3 aruncări (arunc; tine 3 3 5 sau tine -; fin).
- Mâini de poker: cinci la fel > careu > full > scară > trei > două perechi > pereche > nimic.
- Mâna terminată a primului jucător rămâne acoperită până termină al doilea; cine are mâna mai bună ia punctul rundei.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `7d`, `Jh`
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
- Exemple legale la început: `5s`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `6-6`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="backgammon"></a>
## Table (Backgammon)

- **Id:** `backgammon`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play backgammon` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Două zaruri aruncate automat; 15 piese, bară, bear-off, dubla se joacă de 4 ori, fără cub de dublare.
- Un pas = un zar; trebuie jucate cât mai multe zaruri.
- Numerotarea e a jucătorului la mutare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 13/7, bar/22, 3/off.
- Exemple legale la început: `8/3`, `13/8`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ludo"></a>
## Nu te supăra, frate (Ludo)

- **Id:** `ludo`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
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
- Exemple legale la început: `Jd`, `9c`
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
- Exemple legale la început: `1b`, `3d`
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
- Exemple legale la început: `1b`, `3c`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `3`, `4`
- La început sunt 22 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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

<a id="riichi"></a>
## Riichi Mahjong (2 jucători)

- **Id:** `riichi`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play riichi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mahjong japonez pentru doi: 136 de piese (1m–9m, 1p–9p, 1s–9s, 1z–7z = vânturi și dragoni), 13 piese, 4 mâini (Est 1–2, Sud 1–2), 25 000 de puncte.
- Tura: trage, apoi arunci (5m), riichi 5m, tsumo, ankan 5m, kakan 5m (după careu: trage-moarta); pe aruncarea adversarului: ron, pon, kan, chi 4m 6m sau pas.
- Mână = 4 seturi + pereche (sau 7 perechi / 13 orfani) cu cel puțin un yaku; furiten, dora/ura-dora, fu și han până la yakuman.
- Zidul e ascuns.
- Fără piese roșii.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sicbo"></a>
## Sic Bo

- **Id:** `sicbo`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sicbo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Trei zaruri; duel contra „casei” cu 100 de jetoane, 12 runde (ordinea de pariere alternează).
- Pariuri (plăți X:1): mic 1, mare 1 (pierd la triplu), cifra N 1/2/3, dublu N 10, triplu N 180, oricare-triplu 30, suma S (4 și 17: 60 … 9–12: 6), pereche a-b 5.
- Cel mult 6 pariuri pe rundă, mize 1/5/10/25.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: mic:5, cifra:3:5, suma:9:5, pereche:1-2:5, fin, arunc, urmatoarea. Câștigă cine are mai multe jetoane.
- Exemple legale la început: `fin`, `mic:1`
- La început sunt 201 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ashtachamma"></a>
## Ashta Chamma

- **Id:** `ashtachamma`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ashtachamma` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 5×5, 4 piese pe jucător, 4 cauri (1–4; zero în sus = 8; 4 și 8 dau încă o aruncare).
- Piesele merg în sens trigonometric pe inelul exterior (16 pătrate) și intră în inelul interior și în centru doar după o captură; centrul se atinge exact.
- Pătrate sigure (X): mijloace de latură, colțuri, centru; două piese ale aceluiași jucător formează un bloc.
- Captura dă încă o aruncare.
- Câștigă cine aduce toate piesele în centru.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, 7 sau 7* (piesa de pe pătratul 7 al căii; * = a capturat), pas.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="daifugo"></a>
## Daifugo (Daihinmin)

- **Id:** `daifugo`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play daifugo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 54 de cărți (cu 2 jokeri), 27 fiecare, 5 runde.
- Ordine 3…K, A, 2, joker.
- Joci un grup de același rang (33, 777), adversarul același număr de cărți mai mari sau pas; după pas conduce cine a jucat ultimul.
- Joker singur (X) sau wild (5X). 4 cărți = revoluție (ordinea se inversează), orice grup cu 8 oprește jocul (conduci din nou).
- Câștigi runda când rămâi fără cărți; perdantul îți dă cele mai bune 2 cărți, tu dai 2 înapoi (da 4 5).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3, 77, JJJ, 55X, X, pas, da 4 5.
- Exemple legale la început: `3`, `3X`
- La început sunt 52 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tienlen"></a>
## Tiến lên

- **Id:** `tienlen`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tienlen` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 13 fiecare (restul ascuns).
- Rang 3…A, 2; culoare ♠ < ♣ < ♦ < ♥.
- Combinații: carte, pereche, treime, careu, scară (3+, fără 2), 3+ perechi consecutive; aceeași formă și mai mare, sau pas.
- Tăieri pe 2: careu sau 3 perechi consecutive pe un 2, careu sau 4 perechi consecutive pe o pereche de 2; 4 perechi consecutive taie și careul.
- Victorie instantanee: patru de 2, șase perechi, scară 3…A.
- Primul la 3 mâini.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3s, 9h 9d, 5c 6d 7h, pas.
- Exemple legale la început: `pas`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bigtwo"></a>
## Big Two (Pusoy dos)

- **Id:** `bigtwo`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play bigtwo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 13 fiecare (restul ascuns).
- Rang 3…A, 2; culoare ♦ < ♣ < ♥ < ♠.
- Carte, pereche, treime sau mână de 5: scară < culoare < full < careu+1 < scară de culoare.
- Dai aceeași formă mai mare ori pas; cine are cea mai mică carte începe cu ea.
- Mâna se termină la golirea cărților: perdantul primește cărțile rămase ca penalizare (dublu de la 10 cărți); la 30 de puncte pierde meciul (cel mult 8 mâini).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3d, 9h 9s, 3d 4c 5h 6s 7d, pas.
- Exemple legale la început: `5c`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tongits"></a>
## Tong-its

- **Id:** `tongits`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tongits` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 12 fiecare (dealerul 13 și începe fără să tragă).
- Tura: trage din stoc sau ia aruncata dacă o folosești imediat (ia 5d 5h 5c / ia sapaw 2), apoi pune seturi sau scări (pune 3h 4h 5h), adaugă la orice combinație (sapaw 6h 2) și aruncă (arunca Kd).
- Mână goală = Tongits, 3 p.; cheama (cu o combinație pusă) → adversarul renunt (1 p.) sau lupta: mai puține puncte în mână câștigă (egalitate: cel care a luptat; dacă pierde cel care a chemat, adversarul ia 2 p.); stoc epuizat: mai puține puncte.
- Primul la 6 p.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pune`, `Kd`
- La început sunt 30 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pasur"></a>
## Pasur

- **Id:** `pasur`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pasur` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 4 pe masă și 4 în mână (se dau din nou câte 4).
- Un număr (A=1…10) capturează orice grupuri de cărți de pe masă cu suma 11 împreună cu el;
- Valetul ia tot în afară de Dame și Regi;
- Dama ia o Damă, Regele un Rege.
- Sur (masa golită, nu cu Valet) 5 p.
- Puncte: cele mai multe cărți 3, ≥7 trefle 7, 10♦ 3, 2♣ 2, Ași 1, Valeți 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7h (pui), 5s x 2c 4d 6h (capturezi), imparte.
- Exemple legale la început: `Kc`, `Jc`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hokm"></a>
## Hokm

- **Id:** `hokm`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play hokm` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, câte 13 (restul ascuns).
- Hakem vede mai întâi 5 cărți și alege atuul (hokm h); conduce prima levată.
- Răspunzi la culoare; atuul bate tot. 7 din 13 levate = 1 p., 13–0 = 2 p., hakem cu zero levate = 3 p.; hakem rămâne dacă câștigă.
- Primul la 7 puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: hokm h (c/d/h/s), 10h, As.
- Exemple legale la început: `hokm`, `c`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pisti"></a>
## Piști

- **Id:** `pisti`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pisti` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți: 4 pe masă (3 cu fața în jos), 4 în mână.
- Cartea ta ia toată grămada dacă are același rang ca cea de deasupra sau e Valet.
- Piști (grămadă de o carte luată cu același rang) 10 p., Valet peste Valet 20 p.
- Puncte: Ași și Valeți 1, 2♣ 2, 10♦ 3, cele mai multe cărți 3.
- Se joacă până la 101.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7h, Js, imparte.
- Exemple legale la început: `Jd`, `2d`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="courtpiece"></a>
## Court Piece (Rang, Mendikot)

- **Id:** `courtpiece`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play courtpiece` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, câte 26.
- Cine nu e dealer vede primele 5 cărți și alege atuul (rang h), apoi se împarte restul și el conduce.
- Răspunzi la culoare; atuul bate tot.
- Mâna o câștigă cine ia mai multe zeci (mendi): 1 p.; toate 4 zecile (Mendikot) 2 p.; toate levatele (Court) 3 p.; la 2–2 decid levatele.
- Primul la 5 puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: rang h, 10h, As.
- Exemple legale la început: `Qh`, `8h`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="callbreak"></a>
## Call Break

- **Id:** `callbreak`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play callbreak` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, câte 13 (restul ascuns); pica e mereu atu.
- Cine nu e dealer licitează primul (licit 4) numărul de levate și conduce.
- Răspunzi la culoare și, dacă poți, cu o carte mai mare; fără culoare tai cu pică.
- Scor: licitația + 0,1 pe levată în plus, sau − licitația dacă ai luat mai puțin. 5 mâini, scorul total mai mare câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: licit 4, 10h, As.
- Exemple legale la început: `licit`, `1`
- La început sunt 26 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `T2`, `T5`
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
- Exemple legale la început: `rsari`, `r6`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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

<a id="itinerar"></a>
## Itinerar (turism pe tablă, rezervi destinații)

- **Id:** `itinerar`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play itinerar` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original, 28 de câmpuri (7 regiuni × 3 destinații + aeroporturi, tombolă, vamă, bac, hotel de lux, taxă de turist): arunc, apoi mut (zarurile 2d6 sunt automate); pe o destinație liberă: rezerv (taxa de intrare) sau las; pe una ocupată plătești taxa de cazare.
- La final: construiesc N (cazare, 3 niveluri), rezerv N (la distanță, preț ×1,5, o dată pe tură), zbor (între aeroporturi, 60) sau gata.
- Colecție completă de regiune = taxe ×2; dublă = încă o aruncare, a treia dublă = vamă (pierzi tura); lux (nivel gratuit la Hotelul de lux).
- Faliment = eliminare; după 40 de runde câștigă averea mai mare.
- Informație publică.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="magistrale"></a>
## Magistrale (rute cu cărți de vagon)

- **Id:** `magistrale`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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
- **Informație ascunsă:** nu
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play litere` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original pe tablă 15×15 cu 97 de plăcuțe (2 blank): formezi cuvinte legate de cele existente, cu pătrate premiate (# cuvânt ×3, + cuvânt ×2, x literă ×3, - literă ×2); +50 cu toate cele 7 plăcuțe.
- Dicționar mic românesc încorporat (~4000 de cuvinte uzuale, fără diacritice; cuvintele absente sunt respinse).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: casa@h8> (orizontal), casa@h8v (vertical), ca(s)a@h8> (blank între paranteze), schimb (toată mâna), pas. Primul cuvânt acoperă h8. Informație ascunsă: vezi doar mâna ta.
- Exemple legale la început: `bem@f8>`, `bem@g8>`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="faianta"></a>
## Faianța (variantă originală)

- **Id:** `faianta`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play faianta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Draft de plăcuțe pe modele, 2 jucători: iei toate plăcuțele de o culoare dintr-o manufactură (restul trec în centru) sau din centru (primul ia jetonul de start, −1) și le pui pe o linie de model (1–5, o singură culoare, nu dacă peretele are deja culoarea) sau pe podea (−1…−3).
- La sfârșitul rundei (mutarea forțată decontare) liniile pline urcă pe perete 5×5 și punctează adiacența; jocul se oprește la un rând complet; bonus rând +2, coloană +7, culoare +10.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: f3 T 2, c O p, decontare.
- Exemple legale la început: `f1`, `T`
- La început sunt 306 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bresla"></a>
## Bresla (variantă originală)

- **Id:** `bresla`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play curieri` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Deducție cu 16 cărți: tragi, joci una din două. 1 Iscoadă ghicește (iscoada:5), 2 Crainic se uită, 3 Judecător compară, 4 Străjer protejează, 5 Mag (mag:eu/mag:el) schimbă cartea, 6 Cămăraș schimbă mâinile, 7 Contesă (obligatorie cu 5/6), 8 Prințesă (aruncată = ieși).
- Ultimul rămas sau cartea cea mai mare câștigă runda; primul cu 3 runde câștigă.
- Informație ascunsă: vezi doar cartea ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `judecator`, `strajer`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fitil"></a>
## Fitil (variantă originală)

- **Id:** `fitil`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play fitil` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- O Bombă în pachet: joci orice cărți, apoi tragi (trag).
- Fără Dezamorsare, bomba te scoate; cu ea o repui unde vrei (dezamorsez 0..N).
- Cărți: atac (adversarul trage de 2 ori), sari, amesteca, privire (primele 3), cere (adversarul îți dă o carte: dau X), pereche A|B|C (furi o carte la întâmplare).
- Informație ascunsă: vezi doar mâna ta, privirea ta și locul bombei puse de tine.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `trag`, `juc`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `alege`, `salata`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `juc`, `B10`
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
- La început sunt 26 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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

<a id="cursa_probelor"></a>
## Cursa probelor (joc de familie cu mini-probe, 2–4 jucători)

- **Id:** `cursa_probelor`
- **Categorie:** Familie
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cursa_probelor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de familie propriu: pistă circulară de 30 de câmpuri și zar d6 (zar).
- Fiecare câmp declanșează o mini-probă: N numărare (raspuns 1|2|3), M memorie scurtă (gata, apoi literele A–D pe rând), D duel de zaruri (duel 2), R pariu de risc (arunc / opresc: până la 3 zaruri, un 1 pierde, țintă 9), L logică/aritmetică generată procedural (raspuns 1|2|3), O odihnă (a doua șansă).
- Începi cu jucatori 2|3|4; ceilalți sunt roboți (Easy–Expert; răspund doar din ce vezi și tu, cu zgomot).
- Primul cu 5 medalii câștigă; după 3 ture, cele mai multe medalii; egalitate la vârf = remiză.
- Mutări automate: rezolv, robot.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `jucatori`, `2`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="capcana"></a>
## Capcana (variantă originală)

- **Id:** `capcana`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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

<a id="oware"></a>
## Oware (Ghana)

- **Id:** `oware`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play oware` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 × 6 gropi, 4 semințe în fiecare; semănat trigonometric (groapa de start e sărită la 12+ semințe), fără lanț.
- Captură: dacă ultima sămânță cade într-o groapă adversă ajunsă la 2 sau 3 semințe, le iei și continui înapoi cât timp grupele au 2 sau 3; mutarea care ar goli complet partea adversă nu capturează (grand slam).
- Dacă adversarul nu are semințe, trebuie să-l hrănești.
- Câștigă primul cu 25.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–6.
- Exemple legale la început: `1`, `2`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bao"></a>
## Bao (Bao la Kiswahili)

- **Id:** `bao`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play bao` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 4 × 8, două rânduri pe jucător, 22 de semințe în rezervă și nyumba cu 6.
- Namua: captura e obligatorie (pui o sămânță într-o groapă din față, iei groapa opusă și o semeni de la capătul rândului); altfel pui o sămânță și semeni sensul ales.
- Mtaji: muți dintr-o groapă cu ≥ 2 semințe; captură din prima aterizare obligatorie.
- Câștigi golind rândul din față al adversarului.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3r, 3l (sens), b3l (rândul din spate).
- Exemple legale la început: `5r`, `5l`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="seega"></a>
## Seega

- **Id:** `seega`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play seega` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 5 × 5, 12 piese pe jucător.
- Plasare: două piese pe tură, fără centru; apoi mută cine a plasat ultima, un pas ortogonal.
- Captură custodială a unui șir de piese adverse între piesa mutată și o piesă proprie; cine capturează mută din nou.
- Blocaj: adversarul scoate o piesă proprie.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b2 (plasare), b2-b3, scoate-b2.
- Exemple legale la început: `a1`, `b1`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="yote"></a>
## Yote

- **Id:** `yote`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play yote` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 5 × 6, 12 piese pe jucător, inițial în mână.
- La rând: pui o piesă, muți un pas ortogonal sau sari peste o piesă adversă (o captezi și mai scoți încă una la alegere; săritura se poate lanțui).
- Câștigă cine rămâne singur cu piese.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c3, c3-c4, c3xc5+a1, c3xc5+m, opreste.
- Exemple legale la început: `a1`, `b1`
- La început sunt 30 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hus"></a>
## Hus (Namibia)

- **Id:** `hus`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hus` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mancala 4 × 8, 2 semințe în fiecare groapă.
- Muți dintr-o groapă proprie cu ≥ 2 semințe; ultima sămânță într-o groapă ocupată: din rândul din față cu groapa opusă ocupată captezi coloana adversă (față + spate) și o semeni mai departe; altfel continui lanțul.
- Pierde cine nu mai are nicio groapă cu ≥ 2.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3 (față), b3 (spate).
- Exemple legale la început: `1`, `2`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tab"></a>
## Tâb (Egipt)

- **Id:** `tab`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tab` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cursă cu 4 bețe-zaruri pe tablă 4 × 7 (variantă simplificată): 7 piese pe jucător; intri cu 1; 1, 4 și 6 dau aruncare în plus; drum în zigzag pe cele 4 rânduri; aterizarea pe o piesă adversă (în afara rândului ei de plecare) o captură.
- Câștigă primul care scoate toate piesele.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunca, intra-a1, a1-a3, a1-iese, pas.
- Exemple legale la început: `arunca`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="senet"></a>
## Senet

- **Id:** `senet`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play senet` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cursă egipteană pe 30 de pătrate (reconstituirea lui Kendall), 5 piese pe jucător, 4 bețe (1–4, zero = 5; 1, 4, 5 dau aruncare în plus).
- Schimbi locul cu o piesă adversă neprotejată; nu treci de 26; 27 (apa) te trimite pe 15; ieșire din 26 cu 5, din 28 cu 3, din 29 cu 2, din 30 cu 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunca, 7-9, 28-iese, pas.
- Exemple legale la început: `arunca`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="whot"></a>
## Whot (cărți nigeriene)

- **Id:** `whot`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play whot` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet de 54 de cărți (forme o t x p s, plus 5 Whot), 6 în mână.
- Joci aceeași formă sau același număr;
- Whot cere o formă. 1 și 8: mai joci o dată; 14: adversarul trage o carte; 2: trage 2, 5: trage 3 (se pot înlănțui).
- Câștigi golindu-ți mâna.
- Informație ascunsă: vezi mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: o5, t14, w:s, trage, pas.
- Exemple legale la început: `t11`, `w:o`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chkobba"></a>
## Chkobba (Tunisia)

- **Id:** `chkobba`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play chkobba` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet de 40 de cărți (1–7, damă, valet, rege), 3 în mână, 4 pe masă.
- Iei o carte de aceeași valoare, altfel un grup cu aceeași sumă; golești masa = chkobba (nu la ultima carte).
- Punctaj: carte, denari, bermila (cei mai mulți de 7, apoi de 6), haya (7 de denari), chkobba; meci până la 11.
- Informație ascunsă: vezi mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7d, 7d:3c+4b, imparte, urmatoarea.
- Exemple legale la început: `7b:7d`, `Cc`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ronda"></a>
## Ronda (Maroc / Spania)

- **Id:** `ronda`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play ronda` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet spaniol de 40 de cărți, 4 pe masă, 3 în mână.
- Joci o carte: dacă pe masă e același rang, iei toate cărțile acelui rang și apoi, în șir, rangurile următoare prezente.
- Puncte: ronda (pereche în mână) 1, trio 5, missa (masă golită) 1, peste 20 de cărți 1; meci până la 12.
- Informație ascunsă: vezi mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 5o, Rb, imparte, urmatoarea.
- Exemple legale la început: `4o`, `Cc`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="shologuti"></a>
## Sholo Guti (Șaisprezece soldați)

- **Id:** `shologuti`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play shologuti` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 16 piese pe jucător pe o tablă de 37 de puncte (grilă 5 × 5 cu diagonale plus două triunghiuri), rândul din mijloc gol.
- Mută un pas pe un punct liber vecin sau sare peste o piesă adversă (captură obligatorie, lanț opțional).
- Câștigă cine capturează tot sau blochează adversarul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c2-c3, c1xc3, opreste.
- Exemple legale la început: `a2-a3`, `b2-b3`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="aadupuli"></a>
## Aadu Puli Attam (Oi și tigri)

- **Id:** `aadupuli`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play aadupuli` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 3 tigri (Jucătorul 2) contra a 15 oi (Jucătorul 1) pe o tablă de 23 de puncte în formă de triunghi.
- Oile se plasează pe rând, apoi se mută un pas; tigrii se mută sau sar peste o oaie (captură, lanț opțional).
- Tigrii câștigă cu 5 oi capturate; oile câștigă imobilizându-i.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b3, b3-b4, vxb2, opreste.
- Exemple legale la început: `b1`, `a2`
- La început sunt 20 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mangala"></a>
## Mangala (mancala turcească)

- **Id:** `mangala`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play mangala` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 × 6 gropi, 4 semințe, câte un depozit.
- Prima sămânță rămâne în groapa de start.
- Ultima în depozit: mai muți; ultima într-o groapă adversă ajunsă la număr par: o iei; ultima într-o groapă proprie goală: iei groapa opusă.
- Câștigă depozitul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–6.
- Exemple legale la început: `1`, `2`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="dama"></a>
## Dama (dame turcești)

- **Id:** `dama`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play dama` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 8 × 8, 16 piese pe jucător pe rândurile 2–3.
- Piesele merg înainte sau lateral, capturează ortogonal; dama zboară pe orice distanță.
- Captura e obligatorie, cu cele mai multe piese; piesele luate se scot la finalul secvenței.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a3-a4, a3xa5xc5.
- Exemple legale la început: `a3-a4`, `b3-b4`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="okey"></a>
## Okey (2 jucători)

- **Id:** `okey`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play okey` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 106 plăci (4 culori × 1–13 × 2 + 2 okey false), indicator și joker. 15/14 plăci; tragi din teanc (trage) sau iei ultima aruncată (ia), apoi arunci (ata-r5).
- Câștigi închizând 14 plăci în serii (≥ 3 consecutive, aceeași culoare) și grupe (3–4 cu același număr și culori diferite): bitir-r5.
- Informație ascunsă: vezi mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ata-k5`, `ata-r8`
- La început sunt 13 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
## Spargerea codului (cu schimb de roluri)

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
## Perechi (joc de memorie)

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
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pachisi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 6 cauri: niciunul deschis = 25, 1 deschis = 10, 2 = 2, 3 = 3, 4 = 4, 5 = 5, 6 = 6; 6, 10 și 25 sunt grațioase (încă o aruncare și poți scoate o piesă din bază).
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
- **Informație ascunsă:** nu
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
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play roata` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: roata (1–6) e afișată; mutarea avansez alege ramura la bifurcații (facultate/ucenicie, afacere/salariu) și, pe pătratul Casă, cumpar/nu.
- Plată la fiecare trecere, copii și casă valorează la final.
- Scor = bani + 30 casă + 8 per copil.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: avansez, avansez facultate, avansez ucenicie, avansez afacere, avansez salariu, avansez cumpar.
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
- Exemple legale la început: `pune`, `a4`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `b4-b5`, `e4-e5`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play zidurile` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tablă 7×7 cu plăci mobile; introduci placa de rezervă (mete N2 r1) apoi muți pionul pe un drum deschis (mergi c3) ca să iei comorile în ordine, apoi acasă.
- Nu poți anula inserția adversarului.
- După 100 de ture, cine are mai multe comori.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `mete`, `N2`
- La început sunt 72 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="prajituri"></a>
## Drumul Prăjiturilor (cursă de culori)

- **Id:** `prajituri`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
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
- Întrebi (intreb ochelari), adversarul răspunde corect prin mutare forțată (raspund da/nu); ghicești (ghicesc Matei), iar adversarul dă verdictul printr-o mutare forțată (verdict corect / verdict gresit).
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
- Exemple legale la început: `7d`, `Cd`
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
- Exemple legale la început: `7b:7d`, `Cc`
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
- Exemple legale la început: `Kd`, `Ah`
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
- Exemple legale la început: `d`, `3d`
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

<a id="inimi"></a>
## Inimi (evitarea punctelor, 2 jucători)

- **Id:** `inimi`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play inimi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 13 cărți fiecăruia din 52 (restul nevăzut).
- Răspunzi obligatoriu la culoare, ia cartea cea mai mare din culoarea condusă.
- Fiecare inimă = 1 punct, Dama de pică = 13; nu conduci inimă până nu s-a jucat una.
- La 40 de puncte pierde cine are mai multe.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: imparte, 7c, Qs, 10h.
- Exemple legale la început: `8d`, `5d`
- La început sunt 11 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pica"></a>
## Pică (licitație și atu pică, 2 jucători)

- **Id:** `pica`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pica` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 13 cărți fiecăruia din 52; atu = pica.
- Licitezi câte levate iei (licit 1…13) sau licit 0 = nil.
- Reușit: 10×licitație + 1 pe levată în plus (bag; 10 bag-uri = −100), picat: −10×licitație; nil ±50.
- Nu conduci pică până nu s-a jucat una.
- Până la 150 sau 12 mâini.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: imparte, licit 4, As, 7h.
- Exemple legale la început: `licit`, `0`
- La început sunt 28 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="prinde"></a>
## Prinde (licitație exactă, 2 jucători)

- **Id:** `prinde`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play prinde` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 13 runde cu mâini de 1…7…1 cărți; o carte întoarsă dă atuul.
- Spui câte levate iei (licit N); dealerul nu poate aduce suma licitațiilor la numărul de levate.
- Exact = 10 + licitația, altfel 0.
- Culoare obligatorie, atu bate tot.
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: imparte, licit 2, Kd, 4s.
- Exemple legale la început: `licit`, `0`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="whist"></a>
## Whist (2 jucători)

- **Id:** `whist`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play whist` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 13 cărți fiecăruia din 52; ultima carte a dealerului se arată și dă atuul.
- Culoare obligatorie, atu bate tot.
- Cine ia peste 6 levate primește câte un punct pentru fiecare levată peste a șasea; jocul se încheie la 5 puncte (fără onoruri).
- Informație ascunsă: vezi doar mâna ta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: imparte, Ah, 9c.
- Exemple legale la început: `9h`, `2h`
- La început sunt 13 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `9h`, `x`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Exemple legale la început: `3c`, `10c`
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
- **Informație ascunsă:** nu
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
- La început sunt 15 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="strajile"></a>
## Străjile Codrului (cooperativ de apărare, cărți de putere)

- **Id:** `strajile`
- **Categorie:** Cooperativ
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
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
- **Informație ascunsă:** nu
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

<a id="turnul_zorilor"></a>
## Turnul Zorilor Frânte (aventură cooperativă, fantasy eroic)

- **Id:** `turnul_zorilor`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play turnul_zorilor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d20 + modificator ≥ prag.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="arhiva_abisala"></a>
## Arhiva Abisală (aventură cooperativă, groază cosmică de investigator)

- **Id:** `arhiva_abisala`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play arhiva_abisala` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d100 sub abilitate.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ruinele_cartografilor"></a>
## Ruinele Cartografilor (aventură cooperativă, expediție în junglă)

- **Id:** `ruinele_cartografilor`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ruinele_cartografilor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d20 cu grade de succes.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="noapte_de_catifea"></a>
## Noapte de Catifea (aventură cooperativă, urban fantasy cu vampiri)

- **Id:** `noapte_de_catifea`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play noapte_de_catifea` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: grămadă de d10.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="padurea_sticlei"></a>
## Pădurea Ochilor de Sticlă (aventură cooperativă, fantasy de basm sumbru)

- **Id:** `padurea_sticlei`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play padurea_sticlei` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: 3d20 față de trei atribute.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="neon_si_rune"></a>
## Neon și Rune (aventură cooperativă, cyber-urban cu magie)

- **Id:** `neon_si_rune`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play neon_si_rune` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: grămadă de d6 cu lovituri.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="targul_ciumat"></a>
## Târgul Ciumat (aventură cooperativă, fantasy sumbru)

- **Id:** `targul_ciumat`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play targul_ciumat` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d100 cu grade de succes.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="insula_celor_doua_luni"></a>
## Insula celor Două Luni (aventură cooperativă, fantasy clasic)

- **Id:** `insula_celor_doua_luni`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play insula_celor_doua_luni` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: 2d6 + rating față de țintă.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hotii_din_ceata"></a>
## Hoții din Ceață (aventură cooperativă, jaf într-un oraș cu ceață)

- **Id:** `hotii_din_ceata`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hotii_din_ceata` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: cel mai mare d6.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="crucea_de_cenusa"></a>
## Crucea de Cenușă (aventură cooperativă, apocalipsă sumbră)

- **Id:** `crucea_de_cenusa`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play crucea_de_cenusa` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d20 față de prag fix.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="strada_de_otel"></a>
## Strada de Oțel (aventură cooperativă, cyberpunk de cartier)

- **Id:** `strada_de_otel`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play strada_de_otel` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d10 exploziv.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tinutul_furtunilor"></a>
## Ținutul Furtunilor Vii (aventură cooperativă, fantasy colorat)

- **Id:** `tinutul_furtunilor`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tinutul_furtunilor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d20 + modificator ≥ prag.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="epava_din_inelul_sapte"></a>
## Epava din Inelul Șapte (aventură cooperativă, operă spațială)

- **Id:** `epava_din_inelul_sapte`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play epava_din_inelul_sapte` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d20 + modificator ≥ prag.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fiordul_gheatii_negre"></a>
## Fiordul Gheții Negre (aventură cooperativă, fantasy nordic)

- **Id:** `fiordul_gheatii_negre`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play fiordul_gheatii_negre` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d20 sub abilitate.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mlastina_strigoilor"></a>
## Mlaștina Strigoilor (aventură cooperativă, vânătoare de monștri)

- **Id:** `mlastina_strigoilor`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play mlastina_strigoilor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d10 exploziv.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="satul_lanternelor"></a>
## Satul Lanternelor Stinse (aventură cooperativă, groază de sat cu spirite)

- **Id:** `satul_lanternelor`
- **Categorie:** Cooperativ
- **Jucători:** tu + partener AI contra jocului/adversarilor
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play satul_lanternelor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mecanica de test: d100 sub abilitate.
- Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final).
- Începeți cu alege N (arhetip).
- Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez.
- Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept.
- Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `alege`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play santinela` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci locuri (tu + 4 roboți): trei paznici (unul e Ochiul, care știe sabotorii) și doi sabotori (unul e Vulpea).
- Cinci misiuni cu echipe de 2, 3, 2, 3, 3: conducătorul propune, toți votează, cei din echipă aleg în secret succes sau sabotaj (doar sabotorii pot sabota). 3 reușite = paznicii, dar Vulpea poate ghici Ochiul; 3 ratate sau 5 respingeri = sabotorii.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: echipa 1 3, da, nu, succes, sabotaj, tinta 4, masa.
- Exemple legale la început: `echipa`, `1`
- La început sunt 30 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="consiliul"></a>
## Sfatul Negustorilor (alegeri și legi, roluri ascunse)

- **Id:** `consiliul`
- **Categorie:** Party
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play consiliul` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci locuri (tu + 4 roboți): trei negustori, un complice și Cămătarul (se cunosc între ei).
- Președintele nominalizează un cancelar, toți votează; guvernul ales trage 3 legi, una e aruncată, una promulgată (6 P Prosperitate, 11 M Monopol în pachet). 5 P sau Cămătarul executat = negustorii; 6 M sau Cămătarul cancelar după 3 M = conspiratorii.
- A 3-a lege M: vezi pachetul; a 4-a și a 5-a: execuție.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cancelar 3, da, nu, arunca M, promulga P, executa 4, masa.
- Exemple legale la început: `cancelar`, `2`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="valea"></a>
## Valea Uscată (duel western, roluri ascunse)

- **Id:** `valea`
- **Categorie:** Cărți
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play valea` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci pistolari (tu + 4 roboți): Șerif (public), Adjunct, doi Haiduci, Trădător.
- Legea câștigă dacă mor haiducii și trădătorul, haiducii dacă moare Șeriful, trădătorul rămânând singur.
- Pe tură tragi 2 cărți: foc (o dată, distanță 1; 2 cu pușcă), ocolire, leac, salon, mitralieră, pușcă, panică; arunci la câte vieți ai.
- Secetă după runda 30.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: foc 3, leac, salon, mitraliera, pusca, panica 2, incheie, scarta foc, ocolire, accept, masa.
- Exemple legale la început: `incheie`, `foc`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hruba"></a>
## Hoinari prin Hrubă (monștri și comori, masa = roboți)

- **Id:** `hruba`
- **Categorie:** Cărți
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** nu
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
- **Informație ascunsă:** nu
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
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play fulger` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 31 de cărți cu 6 simboluri; oricare două au exact un simbol comun.
- Numești simbolul comun dintre cartea ta și cea din mijloc; masa (robot) are un timp de reacție tras la zar și greșește uneori.
- Cine e corect și mai rapid ia cartea din mijloc (+1 punct).
- Simbol greșit = runda masei.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: numele simbolului (ex. luna, stea), cronometru.
- Exemple legale la început: `ancora`, `dop`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="roade"></a>
## Roade de Toamnă (legume și schimburi, masa = roboți)

- **Id:** `roade`
- **Categorie:** Cărți
- **Jucători:** tu + roboți (restul mesei e simulat); `--you both` nu are sens pentru toți
- **Informație ascunsă:** nu
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
- Exemple legale la început: `activ`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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
- Tură: pesca (mutare forțată la începutul fiecărei ture; în prima tură J1 o scrie tot, dar nu trage nicio carte), cerneala N (o carte cu cerneală pe tură în călimară = 1 cerneală pe tură), joaca N [e2|m1] (personajele se usucă o tură), misiune P (aduce faimă), provocare P e Q (doar contra personajelor ÎNCLINATE), fin.
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
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play spionii` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă proprie de joc de spionaj cu cuvinte: grilă de 25 de cuvinte, 9 agenți roșii (J1), 8 albaștri (J2), 7 neutre, 1 asasin.
- Maestrul-spion al fiecărei echipe este calculatorul: dă ca indiciu numele unei teme din tabela proprie (MARE, CIRC…) și un număr.
- Tu ești ghicitorul: ghici cuvânt (fără diacritice), apoi dezvaluie întoarce cartea; cel mult număr+1 ghiciri, stop încheie tura.
- Asasinul pierde pe loc.
- AI-ul ghicește după aceeași tabelă de teme, fără să vadă cheia.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ghici`, `ceas`
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
- Exemple legale la început: `indiciu`, `rosie`
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
- **Informație ascunsă:** nu
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
- **Informație ascunsă:** nu
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
- Exemple legale la început: `pune`, `piata`
- La început sunt 33 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="taramuri"></a>
## Tărâmuri (transformarea terenului și piste de ritual)

- **Id:** `taramuri`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play taramuri` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc original pe hartă 7×6 cu 7 tipuri de teren pe o roată: J1 e de tip 0, J2 de tip 3.
- Pe rând o acțiune: colonie c3 (transformi terenul vecin în al tău, plătind lopeți, și pui o locuință), oras c3, templu c3 (îmbunătățiri), preot foc|apa|pamant|aer (+2 trepte pe pista de cult; la 5 privilegiu), pas.
- Început: inicial c3. 6 runde cu venit și un bonus de rundă; la final puncte pe piste, cel mai mare grup de clădiri și monede.
- Fără mană, fără facțiuni distincte, vecinătate pe 4 direcții.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `inicial`, `d1`
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
- La început sunt 74 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="reverdire"></a>
## Reverdire (variantă originală)

- **Id:** `reverdire`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play reverdire` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Motor de resurse și proiecte de cărți: ridici Umiditatea, Aerul și Lacurile (RR).
- O generație începe cu mutarea forțată generatie, apoi Cercetare (compra N, 3 L; gata), acțiuni una pe tură (joc N, vinde N, pompa, irigare, plantare, asezare, conv sem, conv caldura, pas; apoi pune c3), producție.
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play salina` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Plasare de agenți și construire de pachet, 10 runde sau 10 PV. agent 3 e (carte + locație a–l cu pictograma potrivită; la locațiile de luptă e–h: agent 3 e 2 = desfășori 2 trupe), revela apoi compra N / apa (1 Apă = +1 persuasiune) / sare (1 Sare = +1 sabie) / fin.
- Conflictul rundei: forța = 2 × trupe + săbii; premiul I/II; runda pregătește runda următoare.
- Informație ascunsă: mâna adversarului și pachetele.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `agent`, `1`
- La început sunt 226 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="catun"></a>
## Cătun (variantă originală)

- **Id:** `catun`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play catun` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Plasare de muncitori și cătun de cărți, 4 anotimpuri (2,3,4,6 muncitori). lucru a (locație a–h; lucru g L la Han), joc N / piata N (carte din mână / piață, max 15 în cătun), sezon (pregătire: muncitorii se întorc, producătoarele lucrează; în Toamnă ieși din joc).
- Scor: puncte de pe cărți, jetoane, prosperitate.
- Informație ascunsă: mâna adversarului și pachetul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `lucru`, `a`
- La început sunt 31 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="solzi"></a>
## Solzi (variantă originală)

- **Id:** `solzi`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play solzi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Motor de cărți-reptile și amfibieni cu habitate, hrană și ouă, 4 runde (8,7,6,5 acțiuni). joc N p|s|b (vietate în Pădure/Stâncă/Baltă, plătești hrană și ouă), padure I V (hrană), stanca p1 s2 (ouă), balta (cărți).
- Puterile cărților se declanșează doar în habitatul acțiunii.
- Obiective de rundă; scor: puncte de pe cărți + ouă + obiective.
- Informație ascunsă: mâna adversarului și pachetul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `joc`, `1`
- La început sunt 19 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="poieni"></a>
## Poieni (variantă originală)

- **Id:** `poieni`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tinuturi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Domino de teritorii cu coroane, 2 jucători: regat 9×9 cu castel în e5, încape în 7×7. 24 de piese-domino trase din setul nostru de 48 (generate de noi) în rânduri de câte 4: proprietarul piesei cu numărul mic joacă primul.
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play rezervatia` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Rezervație naturală cu cărți-animale și sponsori.
- Cinci acțiuni (Faună, Amenajare, Sponsori, Cărți, Conservare) într-un rând: puterea = locul (1–5); cea jucată trece pe locul 1. animal 2 e1 (la putere ≥ 4 încă un animal sau gata), construieste padure 3 / chiosc / pavilion, sponsor 3 / finantare, carti / carti v2, proiect 1 / donatie.
- Pauză la ritm 9: venit.
- Scor = apel + 2×conservare; la 50 jocul se încheie la sfârșitul rundei.
- Informație ascunsă: mâna adversarului și pachetul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `construieste`, `padure`
- La început sunt 49 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hotar"></a>
## Hotarul (variantă originală)

- **Id:** `hotar`
- **Categorie:** Strategie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
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
- **Informație ascunsă:** nu
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
- La început sunt 33 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

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

<a id="coridor"></a>
## Coridor (cursă de pioni cu ziduri)

- **Id:** `coridor`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play coridor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 9×9;
- J1 (X) pornește din e1 și ajunge pe rândul 9, J2 (O) din e9 spre rândul 1.
- Pe rând: un pas ortogonal (se sare peste pionul adversar vecin, în diagonală dacă în spate e zid/margine) SAU un zid (10 de jucător, lung cât două pătrate) care nu se suprapune, nu se încrucișează și nu închide complet calea nimănui.
- Remiză la 200 de semi-mutări.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e2 (pion), d4h (zid orizontal peste d4–e4), d4v (zid vertical la dreapta lui d4–d5).
- Exemple legale la început: `e2`, `f1`
- La început sunt 131 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="drum"></a>
## Drum de pietre (5×5, stive)

- **Id:** `drum`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play drum` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 21 de pietre plate și o căpetenie de jucător.
- Prima mutare: pui o piatră plată a adversarului.
- Apoi: pui o piatră pe un pătrat gol (c3 plată, sc3 zid, cc3 căpetenie) sau muți o stivă proprie: iei de sus 1–5 pietre și le lași pe o linie (+ sus, - jos, < stânga, > dreapta), cel puțin una pe pătrat; ex. c3> sau 3c3>21.
- Zidul și căpetenia blochează stivele; doar căpetenia lăsată singură pe zid îl turtește.
- Câștigă drumul (lanț de pietre plate/căpetenii proprii între margini opuse); la tablă plină, rezervă goală sau 150 de semi-mutări câștigă cine are mai multe pietre plate la vârf.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `a1`, `b1`
- La început sunt 25 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tripierd"></a>
## Trei-pierde (hexagonal)

- **Id:** `tripierd`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tripierd` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă hexagonală cu latura 5 (61 de celule, rânduri a–i).
- Pe rând pui o piesă pe o celulă goală: patru (sau mai multe) în linie câștigă, exact trei în linie (fără patru) pierde; tabla plină = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e5 (litera = rândul, cifra = poziția în rând).
- Exemple legale la început: `e5`, `d4`
- La început sunt 61 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="konane"></a>
## Konane (dame hawaiiene)

- **Id:** `konane`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play konane` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 8×8 plină în șah (negrul = J1 pe a1, c1, …).
- Deschidere: J1 ridică d4, e5, a1 sau h8 (remove d4), J2 ridică o piesă proprie vecină cu golul.
- Apoi sari ortogonal peste o piesă adversă într-un pătrat gol (o scoți), poți continua în aceeași direcție.
- Cine nu mai poate muta pierde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: remove d4, c3-e3, c3-g3 (salt dublu).
- Exemple legale la început: `remove`, `d4`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fanorona"></a>
## Fanorona (Madagascar)

- **Id:** `fanorona`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play fanorona` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 5×9 de puncte (diagonale doar pe punctele pare), 22 piese pe jucător.
- Mutare cu un pas pe un punct liber; dacă pasul continuă spre o piesă adversă o captură pe ea și șirul din spatele ei (abordare, a), iar dacă te îndepărtezi de o piesă adversă o captură la fel (retragere, w).
- Captura e obligatorie; după ea aceeași piesă poate continua (opțional) cu alte capturi, fără să repete direcția și fără să revină pe un punct deja ocupat în tură.
- Câștigi scoțând toate piesele adverse; remiză după 40 de semi-mutări fără captură.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e2-e3, c3-d3a, c3-d3w, c3-d3a-d4w-e5a (lanț; fiecare prefix al unui lanț apare în listă ca mutare separată, iar mutarea aleasă încheie tura: așa alegi unde te oprești).
- Exemple legale la început: `d2-e3a`, `e2-e3a`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tigri"></a>
## Tigri și capre (Nepal)

- **Id:** `tigri`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tigri` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 5×5 de puncte (diagonale pe punctele pare).
- Tu ești caprele (20, în mână) și muți primul; calculatorul are 4 tigri în colțuri.
- Caprele se plasează pe rând pe puncte libere, apoi se mută cu un pas.
- Tigrii se mută cu un pas sau sar peste o capră vecină pe punctul liber de după ea, capturând-o (neobligatoriu).
- Tigrii câștigă cu 5 capre capturate; caprele câștigă imobilizând toți tigrii; remiză după 100 de semi-mutări fără captură în faza de mutare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c3 (plasare), a1-b2 (pas), a1xc1 (săritură cu captură).
- Exemple legale la început: `b1`, `c1`
- La început sunt 21 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="surakarta"></a>
## Surakarta (Java)

- **Id:** `surakarta`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play surakarta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 6×6, 12 piese pe jucător.
- Mutare simplă pe un punct vecin liber (și diagonal).
- Captura: piesa aleargă pe unul din cele două circuite (liniile 2 și 5 = mic; 3 și 4 = mare, legate prin arcuri în colțuri), în oricare sens, peste puncte libere, trece prin cel puțin un arc și ia prima piesă întâlnită dacă e a adversarului; propriile piese blochează, iar locul de plecare contează ca liber.
- Câștigi scoțând toate piesele adverse; remiză după 60 de semi-mutări fără captură.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b2-b3 (pas), a2xb6 (captură).
- Exemple legale la început: `a2-a3`, `a2-b3`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hasami"></a>
## Hasami Shogi (Japonia)

- **Id:** `hasami`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hasami` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 9×9, 9 piese pe jucător pe rândul din spate.
- Piesele se mută ca turnul (fără sărituri).
- Captură prin încadrare: un șir de piese adverse consecutive, pe rând sau coloană, aflat între piesa mutată și o altă piesă a ta, este scos; poți intra liber între două piese adverse.
- Câștigi lăsând adversarul cu cel mult o piesă sau blocat; remiză după 100 de semi-mutări fără captură (colțurile nu au regulă specială).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1-a5 (fișiere a–i, rânduri 1–9).
- Exemple legale la început: `a1-a2`, `a1-a3`
- La început sunt 63 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="puncte"></a>
## Puncte și căsuțe (5×5)

- **Id:** `puncte`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play puncte` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Rețea de 6×6 puncte (5×5 căsuțe).
- Pe rând tragi o linie între două puncte vecine; cine închide a patra latură a unei căsuțe o câștigă și mută din nou (o linie poate închide două căsuțe).
- Când s-au tras toate cele 60 de linii, câștigă cine are mai multe căsuțe (25, deci fără remiză).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: oa1 (linie orizontală, de la punctul a1 spre dreapta), va1 (linie verticală, de la punctul a1 în sus); coloane a–f, rânduri 1–6.
- Exemple legale la început: `oa1`, `ob1`
- La început sunt 60 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="strapungere"></a>
## Străpungere (8×8)

- **Id:** `strapungere`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play strapungere` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Fiecare jucător are 16 piese pe primele două rânduri.
- O piesă înaintează un pas drept pe un pătrat gol sau un pas în diagonală pe un pătrat gol ori ocupat de adversar (captură); înainte nu se capturează.
- Câștigă cine ajunge primul pe rândul de capăt al adversarului sau cine capturează toate piesele adverse; cine nu mai are nicio mutare pierde.
- J1 (X) urcă, J2 (O) coboară.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b2-b3, b2-c3, b2xc3.
- Exemple legale la început: `a2-a3`, `a2-b3`
- La început sunt 22 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="izolare"></a>
## Izolare (7×7)

- **Id:** `izolare`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play izolare` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Fiecare jucător are un pion (X la d1, O la d7).
- O tură: muți pionul ca regina la șah (orice distanță în linie dreaptă sau diagonală, doar peste pătrate libere), apoi scoți definitiv de pe tablă un pătrat liber (nu cel cu pionul adversarului).
- Cine nu mai poate muta la începutul turei pierde; nu există remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d1-d3/e5 (de la, la, pătratul scos).
- Exemple legale la început: `d1-d4/c6`, `d1-d4/d6`
- La început sunt 799 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="rotire"></a>
## Rotire în cinci (6×6)

- **Id:** `rotire`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play rotire` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 6×6 din patru cadrane 3×3 (1 stânga-sus, 2 dreapta-sus, 3 stânga-jos, 4 dreapta-jos).
- O tură: pui o bilă pe un pătrat gol, apoi rotești un cadran cu un sfert de tură (d = în sensul acelor de ceasornic, s = invers).
- Cinci bile în linie (orizontal, vertical, diagonal) după rotire câștigă; dacă ambii jucători au cinci deodată sau tabla se umple fără cinci, e remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c3/1d (bila pe c3, cadranul 1 spre dreapta).
- Exemple legale la început: `c3/1d`, `c3/1s`
- La început sunt 288 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="coloane"></a>
## Coloane (dame în turnuri)

- **Id:** `coloane`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play coloane` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 7×7 pe pătratele închise, 11 piese pe jucător.
- Soldatul merge un pas în diagonală înainte, ofițerul (soldat ajuns pe ultimul rând) în toate diagonalele.
- Captura e obligatorie: sari peste o coloană adversă vecină pe un pătrat liber și iei doar piesa ei de sus, care ajunge sub coloana ta; săriturile se înlănțuie, o coloană nu poate fi sărită de două ori, promovarea încheie tura.
- Coloana e condusă de piesa de sus.
- Pierde cine nu mai poate muta; remiză după 80 de semi-mutări fără captură.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c3-d4 (pas), c3xe5xg3 (salturi).
- Exemple legale la început: `a3-b4`, `c3-d4`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bile"></a>
## Bile pe hexagoane (5 pe latură)

- **Id:** `bile`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play bile` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă hexagonală cu 61 de celule, 14 bile pe jucător.
- Muți 1-3 bile proprii vecine pe aceeași linie cu un pas: pe lungime (poți împinge 1 sau 2 bile adverse dacă ai mai multe bile decât ele și locul de dincolo e liber sau în afara tablei) ori lateral (toate destinațiile libere).
- Bilele împinse în afară sunt capturate; câștigă cine scoate 6; remiză după 120 de semi-mutări fără captură.
- Celule: rândul a (jos) – i (sus) și coloana 1-9.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e5>ne (o bilă), c3-c5>se (grup, de la prima la ultima bilă); direcții ne, e, se, sw, w, nw.
- Exemple legale la început: `c3>ne`, `c3>w`
- La început sunt 68 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="patru"></a>
## Patru atribute (piesa dată)

- **Id:** `patru`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play patru` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 16 piese distincte cu patru însușiri binare (culoare, înălțime, formă, vârf), tablă 4×4.
- În tura ta pui pe tablă piesa primită de la adversar, apoi alegi una dintre cele rămase și i-o dai lui; prima tură doar dă o piesă.
- Câștigă cine completează un rând, o coloană sau o diagonală de patru piese cu o însușire comună; tabla plină = remiză.
- Piesele se numesc 0-9, a-f (biții = însușirile).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: >5 (prima tură), c3>7 (pui pe c3, dai piesa 7), c3 (mutare câștigătoare sau ultima).
- Exemple legale la început: `>0`, `>1`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cupe"></a>
## Cupe încuibate (4×4)

- **Id:** `cupe`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cupe` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 4×4; fiecare jucător are 3 stive de câte 4 cupe (mărimi 1-4) încuibate, se ia doar cea de sus.
- Pui o cupă din rezervă pe o celulă goală sau peste una mai mică, sau muți o cupă proprie vizibilă pe una goală ori mai mică.
- Din rezervă poți acoperi o cupă adversă doar dacă face parte dintr-un șir de trei piese adverse vizibile pe o linie.
- Câștigă patru cupe vizibile pe o linie; dacă ridici o cupă și descoperi șirul de patru al adversarului, el câștigă.
- Remiză după 50 de semi-mutări fără punere din rezervă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3@b2 (din rezervă, mărimea 3), b2-c3 (mutare pe tablă).
- Exemple legale la început: `4@a1`, `4@b1`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="poliedre"></a>
## Poliedre (duo, 14×14)

- **Id:** `poliedre`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play poliedre` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 14×14, 2 jucători, fiecare cu 21 de poliomino (1-5 pătrate, notate a-u).
- Prima piesă acoperă pătratul de start (e5 pentru J1, j10 pentru J2); următoarele trebuie să atingă o piesă proprie prin colț și niciodată pe latură (piesele adverse se pot atinge).
- Cine nu poate pune trece; la două treceri la rând jocul se termină și câștigă cine are mai puține pătrate în piesele nefolosite.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: k3@e5 (piesa k, orientarea 3, colțul stânga-sus al dreptunghiului piesei la e5), pas.
- Exemple legale la început: `a1@e5`, `b1@d5`
- La început sunt 414 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="impletitura"></a>
## Împletitura (echilibru pe covor cu cercuri colorate)

- **Id:** `impletitura`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play impletitura` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Covor de 4 rânduri (a–d) × 6 coloane (1–6) cu cercuri în 4 culori (Roșu, Albastru, Verde, Galben; câte 6, după seed), 2 jucători, fiecare cu 4 membre (ms, md, ps, pd) care încep în afara covorului.
- Tura începe cu rotița (gira), care dă un membru și o culoare; muți membrul pe un cerc liber de acea culoare (b3) la distanță cel mult 3 (Chebyshev) de celelalte membre ale tale de pe covor; un cerc nu poate avea două membre; poți rămâne pe cercul curent dacă are culoarea dată.
- Fără cerc valid, singura mutare este cad: cazi și pierzi; ultimul rămas câștigă (remiză după 300 de mutări fără cădere).
- Fără informație ascunsă;
- AI: Easy aleator, Normal păstrează flexibilitatea, Hard/Expert îți blochează cercurile.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: gira, b3, cad.
- Exemple legale la început: `gira`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pinochle"></a>
## Pinochle

- **Id:** `pinochle`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pinochle` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 48 de cărți (două de fiecare 9–A pe 4 culori), 12 fiecare, 2 jucători, primul în stoc.
- Se joacă 24 de levate; cât timp stocul nu e gol nu ești obligat să urmezi culoarea, după aceea trebuie să urmezi și să învingi când poți.
- Câștigătorul unei levate poate anunța o combinație (pereche, familie, pinochle, cuplu etc.) care conține cel puțin o carte nouă; la sfârșit se numără A, 10, K din levate.
- Primul la 1000.
- Fără schimbul lui 9 de atu.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 9h, anunt <combinatie>, fara, urmatoarea.
- Exemple legale la început: `9c`, `Ah`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="gin_rummy"></a>
## Gin Rummy

- **Id:** `gin_rummy`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play gin_rummy` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 10 fiecare.
- La rând: iei din talon (talon) sau stoc (stoc), apoi arunci (dis X).
- Cu deadwood de cel mult 10 poți bate (bat X) sau da gin (gin X, 0 deadwood); adversarul poate adăuga cărți la combinațiile tale (adauga X la N).
- Se punctează diferența de deadwood, undercut pentru apărător; până la 100.
- Fără big gin.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: talon, pas, stoc, dis 9h, bat 9h, gin 9h, adauga 5h la 1, gata, arata, urmatoarea.
- Exemple legale la început: `talon`, `pas`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="crazy_eights"></a>
## Crazy Eights (Optarii)

- **Id:** `crazy_eights`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play crazy_eights` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 7 fiecare.
- Joci o carte de aceeași culoare sau valoare cu cea de deasupra; optarul (8) se joacă oricând și alege culoarea (8h:s).
- Dacă nu poți, tragi (trage) și poți juca ori pasa.
- Cine rămâne fără cărți ia punctele din mâna adversarului (8 = 50, figuri = 10, A = 1); până la 100.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7h, 8h:s, trage, pas, urmatoarea.
- Exemple legale la început: `2h`, `8c:h`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pitch"></a>
## Pitch (Setback)

- **Id:** `pitch`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pitch` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 6 fiecare, 2 jucători.
- Licitezi 2–4 (licit N) sau pas; licitatorul conduce, iar culoarea primei cărți devine atu.
- Răspunzi la culoare sau tai cu atu.
- Patru puncte: High, Low, Jack, Game; licitatorul care nu atinge contractul pierde cât a licitat.
- Primul la 11.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pas, licit 3, 9h, urmatoarea.
- Exemple legale la început: `pas`, `licit`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bid_whist"></a>
## Bid Whist (2 jucători)

- **Id:** `bid_whist`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play bid_whist` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Adaptare pentru doi jucători: 54 de cărți (cu doi jokeri), 12 fiecare și un kitty de 6.
- Licitezi 3–6 (licit N) sau pas; câștigătorul ia kitty-ul și aruncă 6 cărți, apoi alege Uptown (A mare), Downtown (A mic) sau Fără atu.
- Punctele: levate peste 6 (sau contractul), dublu la fără atu; primul la ±7.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pas, licit 4, dis 9h, sus h, jos h, fara sus, fara jos, 9h, reimparte, urmatoarea.
- Exemple legale la început: `pas`, `licit`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="all_fours"></a>
## All Fours

- **Id:** `all_fours`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play all_fours` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 52 de cărți, 6 fiecare, o carte de atu întoarsă (valet = 3 puncte dealerului).
- Non-dealerul cere (cer) sau joacă (joc); dealerul dă un punct (dau) sau schimbă (schimb, apoi imparte, aruncări dis X).
- Patru puncte: High, Low, Jack, Game; primul la 14.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cer, joc, dau, schimb, imparte, dis 9h, 9h, urmatoarea.
- Exemple legale la început: `cer`, `joc`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="conquian"></a>
## Conquián

- **Id:** `conquian`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play conquian` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet spaniol de 40 de cărți (A–7, S, C, R; patru culori), 2 jucători, 10 cărți.
- Tragi din stoc (stoc) sau iei cartea de sus a talonului (ia ...) doar ca s-o legi imediat într-o combinație (set de 3–4 sau serie de cel puțin 3 din aceeași culoare); apoi cobori combinații (pune ...), lipești cărți (adauga X la N) și arunci (descarta X).
- Cine rămâne fără cărți în mână câștigă mâna și un punct; primul la 3.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: stoc, ia ..., pune ..., adauga 5o la 1, descarta 5o, pas, urmatoarea.
- Exemple legale la început: `stoc`, `ia`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chicken_foot"></a>
## Chicken Foot (domino)

- **Id:** `chicken_foot`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play chicken_foot` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Set dublu-12, 15 piese fiecare, 13 runde (de la dublu-12 la dublu-0).
- Se pornește de la dublul de rundă; un dublu trebuie acoperit (pui piesa pe el) înainte de a continua.
- Runda se termină când cineva rămâne fără piese sau la blocaj; fiecare adaugă la scor punctele pieselor rămase în mână.
- După 13 runde câștigă cel mai mic total.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 5-7@2, trage, pas, urmatoarea.
- Exemple legale la început: `3-12@1`, `3-12@2`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bunco"></a>
## Cele șase ținte (zaruri)

- **Id:** `bunco`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play bunco` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de zaruri după Bunco (nume comercial): 6 runde, în runda N ținta e N.
- Arunci 3 zaruri cât timp ai puncte: fiecare zar egal cu ținta = 1 punct, trei egale cu ținta = 21 (bunco, încheie runda), trei la fel altfel = 5.
- Runda se încheie la 21 de puncte; după 6 runde câștigă cine a luat mai multe runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="farkle"></a>
## Zece mii de puncte (zaruri)

- **Id:** `farkle`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play farkle` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de zaruri după Farkle / 10 000 (nume comercial): arunci 6 zaruri, păstrezi combinații care punctează (1 = 100, 5 = 50, trei la fel, serii, perechi), apoi arunci din nou sau bancrezi.
- Fără combinație pierzi tot ce ai strâns în tură.
- Cu 0 puncte în total banca doar de la 500 în tură; țintă 10 000 cu rundă finală.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, pastrez 1 5, banca, farkle.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="liars_dice"></a>
## Zarurile mincinosului

- **Id:** `liars_dice`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play liars_dice` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă Perudo/Dudo/Cacho: fiecare are 5 zaruri ascunse sub pahar.
- Licitezi cantitatea unei fețe pentru toate zarurile de pe masă (aștii sunt jokeri), mereu mai sus, sau spui dudo; cine greșește pierde un zar.
- Ultimul cu zaruri câștigă.
- Fără calza și fără palifico.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3x4, dudo, arata.
- Exemple legale la început: `1x2`, `1x3`
- La început sunt 50 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="crokinole"></a>
## Crokinole

- **Id:** `crokinole`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play crokinole` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de dexteritate pe tablă rotundă, modelat ca simulare deterministă cu zgomot din seed: alegi poziția, unghiul și puterea tragerii (x-1 u6 p3), apoi lasi discul.
- Trebuie să lovești un disc adversar dacă există. 12 discuri pe jucător; inelele valorează 5, 10 și 15, gaura centrală 20.
- Câștigătorul rundei ia diferența de puncte; primul la 100 (cel mult 6 runde).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: x-1 u6 p3, lasa.
- Exemple legale la început: `x0`, `u0`
- La început sunt 990 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chinchon"></a>
## Chinchón (Spania)

- **Id:** `chinchon`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play chinchon` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Baraja spaniolă de 40 de cărți (o c e b; 1–7, 10, 11, 12), 7 cărți fiecare.
- Tragi din stoc sau talon și arunci; combinații: seturi și secvențe (7 nu se leagă de 10).
- Închizi cu cel puțin 6 cărți potrivite și cel mult 5 puncte libere (−10 dacă toate 7 se potrivesc; chinchón = 1–7 din aceeași culoare câștigă jocul); cine ajunge la 100 pierde.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: stoc, talon, 5o, inchide-5o, urmatoarea.
- Exemple legale la început: `stoc`, `talon`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="burako"></a>
## Burako (Buraco)

- **Id:** `burako`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play burako` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 106 cărți (2 pachete + 2 jokeri), 11 în mână și un muerto de 11.
- Tragi din stoc (mazo) sau iei tot pozo, lași jos scări (≥3 consecutive, aceeași culoare) și piernas (≥3 de același rang), cel mult un comodin (joker sau 2) per combinație, apoi arunci.
- Canastă = 7+ cărți: pură 200, impură 100; închizi cu muertoul luat și o canastă (+100).
- Meci până la 1500 sau 4 mâini.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: mazo, pozo, esc:4h+5h+6h, set:7h+7c+Jk, +1:8h, descarta 5h, muerto, urmatoarea.
- Exemple legale la început: `mazo`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="escoba"></a>
## Escoba (Escoba de 15)

- **Id:** `escoba`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play escoba` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Baraja spaniolă de 40 de cărți (valori 1–7, 10=8, 11=9, 12=10), 3 în mână și 4 pe masă.
- Joci o carte și poți lua cărți de pe masă care, adunate cu ea, dau 15; golești masa = escoba (1 punct, nu la ultima carte).
- Punctaj la capăt de mână: cartas, oros, siete de velo (7o), setenta și escobe; primul la 21.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 7o, 7o:4c+4e, imparte, urmatoarea.
- Exemple legale la început: `5o`, `7o`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cuarenta"></a>
## Cuarenta (Ecuador)

- **Id:** `cuarenta`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play cuarenta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Baraja spaniolă de 40 de cărți, câte 5 în mână, masa începe goală.
- Joci o carte și poți lua: o carte de același rang (potrivire) sau cărți 1–7 cu aceeași sumă (adunare), plus secvența ascendentă care urmează.
- Caída 2, limpia 2, ronda 4 puncte; la capăt de mână puncte după numărul de cărți luate; primul la 40.
- Informație ascunsă: vezi doar mâna Jucătorului 1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 5o, 5o:5c, 6o:2c+4e, imparte, urmatoarea.
- Exemple legale la început: `3b`, `10b`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="generala"></a>
## Generala (zaruri)

- **Id:** `generala`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play generala` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 5 zaruri, până la 3 aruncări pe tură, 10 categorii: unu…sase (suma zarurilor), escalera 20, full 30, poker 40, generala 50.
- Servida (din prima aruncare) +5 la escalera/full/poker; generala servida câștigă jocul pe loc.
- Categoriile nepotrivite se taie (0). 10 ture pe jucător, câștigă totalul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, tine 3 3 5, tine -, scor escalera.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="parques"></a>
## Parqués (Colombia)

- **Id:** `parques`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play parques` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pistă de 68 de pătrate, 4 piese pe jucător, două zaruri.
- Din cárcel ieși doar cu pereche (toate piesele deodată); pereche = încă o aruncare, trei perechi = o piesă direct în corona.
- Captura (obligatorie) trimite piesa adversă în cárcel, pe pătratele sigure nu se capturează; corona se atinge exact.
- Primul cu 4 piese în corona câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 12:5 (piesa de la progresul 12 avansează 5), sale, pasa.
- Exemple legale la început: `sale`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fivehundred"></a>
## Five Hundred (500, doi jucători)

- **Id:** `fivehundred`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play fivehundred` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 500 pentru 2 jucători: 43 de cărți (cu joker), 10 în mână, 5 grămezi de câte 2 (jos + sus) și un kitty de 3.
- Licitezi 6–10 levate cu atu sau fără atu (fără misere); contractantul ia kitty-ul și aruncă 3 cărți.
- Levate de 4 cărți (câte una din mână și una cu fața în sus); jokerul și valeții (bower) sunt atuurile cele mai mari.
- Contract reușit = valoarea lui, eșuat = −valoarea; adversarul ia 10 pe levată; 500 / −500.
- Informație ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: licit 7h, licit 8nt, pas, descarta As, 10h, Jk:h, urmatoarea.
- Exemple legale la început: `pas`, `licit`
- La început sunt 51 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pontoon"></a>
## Pontoon

- **Id:** `pontoon`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pontoon` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Banca (rolul alternează la fiecare mână) contra jucătorului: Pontoon (as + carte de 10) > Five Card Trick (5 cărți ≤ 21) > 21 > cât mai aproape de 21; egalitatea o câștigă banca.
- Jucătorul: carte (twist), cumpar (carte ascunsă, miza +10, doar înainte de carte), stai (de la 15); banca stă de la 17.
- Pontoon și Five Card Trick plătesc dublu. 20 de mâini cu miza 10, câștigă cine are mai multe jetoane.
- Informație ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: carte, cumpar, stai, decontare.
- Exemple legale la început: `carte`, `cumpar`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="twoup"></a>
## Two-up (Australia)

- **Id:** `twoup`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play twoup` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Spinnerul mizează pe stemă (5, 10 sau 20 de jetoane); celălalt acoperă.
- Două monede: cap-cap = spinnerul câștigă și continuă; pajura-pajura = pierde și rolul trece; cap-pajura (odds) = se aruncă din nou, cinci odds la rând = spinnerul pierde. 24 de mize, start 50 de jetoane; câștigă cine are mai multe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: miza 5, miza 10, miza 20, arunc.
- Exemple legale la început: `miza`, `5`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ancora"></a>
## Ancora și coroana

- **Id:** `ancora`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ancora` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Trei zaruri cu simboluri (coroana, ancora, inima, romb, trefla, pica) — joc britanic tradițional «Crown and Anchor».
- Mizezi 5, 10 sau 20 pe unul sau mai multe simboluri; pentru fiecare zar cu simbolul tău primești miza înapoi plus câștig (1:1, 2:1, 3:1), altfel pierzi miza. 12 runde împotriva băncii, câștigă cine are mai multe jetoane.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pun ancora 10, gata, arunc.
- Exemple legale la început: `pun`, `coroana`
- La început sunt 55 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mutorere"></a>
## Mu Torere (Noua Zeelandă)

- **Id:** `mutorere`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play mutorere` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 8 puncte pe cerc (kewai 1–8) și centrul (0); fiecare are 4 piese pe puncte vecine, centrul gol.
- Muți o piesă pe un punct vecin gol; în centru intri doar cu o piesă vecină cu o piesă adversă.
- Pierde cine nu mai poate muta; remiză la a treia repetare sau după 200 de mutări.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3-4 (de pe 3 pe 4), 4-0 (în centru), 0-6 (din centru).
- Exemple legale la început: `1-0`, `4-0`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tictactoe"></a>
## X și 0 (Tic-tac-toe)

- **Id:** `tictactoe`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tictactoe` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 3×3; jucătorii pun pe rând X și 0.
- Trei la rând (linie, coloană sau diagonală) câștigă; tabla plină fără linie e remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1…c3.
- Exemple legale la început: `a1`, `b1`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ur"></a>
## Jocul regal din Ur

- **Id:** `ur`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ur` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cursă mesopotamiană, 7 piese pe jucător pe un drum de 14 pătrate; patru zaruri binare (0–4), ieșire cu număr exact, pe pătratele comune capturezi, rozeta centrală e sigură, rozetele dau o nouă aruncare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, 0-3 (de-la), pas.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="crazyhouse"></a>
## Crazyhouse (șah cu reintroducere)

- **Id:** `crazyhouse`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play crazyhouse` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Șah în care piesele capturate trec în rezerva ta și pot fi introduse pe orice câmp liber în loc de mutare (pionii nu pe prima/ultima linie); o piesă promovată capturată revine pion.
- Reguli complete de șah; remiză prin repetiție triplă sau după 200 de mutări.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e2e4, e7e8q, N@f3.
- Exemple legale la început: `a2a3`, `a2a4`
- La început sunt 20 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="omweso"></a>
## Omweso

- **Id:** `omweso`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play omweso` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mancala din Uganda: 4×8 gropi, două rânduri fiecare, câte 2 semințe (64).
- Alegi o groapă cu cel puțin 2 semințe și semeni în sens trigonometric prin cele 16 gropi ale tale; dacă ultima sămânță cade într-o groapă nevidă din rândul interior, cu ambele gropi adverse din coloană nevide, le capturezi; altfel continui să semeni (releu).
- Pierzi când nu mai ai nicio groapă cu 2 semințe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: i1…i8 (rândul interior), o1…o8 (exterior).
- Exemple legale la început: `i1`, `i2`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="twixt"></a>
## TwixT

- **Id:** `twixt`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play twixt` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de conectare pe tabla 24×24: J1 leagă marginea de sus de cea de jos, J2 pe cea din stânga de cea din dreapta; nu poți pune cuie pe marginile adversarului sau în colțuri.
- Cuiele aflate la mutare de cal se leagă automat dacă puntea nu taie alta.
- Câștigă cine închide primul lanțul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e12 (coloana a–x, rândul 1–24; rândul 24 e sus).
- Exemple legale la început: `b1`, `c1`
- La început sunt 528 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="halma"></a>
## Halma

- **Id:** `halma`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play halma` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Halma pe 16×16 pentru doi: 19 piese fiecare în colțuri opuse.
- Muți un pas (și pe diagonală) sau un lanț de sărituri peste piese vecine; nu se captează nimic.
- Câștigă primul care aduce toate cele 19 piese în tabăra adversă; după 150 de mutări pe jucător, cine are mai multe acolo.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b2d4 (de la, la).
- Exemple legale la început: `c1e3`, `d1f3`
- La început sunt 40 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mahjong_hk"></a>
## Mahjong Hong Kong (minimum 3 fan)

- **Id:** `mahjong_hk`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mahjong_hk` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mahjong în stil Hong Kong pentru 2 jucători, 136 de piese fără flori; o mână câștigă doar cu cel puțin 3 fan (tragere proprie 1, închisă 1, toate secvențele 1, toate pungurile 3, o culoare + onoruri 3, culoare pură 6, doar onoruri 10, pung de dragon/vânt propriu/vântul rundei câte 1).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, d5m, mahjong, kong5m, trage-moarta, pung, kong, chi2m3m, pas.
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fia"></a>
## Fia-med-knuff

- **Id:** `fia`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play fia` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Ludo suedez: 4 piese fiecare, pistă de 48 de câmpuri. 1 sau 6 intră o piesă; 6 dă încă o aruncare; aterizând pe o piesă adversă o trimiți acasă.
- Finalul cere aruncare exactă; câștigă primul cu toate cele patru acasă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: intra sau numărul progresului piesei (ex. 12).
- Exemple legale la început: `intra`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tiengow"></a>
## Tien Gow (Cer și nouă)

- **Id:** `tiengow`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tiengow` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc cantonez de levate cu 32 de piese de domino, adaptat pentru doi: 8 piese fiecare.
- Conduci cu o piesă sau o pereche; celălalt bate cu o piesă mai mare de aceeași culoare (civil/militar) sau o pereche mai mare de același tip, ori aruncă cu fața în jos.
- Ultima levată câștigă mâna; primul la 10 puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 66, 66+66, d64 (d = aruncare ascunsă).
- Exemple legale la început: `66`, `11`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="aceydeucey"></a>
## Acey-deucey

- **Id:** `aceydeucey`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play aceydeucey` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă americană de table: toate cele 15 piese pornesc din afara tablei și intră în casa adversarului; zarul 1+2 (acey-deucey) se joacă, apoi alegi orice dublă și mai arunci; dublele dau patru mutări.
- Scoaterea e exactă când toate sunt acasă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ent-3, 5-9, 20-off, dublet 4.
- Exemple legale la început: `ent-3`, `ent-5`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="barbilliards"></a>
## Bar billiards

- **Id:** `barbilliards`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play barbilliards` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Masa de crâșmă cu găuri și popici, simulată pe lovituri, pentru doi: găurile valorează 10, 20, 30, 50, 100, 200; bila roșie dublează punctele.
- O vizită continuă cât timp loviturile punctează; ratarea o încheie (punctele rămân); un fault (popică doborâtă) o încheie și anulează punctele vizitei; popica neagră anulează tot scorul. 8 vizite fiecare; lovitura se hotărăște la pasul forțat lovitura.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 10 alba, 100 rosie, lovitura, stop.
- Exemple legale la început: `10`, `alba`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tabula"></a>
## Tabula

- **Id:** `tabula`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tabula` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Strămoșul roman al tablelor: 15 piese fiecare pornesc din afara tablei, 3 zaruri, iar ambii jucători merg în același sens pe cele 24 de puncte.
- Intri, muți, apoi scoți exact; câștigă cine scoate primul toate piesele.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ent-3, 5-9, 20-off.
- Exemple legale la început: `ent-3`, `ent-5`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="petitschevaux"></a>
## Jeu des petits chevaux

- **Id:** `petitschevaux`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play petitschevaux` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Ludo francez cu cai: 4 cai fiecare, pistă de 56 de câmpuri și 6 câmpuri de final.
- Un 6 scoate un cal din grajd și dă încă o aruncare; câmpul tău de start e sigur; aterizând pe un cal adversar îl trimiți înapoi; sosirea cere aruncare exactă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: intra sau numărul progresului calului (ex. 12).
- Exemple legale la început: `intra`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="coganh"></a>
## Cờ gánh

- **Id:** `coganh`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play coganh` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc vietnamez de capturi pe tabla 5×5 cu diagonale; 8 piese fiecare.
- Muți un pas pe o linie către un punct liber.
- Capturile schimbă culoarea pieselor: gánh (piesa mutată încadrează o piesă adversă pe o linie) și vây (un grup advers fără niciun punct liber în jur).
- Câștigi lăsând adversarul fără piese sau mutări.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1b2 (de la, la).
- Exemple legale la început: `a1b2`, `b1b2`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="petteia"></a>
## Petteia

- **Id:** `petteia`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play petteia` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Reconstituire a jocului grecesc antic pe 8×8: 16 piese fiecare, orice piesă merge ca un turn; o piesă e capturată când mutarea ta o prinde între două ale tale pe un rând sau o coloană (mai multe capturi într-o mutare).
- Pierzi cu o singură piesă sau fără mutări; după 60 de mutări fără captură câștigă cine are mai multe piese.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a2a4 (de la, la).
- Exemple legale la început: `a2a3`, `a2a4`
- La început sunt 32 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="morra"></a>
## Morra (Mora)

- **Id:** `morra`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play morra` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc italian cu degete: ambii arată pe ascuns 1–5 degete și strigă suma presupusă; dacă exact unul ghicește, ia un punct, altfel runda se reia.
- Alegerea e simultană (ascunsă până la dezvăluire); primul la 7.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3/7 (3 degete, suma presupusă 7), dezvaluie.
- Exemple legale la început: `1/2`, `1/3`
- La început sunt 25 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sanma"></a>
## Sanma (mahjong în trei, adaptat la doi)

- **Id:** `sanma`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play sanma` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mahjong riichi cu schimbările sanma: 108 piese (fără 2m–8m), fără chi, piesele Nord pot fi declarate kita (dora bonus, tragere de înlocuire), indicatorul de dora face cerc 1m↔9m.
- Riichi, ron, tsumo, furiten și punctaj ca la riichi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, 5m, riichi 5p, tsumo, ankan, kakan, kita, trage-moarta, ron, pon, kan, pas, urmatoarea.
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="flipcup"></a>
## Flip Cup

- **Id:** `flipcup`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play flipcup` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Duel de relee de petrecere, fără băutură: două echipe de câte 5 trebuie să termine fiecare 5 răsturnări de pahar.
- Alegi repede (55% reușită, costă 2 timp) sau atent (80%, costă 3); la ratare se repetă, timpul se pierde.
- Joacă echipa cu ceasul mai mic; câștigă echipa care termină cele 5 răsturnări cu cel mai mic timp.
- Rezultatul se hotărăște la pasul forțat rastoarna.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: repede, atent, rastoarna.
- Exemple legale la început: `repede`, `atent`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="madgab"></a>
## Mad Gab

- **Id:** `madgab`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play madgab` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Puzzle fonetic cu cuvinte, în română, pentru doi: un șir de silabe fără sens sună, citit repede cu voce tare, ca o expresie obișnuită.
- Alegi expresia corectă dintre 4 (ghici 1…ghici 4), eventual după ajutor (se vede primul cuvânt, puncte mai puține).
- Corect = 2 puncte (1 cu ajutor); greșit = celălalt are o singură încercare pentru 1 punct. 8 runde; verific la final.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ghici N, ajutor, verific.
- Exemple legale la început: `ghici`, `1`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pubquiz"></a>
## Pub quiz

- **Id:** `pubquiz`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pubquiz` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Seară de întrebări pentru două echipe: 4 runde de câte 5 întrebări cu 4 variante, o altă categorie în fiecare rundă; un răspuns corect = 1 punct.
- Înaintea fiecărei runde o echipă poate juca jokerul (o dată pe meci; dublează punctele rundei) sau refuza.
- Răspunsurile rămân secrete până la verific; scor egal = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: joker, fara joker, raspund a…raspund d, verific.
- Exemple legale la început: `joker`, `fara`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="oldmaid"></a>
## Old Maid (Bătrâna fată)

- **Id:** `oldmaid`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play oldmaid` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Bătrâna fată pentru doi: pachet de 52 de cărți din care lipsește o damă, deci o damă rămâne fără pereche.
- Se aruncă perechile de aceeași valoare; apoi tragi pe nevăzute din mâna celuilalt (trag N = a N-a carte) și arunci perechile.
- Când rămâne o carte, cine o ține (dama) pierde.
- Mâini ascunse; rezultatul apare la pasul final.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trag N, final.
- Exemple legale la început: `trag`, `1`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mus"></a>
## Mus

- **Id:** `mus`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mus` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc spaniol cu 40 de cărți, adaptat pentru doi: 4 cărți fiecare; 3 valorează ca regii, 2 ca asul.
- Mus opțional (arunci și tragi), apoi patru licitații: grande, chica, pares, juego; paso sau envido (miza 2), celălalt răspunde quiero / no quiero.
- Primul la 20.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: mus, corto, descarta 0101, reparte, declara, paso, envido, quiero, no quiero, dezvaluie.
- Exemple legale la început: `mus`, `corto`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bezique"></a>
## Bézique

- **Id:** `bezique`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play bezique` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc francez pentru doi cu levate și figuri, pachet de 64 de cărți (7–A dublu), 8 cărți fiecare, atu întors.
- Cât timp mai e pachet, câștigătorul levatei poate declara o figură (căsătorie 20/40, secvență 250, careu 40–100, bézique 40, dublu bézique 500; dix = schimbi 7 de atu pentru 10), apoi ambii trag; după aceea răspunzi la culoare și bați.
- Brisques (ași și zeci) și ultima levată dau 10.
- Primul la 1000 (cel mult 4 mâini).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: Qs, mariaj s, secventa, patru K, bezic, dublubezic, dix, fara, trag.
- Exemple legale la început: `Qh`, `Ah`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="twentynine"></a>
## Twenty-Nine (29)

- **Id:** `twentynine`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play twentynine` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc sud-asiatic de levate cu licitație, adaptat pentru doi: 32 de cărți, 8 fiecare; valet 3, nouă 2, as 1, zece 1 puncte.
- Licitezi 10–20 de puncte, câștigătorul alege atuul și celălalt conduce; răspunzi la culoare dacă poți.
- Contractul făcut = 1 punct de meci; primul la 5.
- Mâini ascunse.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: licitez 12, pas, atu s, Jh.
- Exemple legale la început: `licitez`, `10`
- La început sunt 22 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tute"></a>
## Tute

- **Id:** `tute`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tute` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc spaniol de levate cu 40 de cărți pentru doi (o mână): 6 cărți fiecare, atu întors.
- Cât timp mai e pachet joci orice carte; apoi trebuie să răspunzi la culoare și să bați dacă poți.
- Câștigătorul poate anunța canto (rege+cal: 20, la atu 40), cambio (schimbi 7-ul de atu) sau tute (patru regi sau patru cai: câștigi pe loc).
- Ultima levată 10.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3o, 12b, canto e, cambio, tute, trag.
- Exemple legale la început: `2c`, `10c`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sotron"></a>
## Șotron (Hopscotch)

- **Id:** `sotron`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sotron` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de copii pentru doi: cursă pe pătratele 1–10.
- La tura ta arunci piatra: prudent (țintești pătratul următor: 80% reușită, +1) sau indraznet (peste un pătrat: 50%, +2; la ratare cazi un pătrat înapoi).
- Câștigă primul care duce piatra la 10 și se întoarce.
- Aruncarea se hotărăște la pasul forțat sar.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: prudent, indraznet, sar.
- Exemple legale la început: `prudent`, `indraznet`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="duckduckgoose"></a>
## Duck Duck Goose

- **Id:** `duckduckgoose`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play duckduckgoose` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de cerc pentru copii, ca duel de risc pentru doi: în fiecare rundă un jucător atinge, celălalt e gâsca.
- Cel care atinge alege câte rațe spune (1–5): cu cât mai multe, cu atât are șanse mai mici să fie prins și mai multe puncte dacă scapă.
- Cursa se hotărăște la pasul forțat fuga; 10 runde, câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: rate 1…rate 5, fuga.
- Exemple legale la început: `rate`, `1`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="statuile"></a>
## Statuile (Statues)

- **Id:** `statuile`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play statuile` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de înghețat pentru copii, pentru doi: fiecare alergător trebuie să ajungă la linia 12; la rundele impare aleargă J1, la cele pare J2.
- Alergătorul alege în ascuns 1–3 pași, cel care strigă alege în ascuns când strigă stop (1–3); dacă face mai mulți pași decât a strigat, e prins în mișcare și se întoarce la start.
- Dezvăluirea e pasul forțat dezvaluie; primul la 12 câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1, 2, 3, dezvaluie.
- Exemple legale la început: `1`, `2`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="conkers"></a>
## Conkers

- **Id:** `conkers`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play conkers` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc britanic de curte pentru doi: fiecare castană are 6 puncte de rezistență.
- La tura ta alegi usor (70% reușită, 1 pagubă) sau tare (45% reușită, 2 pagube; la ratare sfoara se poate încurca și pierzi 1).
- Prima castană ajunsă la 0 se sparge și pierde; după 60 de lovituri câștigă castana mai întreagă.
- Rezultatul se hotărăște la pasul forțat lovitura.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: usor, tare, lovitura.
- Exemple legale la început: `usor`, `tare`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="craps"></a>
## Craps

- **Id:** `craps`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play craps` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Duel la masa de zaruri pentru doi: shooterul alege miza (5, 10, 20) și pariul, pass sau dont (don't pass); celălalt acoperă pariul opus.
- Come-out: 7 sau 11 câștigă pass; 2, 3, 12 = craps (don't pass câștigă la 2 și 3, la 12 miza se anulează); orice alt total devine punct, se aruncă până apare punctul (pass) sau 7 (don't pass). 16 runde, start 100 de jetoane, câștigă cine are mai multe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pass 10, dont 5, arunc.
- Exemple legale la început: `pass`, `5`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mexico"></a>
## Mexico (Mäxchen, Mia)

- **Id:** `mexico`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mexico` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de bluf cu două zaruri și cupă: arunci ascuns și anunți o valoare strict mai mare decât ultimul anunț (31 cea mai mică … dublele … 21 «Mexico» cea mai mare).
- Următorul crede (și aruncă) sau contestă: mincinosul pierde o viață, contestatarul greșit tot una (la Mexico două).
- Câte 5 vieți.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, anunt 65, cred, contest, dezvaluie.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="schocken"></a>
## Schocken

- **Id:** `schocken`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play schocken` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc german cu trei zaruri și 13 jetoane: până la trei aruncări pe rundă (păstrezi zarurile alese); clasament Schock Out (1-1-1) > Schock (1-1-x) > General > Straße > obișnuită (cel mai mare zar întâi).
- Cine pierde runda ia jetoane după valoarea mâinii câștigătoare; o repriză se termină când cineva ține toate cele 13 și o pierde; două reprize pierdute pierd meciul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, stop, re 13 (re-arunci zarurile 1 și 3), decontare.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="passedix"></a>
## Passe-dix

- **Id:** `passedix`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play passedix` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc francez cu trei zaruri pentru doi: banca (alternativ) alege miza (5, 10, 20) și partea, passe (total 11–18) sau manque (3–10); celălalt acoperă partea opusă; tripletele se re-aruncă. 20 de runde, start 100 de jetoane, câștigă cine are mai multe.
- Aruncarea e pasul forțat arunc.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: passe 10, manque 5, arunc.
- Exemple legale la început: `passe`, `5`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="dicebaseball"></a>
## Dice Baseball

- **Id:** `dicebaseball`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play dicebaseball` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Baseball cu două zaruri pentru doi, fără decizii: 9 reprize, J1 bate apoi J2 până la trei outuri; reprize suplimentare (cel mult 12) la egalitate.
- Totalul fiecărei aruncări e un rezultat: 2 aruncare ratată, 3 triplu, 4 dublu, 5 walk, 6 fly out, 7 ground out, 8 simplu, 9 strike out, 10 minge afară (se reia), 11 dublă eliminare, 12 home run.
- Alergătorii avansează la lovituri; trecerea de baza a treia înscrie.
- Mutare: arunc.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="dropdead"></a>
## Drop Dead

- **Id:** `dropdead`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play dropdead` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc cu cinci zaruri și risc pentru doi, fără decizii: la tura ta arunci toate zarurile rămase; dacă nu apare niciun 2 sau 5, adaugi totalul la scor; altfel nu punctezi, iar zarurile cu 2 și 5 «cad».
- Tura se termină când au căzut toate cele cinci zaruri. 5 ture fiecare, câștigă scorul mai mare.
- Mutare: arunc.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="dame10"></a>
## Dame internaționale (10×10)

- **Id:** `dame10`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play dame10` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 10×10, 20 de piese pe jucător.
- Piesele simple mută un pas în diagonală înainte și capturează înainte și înapoi; damele zboară pe orice distanță.
- Captura e obligatorie, cu cele mai multe piese; piesele luate se scot la final.
- O piesă devine damă doar dacă își încheie mutarea pe ultimul rând.
- Remiză după 50 de plie doar cu dame sau 400 în total.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c4-d5, c4xe6xg8.
- Exemple legale la început: `b4-c5`, `b4-a5`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="toguz"></a>
## Toguz Kumalak (Kazahstan)

- **Id:** `toguz`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play toguz` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 × 9 gropi cu câte 9 bile și un depozit (kazan) pentru fiecare.
- Iei toate bilele dintr-o groapă proprie: prima rămâne, restul se seamănă câte una în sens trigonometric (o singură bilă trece mai departe).
- Dacă ultima bilă face o groapă a adversarului pară, o iei; dacă o face 3 (nu groapa 9, una per jucător, nu cu același număr ca a adversarului), groapa devine tuzdyk: orice bilă semănată acolo ajunge în depozitul tău.
- Primul la 82 câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–9.
- Exemple legale la început: `1`, `2`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sittuyin"></a>
## Sittuyin (șahul birman)

- **Id:** `sittuyin`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sittuyin` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 8×8; cei 8 pioni sunt deja pe a3 b3 c3 d4 e4 f3 g3 h3 / a6 b6 c6 d5 e5 f6 g6 h6.
- Întâi fiecare își așază regele, generalul, 2 elefanți, 2 cai și 2 bărci în jumătatea proprie (K@e1).
- Apoi se mută: general = un pas în diagonală, elefant = un pas în diagonală sau înainte, cal ca în șah, barcă = turn; un pion aflat pe o diagonală mare în jumătatea adversă poate deveni general (d5=G) dacă nu mai ai.
- Mat câștigă; patul e interzis.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: K@e1, e3-e4, e4xd5, d5=G.
- Exemple legale la început: `K@a1`, `K@b1`
- La început sunt 120 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="gebeta"></a>
## Gebeta (Etiopia)

- **Id:** `gebeta`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play gebeta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 2 × 6 gropi cu câte 4 semințe, câte un depozit.
- Iei toate semințele dintr-o groapă proprie și le semeni în sens trigonometric.
- Dacă ultima cade într-o groapă ocupată, ridici tot ce e acolo și continui (lanț).
- Dacă cade într-o groapă goală a ta și cea opusă are semințe, iei semințele opuse plus a ta.
- Când o parte rămâne goală jocul se termină și fiecare își ia semințele de pe partea lui; câștigă cine are mai multe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–6.
- Exemple legale la început: `1`, `2`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sprouts"></a>
## Sprouts (Conway–Paterson)

- **Id:** `sprouts`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sprouts` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc topologic, 4 puncte la start.
- La rândul tău unești două puncte (sau un punct cu el însuși, printr-o buclă) cu o curbă care nu taie nimic, apoi pui un punct nou pe curbă.
- Din fiecare punct pleacă cel mult 3 curbe.
- Cine nu mai poate muta pierde.
- Poziția e dată combinatoriu (regiuni și frontiere); mutări: a-b (puncte), a-a pentru buclă, cu .1/.2 când există mai multe variante.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `1-1.1`, `1-1.2`
- La început sunt 22 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pegsol"></a>
## Peg solitaire (duel)

- **Id:** `pegsol`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pegsol` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tabla engleză cu 33 de găuri, 32 de bile, centrul liber.
- O mutare: o bilă sare ortogonal peste o vecină într-o gaură liberă și bila sărită se scoate.
- Duel: fiecare joacă propria tablă identică, pe rând; cine rămâne fără mutări iese din joc.
- La final câștigă cine are mai puține bile (egal: ultima bilă în centru).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d2-d4.
- Exemple legale la început: `d2-d4`, `b4-d4`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="shuffleboard"></a>
## Shuffleboard de masă

- **Id:** `shuffleboard`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play shuffleboard` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Alunecă pucuri pe un traseu de 96 de unități: zonele 60 (1 p), 70 (2), 80 (3), 90–95 (4, agățat); de la 96 cad.
- Câte 4 pucuri pe rundă, pe rând.
- Alegi forța (tir 60), apoi «lansez»: pucul ajunge la ±5 de țintă; pucurile lovite transmit mișcarea.
- Scorează doar jucătorul cu pucul cel mai departe, cu pucurile lui aflate dincolo de cel mai bun al adversarului.
- Primul la 15.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: tir 30…100, lansez.
- Exemple legale la început: `tir`, `30`
- La început sunt 30 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="yatzy"></a>
## Yatzy (Scandinavia)

- **Id:** `yatzy`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play yatzy` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 5 zaruri, 3 aruncări pe tură, 15 categorii: unu…sase (bonus 50 dacă suma ≥ 63), pereche, două perechi, trei/patru la fel, scara mică 1-5 (15), scara mare 2-6 (20), casă, șansă, yatzy (50).
- Aruncarea e mutare separată; ții zaruri (tine 3 3 5) sau scrii într-o categorie.
- Câștigă totalul cel mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, tine 3 3 5, scor pereche.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ceelo"></a>
## Cee-lo (zaruri de stradă)

- **Id:** `ceelo`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ceelo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Trei zaruri. 4-5-6 sau trei egale câștigă pe loc; 1-2-3 pierde pe loc; o pereche plus un zar diferit dă acel zar ca punct; orice altceva se rearuncă.
- Pariorul alege miza (5/10/20); bancherul aruncă până iese un rezultat, apoi pariorul; punctul mai mare câștigă, egal = remiză.
- Banca trece când bancherul pierde. 16 mâini, 50 de jetoane.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pariez 10, arunc.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="oversevens"></a>
## Peste și sub șapte (Over and Under Seven)

- **Id:** `oversevens`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play oversevens` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pariorul alege «sub» (2–6), «peste» (8–12), plătite 1:1, sau «sapte» (exact 7), plătit 4:1, cu miza 5/10/20; banca (celălalt jucător) aruncă două zaruri («arunc»).
- Banca se schimbă la fiecare mână. 20 de mâini, 50 de jetoane; câștigă cine are mai multe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: sub 5, peste 10, sapte 5, arunc.
- Exemple legale la început: `sub`, `5`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="skunk"></a>
## Skunk (zaruri)

- **Id:** `skunk`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play skunk` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de zaruri «împinge-ți norocul», 5 runde.
- În tură arunci două zaruri cât vrei (arunc) sau te oprești (opresc).
- Fără 1: adaugi suma la tură.
- Un singur 1 (skunk): pierzi punctele turei.
- Doi de 1: pierzi tot totalul.
- Confirmi skunk-ul cu ok.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hazard"></a>
## Hazard (strămoșul craps-ului)

- **Id:** `hazard`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hazard` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Aruncătorul strigă un main 5–9 și o miză.
- Prima aruncare cu două zaruri: main = nick (câștigă); 2 sau 3 = pierde; 11 și 12 depind de main (5/9: ambele pierd; 6/8: 11 pierde, 12 nick; 7: 11 nick, 12 pierde); orice altceva devine chance.
- Apoi se aruncă până iese chance (câștigă) sau main (pierde).
- Zarurile trec după trei pierderi la rând. 100 de jetoane, 20 de mâini.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: main 7 10, arunc.
- Exemple legale la început: `main`, `5`
- La început sunt 45 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="howzat"></a>
## Crichet cu zaruri (Howzat!)

- **Id:** `howzat`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play howzat` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cricket cu zaruri pentru doi: 2 reprize de 30 de mingi și 5 wicket-uri;
- J1 bate primul, J2 urmărește.
- La fiecare minge bătătorul alege «lovesc» (atac) sau «apar» (apărare), apoi aruncătorul dă «arunc»: atac 1–2 out, 3=1, 4=2, 5=4, 6=6 runs; apărare 1 out, 2–3=0, 4–5=1, 6=2 runs.
- Câștigă cine are mai multe runs.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: lovesc, apar, arunc.
- Exemple legale la început: `lovesc`, `apar`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="dreidel"></a>
## Dreidel

- **Id:** `dreidel`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play dreidel` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de noroc cu titirezul.
- Fiecare începe cu 10 jetoane și pune 1 în pot (din nou când potul are 0–1 jetoane).
- Pe rând se învârte («invart»): nun = nimic, gimel = iei tot potul, hei = iei jumătate, shin = pui 1.
- Cine nu mai poate plăti pierde; după 300 de învârtiri câștigă cine are mai multe jetoane.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `invart`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="parimpar"></a>
## Pare sau impare

- **Id:** `parimpar`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play parimpar` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- J1 este «impar», J2 este «par».
- La fiecare rundă fiecare alege în secret 1–5 degete (n1…n5), apoi se arată (arat).
- Suma impară dă runda lui J1, cea pară lui J2.
- Primul la 5 runde câștigă.
- Alegerea ta rămâne ascunsă până la arătare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `n1`, `n2`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mehen"></a>
## Mehen (Egiptul antic)

- **Id:** `mehen`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play mehen` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Reconstrucție.
- Un traseu-șarpe de 18 pătrate (coada 0, capul 17), 3 piese fiecare.
- Se aruncă patru bețe (0–4 fețe luminoase, 0 = 6; aruncările 1, 4 și 6 dau încă o aruncare).
- O piesă intră pe coadă doar cu 1, merge spre cap și se întoarce pe același traseu (35 de pași), apoi iese acasă.
- Aterizezi pe adversar = îl trimiți afară; pe piesa ta nu poți ateriza.
- Câștigă primul cu toate 3 piesele acasă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, intra, m1/m2/m3, pas.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cinci_pietricele"></a>
## Cinci pietricele

- **Id:** `cinci_pietricele`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cinci_pietricele` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Jocul copiilor cu cinci pietricele, cu norocul în locul îndemânării. 5 etape (uni, doi, trei, patru, tot).
- În tură alegi «calm» (70 %, +1 etapă) sau «iute» (45 %, +2 etape), apoi «incerc».
- O reușită te lasă să continui; o greșeală dă rândul.
- Primul care termină etapa 5 câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `calm`, `iute`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="jocuri_ata"></a>
## Jocuri cu ață (leagănul pisicii)

- **Id:** `jocuri_ata`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play jocuri_ata` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Doi jucători își pasează figuri de ață în ordinea clasică: leagăn → patul soldatului → lumânări → ieslea → diamante → ochi de pisică → pește în farfurie → ceas → leagăn.
- Alegi «atent» (85 %, 1 punct) sau «rapid» (60 %, 2 puncte), apoi «preiau».
- Ața încurcată costă 1 punct și revine la leagăn; închiderea ciclului dă 3 puncte bonus.
- Primul la 12 câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `atent`, `rapid`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="skittles"></a>
## Table skittles (popice de masă)

- **Id:** `skittles`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play skittles` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Popice de masă cu norocul în locul îndemânării: 9 popice într-o grilă 3×3.
- La fiecare aruncare țintești o coloană (stanga/mijloc/dreapta), apoi «lansez»; popicele din coloana țintă cad cu 60 %, vecine 25 %, opuse 8 %.
- O tură are 3 aruncări; fiecare popică 1 punct (cea din mijloc 2); toate nouă = 5 bonus. 6 ture fiecare; câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `stanga`, `mijloc`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="doppelkopf"></a>
## Doppelkopf (2 jucători)

- **Id:** `doppelkopf`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play doppelkopf` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Adaptare pentru doi a jocului normal: 48 de cărți (9 10 J Q K A ×4 culori ×2), câte 24.
- Atuuri de la mare la mic: 10♥ (Dulle), Q♣ Q♠ Q♥ Q♦, J♣ J♠ J♥ J♦, A♦ 10♦ K♦ 9♦; culorile ♣ ♠ ♥ au ordinea A 10 K 9.
- Respecți culoarea sau atuul; din două cărți egale câștigă prima (a doua 10♥ o bate pe prima).
- Peste 120 din 240 de ochi câștigă mâna; primul la 2 mâini.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: Qc, 10h, Ad, imparte (c=♣ s=♠ h=♥ d=♦).
- Exemple legale la început: `Ah`, `Kh`
- La început sunt 19 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tarock"></a>
## Tarock (2 jucători)

- **Id:** `tarock`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tarock` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Adaptare pentru doi, fără licitație. 54 de cărți: 22 de tarocuri (t1 … t21 și Škis) + 4 culori × 8; câte 18, 18 rămân nefolosite.
- Respecți culoarea, altfel ești obligat să joci un tarok; tarokul bate orice carte de culoare, tarokul mai mare câștigă. Škis (skis) se joacă oricând, nu câștigă niciodată și rămâne al celui care l-a jucat.
- Puncte: K 5, Q 4, C 3, J 2, restul culorilor 1; tarocurile I, XXI, Škis 5, celelalte 1; ultima levată câștigată cu I dă +10.
- Primul la 2 mâini.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: t7, skis, Kh, 10s, 1d, imparte.
- Exemple legale la început: `Jd`, `1d`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sevens"></a>
## Șapte (Sevens)

- **Id:** `sevens`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play sevens` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Șapte în doi: câte 26 de cărți.
- Un 7 deschide culoarea la mijloc; apoi construiești în sus și în jos, cu o carte din aceeași culoare lipită de șir.
- Dacă nu poți juca, treci.
- Câștigă cine rămâne primul fără cărți.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cartea (ex. 7h) sau pas. Mâini ascunse.
- Exemple legale la început: `7h`, `7d`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="skitgubbe"></a>
## Skitgubbe

- **Id:** `skitgubbe`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play skitgubbe` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc suedez de descărcare în două faze.
- Faza 1: cu pachet, joci o carte, adversarul trebuie să o bată cu una mai mare din aceeași culoare sau ia grămada; completezi la trei cărți.
- Faza 2: se joacă mâinile rămase, fără pachet; cine rămâne ultimul cu cărți e Skitgubbe (pierzătorul).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cartea, ridic (ia grămada), risc (riști o carte din pachet). Mâini și pachet ascunse.
- Exemple legale la început: `3d`, `Qs`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ombre"></a>
## Ombre (L'Hombre)

- **Id:** `ombre`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play ombre` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Ombre în doi cu pachetul spaniol de 40 de cărți: câte 9 cărți.
- Licitezi (entrada) sau treci;
- Ombre alege atuul, poate arunca și trage cărți, apoi se joacă 9 levate.
- Matadorii (atuurile cele mai mari) bat restul.
- Ombre câștigă dacă ia mai multe levate decât adversarul.
- Primul la 8 puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: entrada, pas, atu <culoare>, scap <cărți>, gata, cartea, imparte. Mâini ascunse.
- Exemple legale la început: `entrada`, `pas`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="domino9"></a>
## Dominó cubano (doble nueve)

- **Id:** `domino9`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play domino9` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Set dublu-nouă de 55 de piese; câte 10, 35 în rezervă.
- Pui o piesă care se potrivește la un capăt (începe cel cu cel mai mare dublu); dacă nu ai piesă, tragi până poți, iar cu rezerva goală treci.
- Câștigă cine rămâne primul fără piese; la blocaj, cine are mai puține puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: piesa cu capătul (ex. 5-7 stanga), trage, pas. Mâini și rezervă ascunse.
- Exemple legale la început: `1-9:S`, `1-9:D`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="narde"></a>
## Narde scurtă (Short Nardy)

- **Id:** `narde`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play narde` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Narde scurtă (rudă cu table): câte 15 piese, două zaruri (la dublă se joacă de patru ori), aduci toate piesele acasă și le scoți primul.
- Implementarea folosește motorul de table cu regulile standard.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, apoi mutările pe puncte.
- Exemple legale la început: `8/3`, `13/8`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="dog"></a>
## Dog (curse cu cărți)

- **Id:** `dog`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play dog` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cursă pe inel cu cărți pentru doi: inel de 64 de câmpuri, 4 piese, 110 cărți.
- A sau K scot o piesă; cărțile mută piese după valoare (A = 1 sau 11, 4 = înainte sau înapoi, 7 = șapte pași de câte 1, J = schimb cu o piesă a adversarului, joker = orice carte).
- Aterizând exact pe adversar îl trimiți înapoi; piesa de pe start e protejată.
- Câștigă cine aduce primul toate cele 4 piese în casă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: K:intra, 5:p2, 4:p1-, 7:p1, J:p1<>o2, pas. Mâini ascunse.
- Exemple legale la început: `A:intra`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="toc"></a>
## Toc (jeu)

- **Id:** `toc`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play toc` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cursă franceză cu cărți și popice pentru doi: inel de 80 de câmpuri, 4 piese, 54 de cărți.
- A sau K scot o piesă; cărțile mută după valoare; 4 merge numai înapoi; 7 = șapte pași de câte 1; valetul mută 11 sau schimbă; jokerul mută 15 sau scoate o piesă.
- Aterizând exact pe adversar îl trimiți înapoi; piesa de pe start e protejată.
- Câștigă cine aduce primul toate cele 4 piese în casă.
- Mâini ascunse.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `jk:intra`, `K:intra`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="trictrac"></a>
## Trictrac (simplificat)

- **Id:** `trictrac`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play trictrac` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Trictrac simplificat: 24 de puncte, câte 15 piese stivuite pe punctul de start, două zaruri, fără lovire.
- Scoți piese doar când toate sunt în ultimul sfert.
- Punctaj în găuri la sfârșitul turei: dublă +2, trei puncte din ultimul sfert cu 2+ piese +4, fiecare piesă scoasă +1.
- Primul la 15 găuri câștigă.
- Sistemul real de găuri (jan-uri, battre à vrai) nu e implementat.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, zar:punct (ex. 4:1).
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tangram"></a>
## Tangram (duel pe grilă)

- **Id:** `tangram`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tangram` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Duel Tangram simplificat pe grilă 6×6: fiecare jucător are 7 piese (2 triunghiuri mici = 1 pătrățel, triunghi mediu = domino, pătrat = 2×2, paralelogram = S de 4, 2 triunghiuri mari = L de 3).
- Puneți alternativ câte o piesă (orice rotire sau oglindire), fără suprapunere; cine nu mai poate pune nicio piesă pierde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: piesă+orientare@colț (ex. q1@c3). Nu e puzzle-ul cu siluete.
- Exemple legale la început: `a1@a1`, `a1@b1`
- La început sunt 401 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="zipzapzop"></a>
## Zip Zap Zop

- **Id:** `zipzapzop`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play zipzapzop` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de numărat: numărați pe rând de la 1 la 30; spuneți zip la multiplii de 3, zap la multiplii de 5, zop la multiplii de 15.
- Un cuvânt greșit pierde.
- Fiecare are 2 pas-uri (numărul rămâne adversarului).
- Câștigă cine spune corect 30.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: numărul, zip, zap, zop, pas.
- Exemple legale la început: `1`, `zip`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="wouldyourather"></a>
## Ai prefera…

- **Id:** `wouldyourather`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play wouldyourather` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Șase dileme.
- La fiecare, fiecare alege în secret răspunsul propriu (a sau b) și ghicește răspunsul celuilalt: mutări aa, ab, ba, bb (prima literă = răspunsul tău, a doua = ghicitul tău), apoi arat.
- Fiecare ghicit corect = 1 punct; mai multe puncte câștigă.
- Alegeri ascunse.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `aa`, `ab`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chinesewhispers"></a>
## Telefonul fără fir

- **Id:** `chinesewhispers`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play chinesewhispers` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Versiune în duel pentru doi: trimițătorul vede un mesaj de 4 cifre și îl lasă (las) sau strică o cifră cu ±1 (strica 2+).
- Receptorul vede mesajul ajuns și suma de control (suma cifrelor originale mod 10) și arată cifra stricată (restaur 1–4) sau zice restaur 0.
- Restaurarea corectă aduce punct receptorului, altfel trimițătorului. 6 runde, rolurile alternează.
- Originalul e ascuns.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `las`, `strica`
- La început sunt 17 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="namethattune"></a>
## Ghicește melodia

- **Id:** `namethattune`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play namethattune` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Șase melodii din domeniul public date ca nume de note.
- Licitezi de câte note ai nevoie (ofer N); adversarul coboară oferta sau cere dovada (ghicesc).
- Cel mai mic licitator aude doar N note și alege titlul dintre 4 variante (o1–o4), apoi arat.
- Corect: +1 pentru el, greșit: +1 pentru celălalt.
- Mai multe puncte câștigă.
- Titlul corect rămâne ascuns până la arat.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ofer`, `1`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="babaoarba"></a>
## Baba-oarba

- **Id:** `babaoarba`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play babaoarba` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Duel asimetric pe tablă 7×7: J1 e baba cu ochii legați, J2 fugarul.
- Baba mută (n, s, e, v, ne…, sta) sau încearcă să prindă o căsuță vecină (prind c4, apoi arat).
- După fiecare rundă aude un indiciu: cald (fugarul e la cel mult 2 căsuțe) sau rece.
- Fugarul face un pas.
- Baba câștigă prinzând fugarul; fugarul câștigă rezistând 30 de runde.
- Poziția fugarului e ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `sta`, `prind`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="capturetheflag"></a>
## Capture the Flag

- **Id:** `capturetheflag`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play capturetheflag` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 9×7, jumătate de teritoriu fiecare (coloana e e neutră).
- Câte 3 alergători; un pas ortogonal pe mutare (b2-b3).
- Un alergător care intră în teritoriul inamic lângă un apărător, sau un intrus lângă piesa ta în teritoriul tău, e atins și stă 3 mutări în închisoare.
- Ajungi la steagul inamic și îl aduci în teritoriul tău ca să câștigi; purtătorul atins pierde steagul. 160 de mutări: remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `b2-c2`, `b2-a2`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="shut_box"></a>
## Închide cutia (Shut the Box)

- **Id:** `shut_box`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play shut_box` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Plăcuțe 1–9; arunci (arunc) două zaruri (unul, dacă 7–9 sunt închise) și închizi plăcuțe cu suma egală (inchid 1+2+6).
- Când nu mai poți, tura se încheie cu suma plăcuțelor deschise.
- Cutie închisă = victorie pe loc; 5 runde, cel mai mic total câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chuck"></a>
## Zarul norocos (Chuck-a-luck)

- **Id:** `chuck`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chuck` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Alegi un număr 1–6 și miza 5/10/20 (pariez 3 10), apoi arunci trei zaruri (arunc): fiecare zar egal cu numărul tău plătește miza 1:1, altfel pierzi miza. 12 ture fiecare, 100 de jetoane; câștigă cine are mai multe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pariez`, `1`
- La început sunt 54 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bar_dice"></a>
## Zaruri de bar (Bar dice)

- **Id:** `bar_dice`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play bar_dice` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci zaruri, cel mult 3 aruncări (arunc); după fiecare păstrezi cel puțin un zar (tin 6 5).
- Ai nevoie de Vas (6), Căpitan (5) și Echipaj (4); celelalte două zaruri sunt încărcătura (suma lor, 2–12), altfel 0.
- Încărcătura mai mare ia runda; primul la 4 runde (sau cele mai multe din 9).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cosmic"></a>
## Zaruri cosmice (după Cosmic Wimpout)

- **Id:** `cosmic`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cosmic` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cinci zaruri cu fețele 2, 3, 4, 5, 10 și Soare (S, joker în triplete).
- Arunci (arunc), păstrezi zarurile care punctează integral (tin 10 5): 10 = 100, 5 = 50, trei la fel = 100 × fața (trei de 10 = 1000).
- Apoi continui sau banca; fără zaruri care punctează pierzi punctele turei.
- Primul la 2000.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tombola"></a>
## Tombola (Italia)

- **Id:** `tombola`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tombola` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Fiecare alege 1–3 cartele de 3×9 (cartele 2, câte 1 punct); numerele din 1–90 se extrag (extrag) și se bifează automat.
- Primul rând cu 2/3/4/5 numere ia ambo (2), terno (3), quaterna (4), cinquina (6); tombola (cartelă completă) ia 15 și încheie jocul.
- Mai multe puncte câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `cartele`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pachinko"></a>
## Pachinko (Japonia)

- **Id:** `pachinko`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pachinko` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 20 de bile fiecare.
- Lansezi o bilă din coloana 1–9 (lansez 5), apoi cade: coboară prin 8 rânduri de cuie și ajunge într-un buzunar (plătește 3 2 1 2 6 2 1 2 3 bile).
- Coloanele impare ajung doar în buzunare impare, cele pare în buzunare pare. 15 lansări fiecare; mai multe bile câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `lansez`, `1`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fan_tan"></a>
## Fan-Tan (China)

- **Id:** `fan_tan`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play fan_tan` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Banca acoperă un morman de mărgele; pariezi pe «fan» 1–4 (restul la 4, 0 = 4) cu miza 5/10/20 (pun 3 10), apoi descopar.
- Pariul ghicit plătește 3:1 minus 5% comision, altfel pierzi miza. 10 runde, 100 de jetoane; câștigă cine are mai multe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pun`, `1`
- La început sunt 36 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hanoi"></a>
## Turnurile din Hanoi (Tower of Hanoi)

- **Id:** `hanoi`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hanoi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cursă: fiecare are propriul turn cu 5 discuri pe tija a și îl mută pe tija c (a-c = discul de sus de pe a trece pe c), niciodată un disc mare peste unul mic.
- Mutați pe rând; câștigă cine termină cu mai puține mutări (minim 31; egal = remiză).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `a-b`, `a-c`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pingpong"></a>
## Tenis de masă (Ping-pong)

- **Id:** `pingpong`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pingpong` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Un set la 11 (diferență 2).
- Cine lovește alege zona (st, ct, dr) și forța (sigur, atac, smash): ct-atac; apoi schimb rezolvă lovitura.
- Forța mare = mai multe mingi ratate, dar mai greu de returnat; adversarul pierde 25% pentru fiecare zonă dintre poziția lui și minge.
- Serviciul trece la 2 puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `st-sigur`, `st-atac`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hounds"></a>
## Hounds and Jackals (58 de găuri)

- **Id:** `hounds`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hounds` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Reconstrucție modernă (regulile originale s-au pierdut). 5 piese pe jucător; 4 bețe dau 1–5 (arunc).
- Intri pe gaura 1 doar cu 1 (intra) și arunci iar.
- Piesa înaintează (p12); scurtături 6→20, 8→10, 20→6, 10→8; nu sari peste piesă proprie de pe 10/15/20/25.
- Gaura 30 exact; ieși (iese) cu 1 sau 5 după ce toate au intrat.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ataxx"></a>
## Ataxx (7×7)

- **Id:** `ataxx`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ataxx` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Două piese în colțuri opuse.
- La mutare clonezi o piesă pe o groapă liberă la distanță 1 (a1-b2) sau o sari la distanță 2 (a1-c3); toate piesele adverse vecine cu destinația devin ale tale.
- Fără mutări: pas.
- Câștigă cine are mai multe piese când tabla se umple sau nimeni nu mai poate muta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `a1-b1`, `a1-c1`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="shatranj"></a>
## Shatranj (Persia)

- **Id:** `shatranj`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play shatranj` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Strămoșul șahului: ferz (un pas pe diagonală), alfil (sare exact 2 pe diagonală), cal, turn, rege; pionul avansează cu un pas și se promovează în ferz; fără rocadă.
- Câștigi prin mat, pat sau rege gol (cu excepția contra-capturii).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e2e3, b1c3.
- Exemple legale la început: `b1c3`, `b1a3`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="shashki"></a>
## Dame rusești (Shashki)

- **Id:** `shashki`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play shashki` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 8×8: piesele simple mută înainte, dar captură și înapoi; dama zboară pe diagonală și capturează de la distanță.
- Captura e obligatorie, salturile continuă; piesa care atinge ultimul rând în captură devine damă și continuă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c3-d4, c3xe5xg7. Pierde cine nu mai poate muta.
- Exemple legale la început: `a3-b4`, `c3-d4`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="o_an_quan"></a>
## Ô ăn quan (Vietnam)

- **Id:** `o_an_quan`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play o_an_quan` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Inel de 12 gropi: 5 mici pe latură (5 pietre) și două quan cu mandarin (10 puncte).
- Alegi o groapă proprie și sensul (3+ = orar, 3- = antiorar), semeni pietrele; următoarea plină se ia și se continuă; după o groapă goală, groapa plină următoare se capturează.
- Rândul gol: pune5.
- Câștigă cine adună mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `1+`, `1-`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hypergammon"></a>
## Hypergammon

- **Id:** `hypergammon`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hypergammon` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Table cu doar trei piese pe jucător, câte una pe punctele 24, 23 și 22; restul regulilor ca la table: lovire, bară, bear-off, dubla se joacă de patru ori.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 24/20, bar/22, 3/off. Primul care scoate cele 3 piese câștigă.
- Exemple legale la început: `22/17`, `23/18`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="jacquet"></a>
## Jacquet (Franța)

- **Id:** `jacquet`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play jacquet` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 15 piese pe jucător plecând din punctul 1 în colțuri opuse, ambii mută în același sens pe 24 de puncte; fără lovire, un punct ocupat de adversar e blocat.
- Prima piesă mutată (poștașul) trebuie să ajungă în ultimul sfert înainte să se miște altele.
- Zaruri automate, dubla de patru ori.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1>5, 22>off. Primul care scoate toate piesele câștigă.
- Exemple legale la început: `1>6`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="moultezim"></a>
## Moultezim (Grecia / Turcia)

- **Id:** `moultezim`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play moultezim` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă de table fără lovire: 15 piese pe jucător pornesc din colțuri opuse și merg în același sens pe 24 de puncte; un punct ocupat de adversar e blocat.
- O piesă trebuie dusă dincolo de punctul de plecare al adversarului (13) înainte de a muta a doua.
- Bear-off doar din ultimul sfert.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1>5, 22>off. Primul care scoate toate piesele câștigă.
- Exemple legale la început: `1>6`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chu_shogi"></a>
## Chu Shogi (12×12)

- **Id:** `chu_shogi`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chu_shogi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Șahul japonez mare pe tablă 12×12 cu 46 de piese pe parte (Leu, Regină, Tigru, Elefant ebriu, Căpcăun etc.).
- Mutări scrise ca traseu: g6-g7, leul poate face doi pași (g6-g7-g8), iar `+` promovează la intrarea în zona adversă.
- Captura prințului sau a regelui câștigă (versiune simplificată, fără regula de repetare).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `b1-b2`, `c1-d2`
- La început sunt 31 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chowka_bara"></a>
## Chowka Bara

- **Id:** `chowka_bara`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chowka_bara` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de cursă indian pe tablă 5×5 cu patru piese pe jucător și patru scoici ca zaruri (4 și 8 dau o tură în plus).
- O piesă intră în joc doar cu 4 sau 8; piesele fac un tur pe marginea tablei, apoi intră în spirala interioară până la centru.
- Capturezi piese adverse pe pătratele neprotejate; câștigă cine aduce toate piesele în centru.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="uckers"></a>
## Uckers

- **Id:** `uckers`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play uckers` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Ludo marinăresc britanic: patru piese pe jucător, un singur zar, piesele ies doar cu 6 și fac un tur de 48 de pătrate, apoi urcă pe coloana de casă.
- Două piese proprii pe același pătrat formează un bloc (spate-în-spate) care nu poate fi capturat.
- Un 6 sau o captură dă o aruncare în plus; câștigă cine ajunge primul cu toate piesele acasă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `intră`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mau_mau"></a>
## Mau-Mau

- **Id:** `mau_mau`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mau_mau` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc german de eliminare a cărților cu pachet de 32: 5 cărți fiecare; pui o carte de aceeași culoare sau valoare cu cea de deasupra. 7 = adversarul trage 2 (se pot însuma), 8 = adversarul sare, valetul = joker (alegi culoarea).
- Cine rămâne fără cărți ia punctele cărților adversarului; primul la 60 câștigă.
- Mâna adversarului este ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `Jh:h`, `Jh:d`
- La început sunt 5 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="muggins"></a>
## Muggins (All Fives)

- **Id:** `muggins`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play muggins` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Domino cu punctaj la capete: pui o piesă la un capăt al șirului (`3-4:S` sau `:D`); dacă suma capetelor deschise e multiplu de 5 marchezi acea sumă.
- Dacă nu ai piesă potrivită tragi (`pesc`) sau pasezi.
- Cine rămâne fără piese ia în plus punctele adversarului rotunjite la 5; primul la 61 câștigă.
- Piesele adversarului sunt ascunse.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `6-6`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="schafkopf"></a>
## Schafkopf (2 jucători)

- **Id:** `schafkopf`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play schafkopf` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc bavarez de levate cu 32 de cărți, adaptat pentru doi: 8 cărți fiecare (restul nu se joacă).
- Licitezi un contract (`licit wenz` sau `licit solo-c`): atuul este format din Obere (Q), Untere (J) și culoarea aleasă; la wenz doar valeții.
- A 11, 10 = 10, K 4, Q 3, J 2; declarantul are nevoie de 61 din 120 de puncte.
- Meci de 6 mâini; mâna adversarului este ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `licit`, `wenz`
- La început sunt 11 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="preferans"></a>
## Preferans (2 jucători)

- **Id:** `preferans`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play preferans` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc rusesc de levate cu 32 de cărți, adaptat pentru doi: 10 cărți fiecare și un talon de 2.
- Licitezi numărul de levate și atuul (`licit 6h`, `licit 7nt`; ordinea s < c < d < h < fără atu).
- Declarantul ia talonul (`talon`), aruncă 2 cărți (`pune 7h 8c`) și joacă; trebuie să răspunzi la culoare, iar fără culoare tai cu atu.
- Reușit = +2·(nivel−5), ratat = minus pe fiecare levată lipsă. 6 mâini; mâna adversarului este ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `licit`, `7s`
- La început sunt 41 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ulti"></a>
## Ulti (2 jucători)

- **Id:** `ulti`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play ulti` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc maghiar de levate cu 32 de cărți, adaptat pentru doi: 10 cărți fiecare și un talon de 2.
- Contracte: `licit parti-h` (cel puțin 46 de puncte, atuul cupă) sau `licit ulti-h` (parti plus ultima levată câștigată cu 7 de atu).
- Aș și 10 valorează 10, ultima levată 10 (90 în total).
- Parti: ±1; ulti anunțat: ±4 în plus. 6 mâini; mâna adversarului este ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `licit`, `parti-c`
- La început sunt 11 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="brag"></a>
## Three Card Brag

- **Id:** `brag`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play brag` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pocher englezesc cu 3 cărți pentru doi: ante 1 fisă, 3 cărți ascunse fiecare.
- Ierarhie: trio (A-A-A cel mai mare), culoare-secvență, secvență, culoare, pereche, carte mare.
- La rând: `ridic` (miza +1, până la 5), `vad` (egalezi și arăți cărțile; la egalitate pierde cine cere arătarea) sau `fold`. 10 mâini cu 30 de fise; mâna adversarului este ascunsă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ridic`, `vad`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="crossword"></a>
## Cuvinte încrucișate

- **Id:** `crossword`
- **Categorie:** Cuvinte
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play crossword` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Duel pe un careu generat la fiecare partidă (6–9 cuvinte românești care se încrucișează).
- Alegi un cuvânt și propui soluția: `3o:casa` (număr, o = orizontal / v = vertical).
- Corect = puncte egale cu lungimea și rămâi la rând; greșit = −1 și rândul trece.
- La final câștigă scorul mai mare.
- Definițiile și categoria sunt vizibile; litere se dezvăluie la încrucișări.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `1v:veverita`, `1v:crocodil`
- La început sunt 121 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="perquackey"></a>
## Perquackey

- **Id:** `perquackey`
- **Categorie:** Cuvinte
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play perquackey` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de cuvinte cu 15 zaruri-litere, pe ture pentru doi.
- Zarurile se aruncă la începutul fiecărei runde (`arunc`); apoi pe rând propui un cuvânt de cel puțin 3 litere format din zaruri (`cuv casa`, fiecare zar o dată, un cuvânt doar o dată pe rundă) sau `pas`.
- Două `pas` la rând încheie runda.
- Punctaj: lungimea minus 2. 3 runde; scorul mai mare câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ligretto"></a>
## Ligretto

- **Id:** `ligretto`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ligretto` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Jocul de cărți rapid, pe ture pentru doi.
- Fiecare are o stivă Ligretto de 10 cărți, 3 rânduri și o mână de 27; pe masă se construiesc grămezi ascendente pe culoare de la 1 la 10.
- Tura: câte mutări vrei (`l-c2`, `w-r1`, `r2-c1`, `l-cn` pentru un 1 nou), apoi `da` întoarce 3 cărți din mână.
- Pe rânduri pui cu 1 mai mic și altă culoare.
- Câștigă cine golește Ligretto; altfel punctele (cărți puse − 2× cărți rămase).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `da`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="truth_dare"></a>
## Truth or Dare (Adevăr sau provocare)

- **Id:** `truth_dare`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play truth_dare` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de petrecere pentru doi, familial.
- În 12 ture, jucătorul la rând alege `adevar` sau `provocare` (sau `refuz`, −1 punct); jocul arată întrebarea sau provocarea, el o execută, iar celălalt este judecător: `ok` (performer +1 la adevăr / +2 la provocare, judecător +1) sau `nu` (performer −1).
- Scorul mai mare câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `adevar`, `provocare`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sticla"></a>
## Sticla (Spin the bottle)

- **Id:** `sticla`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sticla` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de petrecere familial cu sticla învârtită.
- Șase locuri în cerc (J1: 1, 3, 5;
- J2: 2, 4, 6).
- Învârtitorul rotește sticla (`invart`), apoi alege dificultatea probei `nivel 1`, `nivel 2` sau `nivel 3`; proprietarul locului arătat spune `fac` sau `refuz` (−1).
- Cine încearcă reușește cu 90% / 60% / 30% și primește atâtea puncte cât nivelul, altfel −1 (`incerc`). 8 ture; scorul mai mare câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `invart`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sardines"></a>
## Sardines

- **Id:** `sardines`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play sardines` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Ascunsul inversat, două runde cu roluri schimbate, într-o casă cu 9 camere (grilă 3×3).
- Ascunsul alege în secret camera (`ascund N`); căutătorul pornește din camera 1 și face câte un pas (`ir N`), apoi `asculta` arată dacă l-a găsit sau un indiciu public (respirații aproape / liniște).
- Camerele încă posibile sunt marcate cu «?».
- Ascunsul poate sta (`sta`) sau se poate muta (`ma N`).
- Ascunsul primește atâtea puncte câți pași a făcut căutătorul (13 dacă nu e găsit în 10).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ascund`, `2`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="scaune_muzicale"></a>
## Scaunele muzicale

- **Id:** `scaune_muzicale`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play scaune_muzicale` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Doi dansatori pe un cerc de 8 locuri, cu un singur scaun marcat # în fiecare rundă.
- Pe rând faci un pas de 1, 2 sau 3 locuri (`pas1`, `pas2`, `pas3`; nu aterizezi pe adversar, nu poți sta pe loc).
- După fiecare mutare `asculta` poate opri muzica (1 șansă din 5): cine stă atunci pe scaun ia punctul rundei.
- Primul la 3 puncte câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `pas1`, `pas2`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mikado"></a>
## Mikado (Pick-up sticks)

- **Id:** `mikado`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play mikado` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Un morman de 21 de bețișoare (Mikado 20, Mandarini 10, Bonzi 5, Samurai 3, Kuli 2).
- Ridici bețișorul `ia N`: dacă nu are altele deasupra reușești sigur și continui; dacă este acoperit riști (`incerc`: șanse 50% pentru un bețișor acoperit, 25% pentru două …), iar la eșec pierzi rândul. `pas` încheie rândul după ce ai ridicat ceva.
- Când mormanul e gol câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ia`, `1`
- La început sunt 42 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="kick_can"></a>
## Kick the Can

- **Id:** `kick_can`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play kick_can` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de ascuns și prins pe un teren 7×7.
- J1 este Căutătorul (S), J2 conduce trei ascunși (1–3); cutia O stă în centru (d4).
- Fiecare face două pași pe tură în 8 direcții (`ne`, `s`…; ascunșii `h2 ne`).
- Căutătorul prinde un ascuns călcând pe el; un ascuns liber care pășește pe cutie îi eliberează pe toți prinșii.
- Căutătorul câștigă când toți 3 sunt prinși; ascunșii câștigă dacă rezistă 20 de ture.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `n`, `ne`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mingea"></a>
## Mingea (Dodgeball)

- **Id:** `mingea`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play mingea` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Două echipe de câte 4 jucători pe 7 coloane.
- Pe rând muți un jucător într-o coloană vecină liberă (`3>4`) sau arunci spre o coloană adversă (`arunc 4`).
- Mingea nu cade exact unde ai țintit: `zbor` hotărăște la întâmplare coloana (50% ținta, 20% fiecare vecină, 5% la două coloane).
- Cine e în coloana de aterizare iese.
- Câștigă echipa care rămâne cu jucători.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `2>1`, `3>4`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="kalah"></a>
## Kalah

- **Id:** `kalah`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play kalah` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Kalah 6×4: semănat antiorar (sari peste magazia adversarului); ultima sămânță în magazia ta = mutare în plus; într-o groapă proprie goală cu opusa nevidă capturezi opusa.
- Când o parte rămâne goală, restul merge în magazia celeilalte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1–6 (gropile tale).
- Exemple legale la început: `1`, `2`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="foxgeese"></a>
## Vulpea și gâștele (Fox and Geese)

- **Id:** `foxgeese`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play foxgeese` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă în cruce cu 33 de găuri.
- J1 = vulpea (d4), J2 = 13 gâște.
- Vulpea mută ortogonal sau sare peste o gâscă (d4xd6xf6); gâștele mută doar înainte/lateral.
- Gâștele câștigă blocând vulpea; vulpea câștigă dacă rămân sub 6 gâște.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d4-d5, d4xd6.
- Exemple legale la început: `d4-e4`, `d4-c4`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="alquerque"></a>
## Alquerque

- **Id:** `alquerque`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play alquerque` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 5×5 cu diagonale prin punctele pare, 12 piese pe jucător.
- Pas pe o linie; captura (salt peste o piesă adversă pe loc liber) e obligatorie și se înlănțuie.
- Câștigă cine captează toate piesele sau blochează adversarul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: b3-c3, a3xc3xe3.
- Exemple legale la început: `b2-c3`, `c2-c3`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="jungle"></a>
## Jungle (Dou Shou Qi)

- **Id:** `jungle`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play jungle` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 7×9, 8 animale pe jucător (șobolan 1 … elefant 8).
- Pas ortogonal; capturezi rang egal sau mai mic (șobolanul poate lua elefantul); doar șobolanul intră în apă; leul și tigrul sar lacul; în capcana adversă rangul e 0.
- Câștigi intrând în bârlogul adversarului sau capturând tot.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1-a2, c3xc4.
- Exemple legale la început: `a1-b1`, `a1-a2`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chaturanga"></a>
## Chaturanga (India)

- **Id:** `chaturanga`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chaturanga` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Strămoșul șahului: raja, mantri (pas diagonal), gaja (sare 2 pe diagonală), ashva (cal), ratha (car), padati (pion, promovează în mantri).
- Fără rocadă și pas dublu.
- Mat = pierdere, pat = remiză, raja rămas singur = pierdere.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e2e3, c1e3.
- Exemple legale la început: `b1c3`, `b1a3`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="camelot"></a>
## Camelot

- **Id:** `camelot`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play camelot` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă 160 de pătrate cu castele. 10 pioni + 4 cavaleri.
- Pas în 8 direcții, canter (sari peste o piesă proprie), salt (capturezi, obligatoriu); cavalerul poate încărca (canter + salturi).
- Câștigi cu două piese în castelul adversarului sau prin capturare/blocare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d6-d7, e7xe9, f6-f8xf10.
- Exemple legale la început: `c6-b5`, `c6-c5`
- La început sunt 112 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="fotbalmasa"></a>
## Fotbal de masă pe ture (Table football)

- **Id:** `fotbalmasa`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play fotbalmasa` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 8 bare, fiecare acoperă 2 din 5 culoare.
- O tură = muți o bară (atac 3) sau, cu mingea, trimiți pe un culoar acoperit de bara ta (trimit 2): mingea se oprește la prima bară care acoperă culoarul (pasă sau interceptare); gol dacă trece pe culoarele 2–4.
- Primul la 5.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `portar`, `1`
- La început sunt 28 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tiddlywinks"></a>
## Tiddlywinks (pe ture)

- **Id:** `tiddlywinks`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tiddlywinks` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Masă 11×7, 4 winks pe jucător, găleata în f4.
- Muți un wink liber cu până la 3 câmpuri (c4-d4); aterizarea pe un wink îl acoperă (îl blochează), în găleată îl scoate (3 puncte).
- Scor: 3 × potted + winks adverse acoperite.
- Se termină când cineva nu mai are winks libere sau după 80 de mutări.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `a2-a1`, `a2-b1`
- La început sunt 92 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="joc15"></a>
## Jocul de 15 (cursă)

- **Id:** `joc15`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play joc15` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Fiecare jucător are propria tablă 4×4, amestecată identic.
- Pe rând, mutați o piesă lângă locul gol pe tabla voastră (ex. 7); câștigă primul care aduce piesele în ordinea 1–15.
- După 300 de mutări câștigă cel mai apropiat de ordine.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `6`, `15`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sudoku"></a>
## Sudoku duel

- **Id:** `sudoku`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sudoku` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Grilă cu soluție unică.
- Pe rând alegeți celulă și cifră fără conflict (r3c4=5): corect = +1 (+2 pentru fiecare rând/coloană/pătrat completat), greșit = −1 și cifra se elimină din acea celulă.
- Câștigă cine are mai multe puncte când grila e plină.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `r1c1=4`, `r1c1=6`
- La început sunt 136 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="biliard"></a>
## Biliard francez (carambol)

- **Id:** `biliard`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play biliard` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 3 bile, masă fără buzunare.
- Lovești bila ta (albă J1, galbenă J2) cu unghi și forță (u45f2 = 45°, forța 2); dacă atinge ambele bile marchezi și mai joci.
- Primul la 10 puncte; fizică deterministă fără efect.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `u0f1`, `u0f2`
- La început sunt 540 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tavli"></a>
## Tavli (Portes, Plakoto, Fevga)

- **Id:** `tavli`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tavli` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tabla greacă: J1 alege varianta (portes, plakoto sau fevga).
- Portes = backgammon cu lovire, Plakoto = imobilizezi piesa adversă singură, Fevga = ambii în același sens, fără lovire și cu regula zidului.
- Zaruri automate; 15 piese; scoți piesele când toate sunt acasă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 24/18, bar/20, 6/off.
- Exemple legale la început: `portes`, `plakoto`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="gulbara"></a>
## Gul bara (Golful Persic)

- **Id:** `gulbara`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play gulbara` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă de table fără lovituri: 15 piese pe jucător, colțuri opuse, același sens de mers; un punct ocupat de adversar e blocat.
- Dublurile dau 4 mutări (apoi 4 cu fiecare număr până la 6, după primele 3 aruncări).
- Scoți piesele când toate sunt acasă; primul care termină câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 24/18, 6/off.
- Exemple legale la început: `24/19`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chinchirorin"></a>
## Chinchirorin (zaruri în bol)

- **Id:** `chinchirorin`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chinchirorin` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Trei zaruri; oya (banca) și ko alternează.
- Ko pariază 1–3 (pariez 2), apoi se aruncă (arunc, până la 3 încercări): Pinzoro 1-1-1 = 5×, Arashi (triplet) 3×, Shigoro 4-5-6 2×, punct = zarul singur dintr-o pereche; 1-2-3 pierde 2×. 12 runde sau până la faliment.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="shipcrew"></a>
## Ship, Captain and Crew

- **Id:** `shipcrew`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play shipcrew` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 5 zaruri, 3 aruncări pe tură: ai nevoie, în ordine, de 6 (ship), 5 (captain), 4 (crew); celelalte două zaruri sunt încărcătura (scorul turei).
- După o aruncare reușită: pastrez sau rearunc. 5 ture pe jucător; câștigă suma cea mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, pastrez, rearunc.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="beetle"></a>
## Gândacul (Beetle)

- **Id:** `beetle`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play beetle` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Arunci pe rând un zar: 1 corp, 2 cap, 3 picior (6), 4 antenă (2), 5 ochi (2), 6 coadă.
- Corpul primul; capul, picioarele și coada cer corpul; antenele și ochii cer capul.
- Primul gândac complet (13 piese) câștigă.
- Mutare: arunc.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="kismet"></a>
## Zaruri colorate (Kismet)

- **Id:** `kismet`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play kismet` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 15 ture; până la 3 aruncări cu 5 zaruri (tine 3 3 5), apoi scor într-o categorie: unu…sase (bonus 35/55/75), 2perechi (aceeași culoare), 3la-fel, scara (30), culoare (35), casa (+15), casa-culoare (+20), 4la-fel (+25), sansa, kismet (+50).
- Zaruri impare = roșii, pare = albastre.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="deadmansdice"></a>
## Dead Man's Dice (zarurile piratului)

- **Id:** `deadmansdice`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play deadmansdice` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Fiecare are 2 zaruri și zarul Jolly Roger.
- Aruncate, zarurile se așază într-o grămadă dacă sunt egale cu cel de sus, cu 1 mai mari sau dau 7 împreună; jefuiești zarul de sus al adversarului dacă se potrivește.
- Runda: suma grămezii (JR deasupra dublează, JR neasezat −7).
- Primul la 100.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, arunc-jr, jefuiesc.
- Exemple legale la început: `arunc`, `arunc-jr`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="snap"></a>
## Snap

- **Id:** `snap`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play snap` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet de 52 împărțit în două; pe rând întorci (intorc) câte o carte în mijloc.
- Când ultimele două cărți au aceeași valoare, cine e la mutare poate striga snap și ia grămada; un snap fals costă o carte.
- Cine rămâne fără cărți pierde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `intorc`, `snap`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="simon"></a>
## Simon zice

- **Id:** `simon`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play simon` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Simon dă o poruncă (simon-zice sari sau ordin sari); celălalt o execută (execut sari) doar dacă a zis „Simon zice”, altfel stă (stau).
- Răspuns corect = punct pentru cel care ascultă, greșit = punct pentru Simon. 12 runde, rolurile alternează.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `simon-zice`, `sari`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="settemezzo"></a>
## Sette e mezzo (șapte și jumătate)

- **Id:** `settemezzo`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play settemezzo` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pachet italian de 40 de cărți: figurile (8, 9, 10) valorează ½, restul valoarea lor.
- Te apropii de 7½ fără să treci.
- Prima carte e ascunsă; carta = tragi, stau = rămâi.
- Ko pariază 1–3, banca câștigă la egalitate; 7½ natural (două cărți) plătește dublu. 12 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `carta`, `stau`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="baccarat"></a>
## Baccarat (Punto Banco)

- **Id:** `baccarat`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play baccarat` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Punto Banco: parieți pe punto, banco (comision 5%) sau egal (8:1) cu 1–3 unități (punto 2); cărțile se trag după tabelul fix.
- Totalul = suma mod 10; 8–9 = natural. 20 de coupe; cine are mai multe jetoane câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: punto N, banco N, egal N, impart.
- Exemple legale la început: `punto`, `1`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hangman"></a>
## Spânzurătoarea (Hangman)

- **Id:** `hangman`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play hangman` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cine pune alege în secret un cuvânt din listă (cuvant casa); celălalt ghicește litere (litera a) sau cuvântul (ghicesc casa). 7 greșeli = pierdută.
- Reușită = 1 + (7 − greșeli) puncte.
- Două runde cu roluri inversate; cine are mai multe puncte câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `cuvant`, `casa`
- La început sunt 120 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="douazeci"></a>
## Douăzeci de întrebări

- **Id:** `douazeci`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play douazeci` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cine gândește alege un obiect din listă (obiect caine); celălalt pune întrebări da/nu numerotate (intreb 7) sau încearcă un răspuns (este caine); maxim 20 între întrebări și încercări.
- Cost = câte folosește (eșec = 21).
- Două runde cu roluri inversate; costul total mai mic câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `obiect`, `caine`
- La început sunt 86 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ispy"></a>
## Ghicește ce văd (I Spy)

- **Id:** `ispy`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play ispy` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cine vede alege un obiect din cameră (vad masa) și spune litera lui; celălalt ghicește (este masa) sau cere indicii (indiciu culoare / loc / marime).
- Maxim 6 încercări (eșec = 7).
- Două runde cu roluri inversate; costul total mai mic câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `vad`, `lampa`
- La început sunt 72 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="comori"></a>
## Vânătoarea de comori

- **Id:** `comori`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play comori` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pe o grilă 8×8, cine ascunde alege celula (ascund d5); celălalt sapă (sap c3) și primește indiciul cald/rece după distanța Manhattan. 12 săpături (eșec = 13).
- Două runde cu roluri inversate; costul total mai mic câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `ascund`, `a1`
- La început sunt 128 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mima"></a>
## Mima (Charades)

- **Id:** `mima`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mima` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Adaptare fără mimă fizică: cine mimează alege un cuvânt (mimez delfin) și dă indicii pe rând (gest categorie / silabe / prima-litera / ultima-litera / lungime); celălalt ghicește (ghicesc delfin) sau cere mai-multe.
- Cost = indicii + greșeli (eșec = 8).
- Două runde; costul mai mic câștigă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `mimez`, `elefant`
- La început sunt 64 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="crima"></a>
## Crima cu ochiade (Wink Murder)

- **Id:** `crima`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play crima` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Zece suspecți în cerc.
- J1 = ucigașul (sunt 4), face cu ochiul și un suspect moare (clipesc 7);
- J2 = detectivul primește un martor (4 suspecți, unul e ucigașul) și acuză (acuz 3) sau așteaptă (astept).
- Acuzație corectă = detectivul câștigă; 3 greșeli sau 2 suspecți rămași = ucigașul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `sunt`, `1`
- La început sunt 20 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sheepshead"></a>
## Sheepshead

- **Id:** `sheepshead`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play sheepshead` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Levate cu 32 de cărți și atuuri fixe (Q, J, caro); adaptare pentru doi, cu blind și cărți îngropate.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: iau, pas, ingrop 7c, apoi cartea jucată (ex. Qc).
- Exemple legale la început: `iau`, `pas`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tysiac"></a>
## Tysiąc

- **Id:** `tysiac`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tysiac` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tysiąc (o mie) pentru doi: licitație 100–300, hlášky/mariaje și două musiks; primul la 1000 sau 6 mâini.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: licitez 110, pas, musik 1, las 9h, apoi cartea (ex. Ah) sau Kh marjas.
- Exemple legale la început: `licitez`, `100`
- La început sunt 43 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mariash"></a>
## Mariáš

- **Id:** `mariash`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mariash` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mariáš ceh pentru doi: forhont, talon de două cărți, atu, hlášky 20/40, ultima levată +10; 6 mâini.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: las 7g (două cărți), atu inimi, apoi cartea (ex. Ag) sau Kg hlaska.
- Exemple legale la început: `las`, `Ai`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="texas42"></a>
## Texas 42

- **Id:** `texas42`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play texas42` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Levate cu domino pentru doi: licitație 30–42, atu, 42 de puncte; primul la 4 semne.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: licitez 32, pas, atu 5, apoi piesa (ex. 6-4).
- Exemple legale la început: `licitez`, `40`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pit"></a>
## Pit

- **Id:** `pit`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play pit` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Schimburi ascunse de cărți între doi jucători; corner = toate cele 6 cărți ale unei mărfi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ofer grau 2, dau ovaz, schimb.
- Exemple legale la început: `ofer`, `grau`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mahjong_am"></a>
## American Mah Jongg

- **Id:** `mahjong_am`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mahjong_am` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mah jongg simplificat pentru doi: 152 de piese cu jokeri, pung-uri expuse, 4 seturi + pereche sau șapte perechi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc 5k, pung, pas, mahjong.
- Exemple legale la început: `arunc`, `5k`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="damas"></a>
## Dame braziliene (Damas)

- **Id:** `damas`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play damas` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Dame internaționale pe 8×8: piesele simple mută înainte și capturează în toate direcțiile, damele zboară, captura este obligatorie și trebuie să fie cea mai mare.
- Remiză după 20 de mutări pe parte fără captură sau mutare de piesă simplă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c3-d4, captură c3xe5xg7.
- Exemple legale la început: `a3-b4`, `c3-b4`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="renju"></a>
## Renju

- **Id:** `renju`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play renju` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Gomoku pe 15×15 cu restricții pentru Negru (J1): linia lungă (6+), dublu-patru și dublu-trei deschis sunt interzise;
- Negru câștigă cu exact cinci, Alb cu cinci sau mai multe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: h8 (coloane a–o, rânduri 1–15).
- Exemple legale la început: `h8`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tablut"></a>
## Tablut

- **Id:** `tablut`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tablut` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Hnefatafl lapon pe 9×9: 16 atacatori (J1, primul) contra 8 apărători și un rege.
- Toate piesele mută ca turnul; captură în clește; regele scapă pe orice pătrat de margine, atacatorii îl capturează pe patru laturi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: e1-e3.
- Exemple legale la început: `d1-d2`, `d1-d3`
- La început sunt 80 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="lasca"></a>
## Lasca

- **Id:** `lasca`
- **Categorie:** Abstract
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play lasca` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Dama lui Emanuel Lasker pe 7×7: piesa capturată trece la baza coloanei tale, se ia doar piesa de deasupra; ofițerii mută și capturează în toate direcțiile; captura e obligatorie.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1-b2, a1xc3xe5.
- Exemple legale la început: `a3-b4`, `c3-b4`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ruleta"></a>
## Ruletă (Roulette)

- **Id:** `ruleta`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ruleta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Ruletă europeană (0–36) pentru doi: 100 de jetoane fiecare, 12 învârtiri, un singur pariu pe învârtire (5, 10 sau 20).
- Număr plin 35:1, roșu/negru, par/impar, mic/mare 1:1, duzine și coloane 2:1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: rosu 10, plin 17 5, duz2 20, invart.
- Exemple legale la început: `plin`, `0`
- La început sunt 405 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="marbles"></a>
## Bile (par sau impar)

- **Id:** `marbles`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play marbles` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc antic de ghicit: cel care ține ascunde 1–5 bile, celălalt ghicește par sau impar; ghicești bine = iei atâtea bile, rău = le dai. 10 bile fiecare, 12 runde, rolurile alternează.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ascund 3, par, impar, arat.
- Exemple legale la început: `ascund`, `1`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="mahjong_tw"></a>
## Mahjong taiwanez

- **Id:** `mahjong_tw`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play mahjong_tw` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mahjong taiwanez pentru doi: 144 de piese cu 8 flori, mâini de 16 piese, câștigi cu 5 seturi și o pereche; florile se înlocuiesc din zidul mort și valorează tai.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, d5m, pung, kong, chi2m3m, mahjong, pas.
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="matador"></a>
## Matador (domino)

- **Id:** `matador`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play matador` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Domino în care jumătățile se pun cu suma 7 față de capăt (1+6, 2+5, 3+4); matadorii 0-0, 1-6, 2-5, 3-4 se pun oriunde. 7 piese fiecare, tragi dacă nu poți, câștigă cine termină primul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 6-1@s=1, trag, pas.
- Exemple legale la început: `1-0`, `3-0`
- La început sunt 7 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bagatelle"></a>
## Bagatelle

- **Id:** `bagatelle`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play bagatelle` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Masă înclinată cu cuie și 9 căsuțe: alegi culoarul de lansare (1–5) și puterea (1–3), apoi lași bila să curgă; loviturile tari merg mai drept, dar pot sări peste masă.
- Nouă bile fiecare; câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: tir 3 2, lansez.
- Exemple legale la început: `tir`, `1`
- La început sunt 45 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="latrunculi"></a>
## Latrunculi

- **Id:** `latrunculi`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play latrunculi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc roman de masă, reconstituire modernă pe 8×8: 16 piese fiecare, toate mută ca turnul, captură în clește de către piesa mutată; câștigi lăsând adversarul cu o piesă sau fără mutări.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a2-a5.
- Exemple legale la început: `a2-a3`, `a2-a4`
- La început sunt 32 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="salta"></a>
## Salta

- **Id:** `salta`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play salta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Jocul lui Büttgenbach (1899) pe 10×10: fără capturi; duci toate cele 15 piese pe pătratele de start ale adversarului.
- Pași diagonali (în orice direcție) sau salturi peste o piesă adversă înainte; săritura e obligatorie.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1-b2, c3xe5.
- Exemple legale la început: `a3-b4`, `c3-b4`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chaturaji"></a>
## Chaturaji

- **Id:** `chaturaji`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chaturaji` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Șahul indian cu patru armate, adaptat pentru doi (fiecare conduce două armate aliate).
- Arunci două zaruri, fiecare joacă un tip de piesă: 1/5 pion sau rege, 2 barcă, 3 cal, 4/6 elefant.
- Capturezi ambii regi adverși.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, a2a3, pas.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sjoelen"></a>
## Sjoelen

- **Id:** `sjoelen`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play sjoelen` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Șuflecatul olandez: 30 de discuri, 3 runde, le împingi în patru porți numerotate (2, 3, 4, 1); doar discurile ratate se aruncă din nou.
- Fiecare serie completă de patru porți valorează 20 de puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: stanga, centru, dreapta, atent, arunc.
- Exemple legale la început: `stanga`, `centru`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="nonogram"></a>
## Nonogram (Picross), duel

- **Id:** `nonogram`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play nonogram` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Doi jucători rezolvă aceeași imagine ascunsă 8×8 după indiciile rândurilor și coloanelor: revendici o căsuță necunoscută ca plină (a1) sau goală (a1x); apoi verific o dezvăluie.
- Corect +1, greșit −2.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a1, a1x, verific.
- Exemple legale la început: `a1`, `a1x`
- La început sunt 128 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="snooker"></a>
## Snooker

- **Id:** `snooker`
- **Categorie:** Clasic
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play snooker` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Snooker simplificat, o singură frame, cu fizică 2D a bilelor: bagi o roșie, apoi o culoare, alternativ; faulturile dau adversarului minimum 4.
- Lovitură: bila:buzunar:forța, de ex. r4:ne:2 (buzunare sv, nv, se, ne, ms, mn; forța 1–3).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `r1:sv:1`, `r1:sv:2`
- La început sunt 270 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="scattergories"></a>
## Scattergories

- **Id:** `scattergories`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play scattergories` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de cuvinte pentru doi: 3 runde, fiecare cu o literă și 4 categorii (animal, mâncare, țară, obiect).
- Amândoi aleg în secret un cuvânt din listă; cuvintele identice dau 0, cele unice 1–3 (cu cât mai rar, cu atât mai mult).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: caine, pas, arat.
- Exemple legale la început: `albina`, `arici`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="nhie"></a>
## Eu niciodată

- **Id:** `nhie`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play nhie` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de petrecere pentru doi, cu 5 degete fiecare și un profil ascuns de lucruri făcute.
- Spui o afirmație pe care n-ai făcut-o; celălalt răspunde sincer și coboară un deget dacă a făcut-o.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: eu-niciodata-5, raspund.
- Exemple legale la început: `eu-niciodata-1`, `eu-niciodata-2`
- La început sunt 11 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="spoons"></a>
## Spoons (lingurile)

- **Id:** `spoons`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play spoons` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de petrecere cu cărți pentru doi, cu o singură lingură: tragi o carte, arunci una, strângi patru la fel și iei lingura; cine pierde primește o literă din SPOONS. 12 runde, câștigă cine are mai puține litere.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: iau, dau K, lingura, mai-departe.
- Exemple legale la început: `iau`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="parcel"></a>
## Treci pachetul (Pass the Parcel)

- **Id:** `parcel`
- **Categorie:** Party
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play parcel` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de petrecere cu 8 straturi: pasezi pachetul (gratis) sau îl păstrezi (costă 1); apoi muzica se poate opri (15% plus 10% la fiecare acțiune) și cel care ține pachetul desface stratul și ia premiul (−3 până la 5).
- Câștigă cine are mai multe puncte după 8 straturi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pasez, pastrez, muzica.
- Exemple legale la început: `pasez`, `pastrez`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="klondike"></a>
## Solitaire (Klondike), duel

- **Id:** `klondike`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play klondike` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Klondike clasic (tragere de câte o carte), ca duel: fiecare are propria amestecare și face câte o mutare; câștigă primul care duce toate cele 52 de cărți pe fundații.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d, w>f, w>3, 3>f, 3>5:2.
- Exemple legale la început: `d`, `1>7:1`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="jass"></a>
## Jass (Schieber)

- **Id:** `jass`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play jass` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Jass elvețian pentru doi, pachet de 36 de cărți, 9 cărți fiecare, 4 mâini: cel care conduce alege atuul (atu h) sau îl dă mai departe (schiebe).
- Valetul de atu 20, noua 14, asul 11; ultima levată +5, match +100.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: atu c, schiebe, 10h, Js.
- Exemple legale la început: `atu`, `h`
- La început sunt 9 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="manille"></a>
## Manille

- **Id:** `manille`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play manille` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Manille franceză pentru doi, 32 de cărți, 8 fiecare, o carte întoarsă dă atuul, 4 mâini.
- Ordine 10 > A > K > Q > J > 9 > 8 > 7, puncte 5-4-3-2-1; răspunzi la culoare și, dacă poți, mai sus, altfel tai.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 10h, As, 7c.
- Exemple legale la început: `8h`, `9h`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="klaverjas"></a>
## Klaverjas

- **Id:** `klaverjas`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play klaverjas` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Klaverjas olandez (Amsterdam) pentru doi, 32 de cărți, 8 fiecare, o carte întoarsă dă atuul, 4 mâini.
- Valetul de atu 20, noua 14; ordine simplă A > 10 > K > Q > J; ultima levată +10, pit +100.
- Răspunzi la culoare, bați atuul dacă poți.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: Jh, 10s, Ac.
- Exemple legale la început: `8h`, `9h`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sueca"></a>
## Sueca

- **Id:** `sueca`
- **Categorie:** Cărți
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play sueca` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Sueca portugheză pentru doi, pachet de 40 de cărți (fără 8, 9, 10), 10 fiecare, o carte întoarsă dă atuul, 4 mâini.
- Ordine A > 7 > K > J > Q > 6 > 5 > 4 > 3 > 2, puncte A 11, 7 10, K 4, J 3, Q 2; răspunzi la culoare dacă poți.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: Ah, 7s, Qc.
- Exemple legale la început: `5h`, `6h`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="rps"></a>
## Piatră, hârtie, foarfece

- **Id:** `rps`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play rps` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Piatra sparge foarfeca, foarfeca taie hârtia, hârtia acoperă piatra.
- Amândoi aleg în secret, apoi arată; primul la 5 runde câștigate (maximum 15 runde, egalitățile se reiau).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: piatra, hartie, foarfeca, arat.
- Exemple legale la început: `piatra`, `hartie`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="coarda"></a>
## Sărit coarda (Jump rope)

- **Id:** `coarda`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play coarda` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- 8 runde, jucătorii alternează sărind și învârtind.
- Cel care învârte alege viteza (lent, mediu, rapid); săritorul alege trucuri (simplu 1, incrucisat 3, dublu 5, triplu 9 puncte) apoi sar, sau opresc ca să bage șirul; o greșeală îl pierde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: mediu, dublu, sar, opresc.
- Exemple legale la început: `simplu`, `incrucisat`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="capra"></a>
## Capra (Leapfrog)

- **Id:** `capra`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play capra` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cursă pe un șir de 10 căsuțe: 3 săritori fiecare, care merg spre partea opusă.
- Faci un pas pe căsuța vecină liberă sau sari peste un săritor alăturat pe căsuța liberă din spatele lui; câștigă primul cu toți cei trei pe ultimele căsuțe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3-4, 2-4, pas.
- Exemple legale la început: `2-4`, `3-4`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="kim"></a>
## Jocul lui Kim (Kim's Game)

- **Id:** `kim`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play kim` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de memorie, 5 runde cu tăvi de 6 până la 14 obiecte: privești tava (acopera), un obiect e scos în secret (scot), apoi ghicești după nume care lipsește; ghicit = 1 punct, greșit = rândul trece.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: acopera, scot, cheie, verific.
- Exemple legale la început: `acopera`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="slapjack"></a>
## Slapjack

- **Id:** `slapjack`
- **Categorie:** Copii
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play slapjack` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc de reflexe cu cărți adaptat pe ture: întorci cartea de sus pe teanc; când apare un valet celălalt poate da cu palma (65% reușită, altfel teancul îl ia cel care a întors valetul).
- Palma pe altceva costă o carte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: joc, palma, rezolv.
- Exemple legale la început: `joc`, `palma`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chohan"></a>
## Chō-han (Chō-han bakuchi)

- **Id:** `chohan`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chohan` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pariu japonez cu zaruri: două zaruri sub un pahar, pariezi cho (pară) sau han (impară) cu 5, 10 sau 20 de jetoane; celălalt primește partea opusă. 10 runde, 100 de jetoane fiecare, chemătorul alternează.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cho 10, han 5, agit.
- Exemple legale la început: `cho`, `5`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="balut"></a>
## Balut

- **Id:** `balut`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play balut` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc filipinez cu zaruri: 5 zaruri, până la 3 aruncări pe tură, 7 categorii completate o dată (patru, cinci, sase, scara 50, casa, sansa, balut 30). 7 ture fiecare; câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, tine 6 6, tine -, scor sase.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tali"></a>
## Knucklebones (Tali)

- **Id:** `tali`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tali` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Oscioarele antice: patru astragale cu fețele 1, 3, 4, 6;
- Venus (toate diferite) bate orice, Canis (patru de 1) pierde în fața oricărei alte aruncări, altfel contează suma. 10 runde, 20 de jetoane fiecare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: miza 2, arunc.
- Exemple legale la început: `miza`, `1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="crag"></a>
## Crag

- **Id:** `crag`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play crag` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc american cu 3 zaruri și 13 categorii, 13 ture fiecare: arunci, poți rearunca orice zaruri o dată, apoi scrii o categorie liberă (unu..sase, trei-la-fel 25, treisprezece 26, scări 20, crag 50).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, tine 6 6, tine -, scor crag.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bobing"></a>
## Bo Bing (Pua Tiong Chiu)

- **Id:** `bobing`
- **Categorie:** Zaruri
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play bobing` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Joc cu zaruri de Festivalul Lunii din Xiamen: șase zaruri într-un castron, premii (63 de prăjituri de lună) după combinație: Zhuangyuan cea mai bună, apoi Duitang, Sanhong, Sijin, Erju, Yixiu (după numărul de 4).
- Fiecare categorie are rezervă limitată; câștigă cine are mai multe puncte după 12 aruncări fiecare.
- Mutare: arunc.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vezi lista `?`
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ambuteiaj"></a>
## Ambuteiaj (duel de mașini pe tablă 6×6)

- **Id:** `ambuteiaj`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play ambuteiaj` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: mașini de 2–3 pătrate pe o tablă 6×6;
- A (J1) iese pe dreapta rândului 3, B (J2) pe stânga rândului 4.
- Mutezi o singură mașină pe tură (și pe cele ale adversarului): A+2, C-1.
- Câștigă cine își eliberează primul ținta; 120 de mutări: remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: A+2, C-1.
- Exemple legale la început: `A+1`, `D-1`
- La început sunt 11 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="banchiza"></a>
## Banchiza (pinguini pe sloiuri hexagonale)

- **Id:** `banchiza`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play banchiza` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 60 de plăci hexagonale cu 1–3 pești.
- Fiecare pune 4 pinguini pe plăci cu un singur pește (pune x,y), apoi muți un pinguin în linie dreaptă peste plăci libere (mut x,y>x2,y2); placa părăsită dispare, peștii sunt ai tăi.
- Cine nu poate muta pasează.
- Câștigă cine are mai mulți pești.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune x,y | mut x,y>x2,y2 | pas.
- Exemple legale la început: `pune`, `4,0`
- La început sunt 60 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="balans"></a>
## Balansul Pietrelor (greutăți pe o grindă)

- **Id:** `balans`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play balans` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: o grindă cu 13 poziții (−6..6) și greutățile 1–6 pentru fiecare.
- Pui pe rând o greutate pe o poziție liberă (pune g@p, ex. pune 4@-3).
- Momentul total (greutate × poziție) nu are voie să depășească 18; cine răstoarnă grinda pierde, 12 greutăți puse fără răsturnare = remiză.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune g@p.
- Exemple legale la început: `pune`, `1@-6`
- La început sunt 156 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pietre_puzzle"></a>
## Piese de Mozaic (puzzle cu forme în doi)

- **Id:** `pietre_puzzle`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pietre_puzzle` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tablă 5×6 cu 6 găuri și 6 piese comune de câte 4 celule (I, O, T, S, L, J).
- Pe rând pui o piesă nefolosită, în orice orientare (pune T@r,c,o).
- Cine nu mai poate pune nicio piesă pierde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune T@r,c,o.
- Exemple legale la început: `pune`, `I@1,1,0`
- La început sunt 358 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="firul"></a>
## Firul Luminii (plăci cu drumuri pe tablă 6×6)

- **Id:** `firul`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play firul` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: plăci cu 4 drumuri între 8 capete; pui placa în celula în care intră jetonul tău, rotită 0–3 sferturi (pune t,r; mâinile sunt deschise), apoi jetoanele aleargă pe drumuri.
- Cine iese de pe tablă e eliminat, ciocnirea îi elimină pe amândoi; câștigă ultimul rămas.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune t,r.
- Exemple legale la început: `pune`, `1,0`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="genial"></a>
## Mozaicul Priceput (domino hexagonal în șase culori)

- **Id:** `genial`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play genial` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tablă hexagonală cu 91 de celule; plăcuțe domino de două culori, mâinile sunt deschise.
- Pui plăcuța i cu prima culoare în (q,r) și a doua în direcția d (pune i@q,r,d).
- Scorul pe culoare = șirurile de aceeași culoare pornind din fiecare jumătate (max 18).
- Câștigă cine are cea mai bună culoare cea mai slabă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune i@q,r,d.
- Exemple legale la început: `pune`, `1@-5,1,0`
- La început sunt 2220 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pelerin"></a>
## Drumul Pelerinului (călătorie cu popasuri pe o potecă)

- **Id:** `pelerin`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pelerin` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: potecă cu 30 de popasuri, hanuri obligatorii la 6, 13, 20, 29.
- Muți cel din urmă: mut k pe un popas liber înainte, apoi efectul (priveliște, atelier: cumpar/treci, izvor, fermă, templu: dau 0-3, han: mananc/sar).
- Câștigă cine are mai multe puncte (3 monede = 1 punct).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: mut k, cumpar, treci, dau n, mananc, sar.
- Exemple legale la început: `mut`, `1`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="morcovi"></a>
## Cursa Morcovilor (cursă cu resursă de energie)

- **Id:** `morcovi`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play morcovi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: potecă cu 41 de câmpuri; avans k costă k(k+1)/2 morcovi.
- Câmpuri cu morcovi (avans k:iau / avans k:dau), salată la 12, 24, 36 (oprire obligatorie), inapoi (primești 10 morcovi pe câmp), odihna.
- Câștigă primul care ajunge la 40 având cel mult 20 de morcovi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: avans k[:iau|:dau], inapoi, odihna, treci.
- Exemple legale la început: `avans`, `1`
- La început sunt 24 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="rapide"></a>
## Rapidele (canoe pe râu spre cascadă)

- **Id:** `rapide`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play rapide` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: râu cu 22 de câmpuri și cascadă la 21; 3 canoe fiecare, mâini deschise de cărți-vâsle 1–4. vasla c,k mută canoea c cu k câmpuri și încarcă pietre; acostez c la malurile 12, 16, 20 pune încărcătura la loc sigur.
- Curentul împinge canoele cu 1 la finalul rundei; la cascadă pierzi tot.
- Câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: vasla c,k, acostez c.
- Exemple legale la început: `vasla`, `1,4`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="vapor"></a>
## Vaporul de pe Râu (cursă cu viteză și cărbune)

- **Id:** `vapor`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play vapor` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: râu cu 60 de câmpuri și 3 benzi; mutarea v+1,b2 schimbă viteza (±1 gratis, ±2 costă 1 cărbune; 1–4) și banda.
- Bancurile de nisip te opresc și costă cărbune, pasagerii dau puncte, primul la capăt +4, al doilea +2.
- Câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: v+1,b2.
- Exemple legale la început: `v+0,b0`, `v+0,b1`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="lumina"></a>
## Lumina Pădurii (copaci cu umbre pe tablă hexagonală)

- **Id:** `lumina`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play lumina` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 12 zile, soarele își schimbă direcția; copacii mai înalți fac umbră.
- Pui 2 copaci (pune q,r), apoi planta (1 lumină), creste, culeg (copac 3, 4 lumină, puncte după inel), pas.
- Lumina o primești la finalul zilei pentru copacii neumbriți.
- Câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune/planta/creste/culeg q,r, pas.
- Exemple legale la început: `pune`, `-3,0`
- La început sunt 36 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="zgarie_nori"></a>
## Cartierul Înalt (majorități de zgârie-nori)

- **Id:** `zgarie_nori`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play zgarie_nori` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 16 cartiere, cărți deschise de 4 pe rundă; etaj n adaugă un etaj în cartierul cărții, muta a>n îți mută etajul de sus dintr-un cartier vecin, arunc n.
- La finalul celor 3 runde cartierul dă punctele-etaje celui cu majoritatea (egal: cine e deasupra).
- Câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: etaj n, muta a>n, arunc n.
- Exemple legale la început: `etaj`, `9`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="turnuri_piatra"></a>
## Turnuri de Piatră (cavaleri pe castele în creștere)

- **Id:** `turnuri_piatra`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play turnuri_piatra` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală pe tablă 6×6: fiecare are 4 cavaleri; pune c3, mut a1>a2 (cel mult o treaptă mai sus), etaj c3 (treaptă pe celulă vecină).
- Câștigă cine urcă primul un cavaler pe înălțimea 4; după 80 de acțiuni, cine are cavalerii mai sus.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune c3, mut a1>a2, etaj c3.
- Exemple legale la început: `pune`, `a1`
- La început sunt 72 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="templele"></a>
## Templele din Junglă (explorare și temple pe grilă 5×5)

- **Id:** `templele`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play templele` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 3 exploratori, 3 acțiuni pe tură: mut a1>a2, templu c3 (+1 etaj), comoara c3, pas.
- De 3 ori, după 8 ture, fiecare templu dă etajele celui cu mai mulți exploratori pe celulă sau în vecinătate.
- Câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: mut a1>a2, templu c3, comoara c3, pas.
- Exemple legale la început: `templu`, `c3`
- La început sunt 11 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="caravana"></a>
## Caravana de Mirodenii (negoț între doi neguțători)

- **Id:** `caravana`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play caravana` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 6 mărfuri și cămile, piață de 5 cărți, mâini deschise (max 7).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ia M, camile, schimb A:B, vand M n.
- Exemple legale la început: `ia`, `argint`
- La început sunt 22 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="poteca"></a>
## Poteca Pietrelor (cinci poteci de culori)

- **Id:** `poteca`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play poteca` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: cărți de 5 culori (1–10), mâini deschise de 8.
- Joci (joc C n pe poteca culorii dacă e mai mare decât ultima, sau arunc C n), apoi tragi (trage | ia C).
- Fiecare potecă valorează −4, −3, −1, 2, 6, 11, 17, 24, 32, 41 după număr de cărți.
- Câștigă scorul mai mare când pachetul e gol.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: joc C n, arunc C n, trage, ia C.
- Exemple legale la început: `joc`, `r`
- La început sunt 42 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="evadare"></a>
## Evadarea din Temniță (pirați care fug printr-un tunel)

- **Id:** `evadare`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play evadare` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tunel cu 36 de pătrate (6 simboluri) și barcă; fiecare are 6 pirați și mâna deschisă de 6 cărți-simbol. 3 acțiuni pe tură: avans p,S (cel mai apropiat pătrat liber cu simbolul S), retras p (trage o carte), gata.
- Câștigă primul care îmbarcă toți pirații.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: avans p,S, retras p, gata.
- Exemple legale la început: `avans`, `1,k`
- La început sunt 36 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cetate_intrigi"></a>
## Intrigile Cetății (alegere de roluri și construcție de cartiere)

- **Id:** `cetate_intrigi`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cetate_intrigi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală pentru 2: 6 roluri (Asasin, Hoț, Rege, Negustor, Arhitect, Căpitan), fiecare ia câte 2 pe rundă (alege r).
- Cu rolul tău: tinta r, aur sau carte, construiesc i (3 pentru Arhitect) sau gata, distrug i / nu (Căpitan).
- Mâini deschise.
- La 7 cartiere se termină; scor = costuri + 3 culori complete + 2 primul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: alege r, tinta r, aur, carte, construiesc i, gata, distrug i, nu.
- Exemple legale la început: `alege`, `1`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="menajerie"></a>
## Menajeria (cărucioare cu animale și țarcuri)

- **Id:** `menajerie`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play menajerie` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 5 specii, 3 cărucioare de 3 locuri. trage (apoi pune k) sau ia k (luat căruciorul în țarcurile tale: capacități 3, 4, 5, o specie în fiecare) și ieși din rundă.
- Scor: pătratul numărului de animale per țarc (+2 dacă e plin).
- Câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, pune k, ia k.
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tinut_zane"></a>
## Ținutul Zânelor (călătorie cu cărți de transport)

- **Id:** `tinut_zane`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tinut_zane` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 10 orașe în cerc cu scurtături, drumuri cu teren și cost 1–2. ia t (carte din piața de 4) sau mut X plătind cărțile potrivite; primul care ajunge într-un oraș ia valoarea lui (1–3).
- Se termină la 8 orașe revendicate sau după 80 de mutări.
- Câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ia t, mut X, descarto t.
- Exemple legale la început: `mut`, `2`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="postalioni"></a>
## Poștalionii (rute de livrare între orașe)

- **Id:** `postalioni`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play postalioni` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 12 orașe în cerc cu scurtături, mâini deschise. joc X (extinde ruta la un capăt vecin), nou X, livrez (rută ≥3: 3, 6, 10, 15, 21 puncte), apoi ia k (din 4 cărți deschise) sau trage.
- Se termină când pachetul e gol.
- Câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: joc X, nou X, livrez, ia k, trage.
- Exemple legale la început: `joc`, `1`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cazanul"></a>
## Cazanul Vrăjit (pariu cu pungi de ingrediente)

- **Id:** `cazanul`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cazanul` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tragi ingrediente din punga ta (scot, forțat), apoi mai sau opresc.
- Albele (a1, a2, a3) explodează cazanul peste 7.
- Fără explozie: monede = valoarea și 1 punct la 4; cu explozie alegi bani sau puncte.
- Cumperi ingrediente (cumpar a3/r2/z3, gata). 6 runde; câștigă cine are mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: scot, mai, opresc, bani, puncte, cumpar X, gata.
- Exemple legale la început: `scot`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="santier"></a>
## Șantierul Piramidei (pietre pe corăbii spre șantiere)

- **Id:** `santier`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play santier` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 5 runde, 4 corăbii și 4 șantiere (piramidă, obelisc, templu, piață). iau (+2 pietre), incarc s, navighez s,k, pas.
- Pietrele se descarcă în ordinea locurilor: piramida dă 1, 2, 3, ... puncte, obeliscul n×n/2 la final, templul pe perechi, piața 1 punct și 1 piatră.
- Câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: iau, incarc s, navighez s,k, pas.
- Exemple legale la început: `iau`, `incarc`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="marele_bazar"></a>
## Marele Bazar (neguțători pe o grilă de locații)

- **Id:** `marele_bazar`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play marele_bazar` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: grilă 4×4 de locații (piețe, bănci, ateliere, palate, bijutieri, fântâni). mut a1>b2 (1–2 pași ortogonali; pe adversar plătești 2 lire), apoi se aplică automat acțiunea locației.
- Câștigă primul cu 5 rubine; după 100 de mutări, mai multe rubine și lire.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: mut a1>b2.
- Exemple legale la început: `mut`, `a1>b1`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="curierii_vraji"></a>
## Curierii Vrăjitoarelor (roluri curajoase sau fricoase)

- **Id:** `curierii_vraji`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play curierii_vraji` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală pe 8 locuri: 4 roluri (muta, colecteaza, livreaza, odihna), fiecare în mod f (fricos, sigur) sau c (curajos, mare).
- Un rol curajos rămâne în așteptare: dacă adversarul joacă același rol, curajosul se anulează. muta:f>n, rol:f|c. 4 runde; câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: muta:f>n, colecteaza:c, livreaza:f, odihna:c.
- Exemple legale la început: `muta:f>2`, `muta:f>5`
- La început sunt 16 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="vamesul"></a>
## Vameșul din Poartă (bluf cu geanta de negustor)

- **Id:** `vamesul`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play vamesul` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 6 runde, rolurile alternează.
- Negustorul pune 1–4 cărți în geantă și declară un fel legal (sac a,b,c,d>t); vameșul alege las sau inspectez, apoi verific.
- Cinstit și inspectat: vameșul plătește; altfel penalități, contrabanda valorează 6.
- Informație ascunsă: mâna și geanta.
- Câștigă monede + marfă mai multe.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: sac a,b,c,d>t, las, inspectez, verific.
- Exemple legale la început: `sac`, `0,0,1,0>0`
- La început sunt 90 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="agenti"></a>
## Agenții Tăcuți (agenți pe o stradă cu valori secrete)

- **Id:** `agenti`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play agenti` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 6 agenți (A–F) pe o stradă de 10 pași; fiecare jucător are în secret valori distincte 0–5.
- Pe rând muți orice agent cu 1–3 (mut X,k).
- După 24 de mutări scorul = Σ valoarea ta × poziția.
- Informație ascunsă: vezi doar valorile tale.
- Câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: mut X,k.
- Exemple legale la început: `mut`, `A,1`
- La început sunt 36 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tablou"></a>
## Schița Ascunsă (indicii dintr-un tablou)

- **Id:** `tablou`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tablou` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 6 cărți-tablou cu câte 4 simboluri (A–J); una e ținta, cunoscută doar de schițator.
- Schițatorul spune 1–3 simboluri adevărate (pun S, gata); celălalt ghicește (ghic k, apoi verific).
- Corect: ghicitor +2, schițator +1; greșit: schițator +2. 6 runde.
- Informație ascunsă: ținta.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pun S, gata, ghic k, verific.
- Exemple legale la început: `pun`, `F`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="detectiv"></a>
## Orașul Detectivilor (urmărirea suspecților pe hartă)

- **Id:** `detectiv`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play detectiv` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: grilă 6×6, 6 suspecți, 8 pași de timp; crima la pasul 5 în celula X. arata s,t dezvăluie public poziția suspectului s la pasul t (apoi afisez), acuz s acuză (apoi verific): corect câștigi, greșit pierzi.
- Suspecții care nu puteau ajunge în X sunt eliminați.
- Informație ascunsă: traseele.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arata s,t, afisez, acuz s, verific.
- Exemple legale la început: `arata`, `1,0`
- La început sunt 108 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tinuturi_negociate"></a>
## Ținuturi Negociate (prețul tău, alegerea mea)

- **Id:** `tinuturi_negociate`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play tinuturi_negociate` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 8 runde.
- Se dezvăluie 3 plăcuțe de teren (scot); vânzătorul pune prețuri (pret a,b), cumpărătorul alege una (ia k) și o așază (pune r,c), apoi vânzătorul ia una din rest.
- Peisaj 5×5; grupurile conexe valorează n(n+1)/2 × valoarea terenului, plus monedele.
- Câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: scot, pret a,b, ia k, pune r,c.
- Exemple legale la început: `scot`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="peisaj"></a>
## Peisaj Liniștit (plăcuțe hexagonale cu margini de teren)

- **Id:** `peisaj`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play peisaj` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: plăcuțe hexagonale cu 6 margini de teren (p pădure, c câmp, a apă, s sat).
- Pe rând așezi plăcuța curentă (se vede și următoarea) lipită de cele puse: pune q,r,rot.
- Fiecare margine potrivită +1, nepotrivită −1. 36 de plăcuțe; câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune q,r,rot.
- Exemple legale la început: `pune`, `-1,0,0`
- La început sunt 72 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="cariera"></a>
## Cariera Visată (formula succesului și zarul)

- **Id:** `cariera`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play cariera` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: fiecare alege public o formulă (tinta 1–4) de faimă / avere / fericire; apoi zar (forțat) și munca f|a|h (+d, dublu cu cariera potrivită) sau schimb A|N|F (alegi cariera).
- Câștigă primul care atinge toate țintele; după 160 de ture, cine e mai aproape.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: tinta k, zar, munca f|a|h, schimb A|N|F.
- Exemple legale la început: `tinta`, `1`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="salariu"></a>
## Ziua de Salariu (cursă de buget pe trei luni)

- **Id:** `salariu`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play salariu` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 3 luni × 30 de zile. zar (forțat) te mută; poștă (−6..+4), ofertă (cumpar 5 monede, vinzi cu 9 la o zi de vânzare, sau las), salariu +8 la fiecare 30 de zile.
- Câștigă cine are mai mulți bani la final (loturile nevândute valorează 3).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: zar, cumpar, las.
- Exemple legale la început: `zar`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="plapuma"></a>
## Plapuma (petice, timp și nasturi)

- **Id:** `plapuma`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play plapuma` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: plapumă 7×7, 16 petice în cerc cu cost în nasturi, timp și venit. ia i@r,c,o (unul din 3 petice după marcaj) sau avans (cu un loc înaintea adversarului, primești nasturi).
- Mută cine e în urmă.
- Pista are 40 de locuri; venit la 8, 16, 24, 32.
- Scor: nasturi + 2/celulă acoperită − 2/celulă goală.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ia i@r,c,o, avans.
- Exemple legale la început: `ia`, `1@1,1,0`
- La început sunt 729 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pisici"></a>
## Pisicile din Barcă (pisici-poliomino pe barcă)

- **Id:** `pisici`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pisici` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: 5 runde; se dezvăluie 4 pisici (scot), se aleg pe rând (ia k) și se așază pe barca 5×6 (pune r,c,o) sau se aruncă (arunc, −1).
- Scor: celule acoperite + valoarea pisicilor + 3 pe rând plin.
- Câștigă scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: scot, ia k, pune r,c,o, arunc.
- Exemple legale la început: `scot`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="triunghiuri"></a>
## Triunghiurile Numerotate (domino triunghiular)

- **Id:** `triunghiuri`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play triunghiuri` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală, mâini deschise: 35 de plăcuțe triunghiulare cu numere 0–4 în colțuri. pune i@x,y,s,k pe o celulă lipită de cele puse (s = u sau d, k rotația), cu colțurile comune egale; scor = suma plăcuței. trage / pas dacă nu poți.
- Cine rămâne fără plăcuțe ia 10 + restul adversarului.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune i@x,y,s,k, trage, pas.
- Exemple legale la început: `pune`, `1@0,0,u,0`
- La început sunt 14 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="banane"></a>
## Banane-Cuvinte (cursă de cuvinte cu literele din mână)

- **Id:** `banane`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play banane` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: fiecare are 10 litere deschise; cuvant xyz (≥3 litere, din dicționar, folosește literele din mână, apoi tragi una), arunc l (înapoi în sac, tragi 3), pas.
- Câștigă primul fără litere; după 80 de mutări, mai puține litere.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cuvant xyz, arunc l, pas.
- Exemple legale la început: `cuvant`, `acolo`
- La început sunt 146 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="turnuri_litere"></a>
## Turnuri de Litere (cuvinte încrucișate cu litere suprapuse)

- **Id:** `turnuri_litere`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play turnuri_litere` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tablă 6×6 cu stive de până la 3 litere, mâna de 7 litere ascunsă. pune L@r,c (o literă, pe celulă goală sau peste stivă, cu altă literă); toate șirurile de ≥2 celule prin ea trebuie să fie cuvinte din dicționar.
- Scor: suma înălțimilor cuvintelor. pas.
- Informație ascunsă: mâna.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune L@r,c, pas.
- Exemple legale la început: `pune`, `r@1,1`
- La început sunt 432 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="lanturi"></a>
## Lanțuri Hoteliere (fuziuni și acțiuni pe o tablă mică)

- **Id:** `lanturi`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play lanturi` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: tablă 5×6, 3 lanțuri A, B, C, piață de 4 plăcuțe. pune b3 (fără vecini: izolată; lângă izolate: fond A; lângă lanț: îl crește; între două: fuziune, lanțul mic dispare cu bonus), apoi cumperi până la 3 acțiuni (compro A, gata).
- La final se vând acțiunile și se dau bonusuri.
- Câștigă banii mai mulți.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: pune b3, fond A, compro A, gata.
- Exemple legale la început: `pune`, `c1`
- La început sunt 8 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bambuzeria"></a>
## Bambuzeria (grădinar, panda și obiective de bambus)

- **Id:** `bambuzeria`
- **Categorie:** Familie
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play bambuzeria` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Variantă originală: grădină 5×5 în trei culori.
- O acțiune pe tură: gradinar c3 (crește parcela și vecinii de aceeași culoare), panda c3 (mănâncă un segment), ploaie c3 (+1; maxim 2).
- Obiectivele (3 deschise) se completează automat.
- Primul la 20 de puncte câștigă; după 60 de acțiuni, scorul mai mare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: gradinar c3, panda c3, ploaie c3.
- Exemple legale la început: `gradinar`, `c1`
- La început sunt 66 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="hnefatafl"></a>
## Hnefatafl

- **Id:** `hnefatafl`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play hnefatafl` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Hnefatafl 11×11 (varianta Copenhaga): atacatorii (J1) mută primii și vor să prindă regele apărătorilor (J2); regele câștigă ajungând într-un colț.
- Piesele mută ca turnul, capturează prin încadrare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d1-d4.
- Exemple legale la început: `d1-d2`, `d1-d3`
- La început sunt 116 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="brandubh"></a>
## Brandubh

- **Id:** `brandubh`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play brandubh` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Brandubh 7×7 (Irlanda): 8 atacatori contra 4 apărători și rege; regele câștigă ajungând într-un colț sau pe tron cu apărare liberă, e capturat prin încadrare.
- Piesele mută ca turnul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d1-d4.
- Exemple legale la început: `d1-c1`, `d1-b1`
- La început sunt 40 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pente"></a>
## Pente

- **Id:** `pente`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pente` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pente (SUA) pe tabla 19×19: aliniezi 5 pietre sau capturezi 5 perechi (încadrezi exact două pietre adverse).
- Prima mutare a lui J1 trebuie să fie în centru, iar a doua piatră a lui J1 la cel puțin 3 intersecții de centru.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: k10.
- Exemple legale la început: `k10`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="havannah"></a>
## Havannah

- **Id:** `havannah`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play havannah` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Havannah (Marea Britanie) pe tabla hexagonală cu latura 8: câștigi cu un inel, un pod (legi două colțuri) sau o furcă (legi trei laturi).
- Poți folosi «swap» după prima mutare.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: h8, swap.
- Exemple legale la început: `h1`, `i1`
- La început sunt 169 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chinese_checkers"></a>
## Table chinezească (Chinese Checkers)

- **Id:** `chinese_checkers`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chinese_checkers` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tablă stelată cu 121 de găuri, 10 piese pe jucător: pas pe o gaură vecină sau salturi înlănțuite peste piese; câștigi umplând triunghiul opus.
- După 300 de mutări se compară progresul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d4-e5 (fără traseul sărit).
- Exemple legale la început: `c1-e5`, `c1-e7`
- La început sunt 14 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="goose"></a>
## Jocul gâștei (Game of the Goose)

- **Id:** `goose`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play goose` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cărare de 63 de căsuțe, două zaruri: gâște (mai mergi o dată), pod, han, fântână, labirint, închisoare, moarte; ajungi exact pe 63.
- Fântâna și închisoarea costă 3 ture.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, astept.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="keno"></a>
## Keno

- **Id:** `keno`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play keno` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Fiecare alege 5 numere din 80, apoi se extrag 20; plata (a noastră): 2 nimeriri = 1, 3 = 3, 4 = 20, 5 = 400 de puncte. 5 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: n17, random, extrag.
- Exemple legale la început: `n1`, `n2`
- La început sunt 77 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="sichuan_mahjong"></a>
## Mahjong Sichuan

- **Id:** `sichuan_mahjong`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play sichuan_mahjong` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mahjong Sichuan pentru 2 jucători: 108 piese (fără onoruri), fiecare declară o culoare lipsă (lipsa m|s|p) și o aruncă întâi; fără chi, doar pung, kong și câștig; jocul se oprește la primul câștig.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: lipsa m, trage, d5m, pung, kong, mahjong, pas.
- Exemple legale la început: `lipsa`, `m`
- La început sunt 6 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="long_nard"></a>
## Long Nard (Narde)

- **Id:** `long_nard`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play long_nard` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Narde lungi: 15 piese pe jucător, 24 de puncte pe un singur cerc, fără lovire (punct blocat de adversar), o singură piesă din cap pe tură; câștigă cine scoate primul cele 15 piese.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 13/17, 24/off.
- Exemple legale la început: `1/4`, `1/6`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="shove_ha_penny"></a>
## Shove ha'penny

- **Id:** `shove_ha_penny`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play shove_ha_penny` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Împingi 5 monede pe o tablă cu 9 paturi; un pat se închide după 3 monede; câștigi când ai închis toate cele 9 paturi.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: p4, f7 (țintă + tip), apoi lansez.
- Exemple legale la început: `p1`, `f1`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="rithmomachia"></a>
## Rithmomachia

- **Id:** `rithmomachia`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play rithmomachia` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Rithmomachia (varianta simplificată): 8×16, piese rotunde, triunghiuri și pătrate cu valori; capturi prin întâlnire, asalt și asediu; câștigi cu 8 capturi sau prinzând regele-piramidă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: a3-a5, a3xa5.
- Exemple legale la început: `a3-b4`, `b3-a4`
- La început sunt 14 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pong_hau_kiki"></a>
## Pong hau k'i

- **Id:** `pong_hau_kiki`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pong_hau_kiki` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pong hau k'i (China): 5 puncte, 2 piese pe jucător; muți pe un punct vecin liber; câștigi blocând adversarul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 1-3.
- Exemple legale la început: `1-3`, `2-3`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="patolli"></a>
## Patolli

- **Id:** `patolli`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play patolli` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Patolli (reconstituire): cale în cruce de 52 de pătrate, 6 piese pe jucător, 5 boabe (numărul fețelor marcate = pași); scoți piesele exact la sfârșit.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: piesa 0, piesa 17.
- Exemple legale la început: `piesa`, `0`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="duodecim_scripta"></a>
## Duodecim scripta

- **Id:** `duodecim_scripta`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play duodecim_scripta` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Duodecim scripta (reconstituire modernă): 36 de puncte pe o singură cale, 15 piese pe jucător, 3 zaruri cu pași separați; un punct cu două piese e blocat; piesele singure lovite se întorc pe bară.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: bar/3, 13/17, 36/off.
- Exemple legale la început: `bar/3`, `bar/5`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="jigsaw"></a>
## Puzzle cu piese (Jigsaw)

- **Id:** `jigsaw`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play jigsaw` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Puzzle 4×4 în întrecere: pe rând așezi o piesă care se potrivește cu vecinele (margini plate pe cadru); punctele = 1 + vecinii îmbinați, ultima piesă +3.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: p7r2c3.
- Exemple legale la început: `p0r1c1`, `p3r1c4`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="guobiao"></a>
## Mahjong oficial chinez (Guobiao)

- **Id:** `guobiao`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play guobiao` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Mahjong Guobiao pentru 2 jucători: 144 de piese cu flori; mâna câștigătoare are cel puțin 8 puncte (subset documentat al combinațiilor oficiale) plus florile.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, d5m, pung, kong, chi2m3m, mahjong, pas.
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="boggle"></a>
## Boggle

- **Id:** `boggle`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play boggle` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Boggle: tablă 4×4 de litere; pe rând scrii câte un cuvânt de cel puțin 3 litere format din cuburi vecine, valid în dicționar și încă negăsit; punctele cresc cu lungimea.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: casa, pas.
- Exemple legale la început: `pas`, `alti`
- La început sunt 36 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="kings"></a>
## Kings (Ring of Fire)

- **Id:** `kings`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play kings` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Kings fără alcool: tragi pe rând cărți cu reguli pe rang; înghițiturile sunt jetoane de penalizare; al patrulea rege te face să iei tot paharul.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trag, scut, atac.
- Exemple legale la început: `trag`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="celebrity"></a>
## Celebrity (Fishbowl)

- **Id:** `celebrity`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play celebrity` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Celebrity: vas cu 12 nume, trei runde (descrii, mimezi, un singur cuvânt); coechipierul simulat ghicește; câștigă cine ghicește mai multe.
- Numele de sus se vede doar de cel care îl dă.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: dau, sar, ghicesc.
- Exemple legale la început: `dau`, `sar`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="pin_the_tail"></a>
## Prinde coada măgarului

- **Id:** `pin_the_tail`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play pin_the_tail` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Pe un câmp 7×5 alegi o țintă, te învârți (eroare aleatoare) și prinzi coada: 5 puncte exact, 3, 2, 1 după distanță; 5 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: c3, apoi invart.
- Exemple legale la început: `a1`, `b1`
- La început sunt 35 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="war"></a>
## Război (War)

- **Id:** `war`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play war` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Război: fiecare întoarce cartea de sus, cea mai mare ia ambele; la egalitate «război» (cărți acoperite și una descoperită).
- Câștigă cine ia toate cărțile (limită de mutări, apoi mai multe cărți).

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: joc.
- Exemple legale la început: `joc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="tressette"></a>
## Tressette (Italia)

- **Id:** `tressette`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play tressette` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Tressette (40 de cărți italiene, 2 jucători): obligatoriu să răspunzi la culoare, ordinea 3 2 A R C F 7 6 5 4, fără atu; puncte A 1, 3/2/R/C/F ⅓; ultima levată +1.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 3d, Rs, Cc.
- Exemple legale la început: `4d`, `Cd`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="piquet"></a>
## Piquet (Franța)

- **Id:** `piquet`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play piquet` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Piquet (32 de cărți): schimb de cărți, declarații (punct, secvențe, seturi) și 12 levate; variantă proprie de punctare a jocului.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: d7h, gata, Ah, urmatoarea.
- Exemple legale la început: `d9h`, `d8h`
- La început sunt 12 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="batak"></a>
## Batak (Turcia)

- **Id:** `batak`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play batak` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Batak adaptat la 2 jucători (13 cărți fiecare): licitezi levatele, alegi atu (pică implicit), cine licitează trebuie să facă cel puțin cât a promis.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: 6, pas, atu h, Ah.
- Exemple legale la început: `5`, `6`
- La început sunt 10 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="yaniv"></a>
## Yaniv (Israel)

- **Id:** `yaniv`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play yaniv` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Yaniv: arunci o carte, un set sau o secvență, apoi tragi din stoc sau talon; strigi «yaniv» cu cel mult 7 puncte în mână; dacă adversarul are mai puțin sau egal ești «asaf».

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: yaniv, 5h,5s-stoc, 5h-ia:7c.
- Exemple legale la început: `6c-stoc`, `6c-ia:Js`
- La început sunt 22 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="ascunselea"></a>
## Ascunselea (Hide-and-seek)

- **Id:** `ascunselea`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** da; „la masă” (`--you both`) ascunde informația celui care nu e la mutare
- **Pornire:** `polytabletop play ascunselea` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Ascunselea pe o curte 3×3, 4 runde: ascunzătorul alege în secret o ascunzătoare, căutătorul are 3 încercări cu indiciu cald/rece; mai repede = mai multe puncte.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: ascunde b2, cauta b2.
- Exemple legale la început: `ascunde`, `a1`
- La început sunt 18 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="de_a_prinselea"></a>
## De-a prinselea (Tag)

- **Id:** `de_a_prinselea`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play de_a_prinselea` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Urmărire pe un câmp 7×7: prinzătorul care ajunge adiacent schimbă rolurile (și stă o mutare); cel care nu e prinzător la sfârșitul rundei primește 1 punct; 15 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: n, s, e, v, ne, nv, se, sv, stai.
- Exemple legale la început: `stai`, `s`
- La început sunt 4 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="elasticul"></a>
## Elasticul

- **Id:** `elasticul`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play elasticul` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Elasticul urcă în 7 niveluri; la fiecare nivel alegi salt ușor sau greu (șanse și puncte diferite), apoi sari; cine greșește e eliminat.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: usor, greu, sari.
- Exemple legale la început: `usor`, `greu`
- La început sunt 2 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="oina"></a>
## Oina

- **Id:** `oina`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play oina` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Oina stilizată: 6 reprize cu 3 arși; bătăușul alege lovitura, apărătorul poziția, apoi se aruncă; mingea prinsă în zona aleasă arde bătăușul, altfel aleargă și marchează 1/2/4.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: usoara, medie, puternica, aproape, mijloc, departe, arunca.
- Exemple legale la început: `usoara`, `medie`
- La început sunt 3 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="loto_imagini"></a>
## Loto cu imagini (Picture lotto)

- **Id:** `loto_imagini`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play loto_imagini` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Cartonașe 3×3 cu imagini din 24; tragi pe rând din sac, acoperi imaginea de pe cartonaș sau o lași; primul cu cartonașul plin strigă loto.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: trage, acopera, pas.
- Exemple legale la început: `trage`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="bau_cua"></a>
## Bầu cua (dovleac, crab, pește)

- **Id:** `bau_cua`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play bau_cua` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Bầu cua: trei zaruri cu cerb, dovleac, cocoș, pește, crab, creveți; parierul pune mize pe simboluri, bancherul aruncă; plată 1:1 per zar. 12 runde cu roluri alternante.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: cerb 5, peste 10, gata, arunc.
- Exemple legale la început: `cerb`, `5`
- La început sunt 25 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="yacht"></a>
## Yacht

- **Id:** `yacht`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play yacht` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Yacht: 12 runde, până la 3 aruncări cu 5 zaruri, scrii scorul în 12 categorii (fiecare o dată); yacht = cinci egale = 50.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, tine 3 3 5, scor casa.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="chicago"></a>
## Chicago (zaruri)

- **Id:** `chicago`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play chicago` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Chicago: 11 runde cu ținta 2…12; aruncând două zaruri cu suma egală cu ținta iei atâtea puncte; opțional păstrezi un zar și rearunci.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: arunc, tin 1, tin 2, gata, rearunc.
- Exemple legale la început: `arunc`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="zara"></a>
## Zara (joc)

- **Id:** `zara`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play zara` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Zara (reconstituire): fiecare strigă un număr 4–17, se aruncă 3 zaruri; cine a strigat suma ia atâtea puncte; 10 runde.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: n13, arunc.
- Exemple legale la început: `n4`, `n5`
- La început sunt 14 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

<a id="daldos"></a>
## Daldøs

- **Id:** `daldos`
- **Categorie:** ?
- **Jucători:** 2 (tu contra calculatorului; `--you both` = 2 oameni la masă, `--you none` = AI contra AI)
- **Informație ascunsă:** nu
- **Pornire:** `polytabletop play daldos` (`--you two` = calculatorul începe; `--seed N` = AI reproductibil)

**Reguli pe scurt**

- Daldøs (reconstituire): tablă-barcă cu 3 rânduri, 12 piese, două zaruri cu 4 fețe; un 1 «dalează» o piesă, apoi piesele parcurg circuitul și scot piesele adverse.

**Mutări** (din `?` pe un joc nou, `legal_moves`)

- Sintaxă din regulile jocului: dal, h5+3.
- Exemple legale la început: `dal`
- La început sunt 1 cuvinte în lista `?` (mutările cu spații se numără pe cuvinte).

