package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.lion.Lion;
import com.google.gson.Gson;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.Goal;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class LionProbe {
    public LionProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("lion_fixture").executes(context -> {
            var source = context.getSource();
            try {
                var result = capture(source.getLevel());
                source.sendSuccess(() -> Component.literal("lion_fixture " + new Gson().toJson(result)), false);
            } catch (Exception error) {
                error.printStackTrace();
                source.sendSuccess(() -> Component.literal("lion_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static Map<String, Object> capture(ServerLevel level) throws Exception {
        var out = new LinkedHashMap<String, Object>();
        var adult = add(level, 6.5, 92, 5.5);
        var cub = add(level, 7.5, 92, 5.5);
        cub.setAge(-24000);
        out.put("adult_dims", dims(adult));
        out.put("baby_dims", dims(cub));
        out.put("default_variant", adult.getVariantString());
        out.put("attributes", new double[]{
            adult.getAttributeBaseValue(Attributes.MAX_HEALTH),
            adult.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
            adult.getAttributeBaseValue(Attributes.ATTACK_DAMAGE),
            adult.getAttributeBaseValue(Attributes.FOLLOW_RANGE),
            adult.getAttributeBaseValue(Attributes.STEP_HEIGHT),
            adult.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE)
        });
        out.put("cluster", adult.getMaxSpawnClusterSize());
        out.put("ambient_interval", adult.getAmbientSoundInterval());
        out.put("food", Arrays.stream(new net.minecraft.world.item.Item[]{
            Items.BEEF, Items.COOKED_MUTTON, Items.CHICKEN, Items.COOKED_CHICKEN, Items.RABBIT,
            Items.COD, Items.COOKED_SALMON, Items.ROTTEN_FLESH, Items.WHEAT, Items.GOLDEN_CARROT,
            NaturalistRegistry.BUSHMEAT.get(), NaturalistRegistry.COOKED_BUSHMEAT.get()
        }).map(item -> adult.isFood(new ItemStack(item))).toList());

        var items = net.minecraft.core.registries.BuiltInRegistries.ITEM;
        out.put("food_ids", items.stream().filter(item -> adult.isFood(new ItemStack(item)))
            .map(item -> items.getKey(item).toString()).sorted().toList());
        var sleep = new ArrayList<Boolean>();
        var times = new ArrayList<Long>();
        var clock = level.dimensionType().defaultClock().orElseThrow();
        for (int time : new int[]{5999, 6000, 6001, 12999, 13000, 13001, 18000, 32000}) {
            level.clockManager().setTotalTicks(clock, time);
            times.add(level.getDefaultClockTime());
            sleep.add(adult.canSleep());
        }
        out.put("sleep_clock", sleep);
        out.put("clock_readbacks", times);
        level.clockManager().setTotalTicks(clock, 8000);
        var foot = BlockPos.containing(adult.position());
        level.setBlockAndUpdate(foot, Blocks.WATER.defaultBlockState());
        out.put("sleep_in_water", adult.canSleep());
        level.removeBlock(foot, false);
        var pig = EntityTypes.PIG.create(level, EntitySpawnReason.COMMAND);
        pig.snapTo(20, 92, 5.5, 0f, 0f);
        adult.setTarget(pig);
        out.put("sleep_target", adult.canSleep());
        adult.setTarget(null);
        var owner = FakePlayerFactory.getMinecraft(level);
        cub.tame(owner);
        out.put("sleep_tamed", cub.canSleep());
        var cubMeal = new ItemStack(Items.BEEF, 2);
        owner.setItemInHand(InteractionHand.MAIN_HAND, cubMeal);
        var growth = cub.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("tamed_cub_grows", growth.consumesAction());
        out.put("tamed_cub_growth", new int[]{cub.getAge(), cubMeal.getCount()});
        adult.tame(owner);
        adult.setHealth(12);
        var meal = new ItemStack(Items.COOKED_MUTTON, 2);
        owner.setItemInHand(InteractionHand.MAIN_HAND, meal);
        out.put("owner_heal_consumed", adult.mobInteract(owner, InteractionHand.MAIN_HAND).consumesAction());
        out.put("owner_heal", new float[]{adult.getHealth(), meal.getCount()});
        owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
        adult.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("owner_sits", adult.isOrderedToSit());
        owner.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(NaturalistRegistry.WHISTLE.get()));
        adult.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("owner_whistle_stops_follow", !adult.isFollowingOwner());

        adult.setTame(false, false);
        adult.setOrderedToSit(false);
        adult.setSleeping(false);
        adult.setTarget(pig);
        var prideLead = add(level, 10.5, 92, 6.5);
        var prideOther = add(level, 11.5, 92, 6.5);
        adult.setUUID(new java.util.UUID(0L, 4L));
        prideLead.setUUID(new java.util.UUID(0L, 1L));
        prideOther.setUUID(new java.util.UUID(0L, 2L));
        prideLead.setHasMane(false);
        prideOther.setHasMane(false);
        adult.setHasMane(true);
        cub.setHasMane(false);
        var goalClass = Class.forName(Lion.class.getName() + "$LionFollowLeaderGoal");
        var constructor = goalClass.getDeclaredConstructor(Lion.class, double.class, float.class, float.class);
        constructor.setAccessible(true);
        Goal follow = (Goal) constructor.newInstance(adult, 1.1D, 8.0F, 24.0F);
        var leader = goalClass.getDeclaredField("followingMob");
        leader.setAccessible(true);
        var choices = new ArrayList<Long>();
        if (follow.canUse()) choices.add(((Lion) leader.get(follow)).getUUID().getLeastSignificantBits());
        prideLead.setInvisible(true);
        if (follow.canUse()) choices.add(((Lion) leader.get(follow)).getUUID().getLeastSignificantBits());
        prideOther.setHasMane(true);
        choices.add(follow.canUse() ? ((Lion) leader.get(follow)).getUUID().getLeastSignificantBits() : -1L);
        out.put("leader_choices", choices);
        prideLead.discard();
        prideOther.discard();

        cub.setAge(-24000);
        var speed = new ArrayList<Double>();
        speed.add(cub.getAttributeValue(Attributes.MOVEMENT_SPEED));
        cub.customServerAiStep(level);
        speed.add(cub.getAttributeValue(Attributes.MOVEMENT_SPEED));
        cub.setAge(0);
        cub.customServerAiStep(level);
        speed.add(cub.getAttributeValue(Attributes.MOVEMENT_SPEED));
        out.put("cub_speed", speed);

        var pos = new BlockPos(7, 92, 6);
        var groupSizes = new ArrayList<Integer>();
        var groupFirstMane = new ArrayList<Boolean>();
        var firstAdult = new ArrayList<Boolean>();
        int laterCubs = 0;
        var prideRolls = new ArrayList<java.util.List<java.util.List<Object>>>();
        var levelCubRolls = new ArrayList<java.util.List<Boolean>>();
        for (long seed = 1; seed <= 16; seed++) {
            net.minecraft.world.entity.SpawnGroupData group = null;
            var results = new ArrayList<java.util.List<Object>>();
            var levelRolls = new ArrayList<Boolean>();
            for (int index = 0; index < 5; index++) {
                var spawned = NaturalistEntityTypes.LION.get().create(level, EntitySpawnReason.NATURAL);
                spawned.snapTo(7.5, 92, 6.5, 0f, 0f);
                spawned.getRandom().setSeed(seed * 100 + index);
                long levelSeed = seed * 913 + (index * 19);
                level.getRandom().setSeed(levelSeed);
                var mirror = net.minecraft.util.RandomSource.create(levelSeed);
                mirror.nextInt(1); // The single original Lion variant's weight draw.
                levelRolls.add(index != 0 && mirror.nextFloat() <= 0.05F);
                group = spawned.finalizeSpawn(level, level.getCurrentDifficultyAt(pos), EntitySpawnReason.NATURAL, group);
                if (seed == 1) groupSizes.add(((net.minecraft.world.entity.AgeableMob.AgeableMobGroupData)group).getGroupSize());
                results.add(java.util.List.of(spawned.getAge(), spawned.hasMane()));
                if (index == 0) {
                    groupFirstMane.add(spawned.hasMane());
                    firstAdult.add(!spawned.isBaby());
                } else if (spawned.isBaby()) laterCubs++;
            }
            prideRolls.add(results);
            levelCubRolls.add(levelRolls);
        }
        out.put("pride_member_rolls", prideRolls);
        out.put("pride_level_cub_rolls", levelCubRolls);
        out.put("group_sizes", groupSizes);
        out.put("group_first_has_mane", groupFirstMane);
        out.put("group_first_adult", firstAdult);
        out.put("later_group_cubs", laterCubs);
        adult.setHuntingCooldown(2);
        var cooldown = new ArrayList<Integer>();
        for (int i = 0; i < 3; i++) {
            adult.customServerAiStep(level);
            cooldown.add(adult.getHuntingCooldown());
        }
        out.put("cooldown_countdown", cooldown);
        adult.killedEntity(level, pig, level.damageSources().mobAttack(adult));
        out.put("hunter_cooldown", adult.getHuntingCooldown());
        adult.discard();
        cub.discard();
        return out;
    }

    private static int[] dims(Lion lion) {
        return new int[]{Math.round(lion.getBbWidth() * 10000),
            Math.round(lion.getBbHeight() * 10000), Math.round(lion.getEyeHeight() * 10000)};
    }

    private static Lion add(ServerLevel level, double x, double y, double z) {
        var animal = NaturalistEntityTypes.LION.get().create(level, EntitySpawnReason.COMMAND);
        animal.snapTo(x, y, z, 0f, 0f);
        animal.setNoAi(true);
        animal.setNoGravity(true);
        level.addFreshEntity(animal);
        return animal;
    }
}
