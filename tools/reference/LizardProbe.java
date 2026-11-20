package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.lizard.LizardTail;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.UUID;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class LizardProbe {
    public LizardProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_lizard")
                .executes(ctx -> capture(ctx.getSource())));
    }

    private static int capture(CommandSourceStack source) {
        ServerLevel level = source.getLevel();
        var lizard = NaturalistEntityTypes.LIZARD.get().create(level, EntitySpawnReason.COMMAND);
        var shape = lizard.getDimensions(Pose.STANDING);
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "LizardFixture"));
        var hand = InteractionHand.MAIN_HAND;
        var result = new LinkedHashMap<String, Object>();
        result.put("variant", lizard.getVariantString());
        result.put("dimensions", List.of(shape.width(), shape.height(), shape.eyeHeight()));
        result.put("max_health", lizard.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("movement_speed", lizard.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
        result.put("attack_damage", lizard.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
        result.put("initial_tail", lizard.hasTail());
        var food = new ItemStack(Items.SPIDER_EYE, 2);
        result.put("cant_breed", !lizard.isFood(food) && lizard.getBreedOffspring(level, lizard) == null);
        player.setItemInHand(hand, food.copy());
        lizard.getRandom().setSeed(2L);
        lizard.mobInteract(player, hand);
        result.put("tame_failure_consumed", !lizard.isTame() && player.getItemInHand(hand).getCount() == 1);
        lizard.getRandom().setSeed(1L);
        lizard.mobInteract(player, hand);
        result.put("tame_succeeded", lizard.isOwnedBy(player));
        result.put("tame_sitting", lizard.isOrderedToSit());
        result.put("tame_max_health", lizard.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("tame_attack_damage", lizard.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
        result.put("tame_health", lizard.getHealth());
        lizard.snapTo(5.5D, 71D, 7.5D, 0.0F, 0.0F);
        lizard.setVariantString("naturalist:leopard_gecko");
        lizard.setHealth(10.0F);
        lizard.hurtServer(level, lizard.damageSources().generic(), 1.0F);
        result.put("dropped_tail_at_half_health", !lizard.hasTail());
        var tail = level.getEntitiesOfClass(LizardTail.class, lizard.getBoundingBox().inflate(2)).stream().findFirst().orElseThrow();
        result.put("tail_variant", tail.getVariantString());
        result.put("tail_health", tail.getHealth());
        var tailShape = tail.getDimensions(Pose.STANDING);
        result.put("tail_dimensions", List.of(tailShape.width(), tailShape.height(), tailShape.eyeHeight()));
        tail.discard();
        lizard.setHealth(15.0F);
        player.setItemInHand(hand, food.copy());
        lizard.mobInteract(player, hand);
        result.put("health_after_feeding", lizard.getHealth());
        result.put("tail_restored_when_healed", lizard.hasTail());
        result.put("healing_food_consumed", player.getItemInHand(hand).getCount() == 1);
        source.sendSuccess(() -> Component.literal("naturalist_lizard " + new Gson().toJson(result)), false);
        return 1;
    }
}
