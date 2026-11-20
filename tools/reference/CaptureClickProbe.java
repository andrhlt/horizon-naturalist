package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.nbt.CompoundTag;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.item.Item;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class CaptureClickProbe {
    public CaptureClickProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    record Caught(CompoundTag data, String customName, boolean removed, int netDamage) {}

    private static Caught capture(ServerLevel level, Mob mob, Item found) {
        var player = FakePlayerFactory.getMinecraft(level);
        player.getInventory().clearContent();
        var net = new ItemStack(NaturalistRegistry.CAPTURE_NET.get());
        player.setItemInHand(InteractionHand.MAIN_HAND, net);
        mob.snapTo(8.5, 94, 9.5, 0, 0);
        if (!level.addFreshEntity(mob)) throw new IllegalStateException("Mob did not spawn");
        player.interactOn(mob, InteractionHand.MAIN_HAND, Vec3.ZERO);
        for (int n = 0; n < player.getInventory().getContainerSize(); n++) {
            var stack = player.getInventory().getItem(n);
            if (stack.is(found)) {
                return new Caught(stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag(),
                                  stack.has(DataComponents.CUSTOM_NAME) ? stack.getHoverName().getString() : "",
                                  mob.isRemoved(), net.getDamageValue());
            }
        }
        throw new IllegalStateException("Correct caught item not found for " + mob);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("insect_fixture_capture_click").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var butterfly = NaturalistEntityTypes.BUTTERFLY.get().create(level, EntitySpawnReason.COMMAND);
            var caterpillar = NaturalistEntityTypes.CATERPILLAR.get().create(level, EntitySpawnReason.COMMAND);
            var scorpion = NaturalistEntityTypes.DESERT_SCORPION.get().create(level, EntitySpawnReason.COMMAND);
            var dragonfly = NaturalistEntityTypes.DRAGONFLY.get().create(level, EntitySpawnReason.COMMAND);
            var firefly = NaturalistEntityTypes.FIREFLY.get().create(level, EntitySpawnReason.COMMAND);
            if (butterfly == null || caterpillar == null || scorpion == null || dragonfly == null || firefly == null) {
                throw new IllegalStateException("Missing insects");
            }
            butterfly.setNoAi(true);
            butterfly.setHealth(1.5F);
            butterfly.setCustomName(Component.literal("Beacon"));
            butterfly.setVariantString("naturalist:red_admiral");
            caterpillar.setAge(-12000);
            var b = capture(level, butterfly, NaturalistRegistry.BUTTERFLY.get());
            var c = capture(level, caterpillar, NaturalistRegistry.CATERPILLAR.get());
            var s = capture(level, scorpion, NaturalistRegistry.SCORPION.get());
            var d = capture(level, dragonfly, NaturalistRegistry.DRAGONFLY.get());
            var f = capture(level, firefly, NaturalistRegistry.FIREFLY.get());
            var result = ("{\"butterfly_saves_id\":%s,\"butterfly_saves_uuid\":%s,"
                        + "\"butterfly_saves_variant\":%s,\"butterfly_saves_no_ai\":%s,"
                        + "\"butterfly_saves_health\":%s,\"butterfly_saves_name_component\":%s,"
                        + "\"caterpillar_saves_age\":%s,\"caterpillar_saves_id\":%s,"
                        + "\"scorpion_saves_id\":%s,\"dragonfly_saves_id\":%s,"
                        + "\"firefly_saves_id\":%s,\"all_removed\":%s,\"damage_each\":%s}")
                        .formatted(b.data.contains("id"), b.data.contains("UUID"),
                            b.data.getStringOr("Variant", "").equals("naturalist:red_admiral"),
                            b.data.getBooleanOr("NoAI", false), b.data.getFloatOr("Health", 0) == 1.5F,
                            b.customName.equals("Beacon"),
                            c.data.getIntOr("Age", 0) == -12000, c.data.contains("id"),
                            s.data.contains("id"), d.data.getStringOr("id", "").equals("naturalist:dragonfly"),
                            f.data.getStringOr("id", "").equals("naturalist:firefly"),
                            b.removed && c.removed && s.removed && d.removed && f.removed,
                            b.netDamage == 1 && c.netDamage == 1 && s.netDamage == 1 && d.netDamage == 1 && f.netDamage == 1);
            source.sendSuccess(() -> Component.literal("insect_fixture_capture_click " + result), false);
            return 1;
        }));
    }
}
