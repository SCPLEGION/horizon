//! What a recipe viewer lists beside the game's own tables (`ClientHost::creative_inventory_stacks`,
//! `anvil_recipes`, `brewing_recipes`, `grindstone_recipes`, `item_fuel` and `screen_hovered_stack`): the creative inventory's stacks with
//! their tabs and names, the anvil's results, a fuel's speed, and the stack under the pointer.
use super::{Game, api_values::TryIntoApi};
use horizon_client::items::ItemStack;
use mod_api::{AnvilRecipe, BrewingRecipe, CreativeStack, FuelInfo, GrindstoneRecipe};

impl Game<'_> {
    /// The creative inventory's stacks with their tabs, names and tooltip lines.
    pub(in crate::game) fn creative_inventory_stacks(&self) -> Vec<CreativeStack> {
        let screen = &self.creative_screen;
        screen
            .search_items()
            .iter()
            .filter_map(|stack| {
                let tabs = screen
                    .tabs_containing(stack)
                    .into_iter()
                    .filter_map(|i| screen.tabs.get(i))
                    .map(|tab| self.creative_tab_title(tab))
                    .collect();
                let source = stack;
                let tooltip = self
                    .stack_tooltip_plain(source)
                    .into_iter()
                    .skip(1)
                    .collect();
                let stack = source.clone().try_into_api().ok()?;
                Some(CreativeStack {
                    stack,
                    tabs,
                    name: self.hover_name(&source),
                    tooltip,
                })
            })
            .collect()
    }
    /// The tooltip of `stack` as plain lines, the name first.
    fn stack_tooltip_plain(&self, stack: &ItemStack) -> Vec<String> {
        self.build_stack_tooltip_lines(stack, false, true, false, false)
            .into_iter()
            .map(|(text, _)| text.plain())
            .collect()
    }
    /// `AnvilMenu.createResult` over every repair and every enchanted book.
    pub(in crate::game) fn anvil_recipes(&self) -> Vec<AnvilRecipe> {
        let items = self.items();
        let e = &self.stations().enchantments;
        let ids: Vec<&std::sync::Arc<str>> = items.registry_order.iter().collect();
        let mut out = Vec::new();
        for id in &ids {
            let Some(info) = items.get(id) else { continue };
            // Repairs: a fully damaged item and each material that repairs it.
            if info.max_damage > 0 {
                let mut left = ItemStack::new(id, 1);
                left.damage = info.max_damage;
                let mut recipe = AnvilRecipe {
                    left: left.clone().try_into_api().ok().into_iter().collect(),
                    right: Vec::new(),
                    results: Vec::new(),
                    costs: Vec::new(),
                };
                for mat in &ids {
                    let material = ItemStack::new(mat, 1);
                    if !horizon_client::stations::is_valid_repair_item(e, items, &left, &material) {
                        continue;
                    }
                    let r = horizon_client::stations::anvil_result(
                        e,
                        items,
                        Some(&left),
                        Some(&material),
                        None,
                        "",
                        true,
                    );
                    if let (Some(result), Ok(material)) =
                        (r.result.and_then(|r| r.try_into_api().ok()), material.try_into_api())
                    {
                        recipe.right.push(material);
                        recipe.results.push(result);
                        recipe.costs.push(r.cost);
                    }
                }
                if !recipe.right.is_empty() {
                    out.push(recipe);
                }
            }
            // Enchanting: a book of each level of each enchantment the item takes.
            if e.enchantable(id) > 0 {
                for ench in &e.list {
                    if !e.can_enchant(items, &ench.id, id) {
                        continue;
                    }
                    let left = ItemStack::new(id, 1);
                    let mut recipe = AnvilRecipe {
                        left: left.clone().try_into_api().ok().into_iter().collect(),
                        right: Vec::new(),
                        results: Vec::new(),
                        costs: Vec::new(),
                    };
                    for level in 1..=ench.max_level {
                        let mut book = ItemStack::new("minecraft:enchanted_book", 1);
                        book.enchant(&ench.id, level);
                        let r = horizon_client::stations::anvil_result(
                            e,
                            items,
                            Some(&left),
                            Some(&book),
                            None,
                            "",
                            true,
                        );
                        if let (Some(result), Ok(book)) =
                            (r.result.and_then(|r| r.try_into_api().ok()), book.try_into_api())
                        {
                            recipe.right.push(book);
                            recipe.results.push(result);
                            recipe.costs.push(r.cost);
                        }
                    }
                    if !recipe.right.is_empty() {
                        out.push(recipe);
                    }
                }
            }
        }
        out
    }
    /// `GrindstoneMenu.computeResult` over every repair (two half-worn copies of a damageable
    /// item) and every disenchanting (each item and each book holding one non-curse enchantment).
    pub(in crate::game) fn grindstone_recipes(&self) -> Vec<GrindstoneRecipe> {
        let items = self.items();
        let e = &self.stations().enchantments;
        let context = &*self.loop_context;
        let make = |top: &ItemStack, bottom: Option<&ItemStack>| -> Option<GrindstoneRecipe> {
            let result =
                horizon_client::stations::grindstone_result(context, e, items, Some(top), bottom)?;
            let base = horizon_client::stations::grindstone_xp_base(e, Some(top), bottom);
            let (xp_min, xp_max) = if base > 0 {
                let half = (f64::from(base) / 2.0).ceil() as i32;
                (half, half * 2 - 1)
            } else {
                (0, 0)
            };
            Some(GrindstoneRecipe {
                top: top.try_into_api().ok().into_iter().collect(),
                bottom: bottom.and_then(|b| b.try_into_api().ok()).into_iter().collect(),
                output: result.try_into_api().ok().into_iter().collect(),
                xp_min,
                xp_max,
            })
        };
        let mut out = Vec::new();
        for id in items.registry_order.iter() {
            let Some(info) = items.get(id) else { continue };
            if info.max_damage > 0 {
                let mut worn = ItemStack::new(id, 1);
                worn.damage = info.max_damage * 3 / 4;
                out.extend(make(&worn, Some(&worn)));
            }
            let candidates = if &**id == "minecraft:enchanted_book" {
                true
            } else {
                e.enchantable(id) > 0
            };
            if !candidates {
                continue;
            }
            for ench in &e.list {
                if e.is_curse(&ench.id)
                    || (&**id != "minecraft:enchanted_book"
                        && !e.can_enchant(items, &ench.id, id))
                {
                    continue;
                }
                let mut stack = ItemStack::new(id, 1);
                stack.enchant(&ench.id, ench.max_level);
                out.extend(make(&stack, None));
            }
        }
        out
    }
    /// The `minecraft:brewing` recipes, each combination of a recipe's bottles and reagents listed.
    pub(in crate::game) fn brewing_recipes(&self) -> Vec<BrewingRecipe> {
        let stack_of = |ingredient: &horizon_client::brewing::PotionIngredient| {
            let mut items: Vec<&String> = ingredient.items.iter().collect();
            items.sort();
            let mut stacks = Vec::new();
            for item in items {
                match &ingredient.potions {
                    None => stacks.push(ItemStack::new(item, 1)),
                    Some(potions) => {
                        let mut potions: Vec<&String> = potions.iter().collect();
                        potions.sort();
                        stacks.extend(
                            potions
                                .into_iter()
                                .map(|potion| ItemStack::new(item, 1).with_potion(potion)),
                        );
                    }
                }
            }
            stacks
        };
        self.stations()
            .brewing
            .recipes
            .iter()
            .filter_map(|recipe| {
                let (inputs, reagents) = (stack_of(&recipe.input), stack_of(&recipe.reagent));
                let mut out = BrewingRecipe {
                    input: Vec::new(),
                    ingredient: Vec::new(),
                    output: Vec::new(),
                };
                for input in &inputs {
                    for reagent in &reagents {
                        if let (Ok(i), Ok(r), Ok(o)) = (
                            input.try_into_api(),
                            reagent.try_into_api(),
                            recipe.output.try_into_api(),
                        ) {
                            out.input.push(i);
                            out.ingredient.push(r);
                            out.output.push(o);
                        }
                    }
                }
                (!out.input.is_empty()).then_some(out)
            })
            .collect()
    }
    /// `item`'s cooking-fuel burn time and speed, the smoker's and blast furnace's with `fast`.
    pub(in crate::game) fn item_fuel(&self, item: mod_api::ItemKey, fast: bool) -> Option<FuelInfo> {
        let id = mod_api::__private::item_key_id(item)?;
        let stack = ItemStack::new(id, 1);
        let ticks = self.res.providers().burn_time(self.items(), &stack, fast);
        (ticks > 0).then(|| FuelInfo {
            ticks,
            speed: self.res.providers().speed(self.items(), &stack, fast),
        })
    }
    /// The stack in the slot under the pointer on the open container screen, with its components.
    pub(in crate::game) fn screen_hovered_stack(&self) -> Option<ItemStack> {
        let size = self
            .renderer
            .as_ref()
            .map(|r| [r.config.width, r.config.height])?;
        let batch = crate::game::HudBatch::new(size, self.settings.gui_scale_inputs());
        let slot = self.slot_at(&batch, self.cursor.0, self.cursor.1)?;
        self.slot_view(slot)
    }
    /// The local player's hotbar and main inventory, the empty slots left out.
    pub(in crate::game) fn local_player_inventory(&self) -> Vec<ItemStack> {
        self.inventory.iter().flatten().cloned().collect()
    }
    /// The stack on the cursor.
    pub(in crate::game) fn screen_carried_stack(&self) -> Option<ItemStack> {
        self.carried.clone()
    }
    /// Puts `stack` on the cursor.
    pub(in crate::game) fn set_screen_carried_stack(&mut self, stack: Option<ItemStack>) {
        self.carried = stack;
    }
    /// The item tags `item` is in, sorted.
    pub(in crate::game) fn item_tags(&self, item: mod_api::ItemKey) -> Vec<String> {
        let Some(id) = mod_api::__private::item_key_id(item) else { return Vec::new() };
        let mut tags: Vec<String> = self
            .items()
            .item_tags
            .iter()
            .filter(|(_, members)| members.contains(&*id))
            .map(|(tag, _)| tag.clone())
            .collect();
        tags.sort();
        tags
    }
    /// The fluid tags `fluid` is in, sorted.
    pub(in crate::game) fn fluid_tag_ids(&self, fluid: &str) -> Vec<String> {
        let fluid = if fluid.contains(':') { fluid.to_string() } else { format!("minecraft:{fluid}") };
        let mut tags: Vec<String> = self
            .items()
            .fluid_tags
            .iter()
            .filter(|(_, members)| members.contains(&fluid))
            .map(|(tag, _)| tag.clone())
            .collect();
        tags.sort();
        tags
    }
}
