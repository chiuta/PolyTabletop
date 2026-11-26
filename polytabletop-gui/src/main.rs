//! PolyTabletop GUI: o fereastră egui peste aceleași jocuri `GameObj` ca în CLI.
//!
//! * meniu pe categorii cu căutare;
//! * tablă grafică (Painter) pentru 16 jocuri abstracte, mutări prin click;
//! * „la masă”: respectă `set_viewer` + ecran de predare între mutări (mâini ascunse);
//! * mutări speciale/forțate ca butoane mari;
//! * salvare/încărcare jurnal (replay unde jocul e determinist);
//! * temă clară/întunecată;
//! * 8 limbi (`--lang CODE`, `POLYTABLETOP_LANG`, `LANG`; selector în bara de sus);
//! * `--selftest`: verificări headless.

mod board;
mod capture;
mod cards;
mod cards_ui;
mod compose;
mod draw;
mod gui_i18n;
mod journal;
mod markers;
mod menu;
mod selftest;
mod session;
mod settings;
mod theme;

use board::{hit, kind_of, last_cells, layout, Board};
use compose::Click;
use draw::Theme;
use eframe::egui;
use gui_i18n::{category_title, game_rules, game_title, tr, trf, Lang, APP_NAME};
use journal::Journal;
use polytabletop_core::ai::Level;
use polytabletop_core::{rng::Rng, GameObj, Outcome, Player};
use polytabletop_games::{playable, Playable};
use session::{who, History};

struct App {
    games: Vec<Playable>,
    sel: usize,
    game: Box<dyn GameObj>,
    human: Option<Player>, // None = ambele (hot-seat)
    input: String,
    log: Vec<String>,
    moves: Vec<String>,
    rng: Rng,
    msg: String,
    info: String,
    query: String,
    dark: bool,
    pick: Vec<String>,
    choices: Vec<String>,
    path: String,
    /// „la masă” cu informație ascunsă: ecranul de predare către acest jucător
    handover: Option<Player>,
    /// stări dinaintea mutărilor omului, pentru „Anulează mutarea”
    history: History,
    title: String,
    /// nivelul AI (doar în sesiune)
    level: polytabletop_core::ai::Level,
}

impl App {
    fn new() -> Self {
        let games = playable();
        let game = (games[0].make)();
        let mut a = App {
            games,
            sel: 0,
            game,
            human: Some(Player::One),
            input: String::new(),
            log: Vec::new(),
            moves: Vec::new(),
            rng: Rng::from_clock(),
            msg: String::new(),
            info: String::new(),
            query: String::new(),
            dark: true,
            pick: Vec::new(),
            choices: Vec::new(),
            path: String::new(),
            handover: None,
            history: History::default(),
            title: String::new(),
            level: polytabletop_core::ai::level(),
        };
        a.apply_viewer();
        a.path = a.default_path();
        a
    }

    fn default_path(&self) -> String {
        format!("polytabletop-jurnal-{}.txt", self.games[self.sel].id)
    }

    fn apply_viewer(&mut self) {
        session::apply_viewer(self.game.as_mut(), self.human);
    }

    /// După o mutare: dacă urmează celălalt om la același ecran și jocul are
    /// mâini ascunse, arată ecranul de predare în loc de tablă.
    fn maybe_handover(&mut self) {
        self.handover = if self.game.outcome().is_none()
            && session::needs_handover(&self.games[self.sel], self.human)
        {
            Some(self.game.to_move())
        } else {
            None
        };
    }

    fn clear_selection(&mut self) {
        self.pick.clear();
        self.choices.clear();
    }

    fn restart(&mut self) {
        self.game = (self.games[self.sel].make)();
        self.apply_viewer();
        self.handover = None;
        self.history.clear();
        self.log.clear();
        self.moves.clear();
        self.msg.clear();
        self.info.clear();
        self.input.clear();
        self.clear_selection();
        self.path = self.default_path();
        self.ai_turn();
    }

    fn ai_turn(&mut self) {
        while self.game.outcome().is_none() {
            let me = self.game.to_move();
            if self.human.is_none() || self.human == Some(me) {
                break;
            }
            match self.game.ai_move(self.rng.next_u64()) {
                Some(m) => {
                    if let Ok(t) = self.game.play(&m) {
                        self.log
                            .push(format!("{} ({}): {t}", who(me), tr("cpu_tag")));
                        self.moves.push(t);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    fn try_move(&mut self, text: &str) {
        let me = self.game.to_move();
        let before = self.game.clone_box();
        match self.game.play(text) {
            Ok(m) => {
                self.history
                    .push(before.as_ref(), self.log.len(), self.moves.len());
                self.msg.clear();
                self.log.push(format!("{}: {m}", who(me)));
                self.moves.push(m);
                self.input.clear();
                self.clear_selection();
                self.ai_turn();
                self.maybe_handover();
            }
            Err(e) => self.msg = e,
        }
    }

    /// Revine la starea de dinaintea ultimei mutări a omului (inclusiv răspunsul calculatorului).
    fn undo(&mut self) {
        let Some(s) = self.history.pop() else { return };
        self.game = s.game;
        self.log.truncate(s.log);
        self.moves.truncate(s.moves);
        self.msg.clear();
        self.input.clear();
        self.clear_selection();
        self.info = tr("undone").into();
        self.ai_turn();
        self.maybe_handover();
    }

    fn my_turn(&self) -> bool {
        self.game.outcome().is_none()
            && (self.human.is_none() || self.human == Some(self.game.to_move()))
    }

    fn kind(&self) -> Option<board::Kind> {
        kind_of(self.games[self.sel].id)
    }

    fn on_click(&mut self, legal: &[String], name: &str) {
        match compose::click_k(self.kind(), legal, &self.pick, name) {
            Click::Play(m) => self.try_move(&m),
            Click::Choose(c) => {
                self.pick.push(name.to_string());
                self.choices = c;
            }
            Click::Pending { sel, .. } => {
                self.pick = sel;
                self.choices.clear();
            }
            Click::Reset => self.clear_selection(),
        }
    }

    fn save(&mut self) {
        let p = &self.games[self.sel];
        let j = session::make_journal(p, self.human, &self.moves, &self.log, self.game.as_ref());
        let replay = j.replay;
        match std::fs::write(&self.path, j.to_text()) {
            Ok(()) => {
                self.info = trf(
                    "saved",
                    &[
                        &self.path,
                        &self.moves.len().to_string(),
                        if replay { "" } else { tr("saved_text_only") },
                    ],
                )
            }
            Err(e) => self.msg = trf("err_write", &[&self.path, &e.to_string()]),
        }
    }

    fn load(&mut self) {
        let text = match std::fs::read_to_string(&self.path) {
            Ok(t) => t,
            Err(e) => {
                self.msg = trf("err_read", &[&self.path, &e.to_string()]);
                return;
            }
        };
        let r =
            Journal::parse(&text).and_then(|j| session::replay(&self.games, &j).map(|r| (j, r)));
        match r {
            Ok((j, r)) => {
                self.sel = r.idx;
                self.game = r.game;
                self.human = r.human;
                self.log = r.log;
                self.moves = j.moves;
                self.history.clear();
                self.msg.clear();
                self.input.clear();
                self.clear_selection();
                self.info = trf("loaded", &[&self.moves.len().to_string()]);
                self.apply_viewer();
                self.ai_turn();
                self.maybe_handover();
            }
            Err(e) => self.msg = e,
        }
    }

    /// Bara de sus: selectorul de limbă (nume native).
    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(tr("language"));
            let cur = gui_i18n::lang();
            egui::ComboBox::from_id_salt("lang")
                .selected_text(cur.native())
                .show_ui(ui, |ui| {
                    for l in Lang::ALL {
                        if ui.selectable_label(l == cur, l.native()).clicked() {
                            gui_i18n::set_lang(l);
                            settings::save_lang(l);
                        }
                    }
                });
            ui.separator();
            ui.label(tr("ai_level"));
            let name = |l: Level| match l {
                Level::Easy => tr("level_easy"),
                Level::Normal => tr("level_normal"),
                Level::Hard => tr("level_hard"),
                Level::Expert => tr("level_expert"),
            };
            egui::ComboBox::from_id_salt("level")
                .selected_text(name(self.level))
                .show_ui(ui, |ui| {
                    for l in [Level::Easy, Level::Normal, Level::Hard, Level::Expert] {
                        if ui.selectable_label(l == self.level, name(l)).clicked() {
                            self.level = l;
                            polytabletop_core::ai::set_level(l);
                        }
                    }
                });
        });
    }

    fn left_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("games"));
        ui.add(egui::TextEdit::singleline(&mut self.query).hint_text(tr("search_hint")));
        let items: Vec<(&str, &str, &str)> = self
            .games
            .iter()
            .map(|g| (g.id, game_title(g.id, g.title), g.category))
            .collect();
        let groups = menu::groups(&items, &self.query);
        let searching = !self.query.trim().is_empty();
        let mut pick = None;
        egui::ScrollArea::vertical()
            .max_height((ui.available_height() - 190.0).max(120.0))
            .show(ui, |ui| {
                if groups.is_empty() {
                    ui.label(tr("no_games"));
                }
                for (cat, idx) in &groups {
                    let has_sel = idx.contains(&self.sel);
                    egui::CollapsingHeader::new(format!("{} ({})", category_title(cat), idx.len()))
                        .id_salt(format!("cat-{cat}"))
                        .default_open(has_sel)
                        .open(if searching { Some(true) } else { None })
                        .show(ui, |ui| {
                            for &i in idx {
                                if ui
                                    .selectable_label(
                                        self.sel == i,
                                        game_title(self.games[i].id, self.games[i].title),
                                    )
                                    .clicked()
                                {
                                    pick = Some(i);
                                }
                            }
                        });
                }
            });
        if let Some(i) = pick {
            self.sel = i;
            self.restart();
        }
        ui.separator();
        ui.label(tr("play_as"));
        let mut changed = false;
        changed |= ui
            .radio_value(&mut self.human, Some(Player::One), who(Player::One))
            .changed();
        changed |= ui
            .radio_value(&mut self.human, Some(Player::Two), who(Player::Two))
            .changed();
        changed |= ui
            .radio_value(&mut self.human, None, tr("both_hotseat"))
            .changed();
        if changed || ui.button(tr("new_game")).clicked() {
            self.restart();
        }
        ui.separator();
        ui.horizontal(|ui| {
            ui.label(tr("theme"));
            ui.radio_value(&mut self.dark, false, tr("light"));
            ui.radio_value(&mut self.dark, true, tr("dark"));
        });
    }

    fn right_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading(tr("moves"));
        egui::ScrollArea::vertical()
            .stick_to_bottom(true)
            .max_height((ui.available_height() - 130.0).max(80.0))
            .show(ui, |ui| {
                for l in &self.log {
                    ui.label(l);
                }
            });
        ui.separator();
        ui.label(tr("journal"));
        ui.text_edit_singleline(&mut self.path);
        ui.horizontal(|ui| {
            if ui.button(tr("save")).clicked() {
                self.save();
            }
            if ui.button(tr("load")).clicked() {
                self.load();
            }
        });
        if !self.info.is_empty() {
            ui.small(&self.info);
        }
    }

    /// Butoane mari; întoarce mutarea aleasă.
    fn big_buttons(ui: &mut egui::Ui, moves: &[String], hint: &str) -> Option<String> {
        let mut chosen = None;
        if !hint.is_empty() {
            ui.label(hint);
        }
        ui.horizontal_wrapped(|ui| {
            for m in moves {
                let b = egui::Button::new(egui::RichText::new(m).size(20.0).strong());
                if ui.add_sized([120.0, 46.0], b).clicked() {
                    chosen = Some(m.clone());
                }
            }
        });
        chosen
    }

    /// Sugestie: calculată pe o copie văzută din partea celui la mutare și
    /// refuzată când ar putea scurge informație privată.
    fn hint(&mut self) {
        let p = &self.games[self.sel];
        if let Err(e) = session::hint_allowed(p, self.human, self.handover.is_some()) {
            self.msg = e;
            return;
        }
        let g = session::hint_state(self.game.as_ref(), self.human);
        if let Some(m) = g.ai_move(self.rng.next_u64()) {
            self.input = m;
        }
    }

    fn handover_screen(&mut self, ui: &mut egui::Ui, p: Player) {
        ui.add_space(40.0);
        ui.heading(trf("handover_title", &[who(p)]));
        ui.label(tr("handover_body"));
        ui.add_space(20.0);
        let b = egui::Button::new(egui::RichText::new(trf("handover_btn", &[who(p)])).size(22.0));
        if ui.add_sized([320.0, 60.0], b).clicked() {
            self.handover = None;
        }
    }

    fn center(&mut self, ui: &mut egui::Ui) {
        let (id, title, rules) = {
            let p = &self.games[self.sel];
            (p.id, game_title(p.id, p.title), game_rules(p.id, p.rules))
        };
        ui.heading(title);
        ui.label(rules);
        ui.separator();
        if let Some(p) = self.handover {
            self.handover_screen(ui, p);
            return;
        }
        if self.human.is_none() && session::hotseat_leaks(&self.games[self.sel]) {
            ui.colored_label(egui::Color32::YELLOW, tr("warn_leak"));
        }
        let render = self.game.render();
        let over = self.game.outcome();
        let legal = if over.is_none() {
            self.game.legal_moves()
        } else {
            vec![]
        };
        let mover = self.game.to_move();
        let parsed: Option<Board> = kind_of(id)
            .and_then(|k| board::parse(k, &render))
            .map(|b| b.with_mover(mover));
        let my_turn = self.my_turn();
        let kind = parsed.as_ref().map(|b| b.kind);

        let mut chosen: Option<String> = None;
        let mut cards_handle = false;
        let mut undo = false;
        if let Some(b) = &parsed {
            let th = Theme { dark: self.dark };
            let avail = ui.available_size();
            let side = (avail.y - 210.0).clamp(240.0, avail.x.max(240.0));
            let (resp, painter) =
                ui.allocate_painter(egui::vec2(avail.x, side), egui::Sense::click());
            let l = layout(b, resp.rect.shrink(22.0));
            let next = if my_turn {
                compose::next_squares_k(Some(b.kind), &legal, &self.pick)
            } else {
                Default::default()
            };
            // destinațiile piesei de sub mouse (când nu e nimic selectat)
            let mut hover: std::collections::BTreeSet<String> = Default::default();
            if my_turn && self.pick.is_empty() {
                if let Some(i) = resp
                    .hover_pos()
                    .and_then(|p| hit(b, &l, p))
                    .filter(|&i| b.clickable(i))
                {
                    hover = compose::next_squares_k(
                        Some(b.kind),
                        &legal,
                        std::slice::from_ref(&b.cells[i].name),
                    );
                    hover.extend(b.hover_names(i));
                }
            }
            let last = match (self.moves.last(), session::last_mover(&self.log)) {
                (Some(m), Some(was)) => last_cells(b, m, was, mover),
                _ => vec![],
            };
            draw::draw(&painter, b, &l, &th, &self.pick, &next, &hover, &last);
            if resp.clicked() && my_turn {
                if let Some(p) = resp.interact_pointer_pos() {
                    if let Some(i) = hit(b, &l, p).filter(|&i| b.clickable(i)) {
                        let name = b.cells[i].name.clone();
                        self.on_click(&legal, &name);
                    }
                }
            }
            for line in session::footer_for(b.kind, &render) {
                ui.label(egui::RichText::new(line).monospace());
            }
        } else {
            // cărți / zaruri / domino desenate (doar când jetoanele sunt recunoscute sigur)
            let view = cards_ui::analyze(id, &render, &legal);
            let drawn = view.is_some();
            let handles = view.as_ref().is_some_and(|v| v.handles_actions());
            egui::ScrollArea::vertical()
                .max_height(
                    (ui.available_height()
                        - if handles {
                            130.0
                        } else if drawn {
                            200.0
                        } else {
                            220.0
                        })
                    .max(120.0),
                )
                .show(ui, |ui| {
                    if let Some(v) = &view {
                        if let Some(m) = cards_ui::show(ui, v, my_turn && over.is_none()) {
                            chosen = Some(m);
                        }
                    }
                    ui.label(egui::RichText::new(&render).monospace().size(if drawn {
                        13.0
                    } else {
                        16.0
                    }));
                });
            cards_handle = view.is_some_and(|v| v.handles_actions());
        }

        match over {
            Some(Outcome::Win(p)) => {
                ui.heading(trf("wins", &[who(p)]));
            }
            Some(Outcome::Draw) => {
                ui.heading(tr("draw"));
            }
            None => {
                ui.label(trf("to_move", &[who(self.game.to_move())]));
                if my_turn {
                    if parsed.is_some() {
                        if !self.choices.is_empty() {
                            let c = self.choices.clone();
                            if let Some(m) = Self::big_buttons(ui, &c, tr("choose_move")) {
                                chosen = Some(m);
                            }
                        }
                        let (_, special) = compose::split_special_k(kind, &legal);
                        if !special.is_empty() {
                            if let Some(m) = Self::big_buttons(ui, &special, "") {
                                chosen = Some(m);
                            }
                        }
                        if !self.pick.is_empty() {
                            ui.horizontal(|ui| {
                                ui.label(trf("selection", &[&self.pick.join(" → ")]));
                                // mutări care se pot încheia aici (salt simplu înaintea unui salt
                                // multiplu, lanț de capturi); mai multe dacă diferă doar sufixul
                                for e in compose::ends_here_k(kind, &legal, &self.pick) {
                                    if ui.button(trf("end_here", &[&e])).clicked() {
                                        chosen = Some(e);
                                    }
                                }
                                if ui.button(tr("cancel_sel")).clicked() {
                                    self.clear_selection();
                                }
                            });
                        }
                    } else if cards_handle {
                        // mutările sunt desenate ca cărți / zaruri
                    } else if legal.len() == 1 {
                        if let Some(m) = Self::big_buttons(ui, &legal, tr("forced")) {
                            chosen = Some(m);
                        }
                    } else if legal.len() <= 6 {
                        if let Some(m) = Self::big_buttons(ui, &legal, tr("choose")) {
                            chosen = Some(m);
                        }
                    }
                }
                ui.horizontal(|ui| {
                    let r = ui.text_edit_singleline(&mut self.input);
                    let go = ui.button(tr("play")).clicked()
                        || (r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)));
                    if go {
                        chosen = Some(self.input.clone());
                        r.request_focus();
                    }
                    if ui.button(tr("hint")).clicked() {
                        self.hint();
                    }
                });
                if parsed.is_none() && !cards_handle && legal.len() > 6 && legal.len() <= 60 {
                    ui.separator();
                    ui.label(tr("legal_moves"));
                    ui.horizontal_wrapped(|ui| {
                        for m in &legal {
                            if ui.small_button(m).clicked() {
                                chosen = Some(m.clone());
                            }
                        }
                    });
                }
            }
        }
        ui.horizontal(|ui| {
            let n = self.history.len();
            let b = ui.add_enabled(n > 0, egui::Button::new(tr("undo")));
            if b.on_hover_text(tr("undo_tip")).clicked() {
                undo = true;
            }
            if n > 0 {
                ui.small(format!("({n})"));
            }
        });
        if undo {
            self.undo();
            return;
        }
        if let Some(m) = chosen {
            self.try_move(&m);
        }
        if !self.msg.is_empty() {
            ui.colored_label(egui::Color32::LIGHT_RED, &self.msg);
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let t = session::window_title(game_title(
            self.games[self.sel].id,
            self.games[self.sel].title,
        ));
        if t != self.title {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Title(t.clone()));
            self.title = t;
        }
        theme::apply(ui.ctx(), self.dark);
        egui::Panel::top("menubar").show(ui, |ui| self.menu_bar(ui));
        egui::Panel::left("games").show(ui, |ui| self.left_panel(ui));
        egui::Panel::right("log").show(ui, |ui| self.right_panel(ui));
        let center = egui::Frame::central_panel(ui.style()).fill(theme::palette(self.dark).bg);
        egui::CentralPanel::default()
            .frame(center)
            .show(ui, |ui| self.center(ui));
    }
}

fn main() -> eframe::Result<()> {
    let argv: Vec<String> = std::env::args().collect();
    // verificările headless rămân în română, indiferent de mediu
    gui_i18n::set_lang(if argv.iter().any(|a| a == "--selftest") {
        Lang::Ro
    } else {
        gui_i18n::detect(&argv)
    });
    match session::seed_from_args(&argv) {
        Ok(Some(n)) => polytabletop_core::rng::Rng::set_global_seed(Some(n)),
        Ok(None) => {}
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }
    if std::env::args().any(|a| a == "--selftest") {
        match selftest::run() {
            Ok(()) => std::process::exit(0),
            Err(e) => {
                eprintln!("SELFTEST EȘUAT: {e}");
                std::process::exit(1);
            }
        }
    }
    if let Some(r) = capture::maybe_run(&argv) {
        return r;
    }
    eframe::run_native(
        APP_NAME,
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            theme::install_fonts(&cc.egui_ctx);
            Ok(Box::new(App::new()))
        }),
    )
}
