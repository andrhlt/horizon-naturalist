package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.bear.Bear;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.ArrayList;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import net.minecraft.commands.CommandSourceStack;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.world.entity.ai.goal.MoveToBlockGoal;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.level.block.BeehiveBlock;
import net.minecraft.world.level.block.Blocks;
import net.minecraft.world.level.block.SweetBerryBushBlock;
import net.minecraft.world.level.block.entity.CampfireBlockEntity;
import net.minecraft.world.phys.AABB;
import net.minecraft.network.chat.Component;
import net.minecraft.util.ProblemReporter;
import net.minecraft.world.entity.EntityReference;
import net.minecraft.world.level.storage.TagValueInput;
import net.minecraft.world.level.storage.TagValueOutput;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.item.component.SwingAnimation;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityType;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.Pose;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayer;
import net.neoforged.neoforge.event.RegisterCommandsEvent;
import net.neoforged.neoforge.event.PlayLevelSoundEvent;

@Mod("naturalist_fixture")
public final class BearProbe {
    private static List<String> recordedSounds;

    public BearProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
        NeoForge.EVENT_BUS.addListener((PlayLevelSoundEvent.AtPosition event) -> {
            if (recordedSounds != null && event.getSound() != null) {
                recordedSounds.add(event.getSound().value().location().toString());
            }
        });
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("naturalist_fixture_bear")
                .executes(ctx -> capture(ctx.getSource())));
    }

    private static int capture(CommandSourceStack source) {
        ServerLevel level = source.getLevel();
        var bear = NaturalistEntityTypes.BEAR.get();
        var black = NaturalistEntityTypes.BLACK_BEAR.get();
        var result = Map.of("bear", capture(level, bear, black), "black_bear", capture(level, black, bear));
        source.sendSuccess(() -> Component.literal("naturalist_bear " + new Gson().toJson(result)), false);
        return 1;
    }

    private static Map<String, Object> capture(ServerLevel level, EntityType<? extends Bear> type,
            EntityType<? extends Bear> otherType) {
        var bear = type.create(level, EntitySpawnReason.COMMAND);
        var cub = type.create(level, EntitySpawnReason.COMMAND);
        var mate = type.create(level, EntitySpawnReason.COMMAND);
        var other = otherType.create(level, EntitySpawnReason.COMMAND);
        cub.setAge(-24000);
        var player = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "BearFixture"));
        var hand = InteractionHand.MAIN_HAND;
        var result = new LinkedHashMap<String, Object>();
        result.put("variant", bear.getVariantString());
        result.putAll(voices(level, type));
        result.put("adult", size(bear));
        result.put("juvenile", size(cub));
        result.put("max_health", bear.getAttributeBaseValue(Attributes.MAX_HEALTH));
        result.put("movement_speed", bear.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
        result.put("attack_damage", bear.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
        result.put("knockback_resistance", bear.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE));
        result.put("follow_range", bear.getAttributeBaseValue(Attributes.FOLLOW_RANGE));
        result.put("step_height", bear.getAttributeBaseValue(Attributes.STEP_HEIGHT));
        var items = Map.of("salmon", Items.SALMON, "cooked_salmon", Items.COOKED_SALMON,
                "honeycomb", Items.HONEYCOMB, "sweet_berries", Items.SWEET_BERRIES,
                "venison", NaturalistRegistry.VENISON.get(), "beef", Items.BEEF,
                "honey_bottle", Items.HONEY_BOTTLE);
        result.put("food", items.entrySet().stream().filter(entry -> bear.isFood(new ItemStack(entry.getValue())))
                .map(Map.Entry::getKey).sorted().toList());
        var food = new ItemStack(Items.HONEYCOMB, 2);
        player.setItemInHand(hand, food.copy());
        cub.getRandom().setSeed(1L);
        cub.mobInteract(player, hand);
        result.put("cub_tamed", cub.isOwnedBy(player));
        result.put("tame_consumed", player.getItemInHand(hand).getCount() == 1);
        cub.setHealth(4F);
        player.setItemInHand(hand, food.copy());
        cub.mobInteract(player, hand);
        result.put("healed_health", cub.getHealth());
        result.put("heal_consumed", player.getItemInHand(hand).getCount() == 1);
        player.setItemInHand(hand, food.copy());
        bear.mobInteract(player, hand);
        result.put("adult_wont_tame", !bear.isTame());
        mate.setInLove(player);
        other.setInLove(player);
        result.put("same_species_mate", bear.canMate(mate));
        result.put("cross_species_mate", bear.canMate(other));
        bear.snapTo(5.5D, 71D, 6.5D, 0.0F, 0.0F);
        result.put("can_sleep_at_fixture_time", bear.canSleep());
        var output = TagValueOutput.createWithContext(ProblemReporter.DISCARDING, level.registryAccess());
        long end = level.getGameTime() + 500L;
        bear.setPersistentAngerEndTime(end);
        bear.setPersistentAngerTarget(EntityReference.of(player.getUUID()));
        bear.saveWithoutId(output);
        var saved = output.buildResult();
        result.put("anger_saves_target", saved.contains("angry_at"));
        result.put("anger_saves_end_time", saved.getLongOr("anger_end_time", -1) == end);
        var loaded = type.create(level, EntitySpawnReason.COMMAND);
        loaded.load(TagValueInput.create(ProblemReporter.DISCARDING, level.registryAccess(), saved));
        var target = loaded.getPersistentAngerTarget();
        result.put("anger_loads_same_target", target != null && target.getUUID().equals(player.getUUID()));
        result.put("anger_loads_same_end_time", loaded.getPersistentAngerEndTime() == end);
        player.setItemInHand(hand, ItemStack.EMPTY);
        var otherPlayer = new FakePlayer(level, new GameProfile(UUID.randomUUID(), "OtherBearFixture"));
        player.setPermanentlyInvulnerable(false);
        otherPlayer.setPermanentlyInvulnerable(false);
        result.put("loaded_anger_targets_matching_player", loaded.isAngryAt(player, level));
        result.put("loaded_anger_targets_other_player", loaded.isAngryAt(otherPlayer, level));
        var pig = EntityTypes.PIG.create(level, EntitySpawnReason.COMMAND);
        var cow = EntityTypes.COW.create(level, EntitySpawnReason.COMMAND);
        pig.snapTo(8.5, 71.0, 9.5, 0, 0);
        level.addFreshEntity(pig);
        try {
            bear.setPersistentAngerTarget(EntityReference.of(pig));
            var mobAnger = TagValueOutput.createWithContext(ProblemReporter.DISCARDING, level.registryAccess());
            bear.saveWithoutId(mobAnger);
            loaded.load(TagValueInput.create(ProblemReporter.DISCARDING, level.registryAccess(), mobAnger.buildResult()));
            result.put("loaded_anger_targets_matching_mob", loaded.getTarget() == pig && loaded.isAngryAt(pig, level));
            result.put("loaded_anger_targets_other_mob", loaded.isAngryAt(cow, level));
        } finally {
            pig.discard();
        }
        var shears = new ItemStack(Items.SHEARS);
        result.put("adult_shearable", bear.isShearable(player, shears, level, bear.blockPosition()));
        result.put("cub_shearable", cub.isShearable(player, shears, level, cub.blockPosition()));
        var fur = bear.onSheared(player, shears, level, bear.blockPosition());
        result.put("sheared", bear.isSheared());
        result.put("fur_amount", fur.size());
        result.put("drops_fur", fur.stream().allMatch(stack -> stack.is(NaturalistRegistry.FUR.get())));
        try {
            result.putAll(harvest(level, bear));
        } catch (ReflectiveOperationException failure) {
            throw new IllegalStateException(failure);
        }
        return result;
    }

    private static Map<String, Object> voices(ServerLevel level, EntityType<? extends Bear> type) {
        var voiceBear = type.create(level, EntitySpawnReason.COMMAND);
        voiceBear.snapTo(8.5, 71.0, 8.5, 0, 0);
        var sounds = new ArrayList<String>();
        recordedSounds = sounds;
        try {
            voiceBear.swing(InteractionHand.MAIN_HAND, SwingAnimation.DEFAULT, false);
            voiceBear.hurtServer(level, level.damageSources().generic(), 1.0F);
            var dying = type.create(level, EntitySpawnReason.COMMAND);
            dying.snapTo(8.5, 71.0, 8.5, 0, 0);
            dying.hurtServer(level, level.damageSources().generic(), 99.0F);
        } finally {
            recordedSounds = null;
        }
        var result = new LinkedHashMap<String, Object>();
        result.put("swing_sound", findSound(sounds, ".attack"));
        result.put("hurt_sound", findSound(sounds, ".hurt"));
        result.put("death_sound", findSound(sounds, ".death"));
        return result;
    }

    private static String findSound(List<String> sounds, String suffix) {
        return sounds.stream().filter(id -> id.endsWith(suffix)).findFirst().orElse(null);
    }

    private static Map<String, Object> harvest(ServerLevel level, Bear bear) throws ReflectiveOperationException {
        var goalType = Class.forName(Bear.class.getName() + "$BearHarvestFoodGoal");
        var constructor = goalType.getDeclaredConstructor(Bear.class, double.class, int.class, int.class);
        constructor.setAccessible(true);
        var goal = constructor.newInstance(bear, 1.2D, 12, 3);
        var target = MoveToBlockGoal.class.getDeclaredField("blockPos");
        target.setAccessible(true);
        var harvest = goalType.getDeclaredMethod("onReachedTarget");
        harvest.setAccessible(true);
        var berries = new BlockPos(5, 71, 6);
        var hive = new BlockPos(5, 71, 8);
        var campfire = new BlockPos(5, 71, 10);
        for (var pos : List.of(berries, hive, campfire)) {
            level.setBlockAndUpdate(pos.below(), Blocks.DIRT.defaultBlockState());
            level.getEntitiesOfClass(ItemEntity.class, new AABB(pos).inflate(1.0D)).forEach(ItemEntity::discard);
        }
        level.setBlockAndUpdate(berries, Blocks.SWEET_BERRY_BUSH.defaultBlockState().setValue(SweetBerryBushBlock.AGE, 3));
        level.setBlockAndUpdate(hive, Blocks.BEEHIVE.defaultBlockState().setValue(BeehiveBlock.HONEY_LEVEL, 5));
        level.setBlockAndUpdate(campfire, Blocks.CAMPFIRE.defaultBlockState());
        var fire = (CampfireBlockEntity) level.getBlockEntity(campfire);
        fire.getItems().set(0, new ItemStack(Items.SALMON));
        fire.getItems().set(1, new ItemStack(Items.BEEF));
        target.set(goal, berries);
        harvest.invoke(goal);
        target.set(goal, hive);
        harvest.invoke(goal);
        target.set(goal, campfire);
        harvest.invoke(goal);
        return Map.of(
            "harvest_berry_age", level.getBlockState(berries).getValue(SweetBerryBushBlock.AGE),
            "harvest_berries_dropped", level.getEntitiesOfClass(ItemEntity.class, new AABB(berries).inflate(1.0D))
                    .stream().anyMatch(item -> item.getItem().is(Items.SWEET_BERRIES)),
            "harvest_honey_level", level.getBlockState(hive).getValue(BeehiveBlock.HONEY_LEVEL),
            "harvest_honeycomb_dropped", level.getEntitiesOfClass(ItemEntity.class, new AABB(hive).inflate(1.0D))
                    .stream().anyMatch(item -> item.getItem().is(Items.HONEYCOMB)),
            "harvest_salmon_stolen", fire.getItems().get(0).isEmpty(),
            "harvest_beef_left", fire.getItems().get(1).is(Items.BEEF));
    }

    private static List<Float> size(Bear bear) {
        var shape = bear.getDimensions(Pose.STANDING);
        return List.of(shape.width(), shape.height(), shape.eyeHeight());
    }
}
