package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.snail.Snail;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.item.ItemStack;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.resources.Identifier;
import net.minecraft.world.item.component.CustomData;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SnailProbe {
    public SnailProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("snail_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            var player = FakePlayerFactory.getMinecraft(level);
            var snail = NaturalistEntityTypes.SNAIL.get().create(level, EntitySpawnReason.COMMAND);
            if (snail == null) throw new IllegalStateException("Missing snail");
            snail.snapTo(8.5, 93.0, 8.5, 0, 0);
            level.addFreshEntity(snail);
            player.getInventory().clearContent();
            var dye = new ItemStack(BuiltInRegistries.ITEM.getValue(Identifier.withDefaultNamespace("purple_dye")), 2);
            player.setItemInHand(InteractionHand.MAIN_HAND, dye);
            snail.mobInteract(player, InteractionHand.MAIN_HAND);
            boolean dyed = snail.getSnailColor() == Snail.Color.PURPLE && dye.getCount() == 1;
            snail.mobInteract(player, InteractionHand.MAIN_HAND);
            boolean sameDyeRetained = dye.getCount() == 1;
            snail.setAge(-3000);
            snail.setHealth(2.0F);
            snail.setHasEgg(true);
            var net = new ItemStack(NaturalistRegistry.CAPTURE_NET.get());
            player.setItemInHand(InteractionHand.MAIN_HAND, net);
            player.interactOn(snail, InteractionHand.MAIN_HAND, Vec3.ZERO);
            ItemStack captured = ItemStack.EMPTY;
            for (int i = 0; i < player.getInventory().getContainerSize(); ++i) {
                var candidate = player.getInventory().getItem(i);
                if (candidate.is(NaturalistRegistry.SNAIL.get())) {
                    captured = candidate;
                    break;
                }
            }
            if (captured.isEmpty()) throw new IllegalStateException("Missing caught snail");
            var data = captured.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            var released = NaturalistEntityTypes.SNAIL.get().create(level, EntitySpawnReason.BUCKET);
            if (released == null) throw new IllegalStateException("Missing released snail");
            released.loadFromHandTag(data);

            String result = ("{\"dyes_once\":%s,\"retains_matching_dye\":%s,"
                + "\"body_capture\":%s,\"caught_color\":%d,\"caught_age\":%d,"
                + "\"restores_color_and_health\":%s,\"carried_egg_omitted\":%s}")
                .formatted(dyed, sameDyeRetained, snail.isRemoved() && net.getDamageValue() == 1
                    && !data.contains("id") && !data.contains("UUID"), data.getIntOr("Color", -1),
                    data.getIntOr("Age", 0),
                    released.getSnailColor() == Snail.Color.PURPLE && released.getHealth() == 2.0F,
                    !data.contains("HasEgg") && !released.hasEgg());
            source.sendSuccess(() -> Component.literal("snail_fixture " + result), false);
            return 1;
        }));
    }
}
