//! PolyTabletop games.
//!
//! Clean-room policy: every game here is written only from its publicly known
//! rules. No code, text, art or data of any commercial product is used.
//! Trademarked product names (Monopoly, Othello, Connect Four, …) are never
//! used for our own games: we use the generic names ("Reversi",
//! "Patru în linie", …).

pub mod aadupuli;
pub mod amazoane;
pub mod apus;
pub mod arhipelag;
pub mod artificii;
pub mod ashtachamma;
pub mod aventura;
pub mod aventura_pachete;
pub mod ayo;
pub mod backgammon;
pub mod baloot;
pub mod bao;
pub mod barbut;
pub mod battleship;
pub mod belote;
pub mod bigtwo;
pub mod bile;
pub mod bingo;
pub mod blackjack;
pub mod bresla;
pub mod bridge;
pub mod briscola;
pub mod bufet;
pub mod callbreak;
pub mod canasta;
pub mod capcana;
pub mod cards;
pub mod cards52;
pub mod cardsb_util;
pub mod carrom;
pub mod casino;
pub mod catalog;
pub mod catun;
pub mod ceasornic;
pub mod ceata;
pub mod cenusii;
pub mod cetatuia;
pub mod chaupar;
pub mod chess;
pub mod chess_san;
pub mod codru;
pub mod coloane;
pub mod conac;
pub mod congkak;
pub mod connect4;
pub mod consiliul;
pub mod coridor;
pub mod coroana;
pub mod courtpiece;
pub mod cribbage;
pub mod cripta;
pub mod cristale;
pub mod culori;
pub mod cunoastere;
pub mod cunoastere_bank;
pub mod cupe;
pub mod curieri;
pub mod cursa_probelor;
pub mod cuvantunic;
pub mod cuvantunic_date;
pub mod daifugo;
pub mod dama;
pub mod dara;
pub mod dictionar;
pub mod dictionar_date;
pub mod domino;
pub mod dou_dizhu;
pub mod draughts;
pub mod drum;
pub mod durak;
pub mod epoci;
pub mod euchre;
pub mod expeditii;
pub mod faianta;
pub mod fanorona;
pub mod farul;
pub mod fitil;
pub mod focare;
pub mod fulger;
pub mod furnale;
pub mod ganjifa;
pub mod go;
pub mod gomoku;
pub mod gospodaria;
pub mod gridiron;
pub mod gura;
pub mod gura_date;
pub mod hanafuda;
pub mod hasami;
pub mod hex;
pub mod hokm;
pub mod hotar;
pub mod hruba;
pub mod hus;
pub mod hwatu;
pub mod i18n;
pub mod impletitura;
pub mod impostor;
pub mod impostor_date;
pub mod inimi;
pub mod insula;
pub mod itinerar;
pub mod izolare;
pub mod janggi;
pub mod karuta;
pub mod konane;
pub mod linegraph;
pub mod linii;
pub mod litere;
pub mod ludo;
pub mod lupii;
pub mod macao;
pub mod magistrale;
pub mod mahjong;
pub mod makruk;
pub mod mancala;
pub mod mancala4;
pub mod mangala;
pub mod mastermind;
pub mod memory;
pub mod miniaturi;
pub mod morabaraba;
pub mod morris;
pub mod mosia;
pub mod mosii;
pub mod mozaic;
pub mod nave;
pub mod nim;
pub mod okey;
pub mod oprite;
pub mod oprite_date;
pub mod orasel;
pub mod ordine;
pub mod oware;
pub mod p2_util;
pub mod pachisi;
pub mod pai_gow;
pub mod pallanguzhi;
pub mod pasur;
pub mod patru;
pub mod patrula;
pub mod pescuit;
pub mod pica;
pub mod pisti;
pub mod placute;
pub mod poieni;
pub mod poker;
pub mod poliedre;
pub mod popa;
pub mod portret;
pub mod povestitor;
pub mod povestitor_date;
pub mod prajituri;
pub mod prinde;
pub mod puncte;
pub mod reactii;
pub mod reverdire;
pub mod reversi;
pub mod rezervatia;
pub mod riichi;
pub mod roade;
pub mod roata;
pub mod ronda;
pub mod rotire;
pub mod rummy;
pub mod salina;
pub mod santinela;
pub mod schita;
pub mod schita_date;
pub mod scopa;
pub mod seega;
pub mod senet;
pub mod septica;
pub mod shogi;
pub mod shologuti;
pub mod sicbo;
pub mod sideral;
pub mod sixtysix;
pub mod skat;
pub mod snakes;
pub mod solzi;
pub mod spionii;
pub mod spionii_date;
pub mod strajile;
pub mod strapungere;
pub mod sugoroku;
pub mod sungka;
pub mod surakarta;
pub mod tab;
pub mod tablanet;
pub mod taramuri;
pub mod tarneeb;
pub mod tarot;
pub mod tcg;
pub mod tcg_cerneala;
pub mod tcg_evo;
pub mod tcg_lant;
pub mod tcg_zone;
pub mod teen_patti;
pub mod tienlen;
pub mod tigri;
pub mod tinuturi;
pub mod tongits;
pub mod treicai;
pub mod treicai_date;
pub mod treizeci1;
pub mod trickkit;
pub mod tripierd;
pub mod trix;
pub mod truco;
pub mod turnul;
pub mod umbra;
pub mod valea;
pub mod w4_comun;
pub mod w4_ghici;
pub mod w5_util;
pub mod w7_util;
pub mod whist;
pub mod whot;
pub mod xiangqi;
pub mod yote;
pub mod yut;
pub mod zarpoker;
pub mod zaruri5;
pub mod zidurile;
// --- P4 (jocuri tradiționale din America Latină / Oceania) ---
pub mod ancora;
pub mod baraja_es;
pub mod burako;
pub mod chinchon;
pub mod cuarenta;
pub mod escoba;
pub mod fivehundred;
pub mod generala;
pub mod mutorere;
pub mod parques;
pub mod pontoon;
pub mod twoup;

// Valul P3
pub mod all_fours;
pub mod bid_whist;
pub mod bunco;
pub mod chicken_foot;
pub mod conquian;
pub mod crazy_eights;
pub mod crokinole;
pub mod farkle;
pub mod gin_rummy;
pub mod liars_dice;
pub mod p3_comun;
pub mod pinochle;
pub mod pitch;

use polytabletop_core::GameObj;

/// How a catalog entry will be (or is) handled.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    /// Playable in PolyTabletop today.
    Implemented,
    /// Traditional / public-domain game: planned with its standard rules.
    Traditional,
    /// Modern branded game: the mechanics are free, the name, art and text are
    /// not, so it is planned as an original variant under our own name.
    Branded,
    /// Role-playing, miniature or collectible-card game: needs a generic
    /// toolkit (dice, sheets, decks), not a copy of any publisher's rules.
    Toolkit,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Implemented => "implementat",
            Kind::Traditional => "planificat (domeniu public)",
            Kind::Branded => "planificat (variantă proprie)",
            Kind::Toolkit => {
                "toolkit generic (RPG: vezi polytabletop-rpg; TCG/miniaturi: planificat)"
            }
        }
    }
}

/// One row of the catalog of target games.
pub struct E {
    pub name: &'static str,
    pub category: &'static str,
    pub origin: &'static str,
    pub kind: Kind,
    pub impl_id: &'static str,
}

/// A game that can be played now.
pub struct Playable {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub rules: &'static str,
    pub make: fn() -> Box<dyn GameObj>,
}

/// Games whose `render` honours `set_viewer` (hot-seat shows only the mover's private info).
pub const HOTSEAT_SAFE: &[&str] = &[
    "burako",
    "chinchon",
    "cuarenta",
    "escoba",
    "fivehundred",
    "pontoon",
    "pinochle",
    "gin_rummy",
    "crazy_eights",
    "pitch",
    "bid_whist",
    "all_fours",
    "conquian",
    "chicken_foot",
    "liars_dice",
    "litere",
    "curieri",
    "fitil",
    "codru",
    "conac",
    "insula",
    "magistrale",
    "bresla",
    "catun",
    "solzi",
    "salina",
    "reverdire",
    "rezervatia",
    "epoci",
    "hotar",
    "capcana",
    "cristale",
    "septica",
    "popa",
    "pescuit",
    "treizeci1",
    "casino",
    "zarpoker",
    "domino",
    "macao",
    "gura",
    "culori",
    "cuvantunic",
    "impostor",
    "povestitor",
    "schita",
    "treicai",
    "oprite",
    "dictionar",
    "belote",
    "bridge",
    "tarot",
    "skat",
    "poker",
    "rummy",
    "canasta",
    "cribbage",
    "euchre",
    "inimi",
    "pica",
    "prinde",
    "whist",
    "tarneeb",
    "baloot",
    "tablanet",
    "truco",
    "trix",
    "briscola",
    "scopa",
    "durak",
    "66",
    "doudizhu",
    "teenpatti",
    "riichi",
    "daifugo",
    "tienlen",
    "bigtwo",
    "tongits",
    "pasur",
    "hokm",
    "pisti",
    "courtpiece",
    "callbreak",
    "paigow",
    "ganjifa",
    "hanafuda",
    "hwatu",
    "barbut",
    "karuta",
    "blackjack",
    "battleship",
    "mastermind",
    "bingo",
    "loteria",
    "mahjong",
    "placute",
    "ceata",
    "umbra",
    "portret",
    "tcg",
    "tcg_evo",
    "tcg_zone",
    "tcg_lant",
    "tcg_cerneala",
    "nave",
    "expeditii",
    "artificii",
    "bufet",
    "whot",
    "chkobba",
    "ronda",
    "okey",
];

pub fn playable() -> Vec<Playable> {
    vec![
        Playable {
            id: "chess",
            title: "Șah",
            category: "Abstract",
            rules: "Reguli complete: rocadă (mută regele două pătrate: e1g1 sau e1c1), en passant, promovare (obligatorie; e7e8q, sufixul q/r/b/n alege piesa), remiză prin pat, repetiție de trei ori, 50 de mutări fără captură sau pion, material insuficient. Mutări: e2e4, e7e8q.",
            make: || Box::new(chess::Chess::new()),
        },
        Playable {
            id: "draughts",
            title: "Dame (8×8)",
            category: "Abstract",
            rules: "Dame engleze 8×8: piesele simple mută și capturează doar înainte, regele (piesa care ajunge pe ultimul rând; mutarea se oprește acolo) un pas în toate cele patru diagonale. Captura e obligatorie și salturile continuă cât se poate. Mutări: c3-d4, c3xe5xg7.",
            make: || Box::new(draughts::Draughts::new()),
        },
        Playable {
            id: "reversi",
            title: "Reversi",
            category: "Abstract",
            rules: "Pui o piesă care încadrează în linie dreaptă piese ale adversarului și le întoarce; negrul (J1) începe; câștigă cine are mai multe piese la final. Mutări: d3 (pătratul ales), pass (doar când nu ai nicio mutare).",
            make: || Box::new(reversi::Reversi::new()),
        },
        Playable {
            id: "connect4",
            title: "Patru în linie",
            category: "Abstract",
            rules: "Lasă discuri în coloane; primul cu patru în linie (orizontal, vertical sau diagonal) câștigă. Mutări: 1–7 (numărul coloanei).",
            make: || Box::new(connect4::Connect4::new()),
        },
        Playable {
            id: "gomoku",
            title: "Gomoku (15×15)",
            category: "Abstract",
            rules: "Cinci sau mai multe în linie câștigă; negrul începe în centru. Mutări: h8 (coloane a–o, rânduri 1–15).",
            make: || Box::new(gomoku::Gomoku::new()),
        },
        Playable {
            id: "hex",
            title: "Hex (11×11)",
            category: "Abstract",
            rules: "X unește sus și jos, O unește stânga și dreapta printr-un lanț de hexagoane vecine; nu există remiză, X începe. Mutări: f6 (coloane a–k, rânduri 1–11).",
            make: || Box::new(hex::Hex::new()),
        },
        Playable {
            id: "amazoane",
            title: "Amazoane (10×10)",
            category: "Abstract",
            rules: "Patru amazoane pe parte; muți una ca o regină de șah, apoi tragi o săgeată (tot ca o regină) care blochează pătratul; pierde cine nu mai poate muta. Mutări: d1-d5/d8 (de la–la/săgeată).",
            make: || Box::new(amazoane::Amazoane::new()),
        },
        Playable {
            id: "linii",
            title: "Linii de acțiune (8×8)",
            category: "Abstract",
            rules: "Muți o piesă pe linie atâtea pătrate câte piese sunt pe acea linie; poți sări piese proprii, nu ale adversarului, și captezi aterizând pe una; câștigă cine își unește toate piesele într-un grup (remiză după 300 de mutări). Mutări: b1-b3, b1xd3.",
            make: || Box::new(linii::Linii::new()),
        },
        Playable {
            id: "nim",
            title: "Nim cu grămezi",
            category: "Abstract",
            rules: "Patru grămezi (3, 4, 5, 6 bețe); iei oricâte bețe dintr-o singură grămadă; cine ia ultimul băț câștigă. Mutări: b3 (litera grămezii + număr).",
            make: || Box::new(nim::Nim::new()),
        },
        Playable {
            id: "morris",
            title: "Moara (Țintar)",
            category: "Abstract",
            rules: "9 piese: plasare, apoi mutare pe linii; o moară (trei pe o linie) scoate o piesă adversă; cu 3 piese poți „zbura”. Mutări: d1 (plasare), a1-a4 (mutare); cu moară se adaugă piesa scoasă: g1xb2, a1-a4xb2.",
            make: || Box::new(morris::Morris::new()),
        },
        Playable {
            id: "mancala",
            title: "Mancala (Kalah)",
            category: "Abstract",
            rules: "6 gropi și un depozit; semănatul ajunge în depozit = încă o mutare; groapă goală proprie = captură. Mutări: 1–6 (groapa proprie aleasă).",
            make: || Box::new(mancala::Mancala::new()),
        },
        Playable {
            id: "go",
            title: "Go (9×9)",
            category: "Abstract",
            rules: "Scor pe arie (pietre + teritoriu), komi 7,5 pentru alb, ko simplu, fără sinucidere; două pass-uri consecutive încheie jocul. Mutări: e5 (coloane a–h și j, fără i; rânduri 1–9), pass.",
            make: || Box::new(go::Go::new()),
        },
        Playable {
            id: "xiangqi",
            title: "Xiangqi (șah chinezesc)",
            category: "Abstract",
            rules: "9×10, palat, râu, tun cu ecran, cal blocabil, generalii nu se pot privi. Mat sau pat = pierdere. Mutări: h2e2 (fișiere a–i, rânduri 0–9).",
            make: || Box::new(xiangqi::Xiangqi::new()),
        },
        Playable {
            id: "shogi",
            title: "Shogi (șah japonez)",
            category: "Abstract",
            rules: "9×9; piesele capturate se pun înapoi (drop). Promovare în zona adversă (obligatorie dacă piesa n-ar mai avea mutări). Drop interzis: nifu (doi pioni pe fișier), pion/lance/cal fără mutări, pion care dă mat. Mat = pierdere; a 4-a repetiție sau 320 de semi-mutări = remiză. Mutări: 7g7f, 2b8h+ (promovare), P*5e (drop).",
            make: || Box::new(shogi::Shogi::new()),
        },
        Playable {
            id: "dara",
            title: "Dara (Nigeria)",
            category: "Abstract",
            rules: "Tablă 5×6, 12 piese pe jucător. Faza 1: plasare pe rând (fără capturi). Faza 2: mutare ortogonală cu un pas; o moară de exact 3 (rând sau coloană, nu 4) scoate o piesă adversă oarecare. Pierzi cu sub 3 piese sau blocat; remiză după 100 de mutări fără captură. Mutări: b3, b3-b4, b3-b4xd2 (fișiere a–f, rânduri 1–5).",
            make: || Box::new(dara::Dara::new()),
        },
        Playable {
            id: "morabaraba",
            title: "Morabaraba (Africa de Sud)",
            category: "Abstract",
            rules: "Tablă ca la Moară plus diagonalele prin colțuri (20 de „vaci”), 12 piese. Plasare, apoi mutare pe linii (și diagonale); vaca scoate o piesă adversă (nu din vacă, dacă are alta); cu 3 piese zbori; sub 3 piese sau blocat = pierdere; tablă plină după plasare = remiză. Mutări: d1, a1-b2, c3xg7, a1-d5xg7.",
            make: || Box::new(morabaraba::Morabaraba::new()),
        },
        Playable {
            id: "sungka",
            title: "Sungka (Filipine)",
            category: "Abstract",
            rules: "7 gropi și un depozit pe jucător, 7 semințe în fiecare groapă. Semănat în sens trigonometric prin depozitul propriu, sărind depozitul adversarului. Ultima sămânță în depozit = încă o mutare; în groapă ocupată = ridici și continui; în groapă goală proprie = capturezi groapa opusă; în groapă goală adversă = tura se termină. O singură rundă, fără gropi „arse”, mutări pe rând; câștigă cine are peste 49 în depozit sau mai mult la final. Mutări: 1–7 (groapa proprie, de la stânga la dreapta).",
            make: || Box::new(sungka::Sungka::new()),
        },
        Playable {
            id: "makruk",
            title: "Makruk (șah thailandez)",
            category: "Abstract",
            rules: "8×8: Khun (rege), Met (un pas diagonal), Khon (un pas diagonal sau înainte), cal, turn; pionii mută și capturează cu un pas și se promovează în Met pe rândul 6. Fără rocadă. Mat = victorie; pat = remiză; fără pioni, numărare limitată (8–64 de mutări) = remiză. Mutări: e3e4.",
            make: || Box::new(makruk::Makruk::new()),
        },
        Playable {
            id: "janggi",
            title: "Janggi (șah coreean)",
            category: "Abstract",
            rules: "9×10, palate cu diagonale, Cho (jos) începe. Car pe diagonalele palatului, tun care sare exact o piesă (nu tun), cal blocabil, elefant (1 ortogonal + 2 diagonale) blocabil, soldați înainte/lateral. Pass permis când nu ești în șah; mat = pierdere; repetiție, 2 pass-uri sau 150 de semi-mutări fără captură = remiză. Mutări: b2e2, pass.",
            make: || Box::new(janggi::Janggi::new()),
        },
        Playable {
            id: "popa",
            title: "Popa prostul",
            category: "Cărți",
            rules: "Old Maid românesc: 51 de cărți (fără Dama de treflă), perechile se aruncă automat. Tragi pe rând o carte din mâna adversarului, la alegere după poziție; cine rămâne cu popa (Dama nepereche) pierde. Informație ascunsă: vezi doar mâna ta. Mutări: trage 3, sfarsit.",
            make: || Box::new(popa::Popa::new()),
        },
        Playable {
            id: "pescuit",
            title: "Pescuit (Go Fish)",
            category: "Cărți",
            rules: "52 de cărți, 7 în mână. Ceri o valoare pe care o ai (cer 7); adversarul răspunde forțat da (îți dă toate cărțile) sau pescuieste (tragi din stoc; dacă prinzi valoarea cerută mai ceri). Patru la fel formează un set; cele mai multe seturi câștigă. Informație ascunsă: vezi doar mâna ta. Mutări: cer K, da, pescuieste.",
            make: || Box::new(pescuit::Pescuit::new()),
        },
        Playable {
            id: "treizeci1",
            title: "Treizeci și unu (31)",
            category: "Cărți",
            rules: "3 cărți în mână, A=11, figuri=10; valoarea = suma celei mai bune culori (trei la fel = 30). Tragi (trage) sau iei din aruncate (ia), apoi arunci (arunca 5h), sau bați (bat): adversarul mai are o tură, apoi cea mai mică mână pierde o viață (bătătorul singur cel mai mic: două). 31 = adversarul pierde o viață. 3 vieți. Informație ascunsă: vezi doar mâna ta. Mutări: trage, ia, bat, arunca Kd, sfarsit.",
            make: || Box::new(treizeci1::Treizeci1::new()),
        },
        Playable {
            id: "casino",
            title: "Cassino (capturi)",
            category: "Cărți",
            rules: "52 de cărți, 4 pe masă, 4 în mână. Lași o carte (5h) sau capturezi (5h x 2c 3d): grupuri de cărți identice sau cu suma egală cu a ta (A=1, J/Q/K doar identice). Puncte pe împărțire: cărți 3, pici 1, 10♦ 2, 2♠ 1, ași 1, mături 1; se joacă până la 11. Fără construiri. Informație ascunsă: vezi doar mâna ta.",
            make: || Box::new(casino::Casino::new()),
        },
        Playable {
            id: "zarpoker",
            title: "Poker de zaruri",
            category: "Zaruri",
            rules: "Variantă originală: 7 runde, 5 zaruri, până la 3 aruncări (arunc; tine 3 3 5 sau tine -; fin). Mâini de poker: cinci la fel > careu > full > scară > trei > două perechi > pereche > nimic. Mâna terminată a primului jucător rămâne acoperită până termină al doilea; cine are mâna mai bună ia punctul rundei.",
            make: || Box::new(zarpoker::ZarPoker::new()),
        },
        Playable {
            id: "septica",
            title: "Septica",
            category: "Cărți",
            rules: "Pachet de 32 de cărți, 4 în mână. Preiei masa cu aceeași valoare ca prima carte sau cu 7 / 8; poți tăia la rândul tău; cine dă pas pierde masa. 10 și A = 1 punct (8 în total), peste 4 câștigi. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 9h, 7s, pas.",
            make: || Box::new(septica::Septica::new()),
        },
        Playable {
            id: "macao",
            title: "Macao",
            category: "Cărți",
            rules: "52 de cărți, 5 în mână: culoare sau valoare. 2 / 3 se adună (taie cu alt 2 / 3, altfel tragi); 4 sare peste rând; 7 merge peste orice și alege culoarea (7h:s); As cere culoarea (Ah:d). Fără carte bună tragi una. Câștigă cine rămâne fără cărți (nu cu carte specială). Mutări: 9h, trage, sar, pas.",
            make: || Box::new(macao::Macao::new()),
        },
        Playable {
            id: "domino",
            title: "Domino (dublu-6)",
            category: "Clasic",
            rules: "28 de piese, 7 fiecare, restul stoc. Începe cel mai mare dublu. Așază la stânga (S) sau dreapta (D); fără piesă pescuiești (pesc), cu stocul gol dai pas. Câștigă cine termină piesele (ia punctele adversarului) sau, la blocaj, cine are mai puține puncte. Mutări: 3-4:S, 3-4:D, pesc, pas.",
            make: || Box::new(domino::Domino::new()),
        },
        Playable {
            id: "backgammon",
            title: "Table (Backgammon)",
            category: "Zaruri",
            rules: "Două zaruri aruncate automat; 15 piese, bară, bear-off, dubla se joacă de 4 ori, fără cub de dublare. Un pas = un zar; trebuie jucate cât mai multe zaruri. Numerotarea e a jucătorului la mutare. Mutări: 13/7, bar/22, 3/off.",
            make: || Box::new(backgammon::Backgammon::new()),
        },
        Playable {
            id: "ludo",
            title: "Nu te supăra, frate (Ludo)",
            category: "Zaruri",
            rules: "4 piese, un zar. Cu 6 scoți o piesă din bază; 6 = încă o aruncare; aterizarea pe adversar îl trimite în bază; ținta se atinge exact. Mutări: intră, sau progresul piesei (ex. 12).",
            make: || Box::new(ludo::Ludo::new()),
        },
        Playable {
            id: "snakes",
            title: "Șerpi și scări",
            category: "Zaruri",
            rules: "Tablă 1–100; zarul e aruncat automat și afișat; scările urcă, șerpii coboară; ținta se atinge exact. Singura mutare: arunc.",
            make: || Box::new(snakes::Snakes::new()),
        },
        Playable {
            id: "barbut",
            title: "Barbut (duel simplificat)",
            category: "Zaruri",
            rules: "10 jetoane fiecare. Pariu 1–3; prima aruncare: 7/11 câștigă, 2/3/12 pierde, altfel punct; adversarul plătește (dublează) sau lasă; apoi punctul bate 7. Mutări: pariez 1, arunc, plătesc, las.",
            make: || Box::new(barbut::Barbut::new()),
        },
        Playable {
            id: "66",
            title: "Șase-zeci și șase (Santase)",
            category: "Cărți",
            rules: "2 jucători, 24 de cărți, atu, anunțuri 20/40, schimbul nouarului de atu, închiderea jocului; 66 de puncte încheie mâna, meci până la 7 puncte de joc. Informație ascunsă: vezi doar mâna ta (J1) și numărul de cărți ale adversarului. Mutări: 9h, Qs, 10d, Ac, Qh+ (anunț), schimb, inchide, urmatoarea.",
            make: || Box::new(sixtysix::SixtySix::new()),
        },
        Playable {
            id: "blackjack",
            title: "Blackjack (21)",
            category: "Cărți",
            rules: "Tu (J1) contra dealerului (J2, trage până la 17): carte / stai / dublu; blackjack plătește 3:2. Set de 21 de mâini cu pariu de 10 din 100 de jetoane: câștigi dacă termini peste 100. Pașii dealerului sunt mutări forțate: dealer-trage, dealer-sta, decontare.",
            make: || Box::new(blackjack::Blackjack::new()),
        },
        Playable {
            id: "rummy",
            title: "Remi (Rummy, 13 cărți)",
            category: "Cărți",
            rules: "2 jucători, 13 cărți, un pachet: tragi (stoc sau talon) și arunci; închizi când cele 13 cărți rămase formează secvențe și seturi (cel puțin o secvență; aranjare automată). Penalizare = punctele libere; meci până la 101. Mutări: stoc, talon, Ah, 10c, inchide-Ah, urmatoarea.",
            make: || Box::new(rummy::Rummy::new()),
        },
        Playable {
            id: "poker",
            title: "Poker (Texas hold'em, heads-up)",
            category: "Cărți",
            rules: "20 de mâini, blinduri fixe 10/20, 1000 de jetoane, raise fix (20 / 40 de la turn), maxim 3 raise-uri pe rundă. Câștigă cine are mai multe jetoane. Mutări: fold, call/check, raise; pașii de împărțire sunt forțați: flop, turn, river, showdown, decontare.",
            make: || Box::new(poker::Poker::new()),
        },
        Playable {
            id: "tcg",
            title: "Cărți colecționabile (Ținutul Cenușiu)",
            category: "TCG",
            rules: "Motor generic de cărți colecționabile cu 60 de cărți originale (resurse, creaturi, vrăji); pachete pregătite de 30 de cărți (J1 Cetatea, J2 Furtuna), 20 de viață, 5 cărți în mână. Tură: pescuire (forțată), faza principală, fin. O resursă pe tură (+1 mană). Creaturile noi nu atacă (Grabă = da); cuvinte-cheie: Zbor, Grabă, Armură, Venin, Primul, Străjer. Atac: ataca 1,3; apărătorul alege pe rând bloc N sau fara. Pachet gol la pescuit = pierdere. Informație ascunsă: vezi doar mâna J1. Mutări: pesca, joaca 2, joaca 3 e1 (țintă: e=inamic, m=al meu, fata), ataca 1,2, bloc 1, fara, fin.",
            make: || Box::new(tcg::Tcg::new()),
        },
        Playable {
            id: "miniaturi",
            title: "Miniaturi hexagonale (Trecătoarea Cenușie)",
            category: "Miniaturi",
            rules: "Motor generic de miniaturi pe hartă hexagonală: 7 unități originale pe parte (Lăncieri, Arcași, Călăreți, Colos, Catapulte, Comandant), teren (stâncă blochează vederea, pădure = acoperire), zaruri de lovire și salvare, moral (șovăit, apoi fugă). 6 runde cu activări alternante; 3 obiective (cine are mai multe figuri la ≤1 hex ia 1 punct pe rundă); eliminarea armatei adverse câștigă. Informație completă; zarurile se aruncă la mutarea forțată arunca. Mutări: A pas, A>d4, A>d4 xK (mută și atacă), A xK, arunca.",
            make: || Box::new(miniaturi::Miniaturi::new()),
        },
        Playable {
            id: "mahjong",
            title: "Mahjong (Hong Kong, 2 jucători)",
            category: "Clasic",
            rules: "136 de piese, 13 în mână, zid viu de 96 și zid mort de 14 (ascuns; vezi doar câte au rămas). Tragi și arunci; adversarul poate lua aruncarea cu pung / kong / chi (de la oricare aruncare) sau pe câștig. Mână câștigătoare = 4 seturi + pereche și minimum 1 fan (tragere proprie, mână închisă, toate secvențele, toate pungurile, o culoare, dragoni, vânturi). Fără șapte perechi / mâini speciale. Zid epuizat = remiză. Mutări: trage, d5m, d Dr, mahjong, pung, kong, chi2m3m, kong5m, pas, trage-moarta.",
            make: || Box::new(mahjong::Mahjong::new()),
        },
        Playable {
            id: "hanafuda",
            title: "Hanafuda (Koi-Koi)",
            category: "Cărți",
            rules: "48 de cărți de flori (12 luni × 4), 8 în mână și 8 pe masă; potrivești luna: obligatoriu capturezi dacă ai pereche (doi candidați = alegi: 3a:3c, trei = iei toate). Apoi întorci o carte din stoc (intoarce) și o joci la fel. Yaku nou (luminoase, mistreț-cerb-fluturi, panglici, animale, simple, hanami, tsukimi): koi-koi (continui) sau shobu (încasezi; ×2 la 7+ și ×2 dacă adversarul a spus koi-koi). 3 runde. Mutări: 3b, 3a:3c, intoarce, ia 3c, koi-koi, shobu.",
            make: || Box::new(hanafuda::Hanafuda::new()),
        },
        Playable {
            id: "hwatu",
            title: "Hwatu (Go-Stop)",
            category: "Cărți",
            rules: "Aceleași 48 de cărți de flori; 10 în mână, 8 pe masă, 20 în stoc. Captură ca la Hanafuda (3c, 3a:3c, intoarce, ia 3c). Scor: gwang, godori, hongdan/cheongdan/chodan, tti, yeolkkeut, pi. De la 7 puncte alegi stop (încasezi) sau go (continui: +1 la primele două, apoi ×2; ×2 gwang-bak, pi-bak și go-bak). Măturarea mesei ia o simplă de la adversar. 3 runde. Fără ppeok/jjok/ttadak.",
            make: || Box::new(hwatu::new()),
        },
        Playable {
            id: "karuta",
            title: "Karuta (recitare vs AI, proverbe)",
            category: "Clasic",
            rules: "Karuta cu 40 de proverbe românești: 16 cărți pe masă (8 pe parte, cu sfârșitul proverbului), 24 de citiri (8 false). Începutul se dezvăluie cuvânt cu cuvânt; pe rând: asteapta sau ia N (atingi cartea). Corect: cartea dispare (ținta de pe partea adversarului: îi trimiți una de-a ta). Greșit: adversarul îți trimite o carte. Câștigă cine rămâne primul fără cărți (sau are mai puține după toate citirile). Mutări: asteapta, ia 12.",
            make: || Box::new(karuta::Karuta::new()),
        },
        Playable {
            id: "doudizhu",
            title: "Dou dizhu (2 jucători)",
            category: "Cărți",
            rules: "54 de cărți (3…K, A, 2, jokeri X/Y); 17 fiecare, 20 ascunse nefolosite. Combinații: single, pereche, trio (+1 sau +pereche), scară de 5+ (34567), scară de perechi 3+ (334455), bombă (5555), rachetă (XY). Răspunzi cu același tip și lungime, mai mare, sau bombă/rachetă, ori pas. Cine rămâne fără cărți câștigă runda; cel mai bun din 3. Fără avioane și careu+2. Mutări: 5, 55, 555+3, 34567, 5555, XY, pas.",
            make: || Box::new(dou_dizhu::DouDizhu::new()),
        },
        Playable {
            id: "paigow",
            title: "Pai Gow (domino chinezesc)",
            category: "Clasic",
            rules: "32 de piese; 4 piese fiecare, împărțite în față (mâna mică) și spate (mâna mare); față vs față și spate vs spate contra dealerului (rolul alternează; copiile și 0–0 merg la dealer). Perechi (Gee Joon > Teen > Day > … > Chop-Ng) > Wong > Gong > puncte modulo 10. Ambele câștigate = +10, una și una = remiză. 12 runde, 100 de jetoane. Mutări: aranj 1/2/3, arata, urmatoarea.",
            make: || Box::new(pai_gow::PaiGow::new()),
        },
        Playable {
            id: "teenpatti",
            title: "Teen Patti (heads-up)",
            category: "Cărți",
            rules: "3 cărți fiecare, 12 mâini, 500 de jetoane, boot 10. Ierarhie: trio > scară de culoare > scară (AKQ, A23, KQJ…) > culoare > pereche > carte mare. Mutări: pack (abandon), chaal (plătești miza), raise (dublezi, max 80), show (după cel puțin o acțiune; la egalitate pierde cel care cere show); arata și urmatoarea sunt forțate. Maxim 6 acțiuni pe mână.",
            make: || Box::new(teen_patti::TeenPatti::new()),
        },
        Playable {
            id: "riichi",
            title: "Riichi Mahjong (2 jucători)",
            category: "Clasic",
            rules: "Mahjong japonez pentru doi: 136 de piese (1m–9m, 1p–9p, 1s–9s, 1z–7z = vânturi și dragoni), 13 piese, 4 mâini (Est 1–2, Sud 1–2), 25 000 de puncte. Tura: trage, apoi arunci (5m), riichi 5m, tsumo, ankan 5m, kakan 5m (după careu: trage-moarta); pe aruncarea adversarului: ron, pon, kan, chi 4m 6m sau pas. Mână = 4 seturi + pereche (sau 7 perechi / 13 orfani) cu cel puțin un yaku; furiten, dora/ura-dora, fu și han până la yakuman. Zidul e ascuns. Fără piese roșii.",
            make: || Box::new(riichi::Riichi::new()),
        },
        Playable {
            id: "sicbo",
            title: "Sic Bo",
            category: "Zaruri",
            rules: "Trei zaruri; duel contra „casei” cu 100 de jetoane, 12 runde (ordinea de pariere alternează). Pariuri (plăți X:1): mic 1, mare 1 (pierd la triplu), cifra N 1/2/3, dublu N 10, triplu N 180, oricare-triplu 30, suma S (4 și 17: 60 … 9–12: 6), pereche a-b 5. Cel mult 6 pariuri pe rundă, mize 1/5/10/25. Mutări: mic:5, cifra:3:5, suma:9:5, pereche:1-2:5, fin, arunc, urmatoarea. Câștigă cine are mai multe jetoane.",
            make: || Box::new(sicbo::SicBo::new()),
        },
        Playable {
            id: "ashtachamma",
            title: "Ashta Chamma",
            category: "Familie",
            rules: "Tablă 5×5, 4 piese pe jucător, 4 cauri (1–4; zero în sus = 8; 4 și 8 dau încă o aruncare). Piesele merg în sens trigonometric pe inelul exterior (16 pătrate) și intră în inelul interior și în centru doar după o captură; centrul se atinge exact. Pătrate sigure (X): mijloace de latură, colțuri, centru; două piese ale aceluiași jucător formează un bloc. Captura dă încă o aruncare. Câștigă cine aduce toate piesele în centru. Mutări: arunc, 7 sau 7* (piesa de pe pătratul 7 al căii; * = a capturat), pas.",
            make: || Box::new(ashtachamma::AshtaChamma::new()),
        },
        Playable {
            id: "daifugo",
            title: "Daifugo (Daihinmin)",
            category: "Cărți",
            rules: "54 de cărți (cu 2 jokeri), 27 fiecare, 5 runde. Ordine 3…K, A, 2, joker. Joci un grup de același rang (33, 777), adversarul același număr de cărți mai mari sau pas; după pas conduce cine a jucat ultimul. Joker singur (X) sau wild (5X). 4 cărți = revoluție (ordinea se inversează), orice grup cu 8 oprește jocul (conduci din nou). Câștigi runda când rămâi fără cărți; perdantul îți dă cele mai bune 2 cărți, tu dai 2 înapoi (da 4 5). Mutări: 3, 77, JJJ, 55X, X, pas, da 4 5.",
            make: || Box::new(daifugo::Daifugo::new()),
        },
        Playable {
            id: "tienlen",
            title: "Tiến lên",
            category: "Cărți",
            rules: "52 de cărți, 13 fiecare (restul ascuns). Rang 3…A, 2; culoare ♠ < ♣ < ♦ < ♥. Combinații: carte, pereche, treime, careu, scară (3+, fără 2), 3+ perechi consecutive; aceeași formă și mai mare, sau pas. Tăieri pe 2: careu sau 3 perechi consecutive pe un 2, careu sau 4 perechi consecutive pe o pereche de 2; 4 perechi consecutive taie și careul. Victorie instantanee: patru de 2, șase perechi, scară 3…A. Primul la 3 mâini. Mutări: 3s, 9h 9d, 5c 6d 7h, pas.",
            make: || Box::new(tienlen::TienLen::new()),
        },
        Playable {
            id: "bigtwo",
            title: "Big Two (Pusoy dos)",
            category: "Cărți",
            rules: "52 de cărți, 13 fiecare (restul ascuns). Rang 3…A, 2; culoare ♦ < ♣ < ♥ < ♠. Carte, pereche, treime sau mână de 5: scară < culoare < full < careu+1 < scară de culoare. Dai aceeași formă mai mare ori pas; cine are cea mai mică carte începe cu ea. Mâna se termină la golirea cărților: perdantul primește cărțile rămase ca penalizare (dublu de la 10 cărți); la 30 de puncte pierde meciul (cel mult 8 mâini). Mutări: 3d, 9h 9s, 3d 4c 5h 6s 7d, pas.",
            make: || Box::new(bigtwo::BigTwo::new()),
        },
        Playable {
            id: "tongits",
            title: "Tong-its",
            category: "Cărți",
            rules: "52 de cărți, 12 fiecare (dealerul 13 și începe fără să tragă). Tura: trage din stoc sau ia aruncata dacă o folosești imediat (ia 5d 5h 5c / ia sapaw 2), apoi pune seturi sau scări (pune 3h 4h 5h), adaugă la orice combinație (sapaw 6h 2) și aruncă (arunca Kd). Mână goală = Tongits, 3 p.; cheama (cu o combinație pusă) → adversarul renunt (1 p.) sau lupta: mai puține puncte în mână câștigă (egalitate: cel care a luptat; dacă pierde cel care a chemat, adversarul ia 2 p.); stoc epuizat: mai puține puncte. Primul la 6 p.",
            make: || Box::new(tongits::Tongits::new()),
        },
        Playable {
            id: "pasur",
            title: "Pasur",
            category: "Cărți",
            rules: "52 de cărți, 4 pe masă și 4 în mână (se dau din nou câte 4). Un număr (A=1…10) capturează orice grupuri de cărți de pe masă cu suma 11 împreună cu el; Valetul ia tot în afară de Dame și Regi; Dama ia o Damă, Regele un Rege. Sur (masa golită, nu cu Valet) 5 p. Puncte: cele mai multe cărți 3, ≥7 trefle 7, 10♦ 3, 2♣ 2, Ași 1, Valeți 1. Joc până la 62. Mutări: 7h (pui), 5s x 2c 4d 6h (capturezi), imparte.",
            make: || Box::new(pasur::Pasur::new()),
        },
        Playable {
            id: "hokm",
            title: "Hokm",
            category: "Cărți",
            rules: "52 de cărți, câte 13 (restul ascuns). Hakem vede mai întâi 5 cărți și alege atuul (hokm h); conduce prima levată. Răspunzi la culoare; atuul bate tot. 7 din 13 levate = 1 p., 13–0 = 2 p., hakem cu zero levate = 3 p.; hakem rămâne dacă câștigă. Primul la 7 puncte. Mutări: hokm h (c/d/h/s), 10h, As.",
            make: || Box::new(hokm::Hokm::new()),
        },
        Playable {
            id: "pisti",
            title: "Piști",
            category: "Cărți",
            rules: "52 de cărți: 4 pe masă (3 cu fața în jos), 4 în mână. Cartea ta ia toată grămada dacă are același rang ca cea de deasupra sau e Valet. Piști (grămadă de o carte luată cu același rang) 10 p., Valet peste Valet 20 p. Puncte: Ași și Valeți 1, 2♣ 2, 10♦ 3, cele mai multe cărți 3. Se joacă până la 101. Mutări: 7h, Js, imparte.",
            make: || Box::new(pisti::Pisti::new()),
        },
        Playable {
            id: "courtpiece",
            title: "Court Piece (Rang, Mendikot)",
            category: "Cărți",
            rules: "52 de cărți, câte 26. Cine nu e dealer vede primele 5 cărți și alege atuul (rang h), apoi se împarte restul și el conduce. Răspunzi la culoare; atuul bate tot. Mâna o câștigă cine ia mai multe zeci (mendi): 1 p.; toate 4 zecile (Mendikot) 2 p.; toate levatele (Court) 3 p.; la 2–2 decid levatele. Primul la 5 puncte. Mutări: rang h, 10h, As.",
            make: || Box::new(courtpiece::CourtPiece::new()),
        },
        Playable {
            id: "callbreak",
            title: "Call Break",
            category: "Cărți",
            rules: "52 de cărți, câte 13 (restul ascuns); pica e mereu atu. Cine nu e dealer licitează primul (licit 4) numărul de levate și conduce. Răspunzi la culoare și, dacă poți, cu o carte mai mare; fără culoare tai cu pică. Scor: licitația + 0,1 pe levată în plus, sau − licitația dacă ai luat mai puțin. 5 mâini, scorul total mai mare câștigă. Mutări: licit 4, 10h, As.",
            make: || Box::new(callbreak::CallBreak::new()),
        },
        Playable {
            id: "ganjifa",
            title: "Ganjifa (levate)",
            category: "Cărți",
            rules: "48 de cărți (4 culori: Taj T, Shamsher S, Surkh R, Chang C; Mir M, Vazir V, 1–10). La T și S 10 e cel mai mare număr, la R și C ordinea e inversată (1 cel mai mare). 12 cărți fiecare; cartea de sus din stoc arată atu. 12 levate, culoare obligatorie, atu bate. Mai multe levate câștigă. Mutări: TM, S10, R1, CV.",
            make: || Box::new(ganjifa::Ganjifa::new()),
        },
        Playable {
            id: "yut",
            title: "Yut (Yunnori)",
            category: "Zaruri",
            rules: "4 bețe-zar (arunc): 1 plat = do, 2 = gae, 3 = geol, 4 = yut (încă o aruncare), 0 = mo (5, încă o aruncare). 4 piese pe jucător; rezultatele se folosesc în orice ordine; grupurile de pe același nod merg împreună; capturarea trimite la start și dă o aruncare. Scurtături din colțurile 5 și 10 (centrul 22). Câștigă cine scoate toate piesele. Mutări: arunc, do:start, gae:7, mo:22.",
            make: || Box::new(yut::Yut::new()),
        },
        Playable {
            id: "sugoroku",
            title: "Sugoroku (cursă cu casete)",
            category: "Zaruri",
            rules: "Drum 0–40, două zaruri (arunc), alegi un zar sau suma (avansez N). Atingi exact 40 ca să câștigi (surplusul te întoarce). Casete: 4 +3, 9 −3, 12 și 28 pierzi tura, 15 și 36 încă o aruncare, 18 +5, 21 schimbi locul cu adversarul, 25 −6, 30 +4, 33 −8, 38 −10 (fără lanț). Mutări: arunc, avansez 7.",
            make: || Box::new(sugoroku::Sugoroku::new()),
        },
        Playable {
            id: "chaupar",
            title: "Chaupar (cauri, 2 jucători)",
            category: "Zaruri",
            rules: "4 piese, circuit de 48 de pătrate (start 0 / 24), 4 cauri: 1–4 = pași, 0 = 8; 4 și 8 dau încă o aruncare; scoți din curte cu 1 sau 8 (intra). Castelele (multipli de 6) sunt sigure; captura trimite în curte și dă o aruncare. Coloană de acasă și centru exact. Mutări: arunc, intra, mut:12, mut:casa2, pas.",
            make: || Box::new(chaupar::Chaupar::new()),
        },
        Playable {
            id: "culori",
            title: "Culori (potrivit cărți colorate)",
            category: "Cărți",
            rules: "Joc original: 108 cărți (4 culori cu cifre 0–9, sari, intoarce, plus2, plus joker și joker4). Joci o carte de aceeași culoare sau cu aceeași cifră/simbol; în doi, sari/intoarce = adversarul pierde rândul, plus2 = trage 2, joker4 = trage 4 (jokerii aleg culoarea: joker:r). Fără carte jucabilă tragi (trage), apoi o joci sau dai pas. Câștigă cine rămâne primul fără cărți. Informație ascunsă: vezi doar mâna ta. Mutări: r5, gsari, vplus2, joker:a, trage, pas.",
            make: || Box::new(culori::Culori::new()),
        },
        Playable {
            id: "mosii",
            title: "Moșii (afaceri imobiliare cu zaruri)",
            category: "Zaruri",
            rules: "Joc original, 32 de câmpuri: arunci două zaruri (automat), cumperi terenuri și gări (cumpar / las), încasezi chirii (grup complet = chirie dublă, case: construiesc 3), taxe, evenimente Noroc și închisoare (platesc 50 sau dublă); dublă = încă o aruncare. Faliment = pierdere; după 1500 de mutări câștigă averea mai mare. Fără licitații și ipoteci.",
            make: || Box::new(mosii::Mosii::new()),
        },
        Playable {
            id: "itinerar",
            title: "Itinerar (turism pe tablă, rezervi destinații)",
            category: "Zaruri",
            rules: "Joc original, 28 de câmpuri (7 regiuni × 3 destinații + aeroporturi, tombolă, vamă, bac, hotel de lux, taxă de turist): arunc, apoi mut (zarurile 2d6 sunt automate); pe o destinație liberă: rezerv (taxa de intrare) sau las; pe una ocupată plătești taxa de cazare. La final: construiesc N (cazare, 3 niveluri), rezerv N (la distanță, preț ×1,5, o dată pe tură), zbor (între aeroporturi, 60) sau gata. Colecție completă de regiune = taxe ×2; dublă = încă o aruncare, a treia dublă = vamă (pierzi tura); lux (nivel gratuit la Hotelul de lux). Faliment = eliminare; după 40 de runde câștigă averea mai mare. Informație publică.",
            make: || Box::new(itinerar::Itinerar::new()),
        },
        Playable {
            id: "magistrale",
            title: "Magistrale (rute cu cărți de vagon)",
            category: "Strategie",
            rules: "Joc original: hartă cu 19 orașe și 42 de rute; strângi cărți de vagon (r a v g m, j = locomotivă) și ocupi rute (ocupa 12 r), tragi 2 cărți (trage / trage r) sau bilete de destinație secrete (bilete, pastrez 1,3). La final bilet îndeplinit +valoarea, neîndeplinit −valoarea, drum lung +10. În doi, rutele duble se ocupă o singură dată. Informație ascunsă: vezi doar cărțile și biletele tale.",
            make: || Box::new(magistrale::Magistrale::new()),
        },
        Playable {
            id: "conac",
            title: "Misterul din Conac (deducție)",
            category: "Deducție",
            rules: "Joc original: o crimă cu soluție secretă (suspect s1–s6, armă a1–a6, cameră c1–c9); 18 cărți împărțite 9–9. Sugerezi (sugerez s2 a3 c5), adversarul îți arată în secret o carte pe care o are (arat s2) sau spune nu-am; deduci și acuzi (acuz s2 a3 c5, apoi verific): corect câștigi, greșit pierzi. Fără tablă de mers. Informație ascunsă: vezi doar mâna ta și cărțile arătate ție.",
            make: || Box::new(conac::Conac::new()),
        },
        Playable {
            id: "insula",
            title: "Insula (colonizare cu resurse)",
            category: "Strategie",
            rules: "Joc original pe 19 hexagoane (pădure l, argilă a, pășune p, lan g, munte m) cu numere 2–12: pui colonii și drumuri (colonie v12, drum e7), arunci zarurile (arunc) ca să primești resurse, la 7 mută tâlharul (bandit h5), construiești colonii/orașe/drumuri, schimbi 4:1 cu banca (schimb l g) și închei (sfarsit). Colonie 1 punct, oraș 2, cel mai lung drum (≥5) 2; primul la 10 câștigă. Fără cărți de dezvoltare, porturi sau schimb între jucători. Informație ascunsă: vezi doar mâna ta.",
            make: || Box::new(insula::Insula::new()),
        },
        Playable {
            id: "mozaic",
            title: "Mozaic (plăcuțe cu drumuri, cetăți, mănăstiri)",
            category: "Strategie",
            rules: "Joc original, 65 de plăcuțe: tragi o plăcuță și o lipești cu muchiile potrivite (F câmp, R drum, C cetate; pune(x,y)r1), apoi poți pune un pion pe un drum, o cetate sau o mănăstire (drum:N, oras:E, manastire, sau fara). Cetate închisă 2/plăcuță (+2 per scut), drum 1/plăcuță, mănăstire 9; majoritatea ia punctele, la final se numără și ce-i neterminat. Fără câmpuri/fermieri. Dacă plăcuța nu se potrivește: arunca.",
            make: || Box::new(mozaic::Mozaic::new()),
        },
        Playable {
            id: "litere",
            title: "Litere (cuvinte pe tablă)",
            category: "Cuvinte",
            rules: "Joc original pe tablă 15×15 cu 97 de plăcuțe (2 blank): formezi cuvinte legate de cele existente, cu pătrate premiate (# cuvânt ×3, + cuvânt ×2, x literă ×3, - literă ×2); +50 cu toate cele 7 plăcuțe. Dicționar mic românesc încorporat (~4000 de cuvinte uzuale, fără diacritice; cuvintele absente sunt respinse). Mutări: casa@h8> (orizontal), casa@h8v (vertical), ca(s)a@h8> (blank între paranteze), schimb (toată mâna), pas. Primul cuvânt acoperă h8. Informație ascunsă: vezi doar mâna ta.",
            make: || Box::new(litere::Litere::new()),
        },
        Playable {
            id: "faianta",
            title: "Faianța (variantă originală)",
            category: "Strategie",
            rules: "Draft de plăcuțe pe modele, 2 jucători: iei toate plăcuțele de o culoare dintr-o manufactură (restul trec în centru) sau din centru (primul ia jetonul de start, −1) și le pui pe o linie de model (1–5, o singură culoare, nu dacă peretele are deja culoarea) sau pe podea (−1…−3). La sfârșitul rundei (mutarea forțată decontare) liniile pline urcă pe perete 5×5 și punctează adiacența; jocul se oprește la un rând complet; bonus rând +2, coloană +7, culoare +10. Mutări: f3 T 2, c O p, decontare.",
            make: || Box::new(faianta::Faianta::new()),
        },
        Playable {
            id: "bresla",
            title: "Bresla (variantă originală)",
            category: "Strategie",
            rules: "Construire de pachet pentru doi: 7 Bani + 3 Parcele, mână de 5. Tură: acțiune (joc sat), cumparaturi (comorile se numără automat), cumpar <carte>, gata. 14 cărți: bani/argint/aur, parcelă/moșie/domeniu (1/3/6 p), acțiuni Sat, Meșteșug, Târg, Sărbătoare, Tăietor, Laborator, Atelier (iau <carte>), Cămătar (adversarul aruncă la 3 cărți: dau <carte>). Sfârșit: Domeniile epuizate sau 3 rezerve goale; câștigă cine are mai multe puncte. Informație ascunsă: ordinea pachetului și mâna adversarului.",
            make: || Box::new(bresla::Bresla::new()),
        },
        Playable {
            id: "cristale",
            title: "Cristale (variantă originală)",
            category: "Strategie",
            rules: "Jetoane de minerale și dezvoltări, 2 jucători: o acțiune pe tură — iau G A T (3 culori diferite), iau2 G (2 la fel, dacă sunt ≥4 în bancă), rezerv a1 (+ chihlimbar-joker), cumpar a1 / cumpar rez1. Cărțile cumpărate dau reduceri permanente și puncte; peste 10 jetoane arunci (arunc G). Patronii (3 p) vizitează automat. La 15 puncte se mai joacă până la capătul rundei. Cărțile sunt generate de noi.",
            make: || Box::new(cristale::Cristale::new()),
        },
        Playable {
            id: "curieri",
            title: "Curieri (variantă originală)",
            category: "Cărți",
            rules: "Deducție cu 16 cărți: tragi, joci una din două. 1 Iscoadă ghicește (iscoada:5), 2 Crainic se uită, 3 Judecător compară, 4 Străjer protejează, 5 Mag (mag:eu/mag:el) schimbă cartea, 6 Cămăraș schimbă mâinile, 7 Contesă (obligatorie cu 5/6), 8 Prințesă (aruncată = ieși). Ultimul rămas sau cartea cea mai mare câștigă runda; primul cu 3 runde câștigă. Informație ascunsă: vezi doar cartea ta.",
            make: || Box::new(curieri::Curieri::new()),
        },
        Playable {
            id: "fitil",
            title: "Fitil (variantă originală)",
            category: "Cărți",
            rules: "O Bombă în pachet: joci orice cărți, apoi tragi (trag). Fără Dezamorsare, bomba te scoate; cu ea o repui unde vrei (dezamorsez 0..N). Cărți: atac (adversarul trage de 2 ori), sari, amesteca, privire (primele 3), cere (adversarul îți dă o carte: dau X), pereche A|B|C (furi o carte la întâmplare). Informație ascunsă: vezi doar mâna ta, privirea ta și locul bombei puse de tine.",
            make: || Box::new(fitil::Fitil::new()),
        },
        Playable {
            id: "bufet",
            title: "Bufet (variantă originală)",
            category: "Cărți",
            rules: "Draft rapid de 10 cărți, 3 runde: alegi o carte (alege mici), mâinile se schimbă după revelare (revelare). Mici (perechi 5), salată (triplete 10), colțunași (1/3/6/10/15), cârnați 1-2-3 (muștarul îi triplează), pateuri (majoritate 6/3), papanași (la final cei mai mulți +6, cei mai puțini −6). Alegerea adversarului rămâne ascunsă până la revelare.",
            make: || Box::new(bufet::Bufet::new()),
        },
        Playable {
            id: "expeditii",
            title: "Expediții",
            category: "Cărți",
            rules: "Cinci masive (B F R D C), cărți 2–10 și 3 angajamente (x) pe culoare; mână de 8. Tură: joci o carte pe expediția ta (valori crescătoare) sau o arunci (juc B5, arunc B5), apoi tragi din pachet sau iei de pe o grămadă (trag, ia F). Scor pe expediție: (suma − 20) × (1 + angajamente), +20 cu ≥8 cărți. Se termină la ultima carte din pachet.",
            make: || Box::new(expeditii::Expeditii::new()),
        },
        Playable {
            id: "artificii",
            title: "Artificii (cooperativ, partener AI)",
            category: "Cooperativ",
            rules: "Tu și AI-ul construiți 5 șiruri de artificii (R G V A W) de la 1 la 5; vezi mâna partenerului, nu pe a ta. Tură: juc N, arunc N (+1 indiciu) sau indiciu R / indiciu 3 (cost 1 din 8 jetoane). 3 fitile arse = pierdeți. Obiectiv: 20 de puncte (echipa reușește = „Câștigă Jucătorul 1”; altfel remiză). Încheierea e o mutare forțată (sfârșit).",
            make: || Box::new(artificii::Artificii::new()),
        },
        Playable {
            id: "orasel",
            title: "Orășel (variantă originală)",
            category: "Familie",
            rules: "Zaruri și construit orașe: arunci 1 sau 2 zaruri (după Gară), clădirile albastre plătesc în orice tură, verzi doar în a ta, roșii (cafenea, grătar) iau de la cel care aruncă; cumperi o clădire (cumpar lan) sau un reper (construiesc gara/mall/parc/turn). Câștigă cine are primul cele 4 repere. Mutări: arunc 1, reiau, pastrez, nimic.",
            make: || Box::new(orasel::Orasel::new()),
        },
        Playable {
            id: "cursa_probelor",
            title: "Cursa probelor (joc de familie cu mini-probe, 2–4 jucători)",
            category: "Familie",
            rules: "Joc de familie propriu: pistă circulară de 30 de câmpuri și zar d6 (zar). Fiecare câmp declanșează o mini-probă: N numărare (raspuns 1|2|3), M memorie scurtă (gata, apoi literele A–D pe rând), D duel de zaruri (duel 2), R pariu de risc (arunc / opresc: până la 3 zaruri, un 1 pierde, țintă 9), L logică/aritmetică generată procedural (raspuns 1|2|3), O odihnă (a doua șansă). Începi cu jucatori 2|3|4; ceilalți sunt roboți (Easy–Expert; răspund doar din ce vezi și tu, cu zgomot). Primul cu 5 medalii câștigă; după 3 ture, cele mai multe medalii; egalitate la vârf = remiză. Mutări automate: rezolv, robot.",
            make: || Box::new(cursa_probelor::CursaProbelor::new()),
        },
        Playable {
            id: "capcana",
            title: "Capcana (variantă originală)",
            category: "Party",
            rules: "Bluf și licitație cu discuri ascunse: 3 flori + 1 capcană. Pui discuri (pun floare / pun capcana), apoi licitezi (licitez N) câte discuri poți întoarce fără capcană; celălalt ridică sau dă pas. Provocatorul întoarce întâi ale lui, apoi ale adversarului (intorc). 2 reușite câștigă; capcana te costă un disc.",
            make: || Box::new(capcana::Capcana::new()),
        },
        Playable {
            id: "congkak",
            title: "Congkak (Malaezia)",
            category: "Abstract",
            rules: "2 × 7 case, 7 semințe în fiecare, depozit pe jucător; semănat trigonometric, depozitul adversarului se sare. Ultima în depozit = încă o mutare; în casă ocupată = ridici și continui; în casă goală proprie = capturezi casa opusă plus sămânța ultimă (opusa goală: doar sămânța ultimă); în casă goală adversă = tura se termină. Runda se încheie când jucătorul la mutare nu mai are semințe; apoi casele se reumplu din depozite (7 fiecare), cele nerefăcute sunt arse. Cine nu mai poate umple nicio casă pierde. Mutări pe rând, nu simultan. Mutări: 1–7.",
            make: || Box::new(congkak::Congkak::new()),
        },
        Playable {
            id: "pallanguzhi",
            title: "Pallanguzhi (Tamil Nadu)",
            category: "Abstract",
            rules: "2 × 7 cupe, 5 semințe în fiecare. Ridici o cupă proprie și semeni trigonometric; apoi: cupa următoare ocupată = o ridici și continui; goală = capturezi cupa de după ea și tura se termină. Captură suplimentară: orice cupă care ajunge la exact 4 semințe e capturată. Fără semințe la mutare = jocul se termină, fiecare ia ce are pe partea lui; câștigă cine are peste 35. O singură rundă. Mutări: 1–7.",
            make: || Box::new(pallanguzhi::Pallanguzhi::new()),
        },
        Playable {
            id: "ayo",
            title: "Ayo (Ayoayo, Yoruba)",
            category: "Abstract",
            rules: "2 × 6 gropi, 4 semințe în fiecare; semănat trigonometric cu semănat în lanț (ultima într-o groapă ocupată = continui) și groapa de start sărită. Captură: ultima sămânță într-o groapă goală proprie ia groapa opusă (fără sămânța ultimă). Dacă adversarul nu mai are semințe, ești obligat să-l hrănești; dacă nu poți, jocul se termină. Mutări: 1–6.",
            make: || Box::new(ayo::Ayo::new()),
        },
        Playable {
            id: "oware",
            title: "Oware (Ghana)",
            category: "Abstract",
            rules: "2 × 6 gropi, 4 semințe în fiecare; semănat trigonometric (groapa de start e sărită la 12+ semințe), fără lanț. Captură: dacă ultima sămânță cade într-o groapă adversă ajunsă la 2 sau 3 semințe, le iei și continui înapoi cât timp grupele au 2 sau 3; mutarea care ar goli complet partea adversă nu capturează (grand slam). Dacă adversarul nu are semințe, trebuie să-l hrănești. Câștigă primul cu 25. Mutări: 1–6.",
            make: || Box::new(oware::Oware::new()),
        },
        Playable {
            id: "bao",
            title: "Bao (Bao la Kiswahili)",
            category: "Abstract",
            rules: "Tablă 4 × 8, două rânduri pe jucător, 22 de semințe în rezervă și nyumba cu 6. Namua: captura e obligatorie (pui o sămânță într-o groapă din față, iei groapa opusă și o semeni de la capătul rândului); altfel pui o sămânță și semeni sensul ales. Mtaji: muți dintr-o groapă cu ≥ 2 semințe; captură din prima aterizare obligatorie. Câștigi golind rândul din față al adversarului. Mutări: 3r, 3l (sens), b3l (rândul din spate).",
            make: || Box::new(bao::Bao::new()),
        },
        Playable {
            id: "seega",
            title: "Seega",
            category: "Abstract",
            rules: "Tablă 5 × 5, 12 piese pe jucător. Plasare: două piese pe tură, fără centru; apoi mută cine a plasat ultima, un pas ortogonal. Captură custodială a unui șir de piese adverse între piesa mutată și o piesă proprie; cine capturează mută din nou. Blocaj: adversarul scoate o piesă proprie. Mutări: b2 (plasare), b2-b3, scoate-b2.",
            make: || Box::new(seega::Seega::new()),
        },
        Playable {
            id: "yote",
            title: "Yote",
            category: "Abstract",
            rules: "Tablă 5 × 6, 12 piese pe jucător, inițial în mână. La rând: pui o piesă, muți un pas ortogonal sau sari peste o piesă adversă (o captezi și mai scoți încă una la alegere; săritura se poate lanțui). Câștigă cine rămâne singur cu piese. Mutări: c3, c3-c4, c3xc5+a1, c3xc5+m, opreste.",
            make: || Box::new(yote::Yote::new()),
        },
        Playable {
            id: "hus",
            title: "Hus (Namibia)",
            category: "Abstract",
            rules: "Mancala 4 × 8, 2 semințe în fiecare groapă. Muți dintr-o groapă proprie cu ≥ 2 semințe; ultima sămânță într-o groapă ocupată: din rândul din față cu groapa opusă ocupată captezi coloana adversă (față + spate) și o semeni mai departe; altfel continui lanțul. Pierde cine nu mai are nicio groapă cu ≥ 2. Mutări: 3 (față), b3 (spate).",
            make: || Box::new(hus::Hus::new()),
        },
        Playable {
            id: "tab",
            title: "Tâb (Egipt)",
            category: "Clasic",
            rules: "Cursă cu 4 bețe-zaruri pe tablă 4 × 7 (variantă simplificată): 7 piese pe jucător; intri cu 1; 1, 4 și 6 dau aruncare în plus; drum în zigzag pe cele 4 rânduri; aterizarea pe o piesă adversă (în afara rândului ei de plecare) o captură. Câștigă primul care scoate toate piesele. Mutări: arunca, intra-a1, a1-a3, a1-iese, pas.",
            make: || Box::new(tab::Tab::new()),
        },
        Playable {
            id: "senet",
            title: "Senet",
            category: "Clasic",
            rules: "Cursă egipteană pe 30 de pătrate (reconstituirea lui Kendall), 5 piese pe jucător, 4 bețe (1–4, zero = 5; 1, 4, 5 dau aruncare în plus). Schimbi locul cu o piesă adversă neprotejată; nu treci de 26; 27 (apa) te trimite pe 15; ieșire din 26 cu 5, din 28 cu 3, din 29 cu 2, din 30 cu 1. Mutări: arunca, 7-9, 28-iese, pas.",
            make: || Box::new(senet::Senet::new()),
        },
        Playable {
            id: "whot",
            title: "Whot (cărți nigeriene)",
            category: "Cărți",
            rules: "Pachet de 54 de cărți (forme o t x p s, plus 5 Whot), 6 în mână. Joci aceeași formă sau același număr; Whot cere o formă. 1 și 8: mai joci o dată; 14: adversarul trage o carte; 2: trage 2, 5: trage 3 (se pot înlănțui). Câștigi golindu-ți mâna. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: o5, t14, w:s, trage, pas.",
            make: || Box::new(whot::Whot::new()),
        },
        Playable {
            id: "chkobba",
            title: "Chkobba (Tunisia)",
            category: "Cărți",
            rules: "Pachet de 40 de cărți (1–7, damă, valet, rege), 3 în mână, 4 pe masă. Iei o carte de aceeași valoare, altfel un grup cu aceeași sumă; golești masa = chkobba (nu la ultima carte). Punctaj: carte, denari, bermila (cei mai mulți de 7, apoi de 6), haya (7 de denari), chkobba; meci până la 11. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: 7d, 7d:3c+4b, imparte, urmatoarea.",
            make: || Box::new(scopa::Scopa::chkobba()),
        },
        Playable {
            id: "ronda",
            title: "Ronda (Maroc / Spania)",
            category: "Cărți",
            rules: "Pachet spaniol de 40 de cărți, 4 pe masă, 3 în mână. Joci o carte: dacă pe masă e același rang, iei toate cărțile acelui rang și apoi, în șir, rangurile următoare prezente. Puncte: ronda (pereche în mână) 1, trio 5, missa (masă golită) 1, peste 20 de cărți 1; meci până la 12. Informație ascunsă: vezi mâna Jucătorului 1. Mutări: 5o, Rb, imparte, urmatoarea.",
            make: || Box::new(ronda::Ronda::new()),
        },
        Playable {
            id: "shologuti",
            title: "Sholo Guti (Șaisprezece soldați)",
            category: "Abstract",
            rules: "16 piese pe jucător pe o tablă de 37 de puncte (grilă 5 × 5 cu diagonale plus două triunghiuri), rândul din mijloc gol. Mută un pas pe un punct liber vecin sau sare peste o piesă adversă (captură obligatorie, lanț opțional). Câștigă cine capturează tot sau blochează adversarul. Mutări: c2-c3, c1xc3, opreste.",
            make: || Box::new(shologuti::SholoGuti::new()),
        },
        Playable {
            id: "aadupuli",
            title: "Aadu Puli Attam (Oi și tigri)",
            category: "Abstract",
            rules: "3 tigri (Jucătorul 2) contra a 15 oi (Jucătorul 1) pe o tablă de 23 de puncte în formă de triunghi. Oile se plasează pe rând, apoi se mută un pas; tigrii se mută sau sar peste o oaie (captură, lanț opțional). Tigrii câștigă cu 5 oi capturate; oile câștigă imobilizându-i. Mutări: b3, b3-b4, vxb2, opreste.",
            make: || Box::new(aadupuli::AaduPuli::new()),
        },
        Playable {
            id: "mangala",
            title: "Mangala (mancala turcească)",
            category: "Abstract",
            rules: "2 × 6 gropi, 4 semințe, câte un depozit. Prima sămânță rămâne în groapa de start. Ultima în depozit: mai muți; ultima într-o groapă adversă ajunsă la număr par: o iei; ultima într-o groapă proprie goală: iei groapa opusă. Câștigă depozitul mai mare. Mutări: 1–6.",
            make: || Box::new(mangala::Mangala::new()),
        },
        Playable {
            id: "dama",
            title: "Dama (dame turcești)",
            category: "Abstract",
            rules: "8 × 8, 16 piese pe jucător pe rândurile 2–3. Piesele merg înainte sau lateral, capturează ortogonal; dama zboară pe orice distanță. Captura e obligatorie, cu cele mai multe piese; piesele luate se scot la finalul secvenței. Mutări: a3-a4, a3xa5xc5.",
            make: || Box::new(dama::Dama::new()),
        },
        Playable {
            id: "okey",
            title: "Okey (2 jucători)",
            category: "Clasic",
            rules: "106 plăci (4 culori × 1–13 × 2 + 2 okey false), indicator și joker. 15/14 plăci; tragi din teanc (trage) sau iei ultima aruncată (ia), apoi arunci (ata-r5). Câștigi închizând 14 plăci în serii (≥ 3 consecutive, aceeași culoare) și grupe (3–4 cu același număr și culori diferite): bitir-r5. Informație ascunsă: vezi mâna Jucătorului 1.",
            make: || Box::new(okey::Okey::new()),
        },
        Playable {
            id: "battleship",
            title: "Bătălia navală (10×10)",
            category: "Clasic",
            rules: "Flotă de 5 nave (5, 4, 3, 3, 2). Îți așezi navele (a1h = orizontal, a1v = vertical, sau aleator); flota AI-ului se așază printr-o mutare forțată (plasez) și rămâne ascunsă. Tragi pe rând (e5): apă, lovit sau scufundat. AI-ul vânează cu hartă de densitate și țintește, fără să citească flota ta. Final cu mutarea forțată recunosc. Mutări: a1h, aleator, plasez, e5, recunosc.",
            make: || Box::new(battleship::Battleship::new()),
        },
        Playable {
            id: "mastermind",
            title: "Spargerea codului (cu schimb de roluri)",
            category: "Clasic",
            rules: "Cod de 4 cifre 1–6 (repetiții permise), cel mult 10 încercări; răspuns N = poziție exactă, A = culoare corectă pe altă poziție. Runda 1: spargi codul ascuns al AI-ului. Runda 2: îți alegi un cod (cod:1234) și AI-ul îl sparge. Câștigă cine a avut nevoie de mai puține încercări (nespart = 11). Mutări: 1234, cod:1234, confirm.",
            make: || Box::new(mastermind::Mastermind::new()),
        },
        Playable {
            id: "memory",
            title: "Perechi (joc de memorie)",
            category: "Clasic",
            rules: "24 de cărți (12 perechi A–L), 6 coloane × 4 rânduri. O tură = două întoarceri (mutări separate): pereche = punct și mută din nou, altfel cărțile se întorc și mută adversarul. AI-ul are memorie limitată (memorează 70% din cărțile văzute, uită cu timpul). Mutări: a1–f4.",
            make: || Box::new(memory::Memory::new()),
        },
        Playable {
            id: "bingo",
            title: "Bingo (75 de bile)",
            category: "Clasic",
            rules: "Fișă 5 × 5 (B 1–15, I 16–30, N 31–45 cu centru liber, G 46–60, O 61–75). Tragerea e o mutare (trag), marcarea o alta (marchez); cu o combinație (rând, coloană, diagonală) strigi sau continui. Cine strigă singur câștigă; dacă strigă amândoi = remiză. Fișa AI-ului e ascunsă. Mutări: trag, marchez, strig, continui.",
            make: || Box::new(bingo::CallGame::bingo()),
        },
        Playable {
            id: "loteria",
            title: "Lotería (imagini)",
            category: "Clasic",
            rules: "Tablă 4 × 4 cu 16 din 54 de imagini (set propriu, nu cel comercial). Tragi o imagine (trag), o marchezi dacă o ai (marchez); combinații: rând, coloană, diagonală, cele patru colțuri sau pătratul din centru; strigi sau continui. Fișa AI-ului e ascunsă. Mutări: trag, marchez, strig, continui.",
            make: || Box::new(bingo::CallGame::loteria()),
        },
        Playable {
            id: "pachisi",
            title: "Pachisi (cursă cu cauri)",
            category: "Zaruri",
            rules: "6 cauri: niciunul deschis = 25, 1 deschis = 10, 2 = 2, 3 = 3, 4 = 4, 5 = 5, 6 = 6; 6, 10 și 25 sunt grațioase (încă o aruncare și poți scoate o piesă din bază). Pistă de 48 de pătrate, castele sigure din 6 în 6, două piese proprii = bloc, aterizarea pe o piesă singură o captură (+ aruncare). Ținta exactă de pe pistă; din culoarul de acasă orice aruncare ≥ distanța. Mutări: intră, sau progresul piesei (ex. 12).",
            make: || Box::new(pachisi::Pachisi::new()),
        },
        Playable {
            id: "carrom",
            title: "Carrom (discret, fără fizică)",
            category: "Abstract",
            rules: "Tablă 9 × 9, 12 monede pe jucător, damă, patru buzunare în colțuri. Lovitura = coloană + direcție (L/S/R) + forță 1–3, traseu deterministic cu ricoșeu; monedă proprie băgată = +1 și mai tragi; moneda adversarului = punct pentru el; dama se acoperă la lovitura următoare (+3) sau revine; strikerul în buzunar = greșeală. Mutări: eS3, aL1.",
            make: || Box::new(carrom::Carrom::new()),
        },
        Playable {
            id: "arhipelag",
            title: "Arhipelagul în Flăcări (cucerire cu zaruri)",
            category: "Strategie",
            rules: "Variantă originală: 14 insule în 4 regiuni cu bonus, 7 insule și 3 armate fiecare la start. Tura: întărire (pune Fiord / pune Fiord*), atac (ataca Fiord>Prag, apoi mutarea forțată arunc: 3 zaruri contra 2, egalitatea o câștigă apărătorul; gata atac), o mutare de armate (muta A>B n) sau gata. Câștigă cine rămâne singur; după 70 de ture, cine are mai multe insule.",
            make: || Box::new(arhipelag::Arhipelag::new()),
        },
        Playable {
            id: "roata",
            title: "Roata Destinului (parcurs de viață)",
            category: "Familie",
            rules: "Variantă originală: roata (1–6) e afișată; mutarea avansez alege ramura la bifurcații (facultate/ucenicie, afacere/salariu) și, pe pătratul Casă, cumpar/nu. Plată la fiecare trecere, copii și casă valorează la final. Scor = bani + 30 casă + 8 per copil. Mutări: avansez, avansez facultate, avansez ucenicie, avansez afacere, avansez salariu, avansez cumpar.",
            make: || Box::new(roata::Roata::new()),
        },
        Playable {
            id: "zaruri5",
            title: "Cinci Zaruri (13 categorii)",
            category: "Zaruri",
            rules: "Variantă originală: 13 runde, 5 zaruri, până la 3 aruncări. Aruncarea e mutare forțată (arunc); între aruncări ții zaruri (tine 3 3 5, tine -) sau scrii o categorie (scor casa). Bonus 35 la 63 sus; fără regula joker. Câștigă totalul mai mare.",
            make: || Box::new(zaruri5::Zaruri5::new()),
        },
        Playable {
            id: "placute",
            title: "Plăcuțe (seturi și serii)",
            category: "Cărți",
            rules: "Variantă originală: 106 plăci (4 culori ×1–13 ×2 + 2 jokeri), 14 în mână. Pune serii/grupuri (pune r3 r4 r5), adaugă la masă (adauga r6 la 2), schimbă un joker (schimb J din 2 cu r4); prima ieșire ≥ 30. Terminare: gata; altfel trag / pas; anuleaza. Fără reorganizarea liberă a mesei. Informație ascunsă: vezi doar mâna ta.",
            make: || Box::new(placute::Placute::new()),
        },
        Playable {
            id: "turnul",
            title: "Turnul Șovăitor (turn de blocuri)",
            category: "Familie",
            rules: "Simulare discretă, deterministă: 12 straturi de 3 blocuri; iei un bloc de sub stratul complet de sus și îl pui deasupra. Turnul stă dacă centrul de masă de deasupra fiecărui strat cade în învelișul blocurilor lui rămase; cine îl prăbușește pierde. Mutări: ia 4b pune a.",
            make: || Box::new(turnul::Turnul::new()),
        },
        Playable {
            id: "cunoastere",
            title: "Cupa Cunoașterii (întrebări, 6 felii)",
            category: "Party",
            rules: "Banca proprie de 126 de întrebări în 6 categorii; alege o categorie (alege istorie), răspunde a–d (raspund b), apoi verific. Corect = felia și încă o întrebare; greșit = trece tura. Prima 6/6 câștigă. AI-ul nu are cunoștințe (ghicește); pentru meci echilibrat jucați în doi oameni.",
            make: || Box::new(cunoastere::Cunoastere::new()),
        },
        Playable {
            id: "ceata",
            title: "Câmpul Ceții (piese ascunse, 10×10)",
            category: "Strategie",
            rules: "Variantă originală: 40 de piese pe parte, ranguri 1–X, mine (B), steag (F), lacuri. Vezi doar piesele tale și pe ale adversarului după bătălie. Mutări: e4-e5, e4xe5 (atac), apoi mutarea forțată bataie. Săpătorul dezamorsează minele, Iscoada bate Mareșalul. 100 de mutări fără bătălie = remiză.",
            make: || Box::new(ceata::Ceata::new()),
        },
        Playable {
            id: "umbra",
            title: "Umbra Urbei (urmărire cu bilete)",
            category: "Familie",
            rules: "Variantă originală, J1 = 4 detectivi, J2 = Umbra (ascunsă; apare după mutările 4, 9, 14, 19). Hartă de 30 de stații pe jos/tram/metro cu bilete limitate; Umbra are 3 bilete negre. Mutări: d2 17 tram, x 12 jos. Detectivii câștigă prinzând sau blocând Umbra; Umbra, rezistând 22 de mutări. Se afișează stațiile posibile.",
            make: || Box::new(umbra::Umbra::new()),
        },
        Playable {
            id: "zidurile",
            title: "Zidurile Mișcătoare (labirint cu plăci culisante)",
            category: "Familie",
            rules: "Variantă originală: tablă 7×7 cu plăci mobile; introduci placa de rezervă (mete N2 r1) apoi muți pionul pe un drum deschis (mergi c3) ca să iei comorile în ordine, apoi acasă. Nu poți anula inserția adversarului. După 100 de ture, cine are mai multe comori.",
            make: || Box::new(zidurile::Zidurile::new()),
        },
        Playable {
            id: "prajituri",
            title: "Drumul Prăjiturilor (cursă de culori)",
            category: "Familie",
            rules: "Variantă originală: 58 de pătrate colorate, pachet de cărți (simple, duble, locuri), scurtături și pătrate lipicioase. Cartea e afișată; singura mutare este trag. Primul în Palatul de zahăr câștigă.",
            make: || Box::new(prajituri::Prajituri::new()),
        },
        Playable {
            id: "portret",
            title: "Portretul Misterios (ghicește personajul)",
            category: "Familie",
            rules: "Variantă originală: 24 de personaje inventate cu 14 trăsături. Întrebi (intreb ochelari), adversarul răspunde corect prin mutare forțată (raspund da/nu); ghicești (ghicesc Matei), iar adversarul dă verdictul printr-o mutare forțată (verdict corect / verdict gresit). Ghicit greșit = pierzi.",
            make: || Box::new(portret::Portret::new()),
        },
        Playable {
            id: "briscola",
            title: "Briscola (Italia)",
            category: "Cărți",
            rules: "Pachet italian de 40 de cărți (culori d, c, s, b; rang 2 4 5 6 7 F C R 3 A), 3 în mână, atuul întors rămâne jos. Nu e obligatoriu să răspunzi la culoare; levata o ia cartea mai mare de aceeași culoare sau un atu; câștigătorul trage primul. Puncte: A 11, 3 10, R 4, C 3, F 2; peste 60 câștigi, 60–60 remiză. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: Ad, 3s, Rb, 7c.",
            make: || Box::new(briscola::Briscola::new()),
        },
        Playable {
            id: "scopa",
            title: "Scopa (Italia)",
            category: "Cărți",
            rules: "Pachet italian de 40 de cărți (d c s b; rang 1–7 F C R, valori 1–10), 3 în mână, 4 pe masă. Joci o carte: dacă pe masă e aceeași valoare, iei acea carte; altfel iei un grup cu aceeași sumă; altfel o lași. Golești masa = scopa (1 punct, nu la ultima carte). Punctaj: carte, denari, settebello (7d), primiera, scope; meci până la 11. Informație ascunsă: vezi doar mâna Jucătorului 1; împărțirea următoare e mutare forțată. Mutări: 7d, 7d:3c+4b, 7d:7s, imparte, urmatoarea.",
            make: || Box::new(scopa::Scopa::new()),
        },
        Playable {
            id: "durak",
            title: "Durak (Rusia)",
            category: "Cărți",
            rules: "36 de cărți (6..A, culori c d h s), 6 în mână, atu întors jos. Atacatorul pune o carte; apărătorul o bate (mai mare de aceeași culoare sau atu) ori ia. Atacatorul adaugă cărți cu valori deja pe masă (cel mult 6, cel mult câte are apărătorul; 5 la început); gata = rundă încheiată, apărătorul devine atacator. După fiecare rundă se trage până la 6 (mutare forțată umple). Fără pachet, cine rămâne primul fără cărți câștigă, celălalt e durak; ambii = remiză. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 6h, 9h>6h (bate 6h cu 9h), iau, gata, umple.",
            make: || Box::new(durak::Durak::new()),
        },
        Playable {
            id: "belote",
            title: "Belote (Franța, 2 jucători)",
            category: "Cărți",
            rules: "32 de cărți (7..A; c d h s), 5 în mână + o carte întoarsă. Licitație în două tururi: prends (atu = culoarea cărții întoarse) / pas, apoi prends-c|d|h|s; toți pas = redistribuire. Cine ia primește cartea și se împarte până la 8 (imparte); 16 cărți rămân nejucate. Obligatoriu culoarea, altfel tai cu atu, la atu urci. Atu: J 20, 9 14, A 11, 10 10, K 4, Q 3; altfel A 11, 10 10, K 4, Q 3, J 2. Belote 20, dix de der 10, capot 90; contractul cade dacă ai mai puțin decât adversarul. Meci până la 301. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: prends, pas, prends-h, imparte, Jh, 10s, urmatoarea.",
            make: || Box::new(belote::Belote::new()),
        },
        Playable {
            id: "skat",
            title: "Skat (2 jucători)",
            category: "Cărți",
            rules: "32 de cărți (7..A; c ♣, s ♠, h ♥, d ♦), 10 în mână, 2 în skat, 10 ascunse nejucate. Licitația: cine a împărțit spune 18 20 22 23 24 27 … (licit N), celălalt ține (tin) sau pas; declarantul ia skat-ul (skat) și pune 2 cărți (pune 10h Jc) sau joacă din mână (hand), apoi alege jocul: trefla, pica, cupa, caro (atu = juniorii + culoarea), grand (doar juniorii), null (declarantul nu ia nicio levată). Valoare = bază × (matadori + 1 + mână + schneider + schwarz); câștig +valoarea, pierdere −2×, supralicitare = pierdere. 61 din 120 de puncte. Meci de 6 mâini. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: licit 18, tin, pas, skat, trefla, grand, Jh, reimparte, urmatoarea.",
            make: || Box::new(skat::Skat::new()),
        },
        Playable {
            id: "tarot",
            title: "Tarot francez (78 de cărți, 2 jucători)",
            category: "Cărți",
            rules: "78 de cărți (c d h s: 1–10 V C D R; atuuri T1–T21; Scuza Ex), 36 în mână și 6 în chien. Licitație într-un singur tur: pas, petite, garde, garde-sans, garde-contre (multiplicatori 1/2/4/6); cine ia primește chien-ul (chien) și pune jos 6 cărți (ecart Vh). Culoare obligatorie, altfel taie cu atu, la atu urci; Scuza nu câștigă. 91 de puncte; cerut 56/51/41/36 cu 0/1/2/3 oudlers; scor (25 + diferența + 10 petit au bout) × multiplicator, zero-sum; meci de 3 mâini. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: petite, pas, chien, ecart 2h, T21, Rs, Ex, reimparte, urmatoarea.",
            make: || Box::new(tarot::Tarot::new()),
        },
        Playable {
            id: "cribbage",
            title: "Cribbage",
            category: "Cărți",
            rules: "2 jucători, 6 cărți: dai 2 în crib (al dealerului), se taie startul (valet = 2 pentru dealer), pegging până la 31 (15, 31, perechi, serii, go) și numărarea mâinilor și a cribului (15-uri, perechi, serii, culoare, valet). Tabla de colac de 121 de găuri e desenată în text. Primul la 121 câștigă. Mutări: d 5h 6s, taie, 5h, go, numara, imparte.",
            make: || Box::new(cribbage::Cribbage::new()),
        },
        Playable {
            id: "euchre",
            title: "Euchre (2 jucători)",
            category: "Cărți",
            rules: "24 de cărți (9–A), 5 fiecare, o carte întoarsă. Runda 1: pas / up (dealerul ia cartea și dă una); runda 2: numești altă culoare (dealerul e obligat). Valetul de atu (bower dreapta) și cel de aceeași culoare (stânga, tot atu) sunt cele mai mari. Maker: 3–4 levate = 1 punct, 5 = 2, euchre = 2 pentru adversar; până la 10. Mutări: pas, up, nume h, dis 9h, Jh, imparte.",
            make: || Box::new(euchre::Euchre::new()),
        },
        Playable {
            id: "inimi",
            title: "Inimi (evitarea punctelor, 2 jucători)",
            category: "Cărți",
            rules: "13 cărți fiecăruia din 52 (restul nevăzut). Răspunzi obligatoriu la culoare, ia cartea cea mai mare din culoarea condusă. Fiecare inimă = 1 punct, Dama de pică = 13; nu conduci inimă până nu s-a jucat una. La 40 de puncte pierde cine are mai multe. Informație ascunsă: vezi doar mâna ta. Mutări: imparte, 7c, Qs, 10h.",
            make: || Box::new(inimi::Inimi::new()),
        },
        Playable {
            id: "pica",
            title: "Pică (licitație și atu pică, 2 jucători)",
            category: "Cărți",
            rules: "13 cărți fiecăruia din 52; atu = pica. Licitezi câte levate iei (licit 1…13) sau licit 0 = nil. Reușit: 10×licitație + 1 pe levată în plus (bag; 10 bag-uri = −100), picat: −10×licitație; nil ±50. Nu conduci pică până nu s-a jucat una. Până la 150 sau 12 mâini. Informație ascunsă: vezi doar mâna ta. Mutări: imparte, licit 4, As, 7h.",
            make: || Box::new(pica::Pica::new()),
        },
        Playable {
            id: "prinde",
            title: "Prinde (licitație exactă, 2 jucători)",
            category: "Cărți",
            rules: "13 runde cu mâini de 1…7…1 cărți; o carte întoarsă dă atuul. Spui câte levate iei (licit N); dealerul nu poate aduce suma licitațiilor la numărul de levate. Exact = 10 + licitația, altfel 0. Culoare obligatorie, atu bate tot. Informație ascunsă: vezi doar mâna ta. Mutări: imparte, licit 2, Kd, 4s.",
            make: || Box::new(prinde::Prinde::new()),
        },
        Playable {
            id: "whist",
            title: "Whist (2 jucători)",
            category: "Cărți",
            rules: "13 cărți fiecăruia din 52; ultima carte a dealerului se arată și dă atuul. Culoare obligatorie, atu bate tot. Cine ia peste 6 levate primește câte un punct pentru fiecare levată peste a șasea; jocul se încheie la 5 puncte (fără onoruri). Informație ascunsă: vezi doar mâna ta. Mutări: imparte, Ah, 9c.",
            make: || Box::new(whist::Whist::new()),
        },
        Playable {
            id: "tarneeb",
            title: "Tarneeb (2 jucători)",
            category: "Cărți",
            rules: "13 cărți fiecare din 52 (celelalte 26 ascunse). Licitezi levate 7–13 (urci cu 1 sau 2), câștigătorul numește atuul și conduce; făcut = punctele pe levatele luate (16 la 13), pierdut = minus cât ai licitat; adversarul ia levatele lui (+3 dacă ai căzut); până la 31. Mutări: pas, 7…13, atu h, Ah, imparte.",
            make: || Box::new(tarneeb::Tarneeb::new()),
        },
        Playable {
            id: "baloot",
            title: "Baloot (2 jucători)",
            category: "Cărți",
            rules: "32 de cărți, 8 fiecare, o carte întoarsă. Licitație în două runde: hokm (atu), sun (fără atu) sau pas. Atu: J 20, 9 14, A 11, 10 10, K 4, Q 3; ultima levată +10; la hokm tai obligatoriu cu atu și urci peste atu. Cumpărătorul trebuie să aibă mai multe puncte decât adversarul; baloot (K+Q de atu) = +2; până la 50. Mutări: pas, hokm, hokm h, sun, Jh, imparte.",
            make: || Box::new(baloot::Baloot::new()),
        },
        Playable {
            id: "tablanet",
            title: "Tablanet",
            category: "Cărți",
            rules: "52 de cărți: 4 pe masă, câte 6 fiecare, de patru ori. Capturezi cărți egale sau grupuri care se adună la valoarea ta (A = 1 sau 11; J, Q, K doar egale); captura e obligatorie și maximă. Punctaj: cele mai multe cărți 3, cele mai multe treflă 1, 10♦ 2, 2♣ 1, fiecare As 1, fiecare tablă 1. Mutări: 7h, 7h x 3c 4d, imparte.",
            make: || Box::new(tablanet::Tablanet::new()),
        },
        Playable {
            id: "truco",
            title: "Truco (Argentina)",
            category: "Cărți",
            rules: "Pachet spaniol de 40, 3 cărți fiecare, ierarhie specială (1 de spadă cea mai mare), 3 runde, până la 15. Cânți truco / retruco / vale4 (cel provocat: quiero, noquiero sau urcă) și envido / real / falta în prima rundă; mazo = te retragi. Fără flor. Mutări: 7e, truco, envido, quiero, noquiero, cuenta, mazo, reparte.",
            make: || Box::new(truco::Truco::new()),
        },
        Playable {
            id: "trix",
            title: "Trix (2 jucători)",
            category: "Cărți",
            rules: "32 de cărți, 16 fiecare, 10 mâini (5 „împărăția” fiecăruia). Proprietarul alege contractul: king (K♥ −75), queens (−25 fiecare), diamonds (−10 fiecare), slaps (−15 pe levată) sau trex (aranjament pornit din J, primul care termină +200). Mutări: king, queens, diamonds, slaps, trex, Jh, pas, imparte.",
            make: || Box::new(trix::Trix::new()),
        },
        Playable {
            id: "bridge",
            title: "Bridge duplicat (2 jucători)",
            category: "Cărți",
            rules: "Două tabele cu aceeași donă: J1 conduce N–S la tabla 1 și E–V la tabla 2 (J2 invers); fiecare vede ambele mâini ale cuplului lui. Licitație (1c…7n, x, xx, pass), declarant Sud / Vest, mâna moartă vizibilă după prima carte; scor duplicat (nimeni vulnerabil). Câștigă scorul combinat mai bun. Mutări: 1h, 3n, x, xx, pass, Ah.",
            make: || Box::new(bridge::Bridge::new()),
        },
        Playable {
            id: "canasta",
            title: "Canasta (2 jucători, o mână)",
            category: "Cărți",
            rules: "108 de cărți, 15 fiecare; 2 și jokerii sunt wild, 3 roșii = bonus, 3 negre doar se aruncă. Tura: roba (2 cărți) sau ia aruncarea, combinații de 3+ cărți de același rang (cu cel puțin 2 naturale), arunci o carte. Canastă (7 cărți) = 500 naturală / 300 mixtă; ieși doar cu canastă. O mână, fără minim de coborâre; câștigă scorul mai mare. Mutări: roba, ia, m 7, mw 7, w K, d 9c.",
            make: || Box::new(canasta::Canasta::new()),
        },
        Playable {
            id: "nave",
            title: "Escadrile din Rift (manevre secrete)",
            category: "Miniaturi",
            rules: "Manevre planificate simultan: 3 nave pe parte (Bivol, Șoim, Vrabie) pe o hartă 12×12 cu asteroizi. Fiecare navă primește în secret o manevră (cadran): drept dN, deviere blN/brN, viraj tlN/trN, întoarcere u, oprire s; roșii (viraj 3, u, s) dau stres, iar o navă stresată nu poate alege roșu. Apoi revelarea mută forțat, în ordinea îndemânării pilotului; ciocnire = oprire și fără foc; asteroid = 1 daună. Foc în arcul din față (Bivolul: turelă) la distanță 1–3, zaruri de atac contra zaruri de apărare, scut apoi coca. 10 runde. Informație ascunsă (AI-ul nu vede planurile tale). Mutări: A d2, A bl1, A tr2, A u, A s, revela, A>X, A pas, arunca.",
            make: || Box::new(nave::Nave::new()),
        },
        Playable {
            id: "patrula",
            title: "Patrula (skirmish cu echipă mică)",
            category: "Miniaturi",
            rules: "5 miniaturi individuale pe parte (Lider, 2 Pușcași, Greu, Asalt) pe o hartă 10×10 cu ziduri, acoperiri și 4 obiective; fiecare are răni proprii, armă de foc și armă de luptă. Părțile alternează activări (fiecare miniatură o dată pe rundă) cu ordine: Avans (mută și poate trage), Țintă (pe loc, +1 la lovire; lipit = luptă), Asalt, Retragere (doar lipit), Gardă. 6 runde; obiectivele dau puncte la finalul rundei. Mutări: A>d4, A>d4 xV, A xV, A!d4 xV, A<d4, A garda, arunca.",
            make: || Box::new(patrula::Patrula::new()),
        },
        Playable {
            id: "reactii",
            title: "Ordine și reacții (skirmish cu reacții)",
            category: "Miniaturi",
            rules: "4 soldați pe parte (Lider, Pușcaș, Lunetist, Asalt), hartă 10×10, zaruri d20. Partea activă are un ordin per soldat (+1 pentru Lider); fiecare ordin mută și/sau trage. În tura adversarului, inamicii care văd soldatul activ reacționează pe rând: pas, tira (reacție) sau esiva (doar ținta declarată); față în față câștigă cel mai mare rezultat reușit, egalul la critic. 8 runde; obiective. Mutări: A>d4 xV, A xV, A>d4, fin, V pas, V tira, V esiva, arunca.",
            make: || Box::new(reactii::Reactii::new()),
        },
        Playable {
            id: "ordine",
            title: "Ordine de bătaie (infanterie, zaruri de ordine)",
            category: "Miniaturi",
            rules: "Bătălie de infanterie pe hartă 12×8: Ofițer, 2 Grupe, Mitralieră și Mortier pe parte. Un zar de ordine pe unitate într-un săculeț; mutarea forțată trage extrage un zar la întâmplare și partea respectivă dă un ordin: fire, adv, run (asalt), amb, rally, down. Suprimare: marcajele de suprimare scad moralul și trebuie testate pentru a executa ordinul. 6 ture; obiective. Mutări: trage, B fire xW, B adv d4 xW, B run d4, B amb, B rally, B down, arunca.",
            make: || Box::new(ordine::Ordine::new()),
        },
        Playable {
            id: "gridiron",
            title: "Gridiron (sport fantastic pe ture)",
            category: "Miniaturi",
            rules: "Echipe de 5 pe un teren 12×7; 6 ture pe repriză, 2 reprize. O acțiune per jucător: mutare (escapare, ridicare minge, mutări în plus cu zar), tăvăleală cu zaruri de forță, asalt (mutare + tăvăleală), pasă, predare. O cădere sau o minge scăpată pierde tura. Purtătorul mingii în zona adversă = punct. Mutări: A>d4, A>d4 xV, A xV, A pass B, A hand B, A sus, fin, lovitura, arunca.",
            make: || Box::new(gridiron::Gridiron::new()),
        },
        Playable {
            id: "focare",
            title: "Focare (cooperativ, boli pe hartă, partener AI)",
            category: "Cooperativ",
            rules: "Variantă originală: voi doi, agronomi, opriți 4 boli ale culturilor pe o hartă cu 24 de așezări. Tura: 4 acțiuni (mers X, zbor X, charter X, naveta X, statie, tratez r, dau/iau, vindec r, pas), apoi tragi 2 cărți (trage; 3 epidemii în pachet; max 7 cărți: descarta X) și se infectează (infecteaza). Al 4-lea spor dintr-o boală produce focar în lanț. Vindecare: la stație, 3 cărți de aceeași culoare. Victorie (toate 4 vindecate) = „Câștigă Jucătorul 1”; înfrângere (6 focare, spori epuizați, pachet golit) = remiză. Încheierea e mutare forțată (sfârșit). Informație: mâinile sunt publice.",
            make: || Box::new(focare::Focare::new()),
        },
        Playable {
            id: "strajile",
            title: "Străjile Codrului (cooperativ de apărare, cărți de putere)",
            category: "Cooperativ",
            rules: "Variantă originală: voi, două duhuri (J1 Rădăcinilor, J2 Furtunii, AI), apărați o vale cu 8 ținuturi de invadatori. Tura: creștere (creste N / aduna / reia), apoi până la 2 cărți de putere (joc spini 3; plătești energia, ținut în raza cărții) și gata. Apoi runda invadatorilor, mutări forțate: devastare, constructie, explorare. Colonie ucisă = frică. Victorie: frică 28 sau valea curată după ultima carte de invadatori (= „Câștigă Jucătorul 1”); 10 blesteme sau invadatori rămași = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(strajile::Strajile::new()),
        },
        Playable {
            id: "farul",
            title: "Farul Fantomei (deducție cooperativă cu indicii vizuale)",
            category: "Cooperativ",
            rules: "Variantă originală: tu ești mediumul (J1), AI-ul este fantoma (J2) care îți trimite „viziuni” de câte 3 simboluri (viziune N). Patru etape: portretul, camera, obiectul (6 candidați cu câte 5 simboluri, ghic N sau asteapt; max 10 viziuni) și scena finală (3 candidați, o singură ghicire). Ghicirea se verifică într-o mutare forțată (verific). Victorie = „Câștigă Jucătorul 1”; eșec = remiză. Informație ascunsă: nu vezi soluția, nici mâna fantomei.",
            make: || Box::new(farul::Farul::new()),
        },
        Playable {
            id: "turnul_zorilor",
            title: "Turnul Zorilor Frânte (aventură cooperativă, fantasy eroic)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 + modificator ≥ prag. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("turnul_zorilor")),
        },
        Playable {
            id: "arhiva_abisala",
            title: "Arhiva Abisală (aventură cooperativă, groază cosmică de investigator)",
            category: "Cooperativ",
            rules: "Mecanica de test: d100 sub abilitate. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("arhiva_abisala")),
        },
        Playable {
            id: "ruinele_cartografilor",
            title: "Ruinele Cartografilor (aventură cooperativă, expediție în junglă)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 cu grade de succes. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("ruinele_cartografilor")),
        },
        Playable {
            id: "noapte_de_catifea",
            title: "Noapte de Catifea (aventură cooperativă, urban fantasy cu vampiri)",
            category: "Cooperativ",
            rules: "Mecanica de test: grămadă de d10. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("noapte_de_catifea")),
        },
        Playable {
            id: "padurea_sticlei",
            title: "Pădurea Ochilor de Sticlă (aventură cooperativă, fantasy de basm sumbru)",
            category: "Cooperativ",
            rules: "Mecanica de test: 3d20 față de trei atribute. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("padurea_sticlei")),
        },
        Playable {
            id: "neon_si_rune",
            title: "Neon și Rune (aventură cooperativă, cyber-urban cu magie)",
            category: "Cooperativ",
            rules: "Mecanica de test: grămadă de d6 cu lovituri. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("neon_si_rune")),
        },
        Playable {
            id: "targul_ciumat",
            title: "Târgul Ciumat (aventură cooperativă, fantasy sumbru)",
            category: "Cooperativ",
            rules: "Mecanica de test: d100 cu grade de succes. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("targul_ciumat")),
        },
        Playable {
            id: "insula_celor_doua_luni",
            title: "Insula celor Două Luni (aventură cooperativă, fantasy clasic)",
            category: "Cooperativ",
            rules: "Mecanica de test: 2d6 + rating față de țintă. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("insula_celor_doua_luni")),
        },
        Playable {
            id: "hotii_din_ceata",
            title: "Hoții din Ceață (aventură cooperativă, jaf într-un oraș cu ceață)",
            category: "Cooperativ",
            rules: "Mecanica de test: cel mai mare d6. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("hotii_din_ceata")),
        },
        Playable {
            id: "crucea_de_cenusa",
            title: "Crucea de Cenușă (aventură cooperativă, apocalipsă sumbră)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 față de prag fix. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("crucea_de_cenusa")),
        },
        Playable {
            id: "strada_de_otel",
            title: "Strada de Oțel (aventură cooperativă, cyberpunk de cartier)",
            category: "Cooperativ",
            rules: "Mecanica de test: d10 exploziv. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("strada_de_otel")),
        },
        Playable {
            id: "tinutul_furtunilor",
            title: "Ținutul Furtunilor Vii (aventură cooperativă, fantasy colorat)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 + modificator ≥ prag. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("tinutul_furtunilor")),
        },
        Playable {
            id: "epava_din_inelul_sapte",
            title: "Epava din Inelul Șapte (aventură cooperativă, operă spațială)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 + modificator ≥ prag. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("epava_din_inelul_sapte")),
        },
        Playable {
            id: "fiordul_gheatii_negre",
            title: "Fiordul Gheții Negre (aventură cooperativă, fantasy nordic)",
            category: "Cooperativ",
            rules: "Mecanica de test: d20 sub abilitate. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("fiordul_gheatii_negre")),
        },
        Playable {
            id: "mlastina_strigoilor",
            title: "Mlaștina Strigoilor (aventură cooperativă, vânătoare de monștri)",
            category: "Cooperativ",
            rules: "Mecanica de test: d10 exploziv. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("mlastina_strigoilor")),
        },
        Playable {
            id: "satul_lanternelor",
            title: "Satul Lanternelor Stinse (aventură cooperativă, groază de sat cu spirite)",
            category: "Cooperativ",
            rules: "Mecanica de test: d100 sub abilitate. Variantă originală de aventură cooperativă: voi doi (J1 și J2, AI-ul poate fi partenerul) contra Maestrului-AI care nu trișează, pe 7–8 scene într-un graf mic (explorare, test, luptă, odihnă, stăpân final). Începeți cu alege N (arhetip). Apoi eroii se alternează: ir N, atac N, parez, special [N], bea N, cauta, incearca, odihna, cedez. Zarurile sunt mutări forțate publice (zaruri, eveniment, maestrul); după un zar ratat poți face refac (costă o unitate din resursă) sau accept. Victorie (stăpânul învins) = „Câștigă Jucătorul 1”; eroi doborâți sau timp epuizat = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(aventura::Aventura::new("satul_lanternelor")),
        },
        Playable {
            id: "cenusii",
            title: "Noapte Cenușie (cooperativ de supraviețuire pe grilă)",
            category: "Cooperativ",
            rules: "Variantă originală: Ilinca (J1) și Radu (AI) strâng 3 baterii (B) și ajung amândoi la ieșire (E) pe o grilă 7×6, ținând piept umbrelor. 3 acțiuni pe tură: mers n|s|e|v, atac 1 c3 (zaruri într-o mutare forțată: zaruri), cauta (extrage), ia, pansez, gata; apoi umbre și aparitie (forțate). Rătăciți, alergători și colosi (necesită armă cu forță 2). 4 răni = mort. Victorie = „Câștigă Jucătorul 1”; moartea, roi sau 40 de ture = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(cenusii::Cenusii::new()),
        },
        Playable {
            id: "cripta",
            title: "Cripta Clopotelor (aventură tactică cu cărți de acțiune)",
            category: "Cooperativ",
            rules: "Variantă originală: Paznicul de Far (J1) și Vânătoarea de Umbre (AI) coboară într-o criptă; 3 scenarii (omorâți toți monștrii / ajungeți la ieșire / omorâți Regele Clopot). Rundă: fiecare alege 2 cărți (joc 3 5) sau odihna; revela; apoi în ordinea inițiativei jucați sus 3 jos 5: mers c4, atac m2 / atac toti, aplic, sari; atacul trage modificatorul într-o mutare forțată (trage). Monștrii (monstri) trag cărți de comportament. Victorie = „Câștigă Jucătorul 1”; ambii epuizați sau 25 de runde = remiză. Încheierea e mutare forțată (sfârșit).",
            make: || Box::new(cripta::Cripta::new()),
        },
        Playable {
            id: "lupii",
            title: "Lupii din Hârtop (roluri ascunse, masa = roboți)",
            category: "Party",
            rules: "Tu ești locul 1 la o masă de 8–9; ceilalți sunt roboți care văd doar informația publică. Noaptea acționează lupii și rolurile speciale (văzător, paznic/doctor, vrăjitoare, vânător); ziua toți votează cine pleacă, rolul celui eliminat se arată. Satul câștigă fără lupi; lupii când sunt cât restul. Alegi presetarea la început: preset mafie (clasic), preset sat (stil francez), preset stepa (stil chinezesc). Mutări: preset sat, ucide 3, cerceteaza 3, pazeste 3, salveaza, otraveste 3, dormi, vot 3, impusca 3, masa.",
            make: || Box::new(lupii::Lupii::new()),
        },
        Playable {
            id: "santinela",
            title: "Străjerii Farului (misiuni cu echipe, roluri ascunse)",
            category: "Party",
            rules: "Cinci locuri (tu + 4 roboți): trei paznici (unul e Ochiul, care știe sabotorii) și doi sabotori (unul e Vulpea). Cinci misiuni cu echipe de 2, 3, 2, 3, 3: conducătorul propune, toți votează, cei din echipă aleg în secret succes sau sabotaj (doar sabotorii pot sabota). 3 reușite = paznicii, dar Vulpea poate ghici Ochiul; 3 ratate sau 5 respingeri = sabotorii. Mutări: echipa 1 3, da, nu, succes, sabotaj, tinta 4, masa.",
            make: || Box::new(santinela::Farul::new()),
        },
        Playable {
            id: "consiliul",
            title: "Sfatul Negustorilor (alegeri și legi, roluri ascunse)",
            category: "Party",
            rules: "Cinci locuri (tu + 4 roboți): trei negustori, un complice și Cămătarul (se cunosc între ei). Președintele nominalizează un cancelar, toți votează; guvernul ales trage 3 legi, una e aruncată, una promulgată (6 P Prosperitate, 11 M Monopol în pachet). 5 P sau Cămătarul executat = negustorii; 6 M sau Cămătarul cancelar după 3 M = conspiratorii. A 3-a lege M: vezi pachetul; a 4-a și a 5-a: execuție. Mutări: cancelar 3, da, nu, arunca M, promulga P, executa 4, masa.",
            make: || Box::new(consiliul::Consiliul::new()),
        },
        Playable {
            id: "valea",
            title: "Valea Uscată (duel western, roluri ascunse)",
            category: "Cărți",
            rules: "Cinci pistolari (tu + 4 roboți): Șerif (public), Adjunct, doi Haiduci, Trădător. Legea câștigă dacă mor haiducii și trădătorul, haiducii dacă moare Șeriful, trădătorul rămânând singur. Pe tură tragi 2 cărți: foc (o dată, distanță 1; 2 cu pușcă), ocolire, leac, salon, mitralieră, pușcă, panică; arunci la câte vieți ai. Secetă după runda 30. Mutări: foc 3, leac, salon, mitraliera, pusca, panica 2, incheie, scarta foc, ocolire, accept, masa.",
            make: || Box::new(valea::Valea::new()),
        },
        Playable {
            id: "hruba",
            title: "Hoinari prin Hrubă (monștri și comori, masa = roboți)",
            category: "Cărți",
            rules: "Trei hoinari (tu + 2 roboți); primul la nivelul 10 omorând un monstru câștigă. Deschizi ușa: monstru (lupți: forța ta trebuie să DEPĂȘEASCĂ nivelul lui; poți folosi poțiuni +3; sau fugi cu zar 5+), blestem sau carte în mână; fără monstru poți căuta sau vâna un monstru din mână. Roboții pot îngreuna lupta. Max 5 cărți la final de tură; limită 90 de ture. Mutări: deschide, cauta, echipeaza arma3, vaneaza monstru4, crestere, forta, lupta, fugi, scapa blestem, masa.",
            make: || Box::new(hruba::Hruba::new()),
        },
        Playable {
            id: "apus",
            title: "Imperii în Apus (cucerire cu popoare în declin)",
            category: "Strategie",
            rules: "Hartă de 12 ținuturi, trei jucători (tu + 2 roboți), 8 ture. Alegi o combinație popor + dar din șase (cea din față e gratis, celelalte costă monede lăsate pe cele sărite). Un ținut costă 2 + 1 (munte) + jetoanele ocupantului; trebuie să fie vecin cu ale tale; ultima cucerire cu zarul de întărire. Câștigi 1 monedă pe ținut + bonusuri; poporul vlăguit intră în declin și alegi altul. Mai multe monede câștigă. Mutări: alege 2, continua, cucereste 5, zar 7, gata, declin.",
            make: || Box::new(apus::Apus::new()),
        },
        Playable {
            id: "fulger",
            title: "Fulger (simbolul comun, cronometru simulat)",
            category: "Party",
            rules: "31 de cărți cu 6 simboluri; oricare două au exact un simbol comun. Numești simbolul comun dintre cartea ta și cea din mijloc; masa (robot) are un timp de reacție tras la zar și greșește uneori. Cine e corect și mai rapid ia cartea din mijloc (+1 punct). Simbol greșit = runda masei. Mutări: numele simbolului (ex. luna, stea), cronometru.",
            make: || Box::new(fulger::Fulger::new()),
        },
        Playable {
            id: "roade",
            title: "Roade de Toamnă (legume și schimburi, masa = roboți)",
            category: "Cărți",
            rules: "Trei fermieri (tu + 2 roboți), 8 legume, 2 ogoare (al treilea 3 monede), mână cu ordine fixă: plantezi mereu prima carte. Tura: plantezi 1–2 cărți, întorci 2 cărți (le plantezi sau le oferi mesei în schimbul unei cărți orbe), iei 3 cărți. Poți recolta oricând (cărți → monede după tabel). Roboții îți pot oferi cărți. Final la a doua golire a pachetului. Mutări: planteaza 1, recolta 2, compra, treci, ofera, accepta ceapa, refuza.",
            make: || Box::new(roade::Roade::new()),
        },
        Playable {
            id: "tcg_evo",
            title: "Cărți colecționabile: evoluție și energie (Făpturi de Câmp)",
            category: "TCG",
            rules: "Set original de 40+ cărți (făpturi evolutive pe elemente, energii, ajutoare); pachete pregătite de 30 (J1 Pădurea, J2 Vulcanul). Pregătire: activ N, banca N, gata; 3 cărți-premiu. Tură: pesca (forțat), apoi banca N, evolueaza N T (nu pe făptură nouă, nu în prima ta tură), energie N T (una pe tură), joaca N [T] (un singur «sprijin»), retragere bK, apoi atac 1|2 (încheie tura) sau fin. Țintă: a (activa), b1..b5 (banca mea), e1..e5 (a adversarului). Slăbiciunea dublează daunele; efecte: vindecare, daune la bancă, recul, aruncare de energie, amețire. Făptura activă căzută = adversarul ia un premiu, tu alegi promova bK. Câștigi cu toate premiile, cu adversarul fără făpturi sau dacă el rămâne fără pachet. Informație ascunsă: vezi doar mâna J1.",
            make: || Box::new(tcg_evo::TcgEvo::new()),
        },
        Playable {
            id: "tcg_zone",
            title: "Cărți colecționabile: monștri în zone și capcane (Hotarele Ceții)",
            category: "TCG",
            rules: "Set original de 44 de cărți (monștri, vrăji, capcane); pachete pregătite de 30 (J1 Hotarul, J2 Mlaștina), 8000 PV. Tură: pesca (forțat), apoi o invocare pe tură (nivel 1-4 liber, 5-6 un tribut, 7+ două): invoca 2, seteaza 2 (întors, în apărare), invoca 2 tribut 1,3; intoarce I, schimba I, joaca N [e2|m1|g1] (vrăji), seteaza N (capcană), ataca 1 e2 / ataca 1 direct, fin. ATK vs ATK / ATK vs apărare (Pătrunde, Vânător). După fiecare atac apărătorul primește continua sau activa T (capcane puse într-o tură anterioară). J1 nu atacă în tura 1; pachet gol = pierdere. Informație ascunsă: cărțile întoarse ale adversarului nu se văd.",
            make: || Box::new(tcg_zone::TcgZone::new()),
        },
        Playable {
            id: "tcg_lant",
            title: "Cărți colecționabile: pitch, lanț și echipament (Lanțuri de Oțel)",
            category: "TCG",
            rules: "Set original de 40+ cărți; pachete pregătite de 30 (J1 Gardiana Zorilor, J2 Vânătorul de Furtuni), 30 viață, mână de 4, patru piese de echipament și o armă. Resursele vin din pitch: atac 2 pitch 1,3 (cărțile aruncate în pitch merg la fundul pachetului la sfârșitul turei). Un punct de acțiune pe tură: atac H, arma, joaca H (acțiuni, nu costă punctul), fin; «continuă» îl returnează. Lanț: apărătorul alege bloc 1,3 echip 2 (sau bloc fara), apoi reacții de atac / de apărare (reactie H pitch ..., pasa). Daune = putere - apărare. După 25 de ture pe jucător câștigă cine are mai multă viață. Informație ascunsă: vezi doar mâna J1.",
            make: || Box::new(tcg_lant::TcgLant::new()),
        },
        Playable {
            id: "tcg_cerneala",
            title: "Cărți colecționabile: cerneală, misiuni și provocări (Povestiri din Cerneală)",
            category: "TCG",
            rules: "Set original de 42 de cărți (personaje, acțiuni); pachete pregătite de 30 (J1 Pădurea Cântătoare, J2 Muntele de Fier), 7 cărți în mână; primul la 15 faimă câștigă. Tură: pesca (mutare forțată la începutul fiecărei ture; în prima tură J1 o scrie tot, dar nu trage nicio carte), cerneala N (o carte cu cerneală pe tură în călimară = 1 cerneală pe tură), joaca N [e2|m1] (personajele se usucă o tură), misiune P (aduce faimă), provocare P e Q (doar contra personajelor ÎNCLINATE), fin. Cuvinte-cheie: Elan, Gardian, Zburător, Provocator, Rezistență. Daunele rămân până la eliminare. Informație ascunsă: vezi doar mâna J1.",
            make: || Box::new(tcg_cerneala::TcgCerneala::new()),
        },
        Playable {
            id: "spionii",
            title: "Spionii Cuvintelor (grilă 5×5)",
            category: "Party",
            rules: "Variantă proprie de joc de spionaj cu cuvinte: grilă de 25 de cuvinte, 9 agenți roșii (J1), 8 albaștri (J2), 7 neutre, 1 asasin. Maestrul-spion al fiecărei echipe este calculatorul: dă ca indiciu numele unei teme din tabela proprie (MARE, CIRC…) și un număr. Tu ești ghicitorul: ghici cuvânt (fără diacritice), apoi dezvaluie întoarce cartea; cel mult număr+1 ghiciri, stop încheie tura. Asasinul pierde pe loc. AI-ul ghicește după aceeași tabelă de teme, fără să vadă cheia.",
            make: || Box::new(spionii::Spionii::new()),
        },
        Playable {
            id: "povestitor",
            title: "Povestitorul (cărți-simbol, 2 jucători)",
            category: "Party",
            rules: "Variantă proprie de joc de povestit cu cărți: 231 de descrieri-simbol. Povestitorul alege o carte și unul din indiciile ei (spune 3a); celălalt pune o momeală (momeala 2); amesteca adaugă 3 cărți-fantomă; ghicitorul votează (vot N), Publicul votează după potrivirea cuvintelor-cheie, apoi dezvaluie punctează (3/3 dacă exact un votant găsește cartea; altfel 0 și 2; +1 pentru momeală votată de Public). Până la 12 puncte sau 14 runde.",
            make: || Box::new(povestitor::Povestitor::new()),
        },
        Playable {
            id: "gura",
            title: "Gura Satului (completează fraza)",
            category: "Party",
            rules: "Variantă proprie de joc de completat fraze: 205 fraze cu gol și 231 de răspunsuri originale. Judecătorul alternează; celălalt joacă un răspuns (joaca N), amesteca adaugă 2 răspunsuri ale casei, judecătorul alege cea mai nostimă completare (alege N), numara dă punctul: jucătorului dacă a ales cartea lui, judecătorului dacă a ales una a casei. Primul la 6 puncte sau după 12 runde. AI-ul potrivește etichetele frazei cu ale răspunsului.",
            make: || Box::new(gura::Gura::new()),
        },
        Playable {
            id: "treicai",
            title: "Trei Căi (spune, mimează, desenează)",
            category: "Party",
            rules: "Variantă proprie de joc de explicat și ghicit, în terminal: 218 acțiuni și meserii cu aspecte predefinite redate pe rând vorbit, mimă sau schiță. Descriptorul alege indicii (arata N), adversarul poate fura ghicind (ghici N, pas), apoi robot lasă colegul-robot să ghicească dintre 6 candidați. Cu cât mai puține indicii, cu atât mai multe puncte (4…1); ghicire greșită a adversarului −1. 12 runde, rolurile alternează.",
            make: || Box::new(treicai::new()),
        },
        Playable {
            id: "oprite",
            title: "Cuvinte Oprite (descrie fără cuvinte interzise)",
            category: "Party",
            rules: "Variantă proprie de joc cu cuvinte interzise: 233 de cărți cu cuvânt-țintă, 4 interdicții și indicii-propoziție predefinite; indiciile care calcă o interdicție sunt fauluri (−1, detectate automat). Descriptorul alege indicii (arata N), adversarul poate fura ghicind (ghici N, pas), colegul-robot ghicește la pasul robot. 10 runde, cu cât mai puține indicii cu atât mai multe puncte.",
            make: || Box::new(oprite::new()),
        },
        Playable {
            id: "schita",
            title: "Schița (desen simbolic)",
            category: "Party",
            rules: "Variantă proprie de joc de desenat și ghicit: 224 de cuvinte, fiecare cu 6 tușe textuale (cerc, coadă, urechi ascuțite…). Desenatorul arată tușele una câte una (arata N), adversarul poate fura ghicind (ghici N, pas), apoi colegul-robot ghicește dintre 6 candidați (robot). Cu cât mai puține tușe, cu atât mai multe puncte (5…1). 10 runde, rolurile alternează.",
            make: || Box::new(schita::new()),
        },
        Playable {
            id: "dictionar",
            title: "Dicționar Viu (explică în rafală)",
            category: "Party",
            rules: "Variantă proprie de joc de explicat cuvinte: 215 cuvinte cu trei explicații predefinite. Fiecare tură are 9 acțiuni (arata N sau treci); fiecare cuvânt ghicit de colegul-robot (robot) sau furat de adversar (ghici N) valorează 1 punct; rădăcina cuvântului în explicație este fault (−1). 6 ture, rolurile alternează.",
            make: || Box::new(dictionar::new()),
        },
        Playable {
            id: "impostor",
            title: "Impostorul din Grilă (4 locuri)",
            category: "Party",
            rules: "Variantă proprie de joc cu impostor: 14 subiecte × 16 cuvinte; un cuvânt secret pe care îl știu toți mai puțin impostorul (J1, J2 sau unul din cei doi roboți). Fiecare dă un indiciu dintr-un cuvânt (indiciu cuvânt), apoi J1 și J2 votează (vot N), roboții votează la numara. Impostorul prins ghicește cuvântul (ghici cuvant, verifica). Puncte: vot corect +1, impostor neprins +2, prins fără să ghicească: nevinovații +2, prins care ghicește +1. 8 runde.",
            make: || Box::new(impostor::Impostor::new()),
        },
        Playable {
            id: "cuvantunic",
            title: "Cuvânt Unic (indicii care se anulează)",
            category: "Party",
            rules: "Variantă proprie de joc cu indicii de un cuvânt, în duel: 224 de cuvinte cu lexicon de indicii. Cel care dă indiciul (indiciu N) are doi martori robotici; indiciile identice se anulează (dezvaluie). Ghicitorul alege dintre 6 candidați (ghici N) sau pas; verifica: ghicit +1 ghicitorului, greșit +1 celui care a dat indiciul. 12 runde, rolurile alternează.",
            make: || Box::new(cuvantunic::CuvantUnic::new()),
        },
        Playable {
            id: "mosia",
            title: "Moșia (roluri, plantații și clădiri)",
            category: "Strategie",
            rules: "Joc original: 7 roluri (rol cultivator/constructor/producator/negustor/capitan/vistier/zidar); fiecare alege 3 pe rând, toți execută acțiunea, cine a ales are un privilegiu. Cultivator: planta 2 sau cariera; constructor: construieste targ (costul scade cu carierele); negustor: vinde z; capitan: expediaza z 1 (nave 4 și 6, un fel de marfă pe navă). Culturile p i z t c (prețuri 0–4). Câștigă cine are mai multe puncte (expediere + clădiri + monede/4). Fără colonii, ci doar plantații care produc direct.",
            make: || Box::new(mosia::Mosia::new()),
        },
        Playable {
            id: "gospodaria",
            title: "Gospodăria (plasare de muncitori și fermă)",
            category: "Strategie",
            rules: "Joc original, 6 runde, recoltă după rundele 2, 4, 6: pui pe rând un muncitor pe o acțiune liberă (pune lemn / lut / peste / oi / cereale / legume / zilier / arat / semanat 2 / camera / tarc / familie / inceput); fiecare acțiune ține un singur muncitor pe rundă, lemn, lut, peste și oi se acumulează dacă nu sunt luate. La recoltă câmpurile dau cereale, se hrănește familia (2 pe membru, altfel cerșetorii −3) și oile se înmulțesc. Punctaj pe categorii (câmpuri, țarcuri, cereale, oi, legume), camere, familie. Ferma e doar contoare, fără grilă.",
            make: || Box::new(gospodaria::Gospodaria::new()),
        },
        Playable {
            id: "cetatuia",
            title: "Cetățuia (muncitori pe drum de clădiri, castel)",
            category: "Strategie",
            rules: "Joc original, 9 runde: 5 muncitori pe jucător, plasați pe rând pe locurile unui drum (pune ferma / padure / cariera / piata / atelier / arhitect / castel / pod), primele 3 plasări costă 1 ban, apoi 2; pas dă 1 ban primului care pasează. La pod muncitorul mută jalonerul (pune pod +2): se activează doar locurile până la jaloner. Atelierul și arhitectul construiesc clădiri proprii (construieste gater) care cresc drumul și dau puncte când le folosește altcineva; la castel livrezi piese (castel l2 p1). Final: cetate completă sau runda 9; majoritatea pieselor +4.",
            make: || Box::new(cetatuia::Cetatuia::new()),
        },
        Playable {
            id: "taramuri",
            title: "Tărâmuri (transformarea terenului și piste de ritual)",
            category: "Strategie",
            rules: "Joc original pe hartă 7×6 cu 7 tipuri de teren pe o roată: J1 e de tip 0, J2 de tip 3. Pe rând o acțiune: colonie c3 (transformi terenul vecin în al tău, plătind lopeți, și pui o locuință), oras c3, templu c3 (îmbunătățiri), preot foc|apa|pamant|aer (+2 trepte pe pista de cult; la 5 privilegiu), pas. Început: inicial c3. 6 runde cu venit și un bonus de rundă; la final puncte pe piste, cel mai mare grup de clădiri și monede. Fără mană, fără facțiuni distincte, vecinătate pe 4 direcții.",
            make: || Box::new(taramuri::Taramuri::new()),
        },
        Playable {
            id: "furnale",
            title: "Furnale (rețea de legături și industrii)",
            category: "Strategie",
            rules: "Joc original, 8 runde, 2 acțiuni pe tură: pe o hartă cu 8 orașe construiești industrii (fabrica 3 t; c cărbune, f fier, t textil, p port) și legături (link 2-4) lipite de rețeaua ta; cărbunele se ia din cea mai apropiată mină legată sau din piață (2 bani), fierul din orice fierărie sau piață. Plăcuța se întoarce când se golește sau se vinde (vinde 4, prin legături până la un port) și dă venit și puncte; la final legăturile punctează după plăcuțele întoarse din orașele lor. imprumut = +8 bani, −2 puncte. Fără cărți, o singură eră.",
            make: || Box::new(furnale::Furnale::new()),
        },
        Playable {
            id: "reverdire",
            title: "Reverdire (variantă originală)",
            category: "Strategie",
            rules: "Motor de resurse și proiecte de cărți: ridici Umiditatea, Aerul și Lacurile (RR). O generație începe cu mutarea forțată generatie, apoi Cercetare (compra N, 3 L; gata), acțiuni una pe tură (joc N, vinde N, pompa, irigare, plantare, asezare, conv sem, conv caldura, pas; apoi pune c3), producție. Cărămizile plătesc proiectele B, Cristalele pe cele S. Scor: RR + PV + arbori + orașe. Informație ascunsă: mâna, oferta și pachetul.",
            make: || Box::new(reverdire::Reverdire::new()),
        },
        Playable {
            id: "salina",
            title: "Salina (variantă originală)",
            category: "Strategie",
            rules: "Plasare de agenți și construire de pachet, 10 runde sau 10 PV. agent 3 e (carte + locație a–l cu pictograma potrivită; la locațiile de luptă e–h: agent 3 e 2 = desfășori 2 trupe), revela apoi compra N / apa (1 Apă = +1 persuasiune) / sare (1 Sare = +1 sabie) / fin. Conflictul rundei: forța = 2 × trupe + săbii; premiul I/II; runda pregătește runda următoare. Informație ascunsă: mâna adversarului și pachetele.",
            make: || Box::new(salina::Salina::new()),
        },
        Playable {
            id: "catun",
            title: "Cătun (variantă originală)",
            category: "Strategie",
            rules: "Plasare de muncitori și cătun de cărți, 4 anotimpuri (2,3,4,6 muncitori). lucru a (locație a–h; lucru g L la Han), joc N / piata N (carte din mână / piață, max 15 în cătun), sezon (pregătire: muncitorii se întorc, producătoarele lucrează; în Toamnă ieși din joc). Scor: puncte de pe cărți, jetoane, prosperitate. Informație ascunsă: mâna adversarului și pachetul.",
            make: || Box::new(catun::Catun::new()),
        },
        Playable {
            id: "solzi",
            title: "Solzi (variantă originală)",
            category: "Cărți",
            rules: "Motor de cărți-reptile și amfibieni cu habitate, hrană și ouă, 4 runde (8,7,6,5 acțiuni). joc N p|s|b (vietate în Pădure/Stâncă/Baltă, plătești hrană și ouă), padure I V (hrană), stanca p1 s2 (ouă), balta (cărți). Puterile cărților se declanșează doar în habitatul acțiunii. Obiective de rundă; scor: puncte de pe cărți + ouă + obiective. Informație ascunsă: mâna adversarului și pachetul.",
            make: || Box::new(solzi::Solzi::new()),
        },
        Playable {
            id: "poieni",
            title: "Poieni (variantă originală)",
            category: "Strategie",
            rules: "Plăcuțe hexagonale de habitat și jetoane de animale. Tura: ia N (pereche din piață), pune q,r rN (plăcuța lipită de ținutul tău, rotire 0–5), animal q,r (jetonul pe o plăcuță liberă care îl acceptă) sau arunca. Jeton-natură: nat N M (orice plăcuță cu orice jeton), nat reset. 20 de ture. Scor: animale Urs/Vidră/Zimbru/Gaiță/Vulpe cu reguli proprii, cel mai mare coridor al fiecărui habitat (+3 pentru cel mai mare), jetoane-natură.",
            make: || Box::new(poieni::Poieni::new()),
        },
        Playable {
            id: "epoci",
            title: "Epoci (variantă originală)",
            category: "Cărți",
            rules: "Draft de cărți în 3 epoci cu comerț între vecini, pentru J1, J2 și Orașul Oarb (automat, al treilea oraș din inel, vecin cu amândoi). Fiecare epocă: 7 cărți pe oraș, 6 ture; alegi o carte și pasezi mâna. joc N (construiești, cumpărând resursele lipsă de la vecini cu 2 monede, 1 cu reducere), etapa N (cartea ține loc de etapă a minunii), vinde N (+3 monede). Scor: civice, minune, victorii militare (1/3/5, înfrângere −1), 1 p la 3 monede, știință, bresle. Împărțirea epocii următoare: imparte.",
            make: || Box::new(epoci::Epoci::new()),
        },
        Playable {
            id: "tinuturi",
            title: "Ținuturi (variantă originală)",
            category: "Strategie",
            rules: "Domino de teritorii cu coroane, 2 jucători: regat 9×9 cu castel în e5, încape în 7×7. 24 de piese-domino trase din setul nostru de 48 (generate de noi) în rânduri de câte 4: proprietarul piesei cu numărul mic joacă primul. Tura: pune e6-e7 (prima jumătate pe e6) lipit de castel sau de același teritoriu, apoi alege 2 din rândul următor; descarta doar dacă nu ai nicio așezare. Primul rând: alege N (J1, J2, J2, J1); revelare dezvăluie rândul. Scor: regiune = pătrate × coroane, +5 fără aruncări, +10 castel central.",
            make: || Box::new(tinuturi::Tinuturi::new()),
        },
        Playable {
            id: "rezervatia",
            title: "Rezervația (variantă originală)",
            category: "Strategie",
            rules: "Rezervație naturală cu cărți-animale și sponsori. Cinci acțiuni (Faună, Amenajare, Sponsori, Cărți, Conservare) într-un rând: puterea = locul (1–5); cea jucată trece pe locul 1. animal 2 e1 (la putere ≥ 4 încă un animal sau gata), construieste padure 3 / chiosc / pavilion, sponsor 3 / finantare, carti / carti v2, proiect 1 / donatie. Pauză la ritm 9: venit. Scor = apel + 2×conservare; la 50 jocul se încheie la sfârșitul rundei. Informație ascunsă: mâna adversarului și pachetul.",
            make: || Box::new(rezervatia::Rezervatia::new()),
        },
        Playable {
            id: "hotar",
            title: "Hotarul (variantă originală)",
            category: "Strategie",
            rules: "Motor de resurse și teritorii pe hartă hexagonală (a1–g4), facțiuni cu avantaje diferite (alege luncenii/munteanii/padurenii/campenii). Tura: o secțiune a tablei tale (nu cea de data trecută): acțiunea de sus (s1 mars h:g1>f1 / bani, s2 putere / carti, s3 produce a1 b2 / muncitor, s4 comert grane / faima, sau sari), apoi cea de jos (mobilizeaza f1, imbunatateste mars, construieste moara f1, inrolare 2, sau gata). Luptă: lupta P C (putere și carte angajate pe ascuns), apoi rezolva; egal câștigă apărătorul. 5 stele sau 32 de ture încheie jocul; scor = monede + stele + hexuri + resurse (după popularitate). Informație ascunsă: cărțile de luptă.",
            make: || Box::new(hotar::Hotar::new()),
        },
        Playable {
            id: "codru",
            title: "Codrul Nesupus (război asimetric, variantă originală)",
            category: "Strategie",
            rules: "Pădure cu 8 poieni și drumuri; J1/J2 primesc la sorți Regența (gărzi, Cherestea/Atelier/Cazarmă, lemn) sau Frunzișul (luptători, simpatie, suporteri, baze). Regența: trag, 3 acțiuni (mars A>B n, lupta P, recruteaza, construieste c|a|k P — în poiană stăpânită, 1 lemn), seara fabrica P, gata. Frunzișul: trag, 3 + baze acțiuni (mobilizeaza fel, propaga P, revolta P, recruteaza, mars, lupta). Lupta: ambuscada/fara, contra/lasa, apoi zar (gherila: apărând, Frunzișul lovește cu zarul mare). Mâna max 5 (arunca fel). 18 puncte sau 30 de ture. Informație ascunsă: mâna și suporterii adversarului.",
            make: || Box::new(codru::Codru::new()),
        },
        Playable {
            id: "sideral",
            title: "Coroana Siderală (4X compact, variantă originală)",
            category: "Strategie",
            rules: "9 sisteme (Citadela în centru, inel, două capete), planete cu resurse/influență, nave, 5 tehnologii. Runda: alegi pe rând 2 din 6 cărți de strategie (alege Comert), apoi acțiuni în ordinea inițiativei: taktic A>B n (activezi B, muți n nave; adauga C>B n; gata), carte Nume (efect principal; adversarul poate lua secundarul: sec / nu), pas. Lupta: zar până rămâne o singură parte; ocupi planeta cu ≥ r nave (r+1 dacă e a adversarului; capitalele nu se cuceresc). În sub-faze: nava Sistem, tech Nume, gata. Status: obiectiv N. 8 puncte sau 12 runde.",
            make: || Box::new(sideral::Sideral::new()),
        },
        Playable {
            id: "ceasornic",
            title: "Ceasornicul din Valea Ceții (deducție cu povestitor, masa = roboți)",
            category: "Party",
            rules: "Variantă originală: 12 locuri (tu + 11 roboți), 14 roluri în scenariu (7 localnici din 9, Bețivul, Sfântul, Otrăvitorul, Moștenitoarea, Strigoiul). Povestitorul e automat; Bețivul și cei otrăviți primesc informații false. Noaptea rolurile cu informație (Ghicitoarea, Cronicarul, Veghea, Cercetașul, Groparul) află ceva, Călugărul ocrotește, Strigoiul ucide din noaptea 2. Ziua roboții își spun rolul (cer N îi întreabă), tu declari adevărul (declar) sau minți (mint rol), nominalizezi și votezi; cine strânge jumătate din vii și cel mai mult e executat. Morții au un vot fantomă. Binele câștigă dacă moare Strigoiul; răul la 2 vii, Sfânt executat sau 20 de zile. Mutări: ghicesc 3 7, otravesc 3, ocrotesc 3, ucid 3, declar, mint veghea, cer 4, vanez 5, treci, nominez 6, pas, da, nu, masa.",
            make: || Box::new(ceasornic::Ceasornic::new()),
        },
        Playable {
            id: "coroana",
            title: "Drumul spre Coroană (aventură pe inele, om contra roboți)",
            category: "Strategie",
            rules: "Variantă originală: tu și doi roboți, șase personaje cu avantaje (alege haiducul, vrajitoarea, negustorul, padurarul, pelerinul, calauza). Pornești din Câmpie (16 câmpuri), treci Podul Gri (plată de 2 Aur, Cheia sau luptă), în Țara Cenușie câștigi Pecetea în Turn (asalt forta/iscusinta), urci Scara Neagră în Inelul de Jar și intri pe Ușa Coroanei. Tura: zar, apoi dreapta sau stanga; cărți de aventură (monștri, obiecte, descântece, întâmplări), han, capelă, târg (cumpar sabie, plec). Lupta: Forța + d6 trebuie să depășească forța monstrului (lupt, lupt fulger, fug, ceata). Cine rămâne fără Viață cade la Han. Cine e în camera Coroanei dă Porunca: ceilalți pierd 1 Viață pe tură (la 0 sunt scoși). Mutări: zar, bea, leac, urc, urc plata, urc cheie, urc lupta, intru, mantin, masa.",
            make: || Box::new(coroana::Coroana::new()),
        },
        Playable {
            id: "coridor",
            title: "Coridor (cursă de pioni cu ziduri)",
            category: "Abstract",
            rules: "Tablă 9×9; J1 (X) pornește din e1 și ajunge pe rândul 9, J2 (O) din e9 spre rândul 1. Pe rând: un pas ortogonal (se sare peste pionul adversar vecin, în diagonală dacă în spate e zid/margine) SAU un zid (10 de jucător, lung cât două pătrate) care nu se suprapune, nu se încrucișează și nu închide complet calea nimănui. Remiză la 200 de semi-mutări. Mutări: e2 (pion), d4h (zid orizontal peste d4–e4), d4v (zid vertical la dreapta lui d4–d5).",
            make: || Box::new(coridor::Coridor::new()),
        },
        Playable {
            id: "drum",
            title: "Drum de pietre (5×5, stive)",
            category: "Abstract",
            rules: "21 de pietre plate și o căpetenie de jucător. Prima mutare: pui o piatră plată a adversarului. Apoi: pui o piatră pe un pătrat gol (c3 plată, sc3 zid, cc3 căpetenie) sau muți o stivă proprie: iei de sus 1–5 pietre și le lași pe o linie (+ sus, - jos, < stânga, > dreapta), cel puțin una pe pătrat; ex. c3> sau 3c3>21. Zidul și căpetenia blochează stivele; doar căpetenia lăsată singură pe zid îl turtește. Câștigă drumul (lanț de pietre plate/căpetenii proprii între margini opuse); la tablă plină, rezervă goală sau 150 de semi-mutări câștigă cine are mai multe pietre plate la vârf.",
            make: || Box::new(drum::Drum::new()),
        },
        Playable {
            id: "tripierd",
            title: "Trei-pierde (hexagonal)",
            category: "Abstract",
            rules: "Tablă hexagonală cu latura 5 (61 de celule, rânduri a–i). Pe rând pui o piesă pe o celulă goală: patru (sau mai multe) în linie câștigă, exact trei în linie (fără patru) pierde; tabla plină = remiză. Mutări: e5 (litera = rândul, cifra = poziția în rând).",
            make: || Box::new(tripierd::TriPierd::new()),
        },
        Playable {
            id: "konane",
            title: "Konane (dame hawaiiene)",
            category: "Abstract",
            rules: "8×8 plină în șah (negrul = J1 pe a1, c1, …). Deschidere: J1 ridică d4, e5, a1 sau h8 (remove d4), J2 ridică o piesă proprie vecină cu golul. Apoi sari ortogonal peste o piesă adversă într-un pătrat gol (o scoți), poți continua în aceeași direcție. Cine nu mai poate muta pierde. Mutări: remove d4, c3-e3, c3-g3 (salt dublu).",
            make: || Box::new(konane::Konane::new()),
        },
        Playable {
            id: "fanorona",
            title: "Fanorona (Madagascar)",
            category: "Abstract",
            rules: "Tablă 5×9 de puncte (diagonale doar pe punctele pare), 22 piese pe jucător. Mutare cu un pas pe un punct liber; dacă pasul continuă spre o piesă adversă o captură pe ea și șirul din spatele ei (abordare, a), iar dacă te îndepărtezi de o piesă adversă o captură la fel (retragere, w). Captura e obligatorie; după ea aceeași piesă poate continua (opțional) cu alte capturi, fără să repete direcția și fără să revină pe un punct deja ocupat în tură. Câștigi scoțând toate piesele adverse; remiză după 40 de semi-mutări fără captură. Mutări: e2-e3, c3-d3a, c3-d3w, c3-d3a-d4w-e5a (lanț; fiecare prefix al unui lanț apare în listă ca mutare separată, iar mutarea aleasă încheie tura: așa alegi unde te oprești).",
            make: || Box::new(fanorona::Fanorona::new()),
        },
        Playable {
            id: "tigri",
            title: "Tigri și capre (Nepal)",
            category: "Abstract",
            rules: "Tablă 5×5 de puncte (diagonale pe punctele pare). Tu ești caprele (20, în mână) și muți primul; calculatorul are 4 tigri în colțuri. Caprele se plasează pe rând pe puncte libere, apoi se mută cu un pas. Tigrii se mută cu un pas sau sar peste o capră vecină pe punctul liber de după ea, capturând-o (neobligatoriu). Tigrii câștigă cu 5 capre capturate; caprele câștigă imobilizând toți tigrii; remiză după 100 de semi-mutări fără captură în faza de mutare. Mutări: c3 (plasare), a1-b2 (pas), a1xc1 (săritură cu captură).",
            make: || Box::new(tigri::Tigri::new()),
        },
        Playable {
            id: "surakarta",
            title: "Surakarta (Java)",
            category: "Abstract",
            rules: "Tablă 6×6, 12 piese pe jucător. Mutare simplă pe un punct vecin liber (și diagonal). Captura: piesa aleargă pe unul din cele două circuite (liniile 2 și 5 = mic; 3 și 4 = mare, legate prin arcuri în colțuri), în oricare sens, peste puncte libere, trece prin cel puțin un arc și ia prima piesă întâlnită dacă e a adversarului; propriile piese blochează, iar locul de plecare contează ca liber. Câștigi scoțând toate piesele adverse; remiză după 60 de semi-mutări fără captură. Mutări: b2-b3 (pas), a2xb6 (captură).",
            make: || Box::new(surakarta::Surakarta::new()),
        },
        Playable {
            id: "hasami",
            title: "Hasami Shogi (Japonia)",
            category: "Abstract",
            rules: "Tablă 9×9, 9 piese pe jucător pe rândul din spate. Piesele se mută ca turnul (fără sărituri). Captură prin încadrare: un șir de piese adverse consecutive, pe rând sau coloană, aflat între piesa mutată și o altă piesă a ta, este scos; poți intra liber între două piese adverse. Câștigi lăsând adversarul cu cel mult o piesă sau blocat; remiză după 100 de semi-mutări fără captură (colțurile nu au regulă specială). Mutări: a1-a5 (fișiere a–i, rânduri 1–9).",
            make: || Box::new(hasami::Hasami::new()),
        },
        Playable {
            id: "puncte",
            title: "Puncte și căsuțe (5×5)",
            category: "Abstract",
            rules: "Rețea de 6×6 puncte (5×5 căsuțe). Pe rând tragi o linie între două puncte vecine; cine închide a patra latură a unei căsuțe o câștigă și mută din nou (o linie poate închide două căsuțe). Când s-au tras toate cele 60 de linii, câștigă cine are mai multe căsuțe (25, deci fără remiză). Mutări: oa1 (linie orizontală, de la punctul a1 spre dreapta), va1 (linie verticală, de la punctul a1 în sus); coloane a–f, rânduri 1–6.",
            make: || Box::new(puncte::Puncte::new()),
        },
        Playable {
            id: "strapungere",
            title: "Străpungere (8×8)",
            category: "Abstract",
            rules: "Fiecare jucător are 16 piese pe primele două rânduri. O piesă înaintează un pas drept pe un pătrat gol sau un pas în diagonală pe un pătrat gol ori ocupat de adversar (captură); înainte nu se capturează. Câștigă cine ajunge primul pe rândul de capăt al adversarului sau cine capturează toate piesele adverse; cine nu mai are nicio mutare pierde. J1 (X) urcă, J2 (O) coboară. Mutări: b2-b3, b2-c3, b2xc3.",
            make: || Box::new(strapungere::Strapungere::new()),
        },
        Playable {
            id: "izolare",
            title: "Izolare (7×7)",
            category: "Abstract",
            rules: "Fiecare jucător are un pion (X la d1, O la d7). O tură: muți pionul ca regina la șah (orice distanță în linie dreaptă sau diagonală, doar peste pătrate libere), apoi scoți definitiv de pe tablă un pătrat liber (nu cel cu pionul adversarului). Cine nu mai poate muta la începutul turei pierde; nu există remiză. Mutări: d1-d3/e5 (de la, la, pătratul scos).",
            make: || Box::new(izolare::Izolare::new()),
        },
        Playable {
            id: "rotire",
            title: "Rotire în cinci (6×6)",
            category: "Abstract",
            rules: "Tablă 6×6 din patru cadrane 3×3 (1 stânga-sus, 2 dreapta-sus, 3 stânga-jos, 4 dreapta-jos). O tură: pui o bilă pe un pătrat gol, apoi rotești un cadran cu un sfert de tură (d = în sensul acelor de ceasornic, s = invers). Cinci bile în linie (orizontal, vertical, diagonal) după rotire câștigă; dacă ambii jucători au cinci deodată sau tabla se umple fără cinci, e remiză. Mutări: c3/1d (bila pe c3, cadranul 1 spre dreapta).",
            make: || Box::new(rotire::Rotire::new()),
        },
        Playable {
            id: "coloane",
            title: "Coloane (dame în turnuri)",
            category: "Abstract",
            rules: "Tablă 7×7 pe pătratele închise, 11 piese pe jucător. Soldatul merge un pas în diagonală înainte, ofițerul (soldat ajuns pe ultimul rând) în toate diagonalele. Captura e obligatorie: sari peste o coloană adversă vecină pe un pătrat liber și iei doar piesa ei de sus, care ajunge sub coloana ta; săriturile se înlănțuie, o coloană nu poate fi sărită de două ori, promovarea încheie tura. Coloana e condusă de piesa de sus. Pierde cine nu mai poate muta; remiză după 80 de semi-mutări fără captură. Mutări: c3-d4 (pas), c3xe5xg3 (salturi).",
            make: || Box::new(coloane::Coloane::new()),
        },
        Playable {
            id: "bile",
            title: "Bile pe hexagoane (5 pe latură)",
            category: "Abstract",
            rules: "Tablă hexagonală cu 61 de celule, 14 bile pe jucător. Muți 1-3 bile proprii vecine pe aceeași linie cu un pas: pe lungime (poți împinge 1 sau 2 bile adverse dacă ai mai multe bile decât ele și locul de dincolo e liber sau în afara tablei) ori lateral (toate destinațiile libere). Bilele împinse în afară sunt capturate; câștigă cine scoate 6; remiză după 120 de semi-mutări fără captură. Celule: rândul a (jos) – i (sus) și coloana 1-9. Mutări: e5>ne (o bilă), c3-c5>se (grup, de la prima la ultima bilă); direcții ne, e, se, sw, w, nw.",
            make: || Box::new(bile::Bile::new()),
        },
        Playable {
            id: "patru",
            title: "Patru atribute (piesa dată)",
            category: "Abstract",
            rules: "16 piese distincte cu patru însușiri binare (culoare, înălțime, formă, vârf), tablă 4×4. În tura ta pui pe tablă piesa primită de la adversar, apoi alegi una dintre cele rămase și i-o dai lui; prima tură doar dă o piesă. Câștigă cine completează un rând, o coloană sau o diagonală de patru piese cu o însușire comună; tabla plină = remiză. Piesele se numesc 0-9, a-f (biții = însușirile). Mutări: >5 (prima tură), c3>7 (pui pe c3, dai piesa 7), c3 (mutare câștigătoare sau ultima).",
            make: || Box::new(patru::Patru::new()),
        },
        Playable {
            id: "cupe",
            title: "Cupe încuibate (4×4)",
            category: "Abstract",
            rules: "Tablă 4×4; fiecare jucător are 3 stive de câte 4 cupe (mărimi 1-4) încuibate, se ia doar cea de sus. Pui o cupă din rezervă pe o celulă goală sau peste una mai mică, sau muți o cupă proprie vizibilă pe una goală ori mai mică. Din rezervă poți acoperi o cupă adversă doar dacă face parte dintr-un șir de trei piese adverse vizibile pe o linie. Câștigă patru cupe vizibile pe o linie; dacă ridici o cupă și descoperi șirul de patru al adversarului, el câștigă. Remiză după 50 de semi-mutări fără punere din rezervă. Mutări: 3@b2 (din rezervă, mărimea 3), b2-c3 (mutare pe tablă).",
            make: || Box::new(cupe::Cupe::new()),
        },
        Playable {
            id: "poliedre",
            title: "Poliedre (duo, 14×14)",
            category: "Abstract",
            rules: "Tablă 14×14, 2 jucători, fiecare cu 21 de poliomino (1-5 pătrate, notate a-u). Prima piesă acoperă pătratul de start (e5 pentru J1, j10 pentru J2); următoarele trebuie să atingă o piesă proprie prin colț și niciodată pe latură (piesele adverse se pot atinge). Cine nu poate pune trece; la două treceri la rând jocul se termină și câștigă cine are mai puține pătrate în piesele nefolosite. Mutări: k3@e5 (piesa k, orientarea 3, colțul stânga-sus al dreptunghiului piesei la e5), pas.",
            make: || Box::new(poliedre::Poliedre::new()),
        },
        Playable {
            id: "impletitura",
            title: "Împletitura (echilibru pe covor cu cercuri colorate)",
            category: "Party",
            rules: "Covor de 4 rânduri (a–d) × 6 coloane (1–6) cu cercuri în 4 culori (Roșu, Albastru, Verde, Galben; câte 6, după seed), 2 jucători, fiecare cu 4 membre (ms, md, ps, pd) care încep în afara covorului. Tura începe cu rotița (gira), care dă un membru și o culoare; muți membrul pe un cerc liber de acea culoare (b3) la distanță cel mult 3 (Chebyshev) de celelalte membre ale tale de pe covor; un cerc nu poate avea două membre; poți rămâne pe cercul curent dacă are culoarea dată. Fără cerc valid, singura mutare este cad: cazi și pierzi; ultimul rămas câștigă (remiză după 300 de mutări fără cădere). Fără informație ascunsă; AI: Easy aleator, Normal păstrează flexibilitatea, Hard/Expert îți blochează cercurile. Mutări: gira, b3, cad.",
            make: || Box::new(impletitura::Impletitura::new()),
        },
        Playable {
            id: "pinochle",
            title: "Pinochle",
            category: "Cărți",
            rules: "48 de cărți (două de fiecare 9–A pe 4 culori), 12 fiecare, 2 jucători, primul în stoc. Se joacă 24 de levate; cât timp stocul nu e gol nu ești obligat să urmezi culoarea, după aceea trebuie să urmezi și să învingi când poți. Câștigătorul unei levate poate anunța o combinație (pereche, familie, pinochle, cuplu etc.) care conține cel puțin o carte nouă; la sfârșit se numără A, 10, K din levate. Primul la 1000. Fără schimbul lui 9 de atu. Mutări: 9h, anunt <combinatie>, fara, urmatoarea.",
            make: || Box::new(pinochle::Pinochle::new()),
        },
        Playable {
            id: "gin_rummy",
            title: "Gin Rummy",
            category: "Cărți",
            rules: "52 de cărți, 10 fiecare. La rând: iei din talon (talon) sau stoc (stoc), apoi arunci (dis X). Cu deadwood de cel mult 10 poți bate (bat X) sau da gin (gin X, 0 deadwood); adversarul poate adăuga cărți la combinațiile tale (adauga X la N). Se punctează diferența de deadwood, undercut pentru apărător; până la 100. Fără big gin. Mutări: talon, pas, stoc, dis 9h, bat 9h, gin 9h, adauga 5h la 1, gata, arata, urmatoarea.",
            make: || Box::new(gin_rummy::GinRummy::new()),
        },
        Playable {
            id: "crazy_eights",
            title: "Crazy Eights (Optarii)",
            category: "Cărți",
            rules: "52 de cărți, 7 fiecare. Joci o carte de aceeași culoare sau valoare cu cea de deasupra; optarul (8) se joacă oricând și alege culoarea (8h:s). Dacă nu poți, tragi (trage) și poți juca ori pasa. Cine rămâne fără cărți ia punctele din mâna adversarului (8 = 50, figuri = 10, A = 1); până la 100. Mutări: 7h, 8h:s, trage, pas, urmatoarea.",
            make: || Box::new(crazy_eights::CrazyEights::new()),
        },
        Playable {
            id: "pitch",
            title: "Pitch (Setback)",
            category: "Cărți",
            rules: "52 de cărți, 6 fiecare, 2 jucători. Licitezi 2–4 (licit N) sau pas; licitatorul conduce, iar culoarea primei cărți devine atu. Răspunzi la culoare sau tai cu atu. Patru puncte: High, Low, Jack, Game; licitatorul care nu atinge contractul pierde cât a licitat. Primul la 11. Mutări: pas, licit 3, 9h, urmatoarea.",
            make: || Box::new(pitch::Pitch::new()),
        },
        Playable {
            id: "bid_whist",
            title: "Bid Whist (2 jucători)",
            category: "Cărți",
            rules: "Adaptare pentru doi jucători: 54 de cărți (cu doi jokeri), 12 fiecare și un kitty de 6. Licitezi 3–6 (licit N) sau pas; câștigătorul ia kitty-ul și aruncă 6 cărți, apoi alege Uptown (A mare), Downtown (A mic) sau Fără atu. Punctele: levate peste 6 (sau contractul), dublu la fără atu; primul la ±7. Mutări: pas, licit 4, dis 9h, sus h, jos h, fara sus, fara jos, 9h, reimparte, urmatoarea.",
            make: || Box::new(bid_whist::BidWhist::new()),
        },
        Playable {
            id: "all_fours",
            title: "All Fours",
            category: "Cărți",
            rules: "52 de cărți, 6 fiecare, o carte de atu întoarsă (valet = 3 puncte dealerului). Non-dealerul cere (cer) sau joacă (joc); dealerul dă un punct (dau) sau schimbă (schimb, apoi imparte, aruncări dis X). Patru puncte: High, Low, Jack, Game; primul la 14. Mutări: cer, joc, dau, schimb, imparte, dis 9h, 9h, urmatoarea.",
            make: || Box::new(all_fours::AllFours::new()),
        },
        Playable {
            id: "conquian",
            title: "Conquián",
            category: "Cărți",
            rules: "Pachet spaniol de 40 de cărți (A–7, S, C, R; patru culori), 2 jucători, 10 cărți. Tragi din stoc (stoc) sau iei cartea de sus a talonului (ia ...) doar ca s-o legi imediat într-o combinație (set de 3–4 sau serie de cel puțin 3 din aceeași culoare); apoi cobori combinații (pune ...), lipești cărți (adauga X la N) și arunci (descarta X). Cine rămâne fără cărți în mână câștigă mâna și un punct; primul la 3. Mutări: stoc, ia ..., pune ..., adauga 5o la 1, descarta 5o, pas, urmatoarea.",
            make: || Box::new(conquian::Conquian::new()),
        },
        Playable {
            id: "chicken_foot",
            title: "Chicken Foot (domino)",
            category: "Clasic",
            rules: "Set dublu-12, 15 piese fiecare, 13 runde (de la dublu-12 la dublu-0). Se pornește de la dublul de rundă; un dublu trebuie acoperit (pui piesa pe el) înainte de a continua. Runda se termină când cineva rămâne fără piese sau la blocaj; fiecare adaugă la scor punctele pieselor rămase în mână. După 13 runde câștigă cel mai mic total. Mutări: 5-7@2, trage, pas, urmatoarea.",
            make: || Box::new(chicken_foot::ChickenFoot::new()),
        },
        Playable {
            id: "bunco",
            title: "Cele șase ținte (zaruri)",
            category: "Clasic",
            rules: "Joc de zaruri după Bunco (nume comercial): 6 runde, în runda N ținta e N. Arunci 3 zaruri cât timp ai puncte: fiecare zar egal cu ținta = 1 punct, trei egale cu ținta = 21 (bunco, încheie runda), trei la fel altfel = 5. Runda se încheie la 21 de puncte; după 6 runde câștigă cine a luat mai multe runde. Mutări: arunc.",
            make: || Box::new(bunco::Bunco::new()),
        },
        Playable {
            id: "farkle",
            title: "Zece mii de puncte (zaruri)",
            category: "Clasic",
            rules: "Joc de zaruri după Farkle / 10 000 (nume comercial): arunci 6 zaruri, păstrezi combinații care punctează (1 = 100, 5 = 50, trei la fel, serii, perechi), apoi arunci din nou sau bancrezi. Fără combinație pierzi tot ce ai strâns în tură. Cu 0 puncte în total banca doar de la 500 în tură; țintă 10 000 cu rundă finală. Mutări: arunc, pastrez 1 5, banca, farkle.",
            make: || Box::new(farkle::Farkle::new()),
        },
        Playable {
            id: "liars_dice",
            title: "Zarurile mincinosului",
            category: "Clasic",
            rules: "Variantă Perudo/Dudo/Cacho: fiecare are 5 zaruri ascunse sub pahar. Licitezi cantitatea unei fețe pentru toate zarurile de pe masă (aștii sunt jokeri), mereu mai sus, sau spui dudo; cine greșește pierde un zar. Ultimul cu zaruri câștigă. Fără calza și fără palifico. Mutări: 3x4, dudo, arata.",
            make: || Box::new(liars_dice::LiarsDice::new()),
        },
        Playable {
            id: "crokinole",
            title: "Crokinole",
            category: "Familie",
            rules: "Joc de dexteritate pe tablă rotundă, modelat ca simulare deterministă cu zgomot din seed: alegi poziția, unghiul și puterea tragerii (x-1 u6 p3), apoi lasi discul. Trebuie să lovești un disc adversar dacă există. 12 discuri pe jucător; inelele valorează 5, 10 și 15, gaura centrală 20. Câștigătorul rundei ia diferența de puncte; primul la 100 (cel mult 6 runde). Mutări: x-1 u6 p3, lasa.",
            make: || Box::new(crokinole::Crokinole::new()),
        },
        // --- P4 ---
        Playable {
            id: "chinchon",
            title: "Chinchón (Spania)",
            category: "Cărți",
            rules: "Baraja spaniolă de 40 de cărți (o c e b; 1–7, 10, 11, 12), 7 cărți fiecare. Tragi din stoc sau talon și arunci; combinații: seturi și secvențe (7 nu se leagă de 10). Închizi cu cel puțin 6 cărți potrivite și cel mult 5 puncte libere (−10 dacă toate 7 se potrivesc; chinchón = 1–7 din aceeași culoare câștigă jocul); cine ajunge la 100 pierde. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: stoc, talon, 5o, inchide-5o, urmatoarea.",
            make: || Box::new(chinchon::Chinchon::new()),
        },
        Playable {
            id: "burako",
            title: "Burako (Buraco)",
            category: "Cărți",
            rules: "106 cărți (2 pachete + 2 jokeri), 11 în mână și un muerto de 11. Tragi din stoc (mazo) sau iei tot pozo, lași jos scări (≥3 consecutive, aceeași culoare) și piernas (≥3 de același rang), cel mult un comodin (joker sau 2) per combinație, apoi arunci. Canastă = 7+ cărți: pură 200, impură 100; închizi cu muertoul luat și o canastă (+100). Meci până la 1500 sau 4 mâini. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: mazo, pozo, esc:4h+5h+6h, set:7h+7c+Jk, +1:8h, descarta 5h, muerto, urmatoarea.",
            make: || Box::new(burako::Burako::new()),
        },
        Playable {
            id: "escoba",
            title: "Escoba (Escoba de 15)",
            category: "Cărți",
            rules: "Baraja spaniolă de 40 de cărți (valori 1–7, 10=8, 11=9, 12=10), 3 în mână și 4 pe masă. Joci o carte și poți lua cărți de pe masă care, adunate cu ea, dau 15; golești masa = escoba (1 punct, nu la ultima carte). Punctaj la capăt de mână: cartas, oros, siete de velo (7o), setenta și escobe; primul la 21. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 7o, 7o:4c+4e, imparte, urmatoarea.",
            make: || Box::new(escoba::Escoba::new()),
        },
        Playable {
            id: "cuarenta",
            title: "Cuarenta (Ecuador)",
            category: "Cărți",
            rules: "Baraja spaniolă de 40 de cărți, câte 5 în mână, masa începe goală. Joci o carte și poți lua: o carte de același rang (potrivire) sau cărți 1–7 cu aceeași sumă (adunare), plus secvența ascendentă care urmează. Caída 2, limpia 2, ronda 4 puncte; la capăt de mână puncte după numărul de cărți luate; primul la 40. Informație ascunsă: vezi doar mâna Jucătorului 1. Mutări: 5o, 5o:5c, 6o:2c+4e, imparte, urmatoarea.",
            make: || Box::new(cuarenta::Cuarenta::new()),
        },
        Playable {
            id: "generala",
            title: "Generala (zaruri)",
            category: "Zaruri",
            rules: "5 zaruri, până la 3 aruncări pe tură, 10 categorii: unu…sase (suma zarurilor), escalera 20, full 30, poker 40, generala 50. Servida (din prima aruncare) +5 la escalera/full/poker; generala servida câștigă jocul pe loc. Categoriile nepotrivite se taie (0). 10 ture pe jucător, câștigă totalul mai mare. Mutări: arunc, tine 3 3 5, tine -, scor escalera.",
            make: || Box::new(generala::Generala::new()),
        },
        Playable {
            id: "parques",
            title: "Parqués (Colombia)",
            category: "Zaruri",
            rules: "Pistă de 68 de pătrate, 4 piese pe jucător, două zaruri. Din cárcel ieși doar cu pereche (toate piesele deodată); pereche = încă o aruncare, trei perechi = o piesă direct în corona. Captura (obligatorie) trimite piesa adversă în cárcel, pe pătratele sigure nu se capturează; corona se atinge exact. Primul cu 4 piese în corona câștigă. Mutări: 12:5 (piesa de la progresul 12 avansează 5), sale, pasa.",
            make: || Box::new(parques::Parques::new()),
        },
        Playable {
            id: "fivehundred",
            title: "Five Hundred (500, doi jucători)",
            category: "Cărți",
            rules: "500 pentru 2 jucători: 43 de cărți (cu joker), 10 în mână, 5 grămezi de câte 2 (jos + sus) și un kitty de 3. Licitezi 6–10 levate cu atu sau fără atu (fără misere); contractantul ia kitty-ul și aruncă 3 cărți. Levate de 4 cărți (câte una din mână și una cu fața în sus); jokerul și valeții (bower) sunt atuurile cele mai mari. Contract reușit = valoarea lui, eșuat = −valoarea; adversarul ia 10 pe levată; 500 / −500. Informație ascunsă. Mutări: licit 7h, licit 8nt, pas, descarta As, 10h, Jk:h, urmatoarea.",
            make: || Box::new(fivehundred::FiveHundred::new()),
        },
        Playable {
            id: "pontoon",
            title: "Pontoon",
            category: "Cărți",
            rules: "Banca (rolul alternează la fiecare mână) contra jucătorului: Pontoon (as + carte de 10) > Five Card Trick (5 cărți ≤ 21) > 21 > cât mai aproape de 21; egalitatea o câștigă banca. Jucătorul: carte (twist), cumpar (carte ascunsă, miza +10, doar înainte de carte), stai (de la 15); banca stă de la 17. Pontoon și Five Card Trick plătesc dublu. 20 de mâini cu miza 10, câștigă cine are mai multe jetoane. Informație ascunsă. Mutări: carte, cumpar, stai, decontare.",
            make: || Box::new(pontoon::Pontoon::new()),
        },
        Playable {
            id: "twoup",
            title: "Two-up (Australia)",
            category: "Zaruri",
            rules: "Spinnerul mizează pe stemă (5, 10 sau 20 de jetoane); celălalt acoperă. Două monede: cap-cap = spinnerul câștigă și continuă; pajura-pajura = pierde și rolul trece; cap-pajura (odds) = se aruncă din nou, cinci odds la rând = spinnerul pierde. 24 de mize, start 50 de jetoane; câștigă cine are mai multe. Mutări: miza 5, miza 10, miza 20, arunc.",
            make: || Box::new(twoup::TwoUp::new()),
        },
        Playable {
            id: "ancora",
            title: "Ancora și coroana",
            category: "Zaruri",
            rules: "Trei zaruri cu simboluri (coroana, ancora, inima, romb, trefla, pica) — joc britanic tradițional «Crown and Anchor». Mizezi 5, 10 sau 20 pe unul sau mai multe simboluri; pentru fiecare zar cu simbolul tău primești miza înapoi plus câștig (1:1, 2:1, 3:1), altfel pierzi miza. 12 runde împotriva băncii, câștigă cine are mai multe jetoane. Mutări: pun ancora 10, gata, arunc.",
            make: || Box::new(ancora::Ancora::new()),
        },
        Playable {
            id: "mutorere",
            title: "Mu Torere (Noua Zeelandă)",
            category: "Abstract",
            rules: "8 puncte pe cerc (kewai 1–8) și centrul (0); fiecare are 4 piese pe puncte vecine, centrul gol. Muți o piesă pe un punct vecin gol; în centru intri doar cu o piesă vecină cu o piesă adversă. Pierde cine nu mai poate muta; remiză la a treia repetare sau după 200 de mutări. Mutări: 3-4 (de pe 3 pe 4), 4-0 (în centru), 0-6 (din centru).",
            make: || Box::new(mutorere::MuTorere::new()),
        },
    ]
}

/// The full catalog of target games (all categories).
pub fn catalog() -> &'static [E] {
    catalog::CATALOG
}

pub fn find(id: &str) -> Option<Playable> {
    playable().into_iter().find(|p| p.id == id)
}
