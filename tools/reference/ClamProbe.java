package naturalist.fixture;


import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.world.entity.animal.clam.Clam;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.lang.reflect.Field;
import java.lang.reflect.Method;
import java.util.LinkedHashMap;
import java.util.UUID;
import net.minecraft.commands.Commands;
import net.minecraft.network.Connection;
import net.minecraft.network.chat.Component;
import net.minecraft.network.protocol.PacketFlow;
import net.minecraft.server.level.ClientInformation;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.server.network.CommonListenerCookie;
import net.minecraft.server.network.ServerGamePacketListenerImpl;
import net.minecraft.world.InteractionHand;
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.ai.attributes.Attributes;
import net.minecraft.world.item.ItemStack;
import net.minecraft.world.item.Items;
import net.minecraft.world.level.GameType;
import net.minecraft.world.phys.Vec3;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class ClamProbe {
    private static final Gson GSON = new Gson();

    public ClamProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        event.getDispatcher().register(Commands.literal("clam_fixture").executes(ctx -> {
            var source = ctx.getSource();
            var level = source.getLevel();
            try {
                var ai = Clam.class.getDeclaredMethod("customServerAiStep", ServerLevel.class);
                ai.setAccessible(true);
                var timer = field("stateTimer");
                var cooldown = field("snapCooldown");
                var delay = field("launchPushDelay");
                var player = newPlayer(level);
                var clam = newClam(level);
                var result = new LinkedHashMap<String, Object>();
                result.put("health", clam.getAttributeBaseValue(Attributes.MAX_HEALTH));
                result.put("knockback_resistance", clam.getAttributeBaseValue(Attributes.KNOCKBACK_RESISTANCE));
                result.put("speed", clam.getAttributeBaseValue(Attributes.MOVEMENT_SPEED));
                result.put("variant", clam.getVariantString());
                result.put("cluster", clam.getMaxSpawnClusterSize());
                result.put("pushable", clam.isPushable());
                result.put("initial_closed", !clam.isOpen());
                result.put("initial_no_treasure", !clam.hasTreasure());
                result.put("closed_rejects_hit", !clam.hurtServer(level, level.damageSources().generic(), 4));
                var drownTest = newClam(level);
                result.put("closed_accepts_drowning", drownTest.hurtServer(level, level.damageSources().drown(), 2));
                drownTest.discard();

                player.setShiftKeyDown(true);
                ai.invoke(clam, level);
                result.put("sneak_stays_closed", !clam.isOpen());
                player.setShiftKeyDown(false);
                ai.invoke(clam, level);
                result.put("near_opens", clam.isOpen());
                result.put("launch_delay", delay.getInt(clam));
                result.put("launch_cooldown", cooldown.getInt(clam));
                result.put("opens_timer_range", timer.getInt(clam) >= 100 && timer.getInt(clam) <= 200);
                player.setDeltaMovement(Vec3.ZERO);
                for (int i = 0; i < 3; i++) ai.invoke(clam, level);
                result.put("before_launch", player.getDeltaMovement().y);
                ai.invoke(clam, level);
                result.put("launch_vertical", player.getDeltaMovement().y);
                result.put("open_accepts_hit", clam.hurtServer(level, level.damageSources().generic(), 4));

                player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(Items.ENDER_PEARL, 3));
                clam.mobInteract(player, InteractionHand.MAIN_HAND);
                result.put("inserted", clam.hasTreasure() && clam.getMainHandItem().is(Items.ENDER_PEARL)
                    && clam.getMainHandItem().getCount() == 1 && player.getMainHandItem().getCount() == 2);
                result.put("stored_persistent", clam.requiresCustomPersistence());
                clam.mobInteract(player, InteractionHand.MAIN_HAND);
                result.put("returned", !clam.hasTreasure() && clam.getMainHandItem().isEmpty()
                    && player.getInventory().countItem(Items.ENDER_PEARL) == 3);
                result.put("withdraw_closes", !clam.isOpen());
                result.put("closed_timer_range", timer.getInt(clam) >= 200 && timer.getInt(clam) <= 400);

                player.setGameMode(GameType.CREATIVE);
                player.setItemInHand(InteractionHand.MAIN_HAND, new ItemStack(Items.DEBUG_STICK));
                clam.mobInteract(player, InteractionHand.MAIN_HAND);
                result.put("creative_debug_opens", clam.isOpen());
                result.put("creative_debug_timer", timer.getInt(clam));
                result.put("creative_debug_cooldown", cooldown.getInt(clam));

                player.setGameMode(GameType.SURVIVAL);
                clam.setHasTreasure(true);
                clam.setItemSlot(net.minecraft.world.entity.EquipmentSlot.MAINHAND, new ItemStack(Items.ENDER_PEARL));
                cooldown.setInt(clam, 0);
                player.snapTo(8.5, 92, 5.5, 0, 0);
                ai.invoke(clam, level);
                result.put("approach_slams_treasure", !clam.isOpen() && clam.hasTreasure());
                result.put("slam_no_launch", delay.getInt(clam) == 0);
                clam.discard();
                player.discard();

                source.sendSuccess(() -> Component.literal("clam_fixture " + GSON.toJson(result)), false);
            } catch (Exception error) {
                error.printStackTrace();
                source.sendSuccess(() -> Component.literal("clam_fixture_error " + error), false);
            }
            return 1;
        }));
    }

    private static Field field(String name) throws Exception {
        var field = Clam.class.getDeclaredField(name);
        field.setAccessible(true);
        return field;
    }

    private static Clam newClam(ServerLevel level) {
        var mob = NaturalistEntityTypes.CLAM.get().create(level, EntitySpawnReason.COMMAND);
        mob.snapTo(8.5, 92, 8.5, 0, 0);
        mob.setNoAi(true);
        mob.setNoGravity(true);
        level.addFreshEntity(mob);
        return mob;
    }

    private static ServerPlayer newPlayer(ServerLevel level) {
        var profile = new GameProfile(UUID.randomUUID(), "ClamFixture");
        var player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
        var connection = new Connection(PacketFlow.SERVERBOUND);
        player.connection = new ServerGamePacketListenerImpl(level.getServer(), connection, player,
            CommonListenerCookie.createInitial(profile, false));
        player.snapTo(8.5, 92, 7.5, 0, 0);
        level.addNewPlayer(player);
        return player;
    }
}
