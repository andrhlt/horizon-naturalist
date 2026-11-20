package naturalist.fixture;

import com.google.gson.Gson;
import java.util.Locale;
import java.util.TreeMap;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Rarity;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class ItemProfileProbe {
    public ItemProfileProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    record Food(int nutrition, float saturation, boolean always_eat) {}
    record Profile(int max_stack, int max_damage, String rarity, Food food) {}

    private static String profiles() {
        var results = new TreeMap<String, Profile>();
        for (var item : BuiltInRegistries.ITEM) {
            var id = BuiltInRegistries.ITEM.getKey(item);
            if (!id.getNamespace().equals("naturalist")) continue;
            var stack = new ItemStack(item);
            var food = stack.get(DataComponents.FOOD);
            results.put(id.toString(), new Profile(
                stack.getMaxStackSize(), stack.getMaxDamage(),
                stack.getOrDefault(DataComponents.RARITY, Rarity.COMMON).name().toLowerCase(Locale.ROOT),
                food == null ? null : new Food(food.nutrition(), food.saturation(), food.canAlwaysEat())));
        }
        return new Gson().toJson(results);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_item_profiles").executes(ctx -> {
            var profiles = profiles();
            ctx.getSource().sendSuccess(() -> Component.literal("naturalist_fixture_item_profiles " + profiles), false);
            return 1;
        }));
    }
}
