//! Explicit movement toggles and directional shadow attachments.
use super::{DocumentContent, LayerId};
use crate::preview::{Dirty, PixelRect};

impl DocumentContent {
    // Keep the historical wire shape, with at most one movement set. A single
    // enabled toggle is meaningful and survives until the user turns it off.
    pub fn layer_links(&self) -> &[Vec<LayerId>] {
        &self.layer_links
    }

    pub fn linked_layers(&self, id: LayerId) -> &[LayerId] {
        self.layer_links
            .first()
            .filter(|set| set.contains(&id))
            .map_or(&[], Vec::as_slice)
    }

    pub fn set_layer_links(&mut self, groups: Vec<Vec<LayerId>>) -> anyhow::Result<()> {
        let mut seen = std::collections::HashSet::new();
        anyhow::ensure!(
            groups.iter().all(|group| !group.is_empty()
                && group
                    .iter()
                    .all(|id| self.layer(*id).is_some() && seen.insert(*id))),
            "Invalid layer links"
        );
        self.merged_legacy_links = groups.len() > 1;
        // Older files can have several groups. Their enabled members become
        // the one explicit movement set; never infer attachments from names.
        let mut joined: Vec<_> = groups.into_iter().flatten().collect();
        joined.sort_unstable_by_key(|id| id.0);
        self.layer_links = if joined.is_empty() {
            vec![]
        } else {
            vec![joined]
        };
        Ok(())
    }

    pub fn take_link_migration_notice(&mut self) -> Option<&'static str> {
        std::mem::take(&mut self.merged_legacy_links).then_some(
            "This project used separate link groups. Their enabled layers now form one movement set; use the chain toggles to choose which layers move together.",
        )
    }

    #[cfg(test)]
    pub fn link_layers(&mut self, first: LayerId, second: LayerId) -> Dirty {
        if first == second || self.layer(first).is_none() || self.layer(second).is_none() {
            return Dirty::None;
        }
        let a = self.set_layer_linked(first, true);
        let b = self.set_layer_linked(second, true);
        if a == Dirty::None && b == Dirty::None {
            Dirty::None
        } else {
            Dirty::Unpainted
        }
    }

    pub fn set_layer_linked(&mut self, id: LayerId, linked: bool) -> Dirty {
        if self.layer(id).is_none() || (!self.linked_layers(id).is_empty()) == linked {
            return Dirty::None;
        }
        if !linked {
            return self.unlink_layer(id);
        }
        if self.layer_links.is_empty() {
            self.layer_links.push(Vec::new());
        }
        self.layer_links[0].push(id);
        self.layer_links[0].sort_unstable_by_key(|id| id.0);
        Dirty::Unpainted
    }

    pub fn unlink_layer(&mut self, id: LayerId) -> Dirty {
        let Some(set) = self.layer_links.first_mut().filter(|set| set.contains(&id)) else {
            return Dirty::None;
        };
        set.retain(|member| *member != id);
        self.layer_links.retain(|set| !set.is_empty());
        Dirty::Unpainted
    }

    pub fn shadow_attachments(&self) -> &[(LayerId, LayerId)] {
        &self.shadow_attachments
    }

    pub fn shadow_source(&self, shadow: LayerId) -> Option<LayerId> {
        self.shadow_attachments
            .iter()
            .find(|(child, _)| *child == shadow)
            .map(|(_, parent)| *parent)
    }

    pub fn set_shadow_attachments(
        &mut self,
        attachments: Vec<(LayerId, LayerId)>,
    ) -> anyhow::Result<()> {
        let mut children = std::collections::HashSet::new();
        anyhow::ensure!(
            attachments.len() < self.layers.len()
                && attachments.iter().all(|(child, parent)| child != parent
                    && self.layer(*child).is_some()
                    && self.layer(*parent).is_some()
                    && children.insert(*child)),
            "Invalid shadow attachments"
        );
        for &(child, mut parent) in &attachments {
            let mut remaining = attachments.len();
            loop {
                anyhow::ensure!(parent != child && remaining > 0, "Cyclic shadow attachment");
                let Some((_, next)) = attachments.iter().find(|(id, _)| *id == parent) else {
                    break;
                };
                parent = *next;
                remaining -= 1;
            }
        }
        self.shadow_attachments = attachments;
        Ok(())
    }

    pub fn attachment_above(&self, child: LayerId) -> Option<LayerId> {
        let index = self.layer_index(child)?;
        let parent = self.layers.get(index + 1)?.id;
        if self.layer(child)?.text.is_some() || self.shadow_source(child) == Some(parent) {
            return None;
        }
        // Reparenting is safe only when the proposed source is not below the
        // child in the attachment graph. Stack order alone cannot prove this.
        let mut ancestor = parent;
        while let Some(next) = self.shadow_source(ancestor) {
            if next == child {
                return None;
            }
            ancestor = next;
        }
        Some(parent)
    }

    pub fn attach_layer_above(&mut self, child: LayerId) -> Dirty {
        let Some(parent) = self.attachment_above(child) else {
            return Dirty::None;
        };
        self.shadow_attachments.retain(|(id, _)| *id != child);
        self.shadow_attachments.push((child, parent));
        Dirty::Unpainted
    }

    pub fn detach_shadow(&mut self, id: LayerId) -> Dirty {
        let before = self.shadow_attachments.len();
        self.shadow_attachments.retain(|(child, _)| *child != id);
        if before == self.shadow_attachments.len() {
            Dirty::None
        } else {
            Dirty::Unpainted
        }
    }

    pub(super) fn prune_layer_links(&mut self) {
        let layers = &self.layers;
        for set in &mut self.layer_links {
            set.retain(|id| layers.iter().any(|layer| layer.id == *id));
        }
        self.layer_links.retain(|set| !set.is_empty());
        self.shadow_attachments.retain(|(child, parent)| {
            layers.iter().any(|layer| layer.id == *child)
                && layers.iter().any(|layer| layer.id == *parent)
        });
    }

    // Resolve the complete move: follow attachments downward, and include the
    // whole explicit set whenever any reached layer is linked. Never follow an
    // attachment upward; sources move only if independently included by links.
    pub fn movement_layers(&self, id: LayerId) -> Vec<LayerId> {
        if self.layer(id).is_none() {
            return vec![];
        }
        let mut members = match self.linked_layers(id) {
            [] => vec![id],
            set => set.to_vec(),
        };
        let mut index = 0;
        while index < members.len() {
            let parent = members[index];
            for &linked in self.linked_layers(parent) {
                if !members.contains(&linked) {
                    members.push(linked);
                }
            }
            for &(child, source) in &self.shadow_attachments {
                if source == parent && !members.contains(&child) {
                    members.push(child);
                }
            }
            index += 1;
        }
        members
    }

    pub fn move_blocker(&self, id: LayerId) -> Option<LayerId> {
        self.movement_layers(id).into_iter().find(|member| {
            self.layer(*member)
                .is_some_and(|layer| layer.locks.position)
        })
    }

    pub fn can_move_linked(&self, id: LayerId) -> bool {
        self.layer(id).is_some() && self.move_blocker(id).is_none()
    }

    // Validate the whole translation before moving anything, including hidden
    // members and attached descendants. No overflow may publish a partial move.
    pub fn can_move_linked_to(&self, id: LayerId, offset: (i32, i32)) -> bool {
        let Some(layer) = self.layer(id) else {
            return false;
        };
        let delta = (
            i64::from(offset.0) - i64::from(layer.offset.0),
            i64::from(offset.1) - i64::from(layer.offset.1),
        );
        self.can_move_linked(id)
            && self.movement_layers(id).iter().all(|member| {
                self.layer(*member).is_some_and(|layer| {
                    i32::try_from(i64::from(layer.offset.0) + delta.0).is_ok()
                        && i32::try_from(i64::from(layer.offset.1) + delta.1).is_ok()
                })
            })
    }

    pub fn move_linked_layers(&mut self, id: LayerId, offset: (i32, i32)) -> Dirty {
        if !self.can_move_linked_to(id, offset) {
            return Dirty::None;
        }
        let origin = self.layer(id).unwrap().offset;
        if origin == offset {
            return Dirty::None;
        }
        let delta = (
            i64::from(offset.0) - i64::from(origin.0),
            i64::from(offset.1) - i64::from(origin.1),
        );
        let members = self.movement_layers(id);
        let mut bounds: Option<PixelRect> = None;
        for member in members {
            let origin = self.layer(member).unwrap().offset;
            let target = (
                (i64::from(origin.0) + delta.0) as i32,
                (i64::from(origin.1) + delta.1) as i32,
            );
            if let Dirty::Composite(rect) = self.set_offset(member, target) {
                bounds = Some(bounds.map_or(rect, |old| {
                    let left = old.x.min(rect.x);
                    let top = old.y.min(rect.y);
                    PixelRect::new(
                        left,
                        top,
                        (old.right().max(rect.right()) - left) as u32,
                        (old.bottom().max(rect.bottom()) - top) as u32,
                    )
                }));
            }
        }
        bounds.map_or(Dirty::Unpainted, Dirty::Composite)
    }
}
