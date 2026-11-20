package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.ai.goal.EggLayingBreedGoal;
import com.crispytwig.naturalist.world.entity.ai.goal.LayEggGoal;
import com.crispytwig.naturalist.world.entity.animal.alligator.Alligator;
import com.crispytwig.naturalist.world.level.block.AlligatorEggBlock;
import com.google.gson.Gson;
import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.BreedGoal;
import net.minecraft.world.entity.ai.goal.FollowParentGoal;
import net.minecraft.world.entity.ai.goal.MoveToBlockGoal;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.phys.AABB;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class AlligatorProbe {
    public AlligatorProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("alligator_fixture").executes(ctx -> {
            var source = ctx.getSource();
            try {
                var result = capture(source.getLevel());
                source.sendSuccess(() -> Component.literal("alligator_fixture " + new Gson().toJson(result)), false);
            } catch (Exception error) {
                error.printStackTrace();
                source.sendSuccess(() -> Component.literal("alligator_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static Map<String, Object> capture(ServerLevel level) throws Exception {
        var result = new LinkedHashMap<String, Object>();
        var runner = add(level, 6.5, 92, 5.5);
        runner.setAge(-24000);
        var parent = add(level, 14.75, 92, 5.5);
        var followCases = new LinkedHashMap<String, Object>();
        for (double distance : new double[]{2.75, 3.25, 8.25, 12.0}) {
            parent.snapTo(6.5 + distance, 92, 5.5, 0, 0);
            followCases.put(Double.toString(distance), new FollowParentGoal(runner, 1.2).canUse());
        }
        result.put("parent_follow_start", followCases);
        runner.discard();
        parent.discard();
        var egg = (AlligatorEggBlock) NaturalistRegistry.ALLIGATOR_EGG.get();
        var adult = add(level, 6.5, 92, 5.5);
        var baby = add(level, 8.5, 92, 5.5);
        baby.setAge(-24000);
        result.put("adult_dims", dimensions(adult));
        result.put("baby_dims", dimensions(baby));
        result.put("variant", adult.getVariantString());
        result.put("health", adult.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("speed", adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
        result.put("attack", adult.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
        result.put("knockback_resistance", adult.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE));
        result.put("step", adult.getAttributeBaseValue(Attributes.STEP_HEIGHT));
        result.put("head_yaw", adult.getMaxHeadYRot());
        result.put("cluster", adult.getMaxSpawnClusterSize());
        result.put("knockback", new int[]{knockback(adult, level), knockback(baby, level)});
        result.put("food", Arrays.stream(new net.minecraft.world.item.Item[]{
            Items.BEEF, Items.PORKCHOP, Items.CHICKEN, Items.RABBIT, Items.COD,
            Items.MUTTON, Items.COOKED_BEEF, Items.ROTTEN_FLESH
        }).map(item -> adult.isFood(new ItemStack(item))).toList());
        result.put("can_love", adult.canFallInLove());
        adult.setHasEgg(true);
        result.put("egg_stops_love", !adult.canFallInLove() && adult.isDefensive());
        adult.setHasEgg(false);
        adult.setHuntingCooldown(10);
        adult.aiStep();
        result.put("tick_cooldown", adult.getHuntingCooldown());
        var prey = EntityTypes.FROG.create(level, EntitySpawnReason.COMMAND);
        prey.snapTo(6.5, 92, 6.5, 0, 0);
        level.addFreshEntity(prey);
        prey.hurtServer(level, level.damageSources().mobAttack(adult), 50.0f);
        result.put("kill_cooldown", adult.getHuntingCooldown());

        var mate = add(level, 6.5, 92, 6.5);
        adult.setInLoveTime(600);
        mate.setInLoveTime(600);
        var breed = new EggLayingBreedGoal<>(adult, 1.0);
        field(BreedGoal.class, "partner").set(breed, mate);
        var breedMethod = EggLayingBreedGoal.class.getDeclaredMethod("breed");
        breedMethod.setAccessible(true);
        breedMethod.invoke(breed);
        result.put("parent_ages_after_breed", new int[]{adult.getAge(), mate.getAge()});
        result.put("bred", adult.hasEgg() && !mate.hasEgg() && !adult.isInLove() && !mate.isInLove());
        result.put("child_count_after_breed", level.getEntitiesOfClass(Alligator.class,
            new AABB(4, 90, 3, 14, 96, 12)).stream().filter(Alligator::isBaby).count());

        var substrate = new BlockPos(6, 91, 5);
        var site = substrate.above();
        level.setBlockAndUpdate(substrate, Blocks.MUD.defaultBlockState());
        adult.snapTo(6.5, 92, 5.5, 0, 0);
        adult.setDeltaMovement(Vec3.ZERO);
        var lay = new LayEggGoal<>(adult, 1.0);
        var valid = LayEggGoal.class.getDeclaredMethod("isValidTarget",
            net.minecraft.world.level.LevelReader.class, BlockPos.class);
        valid.setAccessible(true);
        var substrateResults = new ArrayList<Boolean>();
        for (var block : new net.minecraft.world.level.block.Block[]{
            Blocks.GRASS_BLOCK, Blocks.DIRT, Blocks.SAND, Blocks.MUD,
            Blocks.STONE, Blocks.RED_SAND, Blocks.COARSE_DIRT, Blocks.MUDDY_MANGROVE_ROOTS
        }) {
            level.setBlockAndUpdate(substrate, block.defaultBlockState());
            substrateResults.add((Boolean) valid.invoke(lay, level, substrate));
        }
        result.put("egg_substrates", substrateResults);
        level.setBlockAndUpdate(substrate, Blocks.MUD.defaultBlockState());
        field(MoveToBlockGoal.class, "blockPos").set(lay, substrate);
        adult.setHasEgg(true);
        int calls = 0;
        while (adult.hasEgg() && calls < 100) {
            lay.tick();
            calls++;
        }
        result.put("lay_goal_calls", calls);
        var laid = level.getBlockState(site);
        result.put("lay_block", laid.is(egg));
        result.put("lay_fresh", laid.is(egg) && laid.getValue(AlligatorEggBlock.HATCH) == 0);
        result.put("lay_eggs_in_1_4", laid.is(egg) &&
            laid.getValue(AlligatorEggBlock.EGGS) >= 1 && laid.getValue(AlligatorEggBlock.EGGS) <= 4);
        result.put("lay_post_love", adult.getInLoveTime());
        adult.aiStep();
        result.put("lay_post_love_after_ai", adult.getInLoveTime());
        result.put("lay_post_flags", new boolean[]{adult.hasEgg(), adult.isLayingEgg()});

        var hatchPos = new BlockPos(11, 92, 5);
        level.setBlockAndUpdate(hatchPos.below(), Blocks.STONE.defaultBlockState());
        level.setBlockAndUpdate(hatchPos, egg.defaultBlockState().setValue(AlligatorEggBlock.EGGS, 3));
        var stages = new ArrayList<Integer>();
        int last = 0;
        for (int attempt = 0; attempt < 3000; attempt++) {
            var state = level.getBlockState(hatchPos);
            if (!state.is(egg)) {
                stages.add(-1);
                break;
            }
            var stage = state.getValue(AlligatorEggBlock.HATCH);
            if (stage != last) {
                stages.add(stage);
                last = stage;
            }
            egg.randomTick(state, level, hatchPos, level.getRandom());
        }
        result.put("hatch_stages_stone", stages);
        var hatchlings = level.getEntitiesOfClass(Alligator.class, new AABB(hatchPos).inflate(1.0));
        result.put("hatch_baby_ages", hatchlings.stream().map(Alligator::getAge).sorted().toList());
        result.put("hatch_offsets", hatchlings.stream().map(h ->
            Math.round((h.getX() - hatchPos.getX()) * 100)).sorted().toList());

        var stomp = new BlockPos(12, 92, 5);
        var playerless = EntityTypes.COW.create(level, EntitySpawnReason.COMMAND);
        playerless.snapTo(12.5, 92, 5.5, 0, 0);
        var zombie = EntityTypes.ZOMBIE.create(level, EntitySpawnReason.COMMAND);
        var bat = EntityTypes.BAT.create(level, EntitySpawnReason.COMMAND);
        result.put("stomp_alligator", fallAttempts(level, egg, stomp, adult));
        result.put("stomp_zombie", fallAttempts(level, egg, stomp, zombie));
        result.put("stomp_bat", fallAttempts(level, egg, stomp, bat));
        result.put("stomp_cow", fallAttempts(level, egg, stomp, playerless));
        level.getGameRules().set(net.minecraft.world.level.gamerules.GameRules.MOB_GRIEFING,
            false, level.getServer());
        result.put("stomp_griefing_off", fallAttempts(level, egg, stomp, playerless));
        adult.discard();
        baby.discard();
        mate.discard();
        prey.discard();
        hatchlings.forEach(Alligator::discard);
        return result;
    }

    private static int[] dimensions(Alligator mob) {
        return new int[]{Math.round(mob.getBbWidth() * 10000), Math.round(mob.getBbHeight() * 10000),
            Math.round(mob.getEyeHeight() * 10000)};
    }

    private static int knockback(Alligator mob, ServerLevel level) {
        mob.setDeltaMovement(Vec3.ZERO);
        mob.knockback(0.4, 1.0, 0.0, level.damageSources().generic(), 0.0f, false);
        return (int) Math.round(Math.abs(mob.getDeltaMovement().x) * 1000);
    }

    private static boolean fallAttempts(ServerLevel level, AlligatorEggBlock egg, BlockPos pos,
        net.minecraft.world.entity.Entity entity) {
        var state = egg.defaultBlockState().setValue(AlligatorEggBlock.EGGS, 4);
        level.setBlockAndUpdate(pos, state);
        for (int tries = 0; tries < 300; tries++) {
            egg.fallOn(level, state, pos, entity, 1.3);
            if (level.getBlockState(pos) != state) return true;
        }
        return false;
    }

    private static Field field(Class<?> type, String name) throws Exception {
        var field = type.getDeclaredField(name);
        field.setAccessible(true);
        return field;
    }

    private static Alligator add(ServerLevel level, double x, double y, double z) {
        var mob = NaturalistEntityTypes.ALLIGATOR.get().create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(x, y, z, 0, 0);
        mob.setNoAi(true);
        mob.setNoGravity(true);
        level.addFreshEntity(mob);
        return mob;
    }
}
