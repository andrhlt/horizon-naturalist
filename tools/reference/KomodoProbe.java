package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.UUID;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class KomodoProbe {
    public KomodoProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_komodo")
                .executes(ctx -> capture(ctx.getSource())));
    }

    private static int capture(CommandSourceStack source) {
        ServerLevel level = source.getLevel();
        var type = NaturalistEntityTypes.KOMODO_DRAGON.get();
        var dragon = type.create(level, EntitySpawnReason.COMMAND);
        var mate = type.create(level, EntitySpawnReason.COMMAND);
        var shape = dragon.getDimensions(Pose.STANDING);
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "KomodoFixture"));
        var hand = InteractionHand.MAIN_HAND;
        var result = new LinkedHashMap<String, Object>();
        result.put("variant", dragon.getVariantString());
        result.put("dimensions", List.of(shape.width(), shape.height(), shape.eyeHeight()));
        result.put("max_health", dragon.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("movement_speed", dragon.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
        result.put("attack_damage", dragon.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
        result.put("step_height", dragon.getAttributeBaseValue(Attributes.STEP_HEIGHT));
        result.put("food", BuiltInRegistries.ITEM.entrySet().stream()
                .filter(entry -> !new ItemStack(entry.getValue()).isEmpty() && dragon.isFood(new ItemStack(entry.getValue())))
                .map(entry -> entry.getKey().identifier().toString()).sorted().toList());
        var food = new ItemStack(Items.BEEF, 2);
        dragon.setHealth(20.0F);
        player.setItemInHand(hand, food.copy());
        dragon.mobInteract(player, hand);
        result.put("healed_health", dragon.getHealth());
        result.put("heal_consumed", player.getItemInHand(hand).getCount() == 1);
        dragon.setHealth(30.0F);
        player.setItemInHand(hand, food.copy());
        dragon.mobInteract(player, hand);
        result.put("fed_adult", dragon.isInLove());
        mate.setInLove(player);
        result.put("same_species_mate", dragon.canMate(mate));
        dragon.snapTo(8.5D, 71D, 7.5D, 0F, 0F);
        result.put("can_bask_in_fixture", dragon.canSleep());
        dragon.setSleeping(true);
        result.put("basking_flag", dragon.isSleeping());
        var pig = EntityTypes.PIG.create(level, EntitySpawnReason.COMMAND);
        pig.snapTo(7.5D, 71D, 7.5D, 0F, 0F);
        level.addFreshEntity(pig);
        result.put("attack_landed", dragon.doHurtTarget(level, pig));
        var poison = pig.getEffect(MobEffects.POISON);
        result.put("poison_duration", poison == null ? 0 : poison.getDuration());
        result.put("poison_amplifier", poison == null ? -1 : poison.getAmplifier());
        pig.discard();
        source.sendSuccess(() -> Component.literal("naturalist_komodo " + new Gson().toJson(result)), false);
        return 1;
    }
}
