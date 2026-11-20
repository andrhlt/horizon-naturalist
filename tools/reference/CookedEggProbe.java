package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.google.gson.Gson;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.tags.TagKey;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.crafting.CraftingInput;
import net.minecraft.world.item.crafting.RecipeType;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class CookedEggProbe {
    public CookedEggProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("cooked_egg_probe").executes(context -> {
            var eggTag = TagKey.create(Registries.ITEM, Identifier.parse("c:eggs"));
            var items = new ArrayList<String>();
            for (var item : BuiltInRegistries.ITEM.getTagOrEmpty(eggTag)) {
                items.add(BuiltInRegistries.ITEM.getKey(item.value()).toString());
            }
            items.sort(String::compareTo);
            var food = new ItemStack(NaturalistRegistry.COOKED_EGG.get()).get(DataComponents.FOOD);
            var level = context.getSource().getLevel();
            var recipes = new LinkedHashMap<String, Object>();
            for (String egg : List.of("naturalist:duck_egg", "minecraft:egg", "minecraft:snowball")) {
                recipes.put("cake/" + egg, craft(level, 3, 3,
                    "milk_bucket", "milk_bucket", "milk_bucket", "sugar", egg, "sugar", "wheat", "wheat", "wheat"));
                recipes.put("pumpkin_pie/" + egg, craft(level, 2, 2, "pumpkin", "sugar", egg, ""));
            }
            var result = Map.of("egg_tag", items, "food", List.of(food.nutrition(), food.saturation(), food.canAlwaysEat()),
                "recipes", recipes);
            context.getSource().sendSuccess(() -> Component.literal("cooked_egg_probe " + new Gson().toJson(result)), false);
            return 1;
        }));
    }

    private static Object craft(ServerLevel level, int width, int height, String... names) {
        var grid = new ArrayList<ItemStack>();
        for (String item : names) {
            grid.add(item.isEmpty() ? ItemStack.EMPTY :
                new ItemStack(BuiltInRegistries.ITEM.getValue(Identifier.parse(item))));
        }
        var input = CraftingInput.of(width, height, grid);
        var recipe = level.recipeAccess().getRecipeFor(RecipeType.CRAFTING, input, level);
        if (recipe.isEmpty()) {
            return List.of();
        }
        var holder = recipe.get();
        var result = holder.value().assemble(input);
        return List.of(holder.id().identifier().toString(), BuiltInRegistries.ITEM.getKey(result.getItem()).toString(),
            result.getCount());
    }
}
