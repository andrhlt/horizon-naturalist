package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.component.CustomData;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class CaptureProbe {
    public CaptureProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("insect_fixture_capture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var butterfly = NaturalistEntityTypes.BUTTERFLY.get().create(level, EntitySpawnReason.COMMAND);
            var caterpillar = NaturalistEntityTypes.CATERPILLAR.get().create(level, EntitySpawnReason.COMMAND);
            var scorpion = NaturalistEntityTypes.DESERT_SCORPION.get().create(level, EntitySpawnReason.COMMAND);
            if (butterfly == null || caterpillar == null || scorpion == null) {
                throw new IllegalStateException("Missing insects");
            }
            butterfly.setNoAi(true);
            butterfly.setHealth(1.5F);
            butterfly.setVariantString("naturalist:red_admiral");
            caterpillar.setAge(-12000);
            var net = new ItemStack(NaturalistRegistry.CAPTURE_NET.get());
            var full = NaturalistRegistry.BUTTERFLY.get().capture(butterfly)
                .getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            var handStack = caterpillar.getCaughtItemStack();
            caterpillar.saveToHandTag(handStack);
            var hand = handStack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            var scorpionStack = NaturalistRegistry.SCORPION.get().capture(scorpion);
            var scorpionData = scorpionStack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            var result = ("{\"net_durability\":%d,\"full_saves_id\":%s,\"full_saves_uuid\":%s,"
                        + "\"full_saves_variant\":%s,\"full_saves_no_ai\":%s,\"full_saves_health\":%s,"
                        + "\"full_preserves_original_from_hand\":%s,\"hand_saves_age\":%s,\"hand_saves_id\":%s,"
                        + "\"scorpion_caught_item_matches\":%s,\"scorpion_original_id_preserved\":%s}")
                        .formatted(net.getMaxDamage(), full.getStringOr("id", "").equals("naturalist:butterfly"),
                            full.contains("UUID"), full.getStringOr("Variant", "").equals("naturalist:red_admiral"),
                            full.getBooleanOr("NoAI", false), full.getFloatOr("Health", 0) == 1.5F,
                            !full.getBooleanOr("FromHand", true), hand.getIntOr("Age", 0) == -12000,
                            hand.contains("id"), scorpionStack.is(NaturalistRegistry.SCORPION.get()),
                            scorpionData.getStringOr("id", "").equals("naturalist:desert_scorpion"));
            source.sendSuccess(() -> Component.literal("insect_fixture_capture " + result), false);
            return 1;
        }));
    }
}
