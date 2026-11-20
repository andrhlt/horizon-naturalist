package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.greatwhiteshark.GreatWhiteShark;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.core.BlockPos;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.Entity;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.Mob;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.entity.ai.goal.GoalSelector;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class SharkProbe {
    public SharkProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("shark_fixture").executes(ctx -> {
            var src = ctx.getSource();
            try {
                var level = src.getLevel();
                var dry = new BlockPos(8, 92, 6);
                var wet = new BlockPos(8, 92, 10);
                var values = new LinkedHashMap<String, Object>();
                var type = NaturalistEntityTypes.GREAT_WHITE_SHARK.get();
                var shark = create(level, dry);
                values.put("variant", shark.getVariantString());
                values.put("adult_dims", dim(shark));
                values.put("health", shark.getAttributeBaseValue(Attributes.MAX_HEALTH));
                values.put("speed", shark.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
                values.put("range", shark.getAttributeBaseValue(Attributes.FOLLOW_RANGE));
                values.put("attack", shark.getAttributeBaseValue(Attributes.ATTACK_DAMAGE));
                values.put("knockback_resistance", shark.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE));
                values.put("step", shark.getAttributeBaseValue(Attributes.STEP_HEIGHT));
                values.put("pushable", shark.isPushable());
                values.put("current_push", shark.isPushedByFluid());
                values.put("cluster", shark.getMaxSpawnClusterSize());
                values.put("part_width_height", Arrays.stream(shark.getMobParts())
                    .map(part -> new int[]{Math.round(part.getBbWidth() * 100), Math.round(part.getBbHeight() * 100)}).toList());
                shark.tick();
                values.put("part_offsets_z", Arrays.stream(shark.getMobParts())
                    .map(part -> Math.round((part.getZ() - shark.getZ()) * 100)).toList());
                var player = player(level);
                var items = List.of(Items.COD, Items.COOKED_COD, Items.SALMON, Items.COOKED_SALMON,
                    Items.TROPICAL_FISH, Items.PUFFERFISH, Items.BEEF, Items.TROPICAL_FISH_BUCKET);
                values.put("breeding_food", items.stream().map(item -> shark.isFood(new ItemStack(item))).toList());
                values.put("healing_amounts", items.stream().map(item -> {
                    shark.setHealth(60);
                    player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(item, 2));
                    shark.mobInteract(player, InteractionHand.MAIN_HAND);
                    return List.of(Math.round(shark.getHealth() - 60), player.getMainHandItem().getCount());
                }).toList());
                shark.setAge(-24000);
                values.put("baby_dims", dim(shark));
                values.put("no_offspring", shark.getBreedOffspring(level, shark) == null);
                shark.setAge(0);
                shark.setHealth(80);
                shark.setAirSupply(0);
                for (int n = 0; n < 19; n++) shark.baseTick();
                values.put("dry_air_19", shark.getAirSupply());
                values.put("dry_health_19", shark.getHealth());
                shark.baseTick();
                values.put("dry_air_20", shark.getAirSupply());
                values.put("dry_health_20", shark.getHealth());
                var contact = Entity.class.getDeclaredMethod("updateFluidInteraction");
                contact.setAccessible(true);
                shark.snapTo(wet.getX() + 0.5, wet.getY() + 0.5, wet.getZ() + 0.5, 0, 0);
                contact.invoke(shark);
                shark.baseTick();
                values.put("wet_air", shark.getAirSupply());
                var selection = Mob.class.getDeclaredField("targetSelector");
                selection.setAccessible(true);
                var targetGoal = ((GoalSelector) selection.get(shark)).getAvailableGoals().stream()
                    .filter(entry -> entry.getPriority() == 2).findFirst().orElseThrow().getGoal();
                var fish = EntityTypes.COD.create(level, EntitySpawnReason.COMMAND);
                fish.snapTo(9.5, 92.5, 10.5, 0, 0);
                level.addFreshEntity(fish);
                contact.invoke(fish);
                values.put("fish_target", takeTarget(shark, fish, targetGoal));
                shark.setTarget(null);
                shark.setHuntingCooldown(150);
                values.put("sated_no_target", !takeTarget(shark, fish, targetGoal));
                shark.setHuntingCooldown(0);
                fish.setHealth(1);
                shark.doHurtTarget(level, fish);
                values.put("prey_dead", !fish.isAlive());
                values.put("kill_cooldown", shark.getHuntingCooldown());
                shark.aiStep();
                values.put("cooldown_tick", shark.getHuntingCooldown());
                values.put("spawn_two_water", rule(level, type, wet));
                level.setBlockAndUpdate(wet.above(), Blocks.AIR.defaultBlockState());
                values.put("spawn_one_water", rule(level, type, wet));
                values.put("spawn_dry", rule(level, type, dry));
                player.discard();
                shark.discard();
                fish.discard();
                src.sendSuccess(() -> Component.literal("shark_fixture " + new Gson().toJson(values)), false);
            } catch (Exception error) {
                error.printStackTrace();
                src.sendSuccess(() -> Component.literal("shark_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static boolean rule(ServerLevel level, net.minecraft.world.entity.EntityType<GreatWhiteShark> type, BlockPos pos) {
        return GreatWhiteShark.checkGreatWhiteSharkSpawnRules(type, level, EntitySpawnReason.NATURAL, pos, level.getRandom());
    }

    private static boolean takeTarget(GreatWhiteShark shark, Mob target, net.minecraft.world.entity.ai.goal.Goal goal) {
        for (int chance = 0; chance < 100; chance++) {
            if (goal.canUse()) {
                goal.start();
                return shark.getTarget() == target;
            }
        }
        return false;
    }

    private static int[] dim(GreatWhiteShark shark) {
        return new int[]{Math.round(shark.getBbWidth() * 10000), Math.round(shark.getBbHeight() * 10000),
            Math.round(shark.getEyeHeight() * 10000)};
    }

    private static GreatWhiteShark create(ServerLevel level, BlockPos pos) {
        var shark = NaturalistEntityTypes.GREAT_WHITE_SHARK.get().create(level, EntitySpawnReason.COMMAND);
        shark.snapTo(pos.getX() + 0.5, pos.getY(), pos.getZ() + 0.5, 0, 0);
        shark.setNoAi(true);
        shark.setNoGravity(true);
        level.addFreshEntity(shark);
        return shark;
    }

    private static ServerPlayer player(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "SharkFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), new Connection(PacketFlow.SERVERBOUND), player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(10.5, 92, 6.5, 0, 0);
        level.addNewPlayer(player);
        player.setGameMode(GameType.SURVIVAL);
        return player;
    }
}
