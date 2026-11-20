package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.fish.Anglerfish;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.component.DataComponents;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.RandomSource;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.entity.ai.goal.MeleeAttackGoal;
import net.minecraft.world.entity.ai.goal.WrappedGoal;
import net.minecraft.world.item.component.CustomData;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class AnglerfishProbe {
    public AnglerfishProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("anglerfish_fixture").executes(ctx -> {
            var source = ctx.getSource();
            ServerLevel level = source.getLevel();
            try {
            var fish = NaturalistEntityTypes.ANGLERFISH.get().create(level, EntitySpawnReason.COMMAND);
            fish.snapTo(8.5, 92, 8.5, 0, 0);
            fish.setNoAi(true);
            level.addFreshEntity(fish);
            var attacker = EntityTypes.COD.create(level, EntitySpawnReason.COMMAND);
            attacker.snapTo(10.25, 92, 8.5, 0, 0);
            attacker.setNoAi(true);
            attacker.setHealth(2.0F);
            level.addFreshEntity(attacker);
            boolean hit = fish.hurtServer(level, level.damageSources().mobAttack(attacker), 1.0f);
            boolean retaliated = hit && fish.getTarget() == attacker;
            invokeMelee(fish, attacker);
            boolean extendedReach = !attacker.isAlive();
            int cooldown = fish.getHuntingCooldown();
            fish.aiStep();
            int nextCooldown = fish.getHuntingCooldown();

            String defaultVariant = fish.getVariantString();
            var dark = new BlockPos(8, 27, 8);
            var bright = new BlockPos(20, 27, 8);
            var middle = new BlockPos(8, 40, 8);
            var surface = new BlockPos(8, 54, 8);
            if (level.getRawBrightness(dark, 0) != 0 || level.getRawBrightness(bright, 0) == 0) {
                throw new IllegalStateException("Wrong lighting " + level.getRawBrightness(dark, 0) + " " + level.getRawBrightness(bright, 0));
            }
            boolean darkSpawn = allows(level, dark);
            boolean brightSpawn = allows(level, bright);
            boolean middleSpawn = allows(level, middle);
            boolean surfaceSpawn = allows(level, surface);
            var deepFish = NaturalistEntityTypes.ANGLERFISH.get().create(level, EntitySpawnReason.NATURAL);
            deepFish.snapTo(8.5, 27.0, 8.5, 0, 0);
            deepFish.finalizeSpawn(level, level.getCurrentDifficultyAt(dark), EntitySpawnReason.NATURAL, null);
            String deepVariant = deepFish.getVariantString();

            fish.setVariantString("naturalist:glow");
            var stack = fish.getBucketItemStack();
            fish.saveToBucketTag(stack);
            var data = stack.getOrDefault(DataComponents.BUCKET_ENTITY_DATA, CustomData.EMPTY).copyTag();
            var custom = stack.getOrDefault(DataComponents.CUSTOM_DATA, CustomData.EMPTY).copyTag();
            var released = NaturalistEntityTypes.ANGLERFISH.get().create(level, EntitySpawnReason.BUCKET);
            released.loadFromBucketTag(data);
            var legacy = data.copy();
            legacy.putInt("Variant", -1);
            var legacyFish = NaturalistEntityTypes.ANGLERFISH.get().create(level, EntitySpawnReason.BUCKET);
            legacyFish.loadFromBucketTag(legacy);
            String result = """
                {"max_health":%s,"movement_speed":%s,"attack_damage":%s,"default_variant":"%s",\
                "retaliated":%s,"extended_reach":%s,"cooldown":%s,"next_cooldown":%s,\
                "dark_spawn":%s,"bright_spawn":%s,"middle_spawn":%s,"surface_spawn":%s,"deep_variant":"%s",\
                "bucket_variant":"%s","custom_variant":"%s","released_variant":"%s","legacy_variant":"%s"}
                """.strip().formatted(fish.getAttributeBaseValue(Attributes.MAX_HEALTH),
                    fish.getAttributeBaseValue(Attributes.MOVEMENT_SPEED), fish.getAttributeBaseValue(Attributes.ATTACK_DAMAGE),
                    defaultVariant, retaliated, extendedReach, cooldown, nextCooldown, darkSpawn, brightSpawn, middleSpawn,
                    surfaceSpawn, deepVariant, data.getStringOr("Variant", ""), custom.getStringOr("Variant", ""),
                    released.getVariantString(), legacyFish.getVariantString());
            fish.discard();
            attacker.discard();
            source.sendSuccess(() -> Component.literal("anglerfish_fixture " + result), false);
            return 1;
            } catch (Exception e) {
                e.printStackTrace();
                source.sendSuccess(() -> Component.literal("anglerfish_fixture_error " + e), false);
                return 0;
            }
        }));
    }

    private static boolean allows(ServerLevel level, BlockPos pos) {
        return Anglerfish.checkAnglerfishSpawnRules(NaturalistEntityTypes.ANGLERFISH.get(), level,
                EntitySpawnReason.NATURAL, pos, RandomSource.create(0));
    }

    private static void invokeMelee(Anglerfish fish, LivingEntity target) {
        try {
            var goalField = Mob.class.getDeclaredField("goalSelector");
            goalField.setAccessible(true);
            var selector = (GoalSelector) goalField.get(fish);
            var melee = selector.getAvailableGoals().stream().map(WrappedGoal::getGoal)
                .filter(MeleeAttackGoal.class::isInstance).findFirst().orElseThrow();
            var attack = melee.getClass().getDeclaredMethod("checkAndPerformAttack", LivingEntity.class);
            attack.setAccessible(true);
            attack.invoke(melee, target);
        } catch (ReflectiveOperationException e) {
            throw new IllegalStateException(e);
        }
    }
}
