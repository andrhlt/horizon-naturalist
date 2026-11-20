package naturalist.fixture;

import com.crispytwig.naturalist.registry.NaturalistEntityTypes;
import com.crispytwig.naturalist.registry.NaturalistRegistry;
import com.crispytwig.naturalist.world.entity.animal.alligator.Alligator;
import com.google.gson.Gson;
import com.mojang.authlib.GameProfile;
import java.util.Map;
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
import net.minecraft.world.entity.EntitySpawnReason;
import net.minecraft.world.entity.EntityTypes;
import net.minecraft.world.entity.LivingEntity;
import net.minecraft.world.level.GameType;
import net.minecraft.world.level.block.Blocks;
import net.neoforged.fml.common.Mod;
import net.neoforged.neoforge.common.NeoForge;
import net.neoforged.neoforge.event.RegisterCommandsEvent;

@Mod("naturalist_fixture")
public final class AlligatorLiveProbe {
    private Alligator gator;
    private LivingEntity candidate;
    private ServerPlayer player;
    private java.lang.reflect.Method updateFluids;

    private void refreshPlayerFluids() {
        if (candidate != player) return;
        try {
            if (updateFluids == null) {
                updateFluids = net.minecraft.world.entity.Entity.class.getDeclaredMethod("updateFluidInteraction");
                updateFluids.setAccessible(true);
            }
            updateFluids.invoke(player);
        } catch (ReflectiveOperationException error) {
            throw new IllegalStateException(error);
        }
    }

    public AlligatorLiveProbe() {
        NeoForge.EVENT_BUS.addListener(this::register);
    }

    private void register(RegisterCommandsEvent event) {
        var command = Commands.literal("alligator_live");
        for (String phase : new String[]{"prey", "sated", "nest", "nest_three_away", "day_player",
                "dark_player", "pregnant_player", "water_player", "baby"}) {
            command.then(Commands.literal(phase).executes(ctx -> {
                start(ctx.getSource().getLevel(), phase);
                ctx.getSource().sendSuccess(() -> Component.literal("Ready " + phase), false);
                return 1;
            }));
        }
        command.then(Commands.literal("observe").executes(ctx -> {
            refreshPlayerFluids();
            var output = new Gson().toJson(Map.of(
                "targeted", candidate == gator.getTarget(),
                "wet", candidate.isInWater(),
                "injured", candidate.getHealth() < candidate.getMaxHealth()));
            ctx.getSource().sendSuccess(() -> Component.literal("alligator_live " + output), false);
            return 1;
        }));
        event.getDispatcher().register(command);
    }

    private void start(ServerLevel level, String phase) {
        if (gator != null) gator.discard();
        if (candidate != null) candidate.discard();
        var egg = new BlockPos(9, 116, 5);
        level.setBlockAndUpdate(egg, Blocks.AIR.defaultBlockState());
        gator = NaturalistEntityTypes.ALLIGATOR.get().create(level, EntitySpawnReason.COMMAND);
        gator.snapTo(phase.equals("water_player") ? 7.2 : 6.5, 116,
            phase.equals("water_player") ? 6.2 : 5.5, 0, 0);
        gator.setOnGround(true);
        gator.setNoGravity(true);
        gator.setPersistenceRequired();
        gator.setHasEgg(phase.equals("pregnant_player"));
        if (phase.equals("baby")) gator.setAge(-24000);
        if (phase.equals("sated")) gator.setHuntingCooldown(2400);
        if (phase.endsWith("_player")) {
            if (player == null || player.isRemoved()) {
                var profile = new GameProfile(UUID.randomUUID(), "GatorFixture");
                player = new ServerPlayer(level.getServer(), level, profile, ClientInformation.createDefault());
                player.connection = new ServerGamePacketListenerImpl(level.getServer(),
                    new Connection(PacketFlow.SERVERBOUND), player, CommonListenerCookie.createInitial(profile, false));
                level.addNewPlayer(player);
            }
            candidate = player;
            player.setGameMode(GameType.SURVIVAL);
            player.setHealth(player.getMaxHealth());
            player.setOnGround(true);
            player.snapTo(phase.equals("water_player") ? 7.5 : 6.5, phase.equals("water_player") ? 115.2 : 116, 7.15, 0, 0);
        } else {
            var mob = phase.startsWith("nest") ? EntityTypes.COW.create(level, EntitySpawnReason.COMMAND)
                : EntityTypes.FROG.create(level, EntitySpawnReason.COMMAND);
            mob.setNoAi(true);
            mob.setNoGravity(true);
            mob.setPersistenceRequired();
            var x = phase.equals("nest_three_away") ? 6.5 : 7.5;
            mob.snapTo(x, 116, 6.6, 0, 0);
            candidate = mob;
            level.addFreshEntity(mob);
        }
        if (phase.startsWith("nest")) {
            level.setBlockAndUpdate(egg, NaturalistRegistry.ALLIGATOR_EGG.get().defaultBlockState());
        }
        refreshPlayerFluids();
        level.addFreshEntity(gator);
    }
}
