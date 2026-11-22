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
public final class AntCaptureProbe {
    public AntCaptureProbe() {
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
        event.getDispatcher().register(Commands.literal("ant_fixture_capture_click").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = FakePlayerFactory.getMinecraft(level);
            var results = new java.util.ArrayList<String>();
            for (boolean tame : new boolean[]{false, true}) {
                var ant = NaturalistEntityTypes.ANT.get().create(level, EntitySpawnReason.COMMAND);
                ant.setNoAi(true);
                ant.setHealth(1.5F);
                ant.setCustomName(Component.literal("Beacon"));
                ant.setVariantString("naturalist:ant");
                if (tame) {
                    ant.setOwnerUUID(player.getUUID());
                    ant.setTame(true, false);
                }
                var caught = capture(level, ant, NaturalistRegistry.ANT.get());
                results.add("{\"tame\":%s,\"data\":\"%s\",\"name\":\"%s\",\"removed\":%s,\"damage\":%d}"
                    .formatted(tame, caught.data.toString().replace("\\", "\\\\").replace("\"", "\\\""), caught.customName, caught.removed, caught.netDamage));
            }
            var result = "[" + String.join(",", results) + "]";
            source.sendSuccess(() -> Component.literal("ant_fixture_capture_click " + result), false);
            return 1;
        }));
    }
}
