# Lists for a recipe viewer

`ClientHost` answers four reads a recipe viewer builds its lists from. All are client side and need a world.

- `creative_inventory_stacks()` is every stack of the creative inventory, variants included, as `CreativeStack { stack, tabs, name, tooltip }`: the stack with its components, the titles of the tabs that show it, its hover name and its tooltip lines after the name (plain text). It answers the contents the game built when the creative inventory last opened, or the exported ones before that.
- `anvil_recipes()` is what `AnvilMenu.createResult` gives, as `AnvilRecipe { left, right, results, costs }`: `results[i]` and `costs[i]` belong to `right[i]`. It repairs each damageable item with each material that repairs it and enchants each item with a book of each level. Computing it takes a moment, so call it once and keep the answer.
- `item_fuel(item, fast)` is `FuelInfo { ticks, speed }` for a fuel, or `None`. `fast` asks for the smoker's and blast furnace's values.
- `screen_hovered_stack()` is the stack under the pointer on the open container screen, with its components (`screen_hovered_item` answers only its id).
