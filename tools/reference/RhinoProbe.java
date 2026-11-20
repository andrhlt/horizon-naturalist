package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.rhino.Rhino;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.lang.reflect.Constructor;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.resources.Identifier;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.pathfinder.Node;
import net.minecraft.world.level.pathfinder.Path;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.damagesource.DamageSource;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.Goal;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class RhinoProbe {
    public RhinoProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_rhino")
                .executes(ctx -> capture(ctx.getSource())));
    }

    private static final class ProbeRhino extends Rhino {
        LivingEntity fixtureTarget;

        ProbeRhino(EntityType<Rhino> type, ServerLevel level) {
            super(type, level);
        }

        @Override
        public LivingEntity getTarget() {
            return fixtureTarget;
        }
    }

    private static int capture(CommandSourceStack source) {
        ServerLevel level = source.getLevel();
        var rhino = new ProbeRhino(NaturalistEntityTypes.RHINO.get(), level);
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "RhinoFixture"));
        rhino.snapTo(7.5D, 71D, 7.5D, 0F, 0F);
        player.snapTo(7.5D, 71D, 10.5D, 0F, 0F);
        var dimensions = rhino.getDimensions(Pose.STANDING);
        var result = new LinkedHashMap<String, Object>();
        result.put("variant", rhino.getVariantString());
        result.put("adult", List.of(dimensions.width(), dimensions.height(), dimensions.eyeHeight()));
        rhino.setAge(-24000);
        var baby = rhino.getDimensions(Pose.STANDING);
        result.put("juvenile", List.of(baby.width(), baby.height(), baby.eyeHeight()));
        rhino.setAge(0);
        result.put("max_health", rhino.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("movement_speed", rhino.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
        result.put("attack_damage", rhino.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
        result.put("attack_knockback", rhino.getAttributeBaseValue(Attributes.ATTACK_KNOCKBACK));
        result.put("knockback_resistance", rhino.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE));
        result.put("step_height", rhino.getAttributeBaseValue(Attributes.STEP_HEIGHT));
        result.put("no_food", !rhino.isFood(new ItemStack(Items.WHEAT)) && !rhino.isFood(new ItemStack(Items.CARROT)));
        level.addFreshEntity(rhino);
        try {
            var prepareType = Class.forName(Rhino.class.getName() + "$RhinoPrepareChargeGoal");
            Constructor<?> constructor = prepareType.getDeclaredConstructor(Rhino.class);
            constructor.setAccessible(true);
            var prepare = (Goal) constructor.newInstance(rhino);
            result.put("wont_charge_without_target", !prepare.canUse());
            result.put("reset_charge_cooldown", rhino.getChargeCooldownTicks());
            rhino.fixtureTarget = player;
            result.put("prepare_can_start_with_player", prepare.canUse());
            prepare.start();
            result.put("preparing_has_target", rhino.hasTarget());
            result.put("preparing_pushable", rhino.isPushable());
            for (int i = 0; i < 42; i++) {
                prepare.tick();
            }
            result.put("preparing_counted_down", rhino.getChargeCooldownTicks());
            prepare.stop();
            result.put("done_preparing_pushable", rhino.isPushable());
            var blocked = Rhino.class.getDeclaredMethod(
                    "blockedByItem", LivingEntity.class, DamageSource.class, float.class, boolean.class);
            blocked.setAccessible(true);
            blocked.invoke(rhino, player, rhino.damageSources().mobAttack(rhino), 10.0F, true);
            var stunned = Rhino.class.getDeclaredField("stunnedTick");
            stunned.setAccessible(true);
            result.put("shield_stun_ticks", stunned.getInt(rhino));
            result.put("shield_charge_cooldown", rhino.getChargeCooldownTicks());
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException(e);
        } finally {
            rhino.discard();
        }
        try {
            result.putAll(naturalGroup(level));
            result.put("horn_registered", BuiltInRegistries.ITEM.containsKey(Identifier.parse("naturalist:rhino_horn")));
            result.putAll(chargeTerrain(level, player));
        } catch (ReflectiveOperationException failure) {
            throw new IllegalStateException(failure);
        }
        source.sendSuccess(() -> Component.literal("naturalist_rhino " + new Gson().toJson(result)), false);
        return 1;
    }

    private static Map<String, Object> naturalGroup(ServerLevel level) {
        var type = NaturalistEntityTypes.RHINO.get();
        var first = type.create(level, EntitySpawnReason.NATURAL);
        var second = type.create(level, EntitySpawnReason.NATURAL);
        var third = type.create(level, EntitySpawnReason.NATURAL);
        var difficulty = level.getCurrentDifficultyAt(new BlockPos(7, 71, 7));
        var group = first.finalizeSpawn(level, difficulty, EntitySpawnReason.NATURAL, null);
        group = second.finalizeSpawn(level, difficulty, EntitySpawnReason.NATURAL, group);
        third.finalizeSpawn(level, difficulty, EntitySpawnReason.NATURAL, group);
        return Map.of("natural_first_adult", !first.isBaby(), "natural_second_calf", second.isBaby(),
                "natural_third_calf", third.isBaby());
    }

    private static Map<String, Object> chargeTerrain(ServerLevel level, FakePlayer player) throws ReflectiveOperationException {
        for (var pos : BlockPos.betweenClosed(3, 68, 3, 14, 75, 14)) {
            var state = pos.getY() == 70 ? Blocks.DIRT.defaultBlockState() : Blocks.AIR.defaultBlockState();
            level.setBlockAndUpdate(pos, state);
        }
        var flower = new BlockPos(7, 71, 8);
        var lamp = new BlockPos(8, 71, 8);
        level.setBlockAndUpdate(flower, Blocks.DANDELION.defaultBlockState());
        level.setBlockAndUpdate(lamp, Blocks.TORCH.defaultBlockState());
        var rhino = new ProbeRhino(NaturalistEntityTypes.RHINO.get(), level);
        rhino.snapTo(7.5D, 71D, 7.5D, 0, 0);
        player.snapTo(7.5D, 71D, 12.5D, 0, 0);
        rhino.fixtureTarget = player;
        level.addFreshEntity(rhino);
        try {
            var type = Class.forName(Rhino.class.getName() + "$RhinoChargeGoal");
            var ctor = type.getDeclaredConstructor(Rhino.class, double.class);
            ctor.setAccessible(true);
            var goal = (Goal) ctor.newInstance(rhino, 2.5D);
            var target = new BlockPos(7, 71, 12);
            var path = type.getDeclaredField("path");
            path.setAccessible(true);
            path.set(goal, new Path(List.of(new Node(7, 71, 9), new Node(7, 71, 12)), target, true));
            goal.start();
            goal.tick();
            return Map.of("charge_removed_dandelion", level.getBlockState(flower).isAir(),
                    "charge_preserved_torch", level.getBlockState(lamp).is(Blocks.TORCH));
        } finally {
            rhino.discard();
        }
    }
}
