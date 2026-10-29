//! Coordinates the way Bedrock Edition shows them with "Show Coordinates" on:
//! `X: 123, Y: 65, Z: -45` on one line in the top-left corner, and nothing
//! else — no facing, no clock. `/coords` hides or shows the panel.
use horizon_mod_sdk::{
    Mod, Registration,
    game::{self, Player},
    hud::{self, Corner, Line},
};

struct ShowCoordinates {
    shown: bool,
    /// The panel last sent, so an unchanged one costs no call.
    last: Vec<String>,
}

/// `X: 123, Y: 65, Z: -45`: the block the player stands on, as Bedrock rounds it.
fn coords(p: &Player) -> String {
    format!(
        "X: {:.0}, Y: {:.0}, Z: {:.0}",
        p.x.floor(),
        p.y.floor(),
        p.z.floor()
    )
}

impl Mod for ShowCoordinates {
    fn init() -> Result<(Self, Registration), String> {
        let hooks = Registration::new().command("coords").on_tick();
        let state = ShowCoordinates {
            shown: true,
            last: Vec::new(),
        };
        Ok((state, hooks))
    }

    fn tick(&mut self, _tick: u64) {
        let Some(p) = game::get_player().filter(|_| self.shown) else {
            if !self.last.is_empty() {
                hud::clear();
                self.last.clear();
            }
            return;
        };
        let now = vec![coords(&p)];
        if now != self.last {
            let panel: Vec<Line> = now
                .iter()
                .map(|text| Line {
                    text: text.clone(),
                    color: 0xFFFFFF,
                })
                .collect();
            hud::set_panel(Corner::TopLeft, &panel);
            self.last = now;
        }
    }

    fn command(&mut self, name: &str, _args: &str) -> Result<Option<String>, String> {
        match name {
            "coords" => {
                self.shown = !self.shown;
                let state = if self.shown { "shown" } else { "hidden" };
                Ok(Some(format!("Coordinates panel {state}")))
            }
            _ => Ok(None),
        }
    }
}

horizon_mod_sdk::export_mod!(ShowCoordinates);
