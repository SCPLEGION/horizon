//! Chunks a native mod keeps loaded (Mekanism's dimensional stabilizer and the anchor upgrade). API 45.
use super::Game;
use std::collections::HashSet;

/// How many ticks a chunk stays loaded after the mod last asked for it.
const LEASE_TICKS: i64 = 40;

#[allow(dead_code)]
impl Game<'_> {
    /// Keeps `chunks` of this level loaded and ticking, as `/forceload` does, for the next
    /// two seconds: a mod calls this every tick (or at least every second) from the block entity
    /// that loads them, and the chunks are let go when it stops (the block was broken or unloaded).
    /// A chunk that `/forceload` already holds stays that command's. Not saved with the world. API 45.
    pub(super) fn native_keep_chunks(&mut self, chunks: &[(i32, i32)]) {
        let now = self.world.game_time;
        for &chunk in chunks {
            if let Some(last) = self.streaming.mod_forced.get_mut(&chunk) {
                *last = now;
            } else if self.streaming.forced.insert(chunk) {
                self.streaming.mod_forced.insert(chunk, now);
                self.streaming
                    .rank_forced(chunk.0, chunk.1, chunk.0, chunk.1);
            }
        }
    }
    /// Lets go of the mod's chunks it stopped asking for.
    pub(super) fn expire_mod_chunks(&mut self) {
        if self.streaming.mod_forced.is_empty() {
            return;
        }
        let now = self.world.game_time;
        let gone: Vec<(i32, i32)> = self
            .streaming
            .mod_forced
            .iter()
            .filter(|(_, last)| now - **last > LEASE_TICKS)
            .map(|(c, _)| *c)
            .collect();
        for chunk in gone {
            self.streaming.mod_forced.remove(&chunk);
            self.streaming.forced.remove(&chunk);
        }
    }
    /// The forced chunks that belong in `forced_chunks.dat`: not the mod's.
    pub(super) fn saved_forced_chunks(&self) -> HashSet<(i32, i32)> {
        self.streaming
            .forced
            .iter()
            .filter(|c| !self.streaming.mod_forced.contains_key(c))
            .copied()
            .collect()
    }
}
