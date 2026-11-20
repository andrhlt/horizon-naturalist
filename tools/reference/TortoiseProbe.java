package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.ai.goal.EggLayingBreedGoal;
import com.crispytwig.naturalist.world.entity.ai.goal.LayEggGoal;
import com.crispytwig.naturalist.world.entity.animal.tortoise.Tortoise;
import com.crispytwig.naturalist.world.level.block.TortoiseEggBlock;
import com.google.gson.Gson;
import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.Map;
import net.minecraft.world.InteractionHand;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.BreedGoal;
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
public final class TortoiseProbe {
    public TortoiseProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("tortoise_fixture").executes(ctx -> {
            var source = ctx.getSource();
            try {
                var result = capture(source.getLevel());
                source.sendSuccess(() -> Component.literal("tortoise_fixture " + new Gson().toJson(result)), false);
            } catch (Exception error) {
                error.printStackTrace();
                source.sendSuccess(() -> Component.literal("tortoise_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static Map<String, Object> capture(ServerLevel level) throws Exception {
        var result = new LinkedHashMap<String, Object>();
        var egg = (TortoiseEggBlock) NaturalistRegistry.TORTOISE_EGG.get();
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
        baby.discard();
        result.put("food", Arrays.stream(new net.minecraft.world.item.Item[]{
            Items.BAMBOO, Items.DANDELION, Items.BROWN_MUSHROOM, Items.CACTUS,
            Items.WHEAT, Items.RED_MUSHROOM, Items.MELON_SLICE, Items.SEAGRASS
        }).map(item -> adult.isFood(new ItemStack(item))).toList());
        var mate = add(level, 6.5, 92, 6.5);
        var player = FakePlayerFactory.getMinecraft(level);
        adult.setInLoveTime(600);
        mate.setInLoveTime(600);
        result.put("wild_mates", adult.canMate(mate));
        adult.setTame(false, false);
        result.put("loaded_wild_stats", stats(adult));
        adult.tame(player);
        mate.tame(player);
        result.put("tamed_stats", stats(adult));
        result.put("tame_mates", adult.canMate(mate));
        adult.setHealth(18);
        adult.setInLoveTime(0);
        var food = new ItemStack(Items.CACTUS, 2);
        player.setItemInHand(InteractionHand.MAIN_HAND, food);
        adult.mobInteract(player, InteractionHand.MAIN_HAND);
        result.put("owner_heals", Arrays.asList(adult.getHealth(), food.getCount()));
        player.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
        adult.mobInteract(player, InteractionHand.MAIN_HAND);
        result.put("owner_sits", adult.isOrderedToSit());
        adult.setOrderedToSit(false);
        player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(NaturalistRegistry.WHISTLE.get()));
        adult.mobInteract(player, InteractionHand.MAIN_HAND);
        result.put("owner_whistle_stops_follow", !adult.isFollowingOwner());
        adult.setInLoveTime(600);
        mate.setInLoveTime(0);
        result.put("can_love", mate.canFallInLove());
        mate.setHasEgg(true);
        result.put("egg_stops_love", !mate.canFallInLove());
        mate.setHasEgg(false);
        mate.setInLoveTime(600);

        var breed = new EggLayingBreedGoal<>(adult, 1.0);
        field(BreedGoal.class, "partner").set(breed, mate);
        var breedMethod = EggLayingBreedGoal.class.getDeclaredMethod("breed");
        breedMethod.setAccessible(true);
        breedMethod.invoke(breed);
        result.put("bred", adult.hasEgg() && !mate.hasEgg() && !adult.isInLove() && !mate.isInLove());
        result.put("child_count_after_breed", level.getEntitiesOfClass(Tortoise.class,
            new AABB(4, 90, 3, 14, 96, 12)).stream().filter(Tortoise::isBaby).count());

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
        adult.setVariantByLegacyIndex(2);
        int calls = 0;
        while (adult.hasEgg() && calls < 100) {
            lay.tick();
            calls++;
        }
        result.put("lay_goal_calls", calls);
        var laid = level.getBlockState(site);
        result.put("lay_block", laid.is(egg));
        result.put("lay_fresh", laid.is(egg) && laid.getValue(TortoiseEggBlock.HATCH) == 0);
        result.put("lay_eggs_in_1_4", laid.is(egg) &&
            laid.getValue(TortoiseEggBlock.EGGS) >= 1 && laid.getValue(TortoiseEggBlock.EGGS) <= 4);
        result.put("lay_variant", laid.getValue(TortoiseEggBlock.VARIANT));
        result.put("lay_post_love", adult.getInLoveTime());
        adult.aiStep();
        result.put("lay_post_love_after_ai", adult.getInLoveTime());
        result.put("lay_post_flags", new boolean[]{adult.hasEgg(), adult.isLayingEgg()});

        var hatchPos = new BlockPos(11, 92, 5);
        level.setBlockAndUpdate(hatchPos.below(), Blocks.STONE.defaultBlockState());
        level.setBlockAndUpdate(hatchPos, egg.defaultBlockState().setValue(TortoiseEggBlock.EGGS, 3).setValue(TortoiseEggBlock.VARIANT, 1));
        var stages = new ArrayList<Integer>();
        int last = 0;
        for (int attempt = 0; attempt < 3000; attempt++) {
            var state = level.getBlockState(hatchPos);
            if (!state.is(egg)) {
                stages.add(-1);
                break;
            }
            var stage = state.getValue(TortoiseEggBlock.HATCH);
            if (stage != last) {
                stages.add(stage);
                last = stage;
            }
            egg.randomTick(state, level, hatchPos, level.getRandom());
        }
        result.put("hatch_stages_stone", stages);
        var hatchlings = level.getEntitiesOfClass(Tortoise.class, new AABB(hatchPos).inflate(1.0));
        result.put("hatch_baby_variants", hatchlings.stream().map(Tortoise::getVariantString).sorted().toList());
        result.put("hatch_baby_tame", hatchlings.stream().map(Tortoise::isTame).toList());
        result.put("hatch_baby_ages", hatchlings.stream().map(Tortoise::getAge).sorted().toList());
        result.put("hatch_offsets", hatchlings.stream().map(h ->
            Math.round((h.getX() - hatchPos.getX()) * 100)).sorted().toList());

        var stomp = new BlockPos(12, 92, 5);
        var playerless = EntityTypes.COW.create(level, EntitySpawnReason.COMMAND);
        playerless.snapTo(12.5, 92, 5.5, 0, 0);
        var zombie = EntityTypes.ZOMBIE.create(level, EntitySpawnReason.COMMAND);
        var bat = EntityTypes.BAT.create(level, EntitySpawnReason.COMMAND);
        result.put("stomp_tortoise", fallAttempts(level, egg, stomp, adult));
        result.put("stomp_zombie", fallAttempts(level, egg, stomp, zombie));
        result.put("stomp_bat", fallAttempts(level, egg, stomp, bat));
        result.put("stomp_cow", fallAttempts(level, egg, stomp, playerless));
        level.getGameRules().set(net.minecraft.world.level.gamerules.GameRules.MOB_GRIEFING,
            false, level.getServer());
        result.put("stomp_griefing_off", fallAttempts(level, egg, stomp, playerless));
        adult.discard();
        mate.discard();
        hatchlings.forEach(Tortoise::discard);
        return result;
    }

    private static double[] stats(Tortoise actor) {
        return new double[]{actor.getAttributeBaseValue(Attributes.MAX_HEALTH),
            actor.getAttributeBaseValue(Attributes.ATTACK_DAMAGE), actor.getHealth()};
    }

    private static int[] dimensions(Tortoise mob) {
        return new int[]{Math.round(mob.getBbWidth() * 10000), Math.round(mob.getBbHeight() * 10000),
            Math.round(mob.getEyeHeight() * 10000)};
    }

    private static int knockback(Tortoise mob, ServerLevel level) {
        mob.setDeltaMovement(Vec3.ZERO);
        mob.knockback(0.4, 1.0, 0.0, level.damageSources().generic(), 0.0f, false);
        return (int) Math.round(Math.abs(mob.getDeltaMovement().x) * 1000);
    }

    private static boolean fallAttempts(ServerLevel level, TortoiseEggBlock egg, BlockPos pos,
        net.minecraft.world.entity.Entity entity) {
        var state = egg.defaultBlockState().setValue(TortoiseEggBlock.EGGS, 4);
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

    private static Tortoise add(ServerLevel level, double x, double y, double z) {
        var mob = NaturalistEntityTypes.TORTOISE.get().create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(x, y, z, 0, 0);
        mob.setNoAi(true);
        mob.setNoGravity(true);
        level.addFreshEntity(mob);
        return mob;
    }
}
