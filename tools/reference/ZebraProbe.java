package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.equine.Zebra;
import com.google.gson.Gson;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.RandomSource;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.animal.equine.AbstractHorse;
import net.minecraft.world.entity.player.Player;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class ZebraProbe {
    public ZebraProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_zebra").executes(ctx -> {
            var source = ctx.getSource();
            Map<String, Object> output;
            try {
                output = capture(source.getLevel());
            } catch (ReflectiveOperationException | RuntimeException error) {
                error.printStackTrace();
                throw new IllegalStateException(error);
            }
            source.sendSuccess(() -> Component.literal("naturalist_fixture_zebra " + new Gson().toJson(output)), false);
            return 1;
        }));
    }

    private static Map<String, Object> capture(ServerLevel level) throws ReflectiveOperationException {
        Player player = FakePlayerFactory.getMinecraft(level);
        var zebra = zebra(level);
        var out = new LinkedHashMap<String, Object>();
        out.put("variant", zebra.getVariantString());
        out.put("default_stats", stats(zebra));
        out.put("adult_dimensions", dimensions(zebra));
        zebra.setAge(-24000);
        out.put("juvenile_dimensions", dimensions(zebra));
        zebra.setAge(0);

        var randomize = Zebra.class.getDeclaredMethod("randomizeAttributes", RandomSource.class);
        randomize.setAccessible(true);
        var random = RandomSource.create(42L);
        var randomized = new java.util.ArrayList<List<Double>>();
        for (int i = 0; i < 8; i++) {
            randomize.invoke(zebra, random);
            randomized.add(stats(zebra));
        }
        out.put("randomized", randomized);
        out.put("max_temper", zebra.getMaxTemper());

        var accepted = new LinkedHashMap<String, Object>();
        for (var item : List.of(Items.WHEAT, Items.SUGAR, Items.HAY_BLOCK, Items.APPLE, Items.GOLDEN_CARROT,
                Items.GOLDEN_APPLE, Items.ENCHANTED_GOLDEN_APPLE, Items.GOLDEN_DANDELION, Items.CARROT, Items.MELON_SLICE)) {
            var test = zebra(level);
            test.setHealth(test.getMaxHealth() - 5);
            var stack = new ItemStack(item, 2);
            boolean food = test.isFood(stack);
            var feed = player.getItemInHand(InteractionHand.MAIN_HAND);
            player.setItemInHand(InteractionHand.MAIN_HAND, stack);
            if (food) {
                test.mobInteract(player, InteractionHand.MAIN_HAND);
            }
            accepted.put(item.toString(), List.of(food, test.getTemper(), test.getHealth(), stack.getCount()));
            player.setItemInHand(InteractionHand.MAIN_HAND, feed);
        }
        out.put("feeding", accepted);

        var other = zebra(level);
        var donkey = EntityTypes.DONKEY.create(level, EntitySpawnReason.BREEDING);
        var horse = EntityTypes.HORSE.create(level, EntitySpawnReason.BREEDING);
        for (var adult : List.of(zebra, other, donkey, horse)) {
            adult.tameWithName(player);
            adult.setHealth(adult.getMaxHealth());
            adult.setInLove(player);
        }
        out.put("can_mate", List.of(zebra.canMate(other), zebra.canMate(donkey), zebra.canMate(horse), zebra.canMate(zebra)));
        var child = zebra.getBreedOffspring(level, other);
        out.put("offspring_is_zebra", child instanceof Zebra);
        out.put("initially_chested", zebra.hasChest());
        var stack = new ItemStack(Items.CHEST, 1);
        player.setItemInHand(InteractionHand.MAIN_HAND, stack);
        zebra.mobInteract(player, InteractionHand.MAIN_HAND);
        out.put("chest_added", zebra.hasChest() && stack.isEmpty());
        var saddle = new ItemStack(Items.SADDLE, 1);
        player.setItemInHand(InteractionHand.MAIN_HAND, saddle);
        zebra.mobInteract(player, InteractionHand.MAIN_HAND);
        out.put("saddle_added", zebra.isSaddled() && saddle.isEmpty());
        return out;
    }

    private static Zebra zebra(ServerLevel level) {
        return NaturalistEntityTypes.ZEBRA.get().create(level, EntitySpawnReason.COMMAND);
    }

    private static List<Double> stats(AbstractHorse horse) {
        return List.of(horse.getAttributeBaseValue(Attributes.MAX_HEALTH), horse.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
                horse.getAttributeBaseValue(Attributes.JUMP_STRENGTH), horse.getAttributeBaseValue(Attributes.STEP_HEIGHT),
                horse.getAttributeBaseValue(Attributes.FOLLOW_RANGE));
    }

    private static List<Float> dimensions(Zebra zebra) {
        return List.of(zebra.getBbWidth(), zebra.getBbHeight(), zebra.getEyeHeight());
    }
}
