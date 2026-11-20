package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.snake.Snake;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.UUID;
import net.minecraft.network.Connection;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.level.GameType;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.network.chat.Component;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.util.RandomSource;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityReference;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.EquipmentSlot;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.item.ItemEntity;
import net.minecraft.world.effect.MobEffects;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.common.util.FakePlayerFactory;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SnakeProbe {
    public SnakeProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("snake_fixture").executes(context -> {
            var source = context.getSource();
            try {
                var result = capture(source.getLevel());
                source.sendSuccess(() -> Component.literal("snake_fixture " + new Gson().toJson(result)), false);
            } catch (Exception error) {
                error.printStackTrace();
                source.sendSuccess(() -> Component.literal("snake_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static Map<String, Object> capture(ServerLevel level) throws Exception {
        var out = new LinkedHashMap<String, Object>();
        var snake = NaturalistEntityTypes.SNAKE.get().create(level, EntitySpawnReason.COMMAND);
        snake.snapTo(6.5, 92, 5.5, 0f, 0f);
        snake.setNoAi(true);
        snake.setNoGravity(true);
        level.addFreshEntity(snake);
        out.put("default_variant", snake.getVariantString());
        out.put("adult_dims", dims(snake));
        snake.setAge(-24000);
        out.put("baby_dims", dims(snake));
        snake.setAge(0);
        out.put("attributes", new double[]{
            snake.getAttributeBaseValue(Attributes.MAX_HEALTH), snake.getAttributeBaseValue(Attributes.MOVEMENT_SPEED),
            snake.getAttributeBaseValue(Attributes.ATTACK_DAMAGE), snake.getAttributeBaseValue(Attributes.FOLLOW_RANGE),
            snake.getAttributeBaseValue(Attributes.STEP_HEIGHT)
        });
        out.put("cluster", snake.getMaxSpawnClusterSize());
        var volume = Snake.class.getDeclaredMethod("getSoundVolume");
        volume.setAccessible(true);
        out.put("voice_volume", volume.invoke(snake));
        var climb = Snake.class.getDeclaredMethod("getClimbSpeedMultiplier");
        climb.setAccessible(true);
        out.put("climbing_speed_multiplier", climb.invoke(snake));
        var items = BuiltInRegistries.ITEM;
        out.put("food_ids", items.stream().filter(item -> snake.isFood(new ItemStack(item)))
            .map(item -> items.getKey(item).toString()).sorted().toList());
        out.put("tame_ids", items.stream().filter(item -> snake.isTameFood(new ItemStack(item)))
            .map(item -> items.getKey(item).toString()).sorted().toList());
        out.put("has_offspring", snake.getBreedOffspring(level, snake) != null);

        var clock = level.dimensionType().defaultClock().orElseThrow();
        int[] boundaries = {12000,12001,18000,18001,22999,23000,27999,28000,36000};
        var sleep = new ArrayList<Boolean>();
        for (int time : boundaries) {
            level.clockManager().setTotalTicks(clock, time);
            sleep.add(snake.canSleep());
        }
        out.put("clock_readbacks", boundaries);
        out.put("sleep_clock", sleep);
        level.clockManager().setTotalTicks(clock, 13000);
        var pos = BlockPos.containing(snake.position());
        level.setBlockAndUpdate(pos, Blocks.WATER.defaultBlockState());
        out.put("sleep_in_water", snake.canSleep());
        level.removeBlock(pos, false);
        snake.setPersistentAngerEndTime(level.getGameTime() + 100);
        out.put("sleep_while_angry", snake.canSleep());
        snake.stopBeingAngry();

        var variants = List.of("naturalist:snake", "naturalist:coral_snake", "naturalist:rattlesnake");
        var venom = new ArrayList<Integer>();
        var bites = new ArrayList<Float>();
        var rattlesnake = new ArrayList<Boolean>();
        for (String variant : variants) {
            snake.setVariantString(variant);
            var prey = EntityTypes.COW.create(level, EntitySpawnReason.COMMAND);
            prey.snapTo(7.5, 92, 5.5, 0.0f, 0.0f);
            level.addFreshEntity(prey);
            snake.doHurtTarget(level, prey);
            var poison = prey.getEffect(MobEffects.POISON);
            venom.add(poison == null ? 0 : poison.getDuration());
            bites.add(prey.getHealth());
            rattlesnake.add(snake.isRattlesnake());
            prey.discard();
        }
        out.put("poison_duration", venom);
        out.put("post_bite_health", bites);
        out.put("rattlesnake", rattlesnake);

        var profile = new GameProfile(UUID.randomUUID(), "SnakeFixture");
        var nearby = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        nearby.connection = new ServerGamePacketListenerImpl(level.getServer(),
                new Connection(PacketFlow.SERVERBOUND), nearby, CommonListenerCookie.createInitial(profile, false));
        nearby.snapTo(8.5, 92, 5.5, 0, 0);
        level.addNewPlayer(nearby);
        nearby.setGameMode(GameType.CREATIVE);
        var rattle = Snake.class.getDeclaredMethod("canRattle");
        rattle.setAccessible(true);
        boolean creativeClose = (boolean) rattle.invoke(snake);
        boolean creativeNotTargeted = snake.getTarget() == null;
        var wall = new BlockPos(7, 92, 5);
        level.setBlockAndUpdate(wall, Blocks.STONE.defaultBlockState());
        level.setBlockAndUpdate(wall.above(), Blocks.STONE.defaultBlockState());
        boolean obstructed = (boolean) rattle.invoke(snake);
        level.removeBlock(wall, false);
        level.removeBlock(wall.above(), false);
        nearby.snapTo(12.5, 92, 5.5, 0, 0);
        boolean distant = (boolean) rattle.invoke(snake);
        nearby.discard();
        out.put("creative_rattle", List.of(creativeClose, creativeNotTargeted, obstructed, distant));

        var eat = Snake.class.getDeclaredMethod("handleEating");
        eat.setAccessible(true);
        var counter = Snake.class.getDeclaredMethod("getEatCounter");
        counter.setAccessible(true);
        var count = new ArrayList<Integer>();
        var hasItem = new ArrayList<Boolean>();
        snake.setSpeed(0.8f);
        out.put("speed_without_food", snake.getSpeed());
        snake.setItemSlot(EquipmentSlot.MAINHAND, new ItemStack(Items.CHICKEN, 3));
        out.put("speed_holding_food", snake.getSpeed());
        for (int tick = 1; tick <= 6001; tick++) {
            eat.invoke(snake);
            if (tick == 1 || tick == 5999 || tick == 6000 || tick == 6001) {
                count.add((Integer) counter.invoke(snake));
                hasItem.add(!snake.getMainHandItem().isEmpty());
            }
        }
        out.put("eat_counter", count);
        out.put("eat_main_hand", hasItem);

        snake.setItemSlot(EquipmentSlot.MAINHAND, new ItemStack(Items.CHICKEN, 3));
        snake.setPermanentlyInvulnerable(true);
        boolean hurtWhileInvulnerable = snake.hurtServer(level, level.damageSources().generic(), 1.0f);
        var spit = level.getEntitiesOfClass(ItemEntity.class, snake.getBoundingBox().inflate(3),
                item -> item.getItem().is(Items.CHICKEN)).getFirst();
        var delay = ItemEntity.class.getDeclaredField("pickupDelay");
        delay.setAccessible(true);
        var thrower = ItemEntity.class.getDeclaredField("thrower");
        thrower.setAccessible(true);
        var entityRef = (EntityReference<?>) thrower.get(spit);
        var look = snake.getLookAngle();
        boolean position = Math.abs(spit.getX() - snake.getX() - look.x) < 1e-6
                && Math.abs(spit.getY() - snake.getY() - 1.0) < 1e-6
                && Math.abs(spit.getZ() - snake.getZ() - look.z) < 1e-6;
        out.put("invulnerable_hurt_spit", List.of(hurtWhileInvulnerable, snake.getMainHandItem().isEmpty(),
                spit.getItem().getCount(), delay.getInt(spit), entityRef.getUUID().equals(snake.getUUID()), position));
        spit.discard();
        snake.setPermanentlyInvulnerable(false);

        var owner = FakePlayerFactory.getMinecraft(level);
        var tameRows = new ArrayList<List<Object>>();
        long[] seeds = {1, 42, 100, 4000, 5000, 9000, 14000, 18000, 22000, 36000};
        for (long seed : seeds) {
            snake.setTame(false, false);
            snake.setOrderedToSit(false);
            snake.getRandom().setSeed(seed);
            var egg = new ItemStack(Items.SPIDER_EYE, 2);
            owner.setItemInHand(InteractionHand.MAIN_HAND, egg);
            var result = snake.mobInteract(owner, InteractionHand.MAIN_HAND);
            tameRows.add(List.of(seed, result.consumesAction(), egg.getCount(), snake.isTame(), snake.isOrderedToSit()));
        }
        out.put("tame_rows", tameRows);
        snake.setTame(false, false);
        snake.setInLoveTime(0);
        var normalEgg = new ItemStack(Items.EGG, 2);
        owner.setItemInHand(InteractionHand.MAIN_HAND, normalEgg);
        snake.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("ordinary_egg_feeds_without_taming", List.of(normalEgg.getCount(), snake.isTame(), snake.isInLove()));
        snake.tame(owner);
        snake.setHealth(5);
        var egg = new ItemStack(Items.SPIDER_EYE, 2);
        owner.setItemInHand(InteractionHand.MAIN_HAND, egg);
        out.put("owner_heal_consumed", snake.mobInteract(owner, InteractionHand.MAIN_HAND).consumesAction());
        out.put("owner_heal", List.of(snake.getHealth(), egg.getCount()));
        snake.setOrderedToSit(false);
        owner.setItemInHand(InteractionHand.MAIN_HAND, ItemStack.EMPTY);
        snake.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("owner_sits", snake.isOrderedToSit());
        owner.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(NaturalistRegistry.WHISTLE.get()));
        snake.mobInteract(owner, InteractionHand.MAIN_HAND);
        out.put("owner_whistle_stops_follow", !snake.isFollowingOwner());

        var equipment = Snake.class.getDeclaredMethod("populateDefaultEquipmentSlots", RandomSource.class, net.minecraft.world.DifficultyInstance.class);
        equipment.setAccessible(true);
        var gear = new java.util.TreeMap<String, Integer>();
        for (long i = 0; i < 500; i++) {
            snake.setItemSlot(EquipmentSlot.MAINHAND, ItemStack.EMPTY);
            equipment.invoke(snake, RandomSource.create(i * 3337), level.getCurrentDifficultyAt(pos));
            var held = snake.getMainHandItem();
            String key = held.isEmpty() ? "empty" : items.getKey(held.getItem()).toString();
            gear.merge(key, 1, Integer::sum);
        }
        out.put("spawn_gear", gear);
        snake.discard();
        return out;
    }

    private static int[] dims(Snake snake) {
        return new int[]{Math.round(snake.getBbWidth() * 10000),
            Math.round(snake.getBbHeight() * 10000), Math.round(snake.getEyeHeight() * 10000)};
    }
}
