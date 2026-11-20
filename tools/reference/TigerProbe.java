package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.tiger.Tiger;
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
public final class TigerProbe {
    public TigerProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("tiger_fixture").executes(context -> {
            var source = context.getSource();
            try {
                var result = capture(source.getLevel());
                source.sendSuccess(() -> Component.literal("tiger_fixture " + new Gson().toJson(result)), false);
            } catch (Exception error) {
                error.printStackTrace();
                source.sendSuccess(() -> Component.literal("tiger_fixture_error " + error), false);
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
        var goalClass = Class.forName(Tiger.class.getName() + "$TigerStalkGoal");
        var constructor = goalClass.getDeclaredConstructor(Tiger.class);
        constructor.setAccessible(true);
        Goal stalk = (Goal) constructor.newInstance(adult);
        var start = new ArrayList<Boolean>();
        var ongoing = new ArrayList<Boolean>();
        for (double delta : new double[]{5.99, 6.0, 6.01, 11.99, 12.0, 12.01}) {
            pig.snapTo(6.5 + delta, 92, 5.5, 0f, 0f);
            start.add(stalk.canUse());
            ongoing.add(stalk.canContinueToUse());
        }
        out.put("stalk_start_by_distance", start);
        out.put("stalk_continue_by_distance", ongoing);
        stalk.start();
        boolean animStart = adult.isStalking();
        stalk.stop();
        out.put("stalking_flag", new boolean[]{animStart, adult.isStalking()});
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

    private static int[] dims(Tiger tiger) {
        return new int[]{Math.round(tiger.getBbWidth() * 10000),
            Math.round(tiger.getBbHeight() * 10000), Math.round(tiger.getEyeHeight() * 10000)};
    }

    private static Tiger add(ServerLevel level, double x, double y, double z) {
        var animal = NaturalistEntityTypes.TIGER.get().create(level, EntitySpawnReason.COMMAND);
        animal.snapTo(x, y, z, 0f, 0f);
        animal.setNoAi(true);
        animal.setNoGravity(true);
        level.addFreshEntity(animal);
        return animal;
    }
}
