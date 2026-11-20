package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.scorpion.Scorpion;
import com.crispytwig.naturalist.world.entity.animal.scorpion.DesertScorpion;
import com.crispytwig.naturalist.world.entity.variant.DataDrivenVariantAnimal;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.Difficulty;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.entity.ai.goal.target.HurtByTargetGoal;
import net.minecraft.world.entity.ai.goal.target.NearestAttackableTargetGoal;
import net.minecraft.world.entity.ai.targeting.TargetingConditions;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class ScorpionProbe {
    public ScorpionProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_scorpion")
                .executes(ctx -> capture(ctx.getSource())));
    }

    private static int capture(CommandSourceStack source) {
        ServerLevel level = source.getLevel();
        var desert = NaturalistEntityTypes.DESERT_SCORPION.get();
        var jungle = NaturalistEntityTypes.JUNGLE_SCORPION.get();
        var fromHand = desert.create(level, EntitySpawnReason.COMMAND);
        fromHand.setCustomName(Component.literal("Fixture Scorpion"));
        fromHand.setHealth(5.5F);
        fromHand.setNoAi(true);
        fromHand.setFromHand(true);
        ItemStack hand = fromHand.getCaughtItemStack();
        fromHand.saveToHandTag(hand);
        var restored = desert.create(level, EntitySpawnReason.COMMAND);
        restored.loadFromHandTag(hand.get(DataComponents.CUSTOM_DATA).copyTag());
        var behavior = new LinkedHashMap<String, Object>(Map.of("caught_item", hand.getItem() == NaturalistRegistry.SCORPION.get(),
                "from_hand_persistent", fromHand.requiresCustomPersistence(),
                "restored_health", restored.getHealth(), "restored_no_ai", restored.isNoAi(),
                "restored_variant", restored.getVariantString(),
                "custom_name", hand.get(DataComponents.CUSTOM_NAME).getString(),
                "portal_allowed", fromHand.canUsePortal(false)));
        behavior.putAll(releasedTargets(level, desert.create(level, EntitySpawnReason.COMMAND)));
        var result = Map.of("desert_scorpion", capture(level, desert), "jungle_scorpion", capture(level, jungle),
                "desert_behavior", behavior);
        source.sendSuccess(() -> Component.literal("naturalist_scorpion " + new Gson().toJson(result)), false);
        return 1;
    }

    private static Map<String, Object> releasedTargets(ServerLevel level, DesertScorpion scorpion) {
        var results = new LinkedHashMap<String, Object>();
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "ReleasedScorpionFixture"));
        var zombie = EntityTypes.ZOMBIE.create(level, EntitySpawnReason.COMMAND);
        player.setPermanentlyInvulnerable(false);
        scorpion.snapTo(8.5, 250, 8.5, 0, 0);
        player.snapTo(8.5, 250, 7.5, 0, 0);
        zombie.snapTo(8.5, 250, 9.5, 0, 0);
        try {
            var selector = Mob.class.getDeclaredField("targetSelector");
            selector.setAccessible(true);
            var goals = ((GoalSelector) selector.get(scorpion)).getAvailableGoals();
            var conditions = NearestAttackableTargetGoal.class.getDeclaredField("targetConditions");
            conditions.setAccessible(true);
            var enemies = goals.stream().filter(goal -> goal.getPriority() == 4)
                    .map(goal -> (NearestAttackableTargetGoal<?>) goal.getGoal()).findFirst().orElseThrow();
            var disabled = (TargetingConditions) conditions.get(enemies);
            results.put("wild_hunts_zombie", disabled.test(level, scorpion, zombie));
            scorpion.setFromHand(true);
            results.put("from_hand_hunts_zombie", disabled.test(level, scorpion, zombie));
            var playerHunt = DesertScorpion.class.getDeclaredMethod("canHuntPlayers");
            playerHunt.setAccessible(true);
            results.put("from_hand_hunts_player", playerHunt.invoke(scorpion));
            scorpion.tickCount = 20;
            scorpion.setLastHurtByMob(player);
            var retaliation = goals.stream().filter(wrapped -> wrapped.getGoal() instanceof HurtByTargetGoal)
                    .findFirst().orElseThrow();
            if (retaliation.canUse()) {
                retaliation.start();
            }
            results.put("from_hand_retaliates", scorpion.getTarget() == player);
        } catch (ReflectiveOperationException failure) {
            throw new IllegalStateException(failure);
        }
        return results;
    }

    private static Map<String, Object> shield(ServerLevel level, Scorpion scorpion) {
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "ScorpionShieldFixture"));
        player.setPermanentlyInvulnerable(false);
        player.snapTo(8.5, 250, 8.5, 0, 0);
        scorpion.snapTo(8.5, 250, 10.5, 180, 0);
        var item = new ItemStack(Items.SHIELD);
        player.setItemInHand(InteractionHand.MAIN_HAND, item);
        player.startUsingItem(InteractionHand.MAIN_HAND);
        try {
            var remaining = LivingEntity.class.getDeclaredField("useItemRemaining");
            remaining.setAccessible(true);
            remaining.setInt(player, item.getUseDuration(player) - 8);
        } catch (ReflectiveOperationException failure) {
            throw new IllegalStateException(failure);
        }
        if (!player.isBlocking()) {
            throw new IllegalStateException("The test shield was not raised");
        }
        var before = player.getHealth();
        var landed = scorpion.doHurtTarget(level, player);
        return Map.of("shielded_hit_landed", landed, "shielded_health_unchanged", player.getHealth() == before,
                "shielded_poisoned", player.hasEffect(MobEffects.POISON));
    }

    private static <T extends Scorpion & DataDrivenVariantAnimal> Map<String, Object> capture(
            ServerLevel level, EntityType<T> kind) {
        T scorpion = kind.create(level, EntitySpawnReason.COMMAND);
        var result = new LinkedHashMap<String, Object>();
        result.put("variant", scorpion.getVariantString());
        result.put("max_health", scorpion.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("movement_speed", scorpion.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
        result.put("attack_damage", scorpion.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
        var dimensions = scorpion.getDimensions(Pose.STANDING);
        result.put("dimensions", List.of(dimensions.width(), dimensions.height(), dimensions.eyeHeight()));
        result.put("no_breeding", scorpion.getBreedOffspring(level, scorpion) == null
                && !scorpion.isFood(new ItemStack(Items.WHEAT)));
        var pig = EntityTypes.PIG.create(level, EntitySpawnReason.COMMAND);
        scorpion.snapTo(7.5D, 71D, 7.5D, 0F, 0F);
        pig.snapTo(7.5D, 71D, 8.5D, 0F, 0F);
        level.addFreshEntity(pig);
        result.put("attack_landed", scorpion.doHurtTarget(level, pig));
        var poison = pig.getEffect(MobEffects.POISON);
        result.put("poison_duration", poison == null ? 0 : poison.getDuration());
        result.put("poison_amplifier", poison == null ? -1 : poison.getAmplifier());
        pig.discard();
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "ScorpionFixture"));
        var hand = InteractionHand.MAIN_HAND;
        scorpion.setHealth(3.0F);
        player.setItemInHand(hand, new ItemStack(NaturalistRegistry.LIZARD_TAIL.get(), 2));
        scorpion.mobInteract(player, hand);
        result.put("healed", scorpion.getHealth());
        result.put("remaining_tail", player.getItemInHand(hand).getCount());
        result.putAll(shield(level, kind.create(level, EntitySpawnReason.COMMAND)));

        scorpion.snapTo(8.5, 250, 8.5, 0, 0);
        result.put("sunlit_attacks_player", scorpion.isInDarkness());
        var difficulty = level.getDifficulty();
        player.setPermanentlyInvulnerable(false);
        level.getServer().setDifficulty(Difficulty.PEACEFUL, true);
        try {
            scorpion.setTarget(player);
            scorpion.checkDespawn();
            result.put("peaceful_attacks_player", scorpion.getTarget() == player);
            result.put("peaceful_survives", !scorpion.isRemoved());
        } finally {
            level.getServer().setDifficulty(difficulty, true);
        }
        return result;
    }
}
